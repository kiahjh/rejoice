//! Tests for the #[derive(PropEnum)] macro

use rejoice_macros::PropEnum;

// =============================================================================
// Basic PropEnum tests
// =============================================================================

#[derive(PropEnum, Clone, Copy, PartialEq, Debug)]
pub enum Color {
    Red,
    Green,
    Blue,
}

#[test]
fn test_prop_enum_variants() {
    let variants = Color::prop_enum_variants();
    assert_eq!(variants, &["Red", "Green", "Blue"]);
}

#[test]
fn test_prop_enum_from_name() {
    assert_eq!(Color::prop_enum_from_name("Red"), Some(Color::Red));
    assert_eq!(Color::prop_enum_from_name("Green"), Some(Color::Green));
    assert_eq!(Color::prop_enum_from_name("Blue"), Some(Color::Blue));
    assert_eq!(Color::prop_enum_from_name("Yellow"), None);
    assert_eq!(Color::prop_enum_from_name(""), None);
}

#[test]
fn test_prop_enum_name() {
    assert_eq!(Color::Red.prop_enum_name(), "Red");
    assert_eq!(Color::Green.prop_enum_name(), "Green");
    assert_eq!(Color::Blue.prop_enum_name(), "Blue");
}

// =============================================================================
// Edge cases
// =============================================================================

#[derive(PropEnum, Clone, Copy, PartialEq, Debug)]
pub enum SingleVariant {
    Only,
}

#[test]
fn test_single_variant_enum() {
    assert_eq!(SingleVariant::prop_enum_variants(), &["Only"]);
    assert_eq!(
        SingleVariant::prop_enum_from_name("Only"),
        Some(SingleVariant::Only)
    );
    assert_eq!(SingleVariant::Only.prop_enum_name(), "Only");
}

#[derive(PropEnum, Clone, Copy, PartialEq, Debug)]
pub enum Size {
    Small,
    Medium,
    Large,
    ExtraLarge,
}

#[test]
fn test_camel_case_variants() {
    let variants = Size::prop_enum_variants();
    assert_eq!(variants, &["Small", "Medium", "Large", "ExtraLarge"]);
    assert_eq!(
        Size::prop_enum_from_name("ExtraLarge"),
        Some(Size::ExtraLarge)
    );
    assert_eq!(Size::ExtraLarge.prop_enum_name(), "ExtraLarge");
}

// =============================================================================
// Round-trip tests
// =============================================================================

#[test]
fn test_round_trip() {
    for variant_name in Color::prop_enum_variants() {
        let variant = Color::prop_enum_from_name(variant_name).unwrap();
        assert_eq!(variant.prop_enum_name(), *variant_name);
    }
}
