use ansi_term::{Colour, Style};

pub fn shader_by_hex(color: &str) -> Style {
    let c = color.trim_start_matches('#');
    let (r, g, b) = (
        u8::from_str_radix(&c[0..2], 16).unwrap(),
        u8::from_str_radix(&c[2..4], 16).unwrap(),
        u8::from_str_radix(&c[4..6], 16).unwrap(),
    );
    Colour::RGB(r, g, b).bold()
}
