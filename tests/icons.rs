#[cfg(feature = "thin")]
#[test]
fn thin_icon_compiles() {
    let _ = iced_phosphor::thin::acorn();
}

#[cfg(feature = "light")]
#[test]
fn light_icon_compiles() {
    let _ = iced_phosphor::light::acorn();
}

#[cfg(feature = "regular")]
#[test]
fn regular_icon_compiles() {
    let _ = iced_phosphor::regular::acorn();
}

#[cfg(feature = "bold")]
#[test]
fn bold_icon_compiles() {
    let _ = iced_phosphor::bold::acorn();
}

#[cfg(feature = "fill")]
#[test]
fn fill_icon_compiles() {
    let _ = iced_phosphor::fill::acorn();
}

#[cfg(feature = "duotone")]
#[test]
fn duotone_icon_compiles() {
    let _ = iced_phosphor::duotone::acorn();
}
