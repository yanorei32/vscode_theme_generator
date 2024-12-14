use std::{fs::File, io::Write, path::Path};

use linearize::StaticMap;
use palette::{FromColor as _, Lch, Srgb};

use crate::{
    model::{ActualThemeMode, Color},
    palette::BasePalette,
    schema::FullPaletteFile,
    util::ColorMapExt,
};

#[derive(Debug, Clone)]
pub struct FullPalette {
    pub actual_mode: ActualThemeMode,

    pub fg: [Srgb; 5],
    pub color_map: StaticMap<Color, [Srgb; 5]>,
}

impl From<BasePalette> for FullPalette {
    fn from(v: BasePalette) -> Self {
        let (actual_mode, color_map) = v.take();

        let generate = |rgb: Srgb, double_width: bool| -> [Srgb; 5] {
            let lch = Lch::from_color(rgb);
            let width_cut = if double_width { 1.0 } else { 2.0 };
            let width = lch.l.min(100.0 - lch.l) / width_cut;

            let mut ls = [
                lch.l + width,
                lch.l + width / 2.0,
                lch.l,
                lch.l - width / 2.0,
                lch.l - width,
            ];

            if actual_mode == ActualThemeMode::Dark {
                ls.reverse();
            }

            ls.map(|l| Srgb::from_color(Lch::new(l, lch.chroma, lch.hue)))
        };

        let fg = Srgb::from_color(Lch::new(color_map.fg_color_luminouse_chroma().0, 0.0, 0.0));

        Self {
            actual_mode,
            fg: generate(fg, true),
            color_map: color_map.map(|k, c| generate(c, k == Color::Bg)),
        }
    }
}

impl FullPalette {
    pub fn export(&self, path: &Path) -> anyhow::Result<()> {
        let palette = FullPaletteFile::from(self.clone());
        let palette = serde_json::to_string(&palette)?;

        File::create(path)?.write_all(palette.as_bytes())?;
        Ok(())
    }
}
