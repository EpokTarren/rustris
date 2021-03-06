use crate::{config::Config, get_input::get_input};
use chrono::{Datelike, Timelike, Utc};
use core::{Bag, Board, Colour, Game, GameType, Input, Recorder, Replay, Score, TickType};
use display::ScreenBuffer;
use rand::{RngCore, SeedableRng};
use std::{
    io::{BufRead, Write},
    path::Path,
    time::{Duration, Instant},
};

mod config;
mod display;
mod get_input;

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

fn game_loop<InputFn: FnMut(u128) -> Input, DisplayFn: FnMut(&Board, &Score, &Duration)>(
    input: &mut InputFn,
    display: &mut DisplayFn,
    frame_time: u128,
    mut game: Game,
) -> (Score, Duration) {
    let start = Instant::now();

    let mut last_update: u128 = 0;

    'game_loop: loop {
        let duration = start.elapsed();
        let now = duration.as_millis();

        if now == last_update {
            continue;
        }

        let tick = game.tick(input(now), now);

        if tick.kind() == TickType::GameOver {
            break 'game_loop;
        }

        last_update = now;

        if now % frame_time as u128 == 0 {
            display(game.board(), &game.score(), &duration);
        }
    }

    (game.score(), start.elapsed())
}

fn play_game(conf: Config) -> (Score, Recorder, Duration) {
    let seed = {
        let mut seed = [0u8; 8];
        let mut rng = rand::rngs::SmallRng::from_entropy();

        rng.fill_bytes(&mut seed);

        u64::from_be_bytes(seed)
    };
    let game_type = GameType::new_lines(40);
    let game = Game::new(Bag::new(seed), game_type);
    let mut recorder = Recorder::new(seed, 0, &game);

    display::clear_terminal();

    let mut input = |now| {
        let input = get_input(conf);
        recorder.record(input, now);

        input
    };

    let mut display = |board: &Board, score: &Score, duration: &Duration| {
        ScreenBuffer::from(board)
            .write_string(26, 16, &format!("Score: {}", score.score()), Colour::White)
            .write_string(26, 18, &format!("Lines: {}", score.lines()), Colour::White)
            .write_string(26, 20, &time_format(*duration), Colour::White)
            .print();
    };

    let (score, duration) = game_loop(&mut input, &mut display, conf.frame_time.into(), game);

    (score, recorder, duration)
}

fn save_replay_prompt(recorder: Recorder, score: Score, duration: u64) {
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

                let end_time = Utc::now().timestamp_millis();

                if let Err(err) = std::fs::write(
                    file_path.clone(),
                    recorder.raw(&name, score, duration, end_time),
                ) {
                    println!(" Failed to save to: {}", file_path);
                    println!("{}", err);
                } else {
                    println!(" Replay saved to: {}", file_path);
                }

                return;
            }
        }
    }

    println!(" Replay discarded");
}

fn re_play_game(conf: Config, filename: &str) -> (Score, Duration) {
    let folder = Config::folder();
    let replay_folder = folder.clone() + if cfg!(windows) { r"replay\" } else { "replay/" };
    let path = format!("{}{}", replay_folder, filename);

    let buf = match std::fs::read(path.clone()) {
        Ok(contents) => contents,
        Err(err) => {
            print!("Unable to read replay file at \"{}\"", path);
            panic!("{}", err);
        }
    };

    let mut recording = Replay::new(buf).unwrap();
    let mut next_input = recording.next().unwrap();

    let game = Game::new(Bag::new(recording.seed()), recording.kind());

    display::clear_terminal();

    let mut input = move |now| {
        let input = get_input(conf);
        if input.quit {
            println!("--------------------");
            println!(" Cancelling replay playback");
            return input;
        }

        let input = if now >= next_input.time {
            next_input.input
        } else {
            Input::default()
        };

        if now >= next_input.time {
            if let Some(next) = recording.next() {
                next_input = next;
            }
        }

        input
    };

    let mut display = |board: &Board, score: &Score, duration: &Duration| {
        ScreenBuffer::from(board)
            .write_string(26, 16, &format!("Score: {}", score.score()), Colour::White)
            .write_string(26, 18, &format!("Lines: {}", score.lines()), Colour::White)
            .write_string(26, 20, &time_format(*duration), Colour::White)
            .write_string(26, 22, &format!("Replay: {}", filename), Colour::Grey)
            .print();
    };

    game_loop(&mut input, &mut display, conf.frame_time.into(), game)
}

fn main() {
    let folder = Config::folder();
    let conf_file = folder.clone() + "config";
    let conf = Config::from_file(&conf_file);

    if let Some(filename) = std::env::args().nth(1) {
        let (score, duration) = re_play_game(conf, &filename);

        println!("--------------------");
        print_score(score, duration);
        println!("--------------------");
    } else {
        let (score, recorder, duration) = play_game(conf);

        println!("--------------------");
        print_score(score, duration);
        println!("--------------------");
        save_replay_prompt(recorder, score, duration.as_millis() as u64);
        println!("--------------------");
        println!(" Thanks for playing");
    }
}
