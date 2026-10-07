//! Integration test for Font manager.

use craft_fonts_ui_martensite::FontManagerApp;

#[test]
fn test_font_workflow() {
    let mut app = FontManagerApp::new();
    app.preview.set_sample_size(72.0);
    assert_eq!(app.preview.sample_size, 72.0);
    assert!(app.preview.ot_features.contains(&"liga".to_string()));
}
