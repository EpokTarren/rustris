use crate::config::Config;
use core::{Input, InputDirection, InputRotation};

#[link(name = "get_key_c", kind = "static")]
extern "C" {
    fn get_key_c() -> u8;
}

fn get_key() -> Option<char> {
    let ch = unsafe { get_key_c() } as char;

    if ch == '\0' {
        None
    } else {
        Some(ch)
    }
}

pub fn get_input(conf: Config) -> Input {
    let mut input = Input::default();

    while let Some(c) = get_key() {
        let c = c.to_ascii_lowercase();
        update_input(&mut input, c, conf);
    }

    input
}

pub fn update_input(input: &mut Input, c: char, conf: Config) {
    if c == conf.left {
        input.direction = InputDirection::Left;
    } else if c == conf.right {
        input.direction = InputDirection::Right;
    } else if c == conf.quit {
        input.quit = true;
    } else if c == conf.hold {
        input.hold = true;
    } else if c == conf.rotate_90 {
        input.rotation = InputRotation::Quarter;
    } else if c == conf.rotate_180 {
        input.rotation = InputRotation::TwoQuarter;
    } else if c == conf.rotate_270 {
        input.rotation = InputRotation::ThreeQuarter;
    } else if c == conf.left {
        input.direction = InputDirection::Left;
    } else if c == conf.soft_drop {
        input.soft_drop = true;
    } else if c == conf.hard_drop {
        input.hard_drop = true;
    }
}
