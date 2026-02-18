#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub const fn from_rgb(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b, a: 255 }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NotariumTheme {
    pub background: Color,
    pub panel: Color,
    pub accent: Color,
    pub highlight: Color,
    pub text_primary: Color,
}

impl NotariumTheme {
    pub const fn noturno() -> Self {
        Self {
            background: Color::from_rgb(0x1a, 0x00, 0x00),
            panel: Color::from_rgb(0x5a, 0x00, 0x00),
            accent: Color::from_rgb(0xcc, 0x00, 0x00),
            highlight: Color::from_rgb(0xff, 0x33, 0x33),
            text_primary: Color::from_rgb(0xf2, 0xea, 0xea),
        }
    }

    pub const fn claro() -> Self {
        Self {
            background: Color::from_rgb(0xf8, 0xf8, 0xf8),
            panel: Color::from_rgb(0xea, 0xea, 0xea),
            accent: Color::from_rgb(0x88, 0x00, 0x00),
            highlight: Color::from_rgb(0xcc, 0x22, 0x22),
            text_primary: Color::from_rgb(0x1c, 0x12, 0x12),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::NotariumTheme;

    #[test]
    fn noturno_palette_matches_spec() {
        let theme = NotariumTheme::noturno();
        assert_eq!(theme.background.r, 0x1a);
        assert_eq!(theme.panel.r, 0x5a);
        assert_eq!(theme.accent.r, 0xcc);
        assert_eq!(theme.text_primary.g, 0xea);
    }
}
