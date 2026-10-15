#[test]
fn font_bytes_are_present() {
    #[cfg(feature = "thin")]
    assert!(!iced_phosphor::FONT_THIN.is_empty());

    #[cfg(feature = "light")]
    assert!(!iced_phosphor::FONT_LIGHT.is_empty());

    #[cfg(feature = "regular")]
    assert!(!iced_phosphor::FONT_REGULAR.is_empty());

    #[cfg(feature = "fill")]
    assert!(!iced_phosphor::FONT_FILL.is_empty());

    #[cfg(feature = "bold")]
    assert!(!iced_phosphor::FONT_BOLD.is_empty());

    #[cfg(feature = "duotone")]
    assert!(!iced_phosphor::FONT_DUOTONE.is_empty());
}
