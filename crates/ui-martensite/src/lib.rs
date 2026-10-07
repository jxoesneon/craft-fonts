//! Sovereign retained-mode font manager interface for Craft Fonts built on Martensite.

pub mod font_preview;
pub mod theme;

pub struct FontManagerApp {
    pub preview: font_preview::FontPreviewState,
}

impl FontManagerApp {
    pub fn new() -> Self {
        Self {
            preview: font_preview::FontPreviewState::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_font_app() {
        let app = FontManagerApp::new();
        assert_eq!(app.preview.sample_size, 32.0);
    }
}
