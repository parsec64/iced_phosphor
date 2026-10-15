use iced::{Font, widget::Text};


#[cfg(feature = "thin")]
pub const FONT_THIN: Font = Font::with_name("Phosphor-Thin");

#[cfg(feature = "light")]
pub const FONT_LIGHT: Font = Font::with_name("Phosphor-Light");

#[cfg(feature = "regular")]
pub const FONT_REGULAR: Font = Font::with_name("Phosphor");

#[cfg(feature = "bold")]
pub const FONT_BOLD: Font = Font::with_name("Phosphor-Bold");

#[cfg(feature = "fill")]
pub const FONT_FILL: Font = Font::with_name("Phosphor-Fill");

#[cfg(feature = "duotone")]
pub const FONT_DUOTONE: Font = Font::with_name("Phosphor-Duotone");


pub(crate) fn create_icon<'a>(codepoint: u32, font: Font) -> Text<'a> {
    let character = char::from_u32(codepoint).expect("Invalid Unicode codepoint");

    Text::new(character).font(font)
}
