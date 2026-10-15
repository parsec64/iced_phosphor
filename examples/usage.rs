use iced::widget::{column, text};
use iced::{Alignment, Element, Theme, color};

use iced_phosphor::{fill, regular};

fn main() -> iced::Result {
    iced::application(|| (), update, view)
        .title("Phosphor Icons")
        .theme(Theme::Dark)
        .font(iced_phosphor::FONT_FILL)
        .font(iced_phosphor::FONT_REGULAR)
        .run()
}

fn update(_: &mut (), _: ()) {}

fn view(_: &()) -> Element<'_, ()> {
    column![
        text("Regular heart").size(20),
        regular::heart().size(32).color(color!(0xd90166)),

        text("Filled heart").size(20),
        fill::heart().size(32).color(color!(0xd90166)),
    ]
    .spacing(12)
    .padding(24)
    .align_x(Alignment::Center)
    .into()
}
