use ratatui::style::Color;

#[derive(Debug, Clone, Copy)]
pub enum PasswordStrength {
    Weak,
    Medium,
    Strong,
    VeryStrong,
}

impl PasswordStrength {
    pub fn from_settings(length: usize, upper: bool, numbers: bool, symbols: bool) -> Self {
        let types = 1 + (upper as usize) + (numbers as usize) + (symbols as usize);
        match (length, types) {
            (l, _) if l < 8 => Self::Weak,
            (l, 1) if l < 12 => Self::Weak,
            (l, 2) if l < 10 => Self::Weak,
            (_, 1..=2) => Self::Medium,
            (l, 3) if l < 12 => Self::Medium,
            (_, 3) => Self::Strong,
            (l, 4) if l < 12 => Self::Strong,
            _ => Self::VeryStrong,
        }
    }

    pub fn color(&self) -> Color {
        match self {
            Self::Weak => Color::Red,
            Self::Medium => Color::Yellow,
            Self::Strong => Color::Green,
            Self::VeryStrong => Color::Cyan,
        }
    }

    pub fn description(&self) -> &str {
        match self {
            Self::Weak => "Weak",
            Self::Medium => "Medium",
            Self::Strong => "Strong",
            Self::VeryStrong => "Very Strong",
        }
    }
}