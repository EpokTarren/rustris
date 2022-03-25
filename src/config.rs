#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Config {
    pub quit: char,
    pub hold: char,
    pub left: char,
    pub right: char,
    pub soft_drop: char,
    pub hard_drop: char,
    pub rotate_90: char,
    pub rotate_180: char,
    pub rotate_270: char,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            quit: 'q',
            hold: 'c',
            left: ',',
            right: '-',
            soft_drop: '.',
            hard_drop: ' ',
            rotate_90: 'x',
            rotate_180: '\0',
            rotate_270: 'z',
        }
    }
}

impl From<&str> for Config {
    fn from(s: &str) -> Self {
        let mut config: Self = Default::default();

        for line in s
            .to_lowercase()
            .lines()
            .filter(|line| line.chars().position(|c| c == '=').unwrap_or(usize::MAX) < line.len())
        {
            let mut split = line.split("=");

            fn key(s: &str) -> Option<char> {
                s.replace("'", "").replace("\"", "").trim().chars().nth(0)
            }

            if let Some(option) = split.nth(0) {
                if let Some(value) = split.nth(1) {
                    if let Some(key) = key(value) {
                        match option.trim() {
                            "hold" => config.hold = key,
                            "left" => config.left = key,
                            "right" => config.right = key,
                            "soft_drop" => config.soft_drop = key,
                            "hard_drop" => config.hard_drop = key,
                            "rotate_90" => config.rotate_90 = key,
                            "rotate_180" => config.rotate_180 = key,
                            "rotate_270" => config.rotate_270 = key,
                            _ => {}
                        }
                    }
                }
            }
        }

        config
    }
}

impl Config {
    pub fn from_file(filename: &str) -> Self {
        if let Ok(contents) = std::fs::read_to_string(filename) {
            Self::from(contents.as_str())
        } else {
            Default::default()
        }
    }

    pub const PATH: &'static str = if cfg!(windows) {
        "%USERPROFILE%\\.rustris\\config"
    } else {
        "~/.rustris/config"
    };
}
