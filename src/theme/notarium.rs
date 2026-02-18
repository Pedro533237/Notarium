/// Cor RGBA em espaço sRGB de 8 bits.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub const fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    pub const fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self::rgba(r, g, b, 255)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThemeMode {
    DarkRed,
    Light,
}

/// Tema visual oficial do Notarium.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NotariumTheme {
    pub background: Color,
    pub panel: Color,
    pub accent: Color,
    pub highlight: Color,
    pub text_primary: Color,
}

impl NotariumTheme {
    pub fn new(mode: ThemeMode) -> Self {
        match mode {
            ThemeMode::DarkRed => Self {
                background: Color::rgb(0x1a, 0x00, 0x00),
                panel: Color::rgb(0x5a, 0x00, 0x00),
                accent: Color::rgb(0x8a, 0x00, 0x00),
                highlight: Color::rgb(0xcc, 0x00, 0x00),
                text_primary: Color::rgb(0xf2, 0xea, 0xea),
            },
            ThemeMode::Light => Self {
                background: Color::rgb(0xf6, 0xf2, 0xf2),
                panel: Color::rgb(0xe6, 0xd6, 0xd6),
                accent: Color::rgb(0x9e, 0x66, 0x66),
                highlight: Color::rgb(0xcc, 0x00, 0x00),
                text_primary: Color::rgb(0x1e, 0x10, 0x10),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dark_theme_matches_spec() {
        let t = NotariumTheme::new(ThemeMode::DarkRed);
        assert_eq!(t.background, Color::rgb(0x1a, 0x00, 0x00));
        assert_eq!(t.panel, Color::rgb(0x5a, 0x00, 0x00));
        assert_eq!(t.highlight, Color::rgb(0xcc, 0x00, 0x00));
        assert_eq!(t.text_primary, Color::rgb(0xf2, 0xea, 0xea));
    }
}
