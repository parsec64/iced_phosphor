use iced::widget::{column, row, text};
use iced::{Element, Theme};

use iced_phosphor::{bold, duotone, fill, light, regular, thin};

fn main() -> iced::Result {
    iced::application(|| (), update, view)
        .title("Iced Phosphor Showcase")
        .theme(Theme::Dark)
        .font(iced_phosphor::FONT_THIN)
        .font(iced_phosphor::FONT_LIGHT)
        .font(iced_phosphor::FONT_REGULAR)
        .font(iced_phosphor::FONT_BOLD)
        .font(iced_phosphor::FONT_FILL)
        .font(iced_phosphor::FONT_DUOTONE)
        .run()
}

fn update(_: &mut (), _: ()) {}

fn view(_: &()) -> Element<'_, ()> {
    column![
        text("Phosphor Icons").size(28),
        row![text("Thin").width(100), thin::acorn().size(36)],
        row![text("Light").width(100), light::acorn().size(36)],
        row![text("Regular").width(100), regular::acorn().size(36)],
        row![text("Bold").width(100), bold::acorn().size(36)],
        row![text("Fill").width(100), fill::acorn().size(36)],
        row![text("Duotone").width(100), duotone::acorn().size(36)],
    ]
    .spacing(16)
    .padding(24)
    .into()
}
