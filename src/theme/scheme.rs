use amane::Color;

// a fixed dark palette that ignores the wallpaper
pub struct Scheme {
    pub background: Color,
    pub surface: Color,
    pub hover_surface: Color,
    pub border: Color,
    pub text: Color,
    pub secondary_text: Color,
    pub muted_text: Color,
    pub accent: Color,
    pub accent_hover: Color,
    pub on_accent: Color,
    pub success: Color,
    pub danger: Color,
}

pub const GRUVBOX: Scheme = Scheme {
    background: Color::rgb(0x1d, 0x20, 0x21),
    surface: Color::rgb(0x28, 0x28, 0x28),
    hover_surface: Color::rgb(0x3c, 0x38, 0x36),
    border: Color::rgb(0x50, 0x49, 0x45),
    text: Color::rgb(0xeb, 0xdb, 0xb2),
    secondary_text: Color::rgb(0xbd, 0xae, 0x93),
    muted_text: Color::rgb(0x92, 0x83, 0x74),
    accent: Color::rgb(0xfe, 0x80, 0x19),
    accent_hover: Color::rgb(0xfa, 0xbd, 0x2f),
    on_accent: Color::rgb(0x1d, 0x20, 0x21),
    success: Color::rgb(0xb8, 0xbb, 0x26),
    danger: Color::rgb(0xfb, 0x49, 0x34),
};

pub const CATPPUCCIN: Scheme = Scheme {
    background: Color::rgb(0x11, 0x11, 0x1b),
    surface: Color::rgb(0x1e, 0x1e, 0x2e),
    hover_surface: Color::rgb(0x31, 0x32, 0x44),
    border: Color::rgb(0x45, 0x47, 0x5a),
    text: Color::rgb(0xcd, 0xd6, 0xf4),
    secondary_text: Color::rgb(0xa6, 0xad, 0xc8),
    muted_text: Color::rgb(0x6c, 0x70, 0x86),
    accent: Color::rgb(0xef, 0x9f, 0x76),
    accent_hover: Color::rgb(0xe5, 0xc8, 0x90),
    on_accent: Color::rgb(0x11, 0x11, 0x1b),
    success: Color::rgb(0xa6, 0xe3, 0xa1),
    danger: Color::rgb(0xf3, 0x8b, 0xa8),
};
