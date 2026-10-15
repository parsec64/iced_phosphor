use iced::{Font, widget::Text};


pub const FONT_THIN: Font = Font::with_name("Phosphor-Thin");
pub const FONT_LIGHT: Font = Font::with_name("Phosphor-Light");
pub const FONT_REGULAR: Font = Font::with_name("Phosphor-Regular");
pub const FONT_BOLD: Font = Font::with_name("Phosphor-Bold");
pub const FONT_FILL: Font = Font::with_name("Phosphor-Fill");
pub const FONT_DUOTONE: Font = Font::with_name("Phosphor-Duotone");


pub(crate) fn create_icon(codepoint: u32, font: Font) -> Text<'static> {
    let character = char::from_u32(codepoint)
        .expect("Invalid Unicode codepoint");

    Text::new(character.to_string()).font(font)
}
