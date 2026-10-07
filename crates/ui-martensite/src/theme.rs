//! Typography theme tokens.

pub struct Color(pub u8, pub u8, pub u8);

pub struct Theme {
    pub preview_bg: Color,
    pub glyph_color: Color,
}

impl Theme {
    pub fn font_studio() -> Self {
        Self {
            preview_bg: Color(250, 250, 252),
            glyph_color: Color(20, 22, 28),
        }
    }
}
