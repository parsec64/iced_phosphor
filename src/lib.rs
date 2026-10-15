mod icon;
pub mod thin;
pub mod light;
pub mod regular;
pub mod bold;
pub mod fill;
pub mod duotone;


pub const FONT_THIN: &[u8] =
    include_bytes!("../assets/thin/Phosphor-Thin.ttf");

pub const FONT_LIGHT: &[u8] =
    include_bytes!("../assets/light/Phosphor-Light.ttf");

pub const FONT_REGULAR: &[u8] =
    include_bytes!("../assets/regular/Phosphor.ttf");

pub const FONT_BOLD: &[u8] =
    include_bytes!("../assets/bold/Phosphor-Bold.ttf");

pub const FONT_FILL: &[u8] =
    include_bytes!("../assets/fill/Phosphor-Fill.ttf");

pub const FONT_DUOTONE: &[u8] =
    include_bytes!("../assets/duotone/Phosphor-Duotone.ttf");
