use chrono::{Datelike, Timelike, Utc};
use input::Input;
use rand::{RngCore, SeedableRng};
use replay::Replay;
use std::{
    io::{BufRead, Write},
    path::Path,
    time::{Duration, Instant},
};

use crate::{
    bag::Bag,
    board::{Board, TickResult},
    config::Config,
    display::Colour,
    replay::Recorder,
    score::Score,
};

mod bag;
mod board;
mod config;
mod display;
mod get_input;
mod input;
mod kicks;
mod piece;
mod point;
mod replay;
mod score;

fn time_format(duration: Duration) -> String {
    let ms = duration.as_millis() % 1000;
    let s = duration.as_secs() % 60;
    let m = duration.as_secs() / 60;

    format!("Time: {:0w$}:{:0w$}.{:0w_ms$}", m, s, ms, w = 2, w_ms = 3)
}

fn print_score(score: Score, duration: Duration) {
    println!(" Game Results");
    println!("--------------------");
    println!(" Score: {}", score.score());
    println!(" Lines: {}", score.lines());
    println!(" {}", time_format(duration));
}

fn play_game(conf: Config) -> (Score, Recorder, Duration) {
    let start = Instant::now();
    let seed = {
        let mut seed: [u8; 32] = [0; 32];
        let mut rng = rand::rngs::SmallRng::from_entropy();

        rng.fill_bytes(&mut seed);

        seed
    };

    let mut score = Score::new();
    let mut last_update: u128 = 0;
    let mut board = Board::new(Bag::new(seed));
    let mut recorder = Recorder::new(seed, start.elapsed().as_millis());

    display::clear_terminal();

    'game_loop: loop {
        let duration = start.elapsed();
        let now = duration.as_millis();

        if now != last_update {
            let input = if let Ok(input) = get_input::get_input(conf) {
                input
            } else {
                break 'game_loop;
            };

            let tick = board.tick(input, now);
            recorder.record(input, now);

            if tick == TickResult::GameOver {
                break 'game_loop;
            } else {
                score.update(tick);
            }

            last_update = now;

            if now % conf.frame_time as u128 == 0 {
                board
                    .to_screen_buffer()
                    .write_string(26, 16, &format!("Score: {}", score.score()), Colour::White)
                    .write_string(26, 18, &format!("Lines: {}", score.lines()), Colour::White)
                    .write_string(26, 20, &time_format(duration), Colour::White)
                    .print();
            }
        }
    }

    (score, recorder, start.elapsed())
}

fn save_replay_prompt(recorder: Recorder) {
    let folder = Config::folder();
    let replay_folder = folder.clone() + if cfg!(windows) { r"replay\" } else { "replay/" };

    print!(" Please enter a name to save replay: ");
    std::io::stdout().flush().unwrap();

    let stdin = std::io::stdin();

    if let Some(line) = stdin.lock().lines().next() {
        if let Ok(name) = line {
            if name.len() > 0 {
                let file_path = if replay_folder.len() > 0 {
                    let dir = Path::new(&replay_folder);

                    if !dir.is_dir() {
                        if let Err(err) = std::fs::create_dir_all(dir) {
                            println!(
                                "Unable to create directory {}, saving replay failed.",
                                replay_folder
                            );
                            println!("{}", err);

                            return;
                        }
                    }

                    let now = Utc::now();
                    let time_stamp = format!(
                        "{}-{:02}-{:02}T{:02}{:02}{:02}",
                        now.year(),
                        now.month(),
                        now.day(),
                        now.hour(),
                        now.minute(),
                        now.second()
                    );

                    format!("{}{}-{}", replay_folder, name, time_stamp)
                } else {
                    panic!("Unable to find home directory and current directory");
                };

                if let Err(err) = std::fs::write(file_path.clone(), recorder.raw()) {
                    println!(" Failed to save to: {}", file_path);
                    println!("{}", err);
                } else {
                    println!(" Replay saved to: {}", file_path);
                }

                return;
            }
        }
    }

    println!("Replay discarded");
}

fn re_play_game(conf: Config, filename: &str) -> Result<(Score, Duration), ()> {
    let folder = Config::folder();
    let replay_folder = folder.clone() + if cfg!(windows) { r"replay\" } else { "replay/" };
    let path = format!("{}{}", replay_folder, filename);

    let buf = match std::fs::read(path.clone()) {
        Ok(contents) => contents,
        Err(err) => {
            print!("Unable to read replay file at \"{}\"", path);
            print!("{}", err);
            return Err(());
        }
    };

    let mut recording = Replay::new(buf);

    let start = Instant::now();
    let seed = recording.seed();

    let mut score = Score::new();
    let mut board = Board::new(Bag::new(seed));
    let mut last_update: u128 = 0;
    let mut next_input = recording.next().unwrap();

    display::clear_terminal();

    'game_loop: loop {
        let duration = start.elapsed();
        let now = duration.as_millis();

        if now != last_update {
            if let Err(()) = get_input::get_input(conf) {
                println!("--------------------");
                println!("Cancelling replay playback");
                println!("--------------------");

                return Err(());
            }

            let input = if now >= next_input.time {
                next_input.input
            } else {
                Input::default()
            };

            let tick = board.tick(input, now);

            if tick == TickResult::GameOver {
                break 'game_loop;
            } else {
                score.update(tick);
            }

            if now % conf.frame_time as u128 == 0 {
                board
                    .to_screen_buffer()
                    .write_string(26, 16, &format!("Score: {}", score.score()), Colour::White)
                    .write_string(26, 18, &format!("Lines: {}", score.lines()), Colour::White)
                    .write_string(26, 20, &time_format(duration), Colour::White)
                    .write_string(26, 22, &format!("Replay: {}", filename), Colour::Grey)
                    .print();
            }

            last_update = now;

            if now >= next_input.time {
                if let Some(next) = recording.next() {
                    next_input = next;
                }
            }
        }
    }

    Ok((score, start.elapsed()))
}

fn main() {
    let folder = Config::folder();
    let conf_file = folder.clone() + "config";
    let conf = Config::from_file(&conf_file);

    if let Some(filename) = std::env::args().nth(1) {
        if let Ok((score, duration)) = re_play_game(conf, &filename) {
            println!("--------------------");
            print_score(score, duration);
            println!("--------------------");
        }
    } else {
        let (score, recorder, duration) = play_game(conf);

        println!("--------------------");
        print_score(score, duration);
        println!("--------------------");
        save_replay_prompt(recorder);
        println!("--------------------");
        println!(" Thanks for playing");
    }
}
