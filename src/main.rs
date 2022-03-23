use display::{Colour, ScreenBuffer};

mod display;
mod piece;

fn main() {
    let mut buf: ScreenBuffer = Default::default();

    let out = [
        ("Cyan", Colour::Cyan),
        ("Yellow", Colour::Yellow),
        ("Purple", Colour::Purple),
        ("Green", Colour::Green),
        ("Red", Colour::Red),
        ("Blue", Colour::Blue),
        ("Orange", Colour::Orange),
        ("Grey", Colour::Grey),
        ("White", Colour::White),
    ];

    for (i, (s, c)) in out.into_iter().enumerate() {
        buf.write_string(0, i, s, c);
    }

    buf.print();
}
