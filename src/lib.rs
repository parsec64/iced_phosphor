mod icon;

#[cfg(feature = "thin")]
pub mod thin;
#[cfg(feature = "light")]
pub mod light;
#[cfg(feature = "regular")]
pub mod regular;
#[cfg(feature = "bold")]
pub mod bold;
#[cfg(feature = "fill")]
pub mod fill;
#[cfg(feature = "duotone")]
pub mod duotone;


#[cfg(feature = "thin")]
pub const FONT_THIN: &[u8] =
    include_bytes!("../assets/thin/Phosphor-Thin.ttf");

#[cfg(feature = "light")]
pub const FONT_LIGHT: &[u8] =
    include_bytes!("../assets/light/Phosphor-Light.ttf");

#[cfg(feature = "regular")]
pub const FONT_REGULAR: &[u8] =
    include_bytes!("../assets/regular/Phosphor.ttf");

#[cfg(feature = "bold")]
pub const FONT_BOLD: &[u8] =
    include_bytes!("../assets/bold/Phosphor-Bold.ttf");

#[cfg(feature = "fill")]
pub const FONT_FILL: &[u8] =
    include_bytes!("../assets/fill/Phosphor-Fill.ttf");

#[cfg(feature = "duotone")]
pub const FONT_DUOTONE: &[u8] =
    include_bytes!("../assets/duotone/Phosphor-Duotone.ttf");
