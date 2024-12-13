use std::{fs::File, io::Write, path::Path};

use ::palette::Srgb;
use palette::{FromColor as _, Lch};

use super::{base_palette::BasePalette, wrap::wrap_full_palette::WrapFullPalette};
use crate::color::Color;

use crate::model::ActualThemeMode;

use linearize::StaticMap;

#[derive(Debug, Clone)]
pub struct FullPalette {
    pub actual_mode: ActualThemeMode,

    pub fg: Vec<Srgb>,
    pub base_color_table: StaticMap<Color, Vec<Srgb>>,
}

impl From<BasePalette> for FullPalette {
    fn from(v: BasePalette) -> Self {
        let generate = |rgb: Srgb, double_width: bool| -> Vec<Srgb> {
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
            if v.actual_mode == ActualThemeMode::Dark {
                ls.reverse();
            }
            ls.into_iter()
                .map(|l| Srgb::from_color(Lch::new(l, lch.chroma, lch.hue)))
                .collect()
        };
        let (l, _) = v.fg_average();
        let fg = Srgb::from_color(Lch::new(l, 0.0, 0.0));

        Self {
            actual_mode: v.actual_mode,
            fg: generate(fg, true),
            base_color_table: v.color_table.map(|k, c| generate(c, k == Color::Bg)),
        }
    }
}

impl FullPalette {
    pub fn export(&self, path: &Path) -> anyhow::Result<()> {
        let wrap_palette: WrapFullPalette = self.clone().into();
        let palette_str = serde_json::to_string(&wrap_palette)?;
        let mut palette_file = File::create(path)?;
        writeln!(palette_file, "{}", palette_str)?;
        Ok(())
    }
}
