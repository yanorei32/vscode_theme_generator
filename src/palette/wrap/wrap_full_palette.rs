use serde::{Deserialize, Serialize};

use crate::{color::wrap::wrap_srgb::WrapSrgb, palette::full_palette::FullPalette};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WrapFullPalette {
    #[serde(rename = "$schema")]
    pub schema: String,
    pub dark: bool,
    pub bg: Vec<WrapSrgb>,
    pub fg: Vec<WrapSrgb>,
    pub gray: Vec<WrapSrgb>,
    pub blue: Vec<WrapSrgb>,
    pub green: Vec<WrapSrgb>,
    pub yellow: Vec<WrapSrgb>,
    pub orange: Vec<WrapSrgb>,
    pub red: Vec<WrapSrgb>,
    pub purple: Vec<WrapSrgb>,
    pub pink: Vec<WrapSrgb>,
}

impl From<FullPalette> for WrapFullPalette {
    fn from(v: FullPalette) -> Self {
        Self {
            schema: "https://raw.githubusercontent.com/ecto0310/vscode_theme_generator/refs/heads/main/schema/full_palette.json".to_string(),
            dark: v.dark,
            bg: v
                .bg
                .into_iter()
                .map(|f| f.into())
                .collect::<Vec<WrapSrgb>>(),
            fg: v
                .fg
                .into_iter()
                .map(|f| f.into())
                .collect::<Vec<WrapSrgb>>(),
            gray: v
                .gray
                .into_iter()
                .map(|f| f.into())
                .collect::<Vec<WrapSrgb>>(),
            blue: v
                .blue
                .into_iter()
                .map(|f| f.into())
                .collect::<Vec<WrapSrgb>>(),
            green: v
                .green
                .into_iter()
                .map(|f| f.into())
                .collect::<Vec<WrapSrgb>>(),
            yellow: v
                .yellow
                .into_iter()
                .map(|f| f.into())
                .collect::<Vec<WrapSrgb>>(),
            orange: v
                .orange
                .into_iter()
                .map(|f| f.into())
                .collect::<Vec<WrapSrgb>>(),
            red: v
                .red
                .into_iter()
                .map(|f| f.into())
                .collect::<Vec<WrapSrgb>>(),
            purple: v
                .purple
                .into_iter()
                .map(|f| f.into())
                .collect::<Vec<WrapSrgb>>(),
            pink: v
                .pink
                .into_iter()
                .map(|f| f.into())
                .collect::<Vec<WrapSrgb>>(),
        }
    }
}
