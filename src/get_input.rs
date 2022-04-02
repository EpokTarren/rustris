use crate::{config::Config, input::Input};

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
        input.update(c, conf);
    }

    input
}
