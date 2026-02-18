#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Color32 {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color32 {
    pub const fn from_rgb(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b, a: 255 }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NotariumTheme {
    pub background: Color32,
    pub panel: Color32,
    pub button: Color32,
    pub button_hover: Color32,
    pub accent: Color32,
    pub text: Color32,
}

impl NotariumTheme {
    pub const fn noturno_vermelho() -> Self {
        Self {
            background: Color32::from_rgb(0x1a, 0x00, 0x00),
            panel: Color32::from_rgb(0x24, 0x00, 0x00),
            button: Color32::from_rgb(0x5a, 0x00, 0x00),
            button_hover: Color32::from_rgb(0x7a, 0x00, 0x00),
            accent: Color32::from_rgb(0xcc, 0x00, 0x00),
            text: Color32::from_rgb(0xf2, 0xea, 0xea),
        }
    }
}
