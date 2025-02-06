use dioxus_translate::{Language, Translate};
use std::str::FromStr;

#[derive(Debug, Translate, PartialEq, Eq)]
pub enum ProjectArea {
    #[translate(ko = "경제")]
    Economy = 1,
    #[translate(ko = "사회")]
    Society = 2,
    #[translate(ko = "기술")]
    Technology = 8,
}

#[test]
fn test_translation() {
    assert_eq!(ProjectArea::Economy.translate(&Language::En), "Economy");
    assert_eq!(ProjectArea::Economy.translate(&Language::Ko), "경제");

    assert_eq!(ProjectArea::Society.translate(&Language::En), "Society");
    assert_eq!(ProjectArea::Society.translate(&Language::Ko), "사회");

    assert_eq!(
        ProjectArea::Technology.translate(&Language::En),
        "Technology"
    );
    assert_eq!(ProjectArea::Technology.translate(&Language::Ko), "기술");
}

#[test]
fn test_display() {
    assert_eq!(format!("{}", ProjectArea::Economy), "economy");
    assert_eq!(format!("{}", ProjectArea::Society), "society");
    assert_eq!(format!("{}", ProjectArea::Technology), "technology");
}

#[test]
fn test_from_str() {
    // Test English names
    assert_eq!(ProjectArea::from_str("Economy"), Ok(ProjectArea::Economy));
    assert_eq!(ProjectArea::from_str("Society"), Ok(ProjectArea::Society));
    assert_eq!(
        ProjectArea::from_str("Technology"),
        Ok(ProjectArea::Technology)
    );

    // Test Korean translations
    assert_eq!(ProjectArea::from_str("경제"), Ok(ProjectArea::Economy));
    assert_eq!(ProjectArea::from_str("사회"), Ok(ProjectArea::Society));
    assert_eq!(ProjectArea::from_str("기술"), Ok(ProjectArea::Technology));

    // Test lowercase names
    assert_eq!(ProjectArea::from_str("economy"), Ok(ProjectArea::Economy));
    assert_eq!(ProjectArea::from_str("society"), Ok(ProjectArea::Society));
    assert_eq!(
        ProjectArea::from_str("technology"),
        Ok(ProjectArea::Technology)
    );

    // Test invalid input
    assert!(ProjectArea::from_str("invalid_field").is_err());
}

#[test]
fn test_variants() {
    assert_eq!(
        ProjectArea::VARIANTS,
        &[
            ProjectArea::Economy,
            ProjectArea::Society,
            ProjectArea::Technology,
        ],
    );
}

#[test]
fn test_fn_variants() {
    println!("{:?}", ProjectArea::variants(&Language::Ko));
    assert_eq!(
        ProjectArea::variants(&Language::En),
        vec!["Economy", "Society", "Technology"],
    );
    assert_eq!(
        ProjectArea::variants(&Language::Ko),
        vec!["경제", "사회", "기술"],
    );
}
