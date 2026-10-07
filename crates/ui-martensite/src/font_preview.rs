//! Glyph and font preview models.

pub struct FontPreviewState {
    pub sample_text: String,
    pub sample_size: f32,
    pub ot_features: Vec<String>,
}

impl FontPreviewState {
    pub fn new() -> Self {
        Self {
            sample_text: "The quick brown fox jumps over the lazy dog".to_string(),
            sample_size: 32.0,
            ot_features: vec!["liga".to_string(), "kern".to_string()],
        }
    }

    pub fn set_sample_size(&mut self, pt: f32) {
        self.sample_size = pt.clamp(8.0, 288.0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_preview_size() {
        let mut p = FontPreviewState::new();
        p.set_sample_size(48.0);
        assert_eq!(p.sample_size, 48.0);
    }
}
