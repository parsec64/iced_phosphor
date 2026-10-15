use iced::widget::{column, row, text};
use iced::{Alignment, Element, Theme, color};

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

        row![
            text("Thin").width(100), thin::heart().size(36).color(color!(0xd90166))
        ].align_y(Alignment::Center),

        row![
            text("Light").width(100), light::heart().size(36).color(color!(0xd90166))
        ].align_y(Alignment::Center),

        row![
            text("Regular").width(100), regular::heart().size(36).color(color!(0xd90166))
        ].align_y(Alignment::Center),

        row![
            text("Bold").width(100), bold::heart().size(36).color(color!(0xd90166))
        ].align_y(Alignment::Center),

        row![
            text("Fill").width(100), fill::heart().size(36).color(color!(0xd90166))
        ].align_y(Alignment::Center),

        row![
            text("Duotone").width(100), duotone::heart().size(36).color(color!(0xd90166))
        ].align_y(Alignment::Center),
    ]
    .spacing(16)
    .padding(24)
    .into()
}
