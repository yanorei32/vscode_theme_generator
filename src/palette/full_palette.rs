use std::{fs::File, io::Write, path::Path};

use ::palette::Srgb;
use palette::{FromColor as _, Lch};

use super::{base_palette::{BasePalette, PaletteColor}, wrap::wrap_full_palette::WrapFullPalette};

#[derive(Debug, Clone)]
pub struct FullPalette {
    pub dark: bool,
    pub bg: Vec<Srgb>,
    pub fg: Vec<Srgb>,
    pub gray: Vec<Srgb>,
    pub blue: Vec<Srgb>,
    pub green: Vec<Srgb>,
    pub yellow: Vec<Srgb>,
    pub orange: Vec<Srgb>,
    pub red: Vec<Srgb>,
    pub purple: Vec<Srgb>,
    pub pink: Vec<Srgb>,
}

impl From<BasePalette> for FullPalette {
    fn from(v: BasePalette) -> Self {
        let generate = |rgb: Srgb, double_width: bool| -> Vec<Srgb> {
            let lch = Lch::from_color(rgb);
            let width_cut = if double_width { 1.0 } else { 2.0 };
            let width = lch.l.min(100.0 - lch.l) / width_cut;
            let mut ls = vec![
                lch.l + width,
                lch.l + width / 2.0,
                lch.l,
                lch.l - width / 2.0,
                lch.l - width,
            ];
            if v.dark {
                ls.reverse();
            }
            ls.into_iter()
                .map(|l| Srgb::from_color(Lch::new(l, lch.chroma, lch.hue)))
                .collect()
        };
        let (l, _) = v.fg_average();
        let fg = Srgb::from_color(Lch::new(l, 0.0, 0.0));

        Self {
            dark: v.dark,
            bg: generate(v.color_table[PaletteColor::Bg], true),
            fg: generate(fg, true),
            gray: generate(v.color_table[PaletteColor::Gray], false),
            blue: generate(v.color_table[PaletteColor::Blue], false),
            green: generate(v.color_table[PaletteColor::Green], false),
            yellow: generate(v.color_table[PaletteColor::Yellow], false),
            orange: generate(v.color_table[PaletteColor::Orange], false),
            red: generate(v.color_table[PaletteColor::Red], false),
            purple: generate(v.color_table[PaletteColor::Purple], false),
            pink: generate(v.color_table[PaletteColor::Pink], false),
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
