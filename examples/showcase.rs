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
    let pink = color!(0xd90166);
    let electric_cyan  = color!(0x00f5d4);
    let neon_yellow    = color!(0xfee440);
    let vivid_purple   = color!(0x9b5de5);
    let bright_orange  = color!(0xf15bb5);
    let acid_green     = color!(0x06d6a0);
    let blazing_orange = color!(0xff6b35);

    column![
        text("Phosphor Icons").size(28),
        row![
            text("Thin").width(100),
            thin::heart().size(36).color(pink),
            thin::star().size(36).color(neon_yellow),
            thin::house().size(36).color(electric_cyan),
            thin::user().size(36).color(bright_orange),
            thin::gear().size(36).color(vivid_purple),
            thin::check().size(36).color(acid_green),
            thin::bell().size(36).color(blazing_orange),
        ]
        .spacing(12)
        .align_y(Alignment::Center),
        row![
            text("Light").width(100),
            light::heart().size(36).color(pink),
            light::star().size(36).color(neon_yellow),
            light::house().size(36).color(electric_cyan),
            light::user().size(36).color(bright_orange),
            light::gear().size(36).color(vivid_purple),
            light::check().size(36).color(acid_green),
            light::bell().size(36).color(blazing_orange),
        ]
        .spacing(12)
        .align_y(Alignment::Center),
        row![
            text("Regular").width(100),
            regular::heart().size(36).color(pink),
            regular::star().size(36).color(neon_yellow),
            regular::house().size(36).color(electric_cyan),
            regular::user().size(36).color(bright_orange),
            regular::gear().size(36).color(vivid_purple),
            regular::check().size(36).color(acid_green),
            regular::bell().size(36).color(blazing_orange),
        ]
        .spacing(12)
        .align_y(Alignment::Center),
        row![
            text("Bold").width(100),
            bold::heart().size(36).color(pink),
            bold::star().size(36).color(neon_yellow),
            bold::house().size(36).color(electric_cyan),
            bold::user().size(36).color(bright_orange),
            bold::gear().size(36).color(vivid_purple),
            bold::check().size(36).color(acid_green),
            bold::bell().size(36).color(blazing_orange),
        ]
        .spacing(12)
        .align_y(Alignment::Center),
        row![
            text("Fill").width(100),
            fill::heart().size(36).color(pink),
            fill::star().size(36).color(neon_yellow),
            fill::house().size(36).color(electric_cyan),
            fill::user().size(36).color(bright_orange),
            fill::gear().size(36).color(vivid_purple),
            fill::check().size(36).color(acid_green),
            fill::bell().size(36).color(blazing_orange),
        ]
        .spacing(12)
        .align_y(Alignment::Center),
        row![
            text("Duotone").width(100),
            duotone::heart().size(36).color(pink),
            duotone::star().size(36).color(neon_yellow),
            duotone::house().size(36).color(electric_cyan),
            duotone::user().size(36).color(bright_orange),
            duotone::gear().size(36).color(vivid_purple),
            duotone::check().size(36).color(acid_green),
            duotone::bell().size(36).color(blazing_orange),
        ]
        .spacing(12)
        .align_y(Alignment::Center),
    ]
    .spacing(16)
    .padding(24)
    .into()
}
