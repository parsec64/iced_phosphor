# iced_phosphor

Phosphor Icons for [Iced](https://iced.rs/), a cross-platform GUI library for Rust.

This crate bundles the Phosphor font files and provides icon functions that return Iced `Text` widgets.

## Features

- Six icon weights: Thin, Light, Regular, Bold, Fill, and Duotone.
- Generated Rust functions for the icons included in the Phosphor selection files.
- Icon functions return `iced::widget::Text` widgets.

![Showcase](./screenshots/showcase.png)

## Installation

Add the crate to your `Cargo.toml`:

```toml
[dependencies]
iced_phosphor = { git = "https://github.com/parsec64/iced_phosphor" }
```

By default only `Regular` weight is enabled.

Enable specific weights:

```toml
iced_phosphor = {
    git = "https://github.com/parsec64/iced_phosphor",
    features = ["fill", "regular"]
}
```

Enable all at once:

```toml
iced_phosphor = {
    git = "https://github.com/parsec64/iced_phosphor",
    features = ["full"]
}
```

Available features: `thin`, `light`, `regular`, `bold`, `fill`, `duotone`, `full`.

## Usage

```rust
use iced::widget::{column, text};
use iced::{Alignment, Element, Theme, color};

use iced_phosphor::{fill, regular}; // Import weights

fn main() -> iced::Result {
    iced::application(|| (), update, view)
        .title("Phosphor Icons")
        .theme(Theme::Dark)
        .font(iced_phosphor::FONT_FILL) // Register fonts
        .font(iced_phosphor::FONT_REGULAR)
        .run()
}

fn update(_: &mut (), _: ()) {}

fn view(_: &()) -> Element<'_, ()> {
    column![
        text("Regular heart").size(20),
        regular::heart().size(32).color(color!(0xd90166)), // Call icons

        text("Filled heart").size(20),
        fill::heart().size(32).color(color!(0xd90166)),
    ]
    .spacing(12)
    .padding(24)
    .align_x(Alignment::Center)
    .into()
}
```

## Credits

This crate uses [Phosphor Icons](https://phosphoricons.com/).

Phosphor Icons are distributed under the MIT License.
See [LICENSE](./LICENSE) for the license text.
