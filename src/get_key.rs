#[link(name = "get_key_c", kind = "static")]
extern "C" {
    fn get_key_c() -> u8;
}

pub fn get_key() -> Option<char> {
    let ch = unsafe { get_key_c() } as char;

    if ch == '\0' {
        None
    } else {
        Some(ch)
    }
}
