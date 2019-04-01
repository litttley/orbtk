use crate::structs::{Brush, Color};

use super::*;

#[test]
fn test_into() {
    let foreground: Foreground = "#000000".into();
    assert_eq!(foreground.0, Brush::SolidColor(Color::rgb(0, 0, 0)));

    let foreground: Foreground = "#ffffff".into();
    assert_eq!(foreground.0, Brush::SolidColor(Color::rgb(255, 255, 255)));
}
