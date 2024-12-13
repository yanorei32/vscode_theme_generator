use std::{
    fs::File,
    io::{Read, Write},
    path::Path,
    str::FromStr,
};

use enum_iterator::{all, Sequence};
use linearize::{static_map, Linearize, StaticMap};
use palette::{FromColor, Lch, Srgb};
use rand::rngs::ThreadRng;
use serde::{Deserialize, Serialize};

use crate::{
    cli::generate::ColorTheme,
    color::util::{generate_base, SrgbExt},
    model::ActualThemeMode,
};

use super::wrap::wrap_base_palette::WrapBasePalette;

#[derive(Debug, Clone, Copy, Linearize, Eq, PartialEq, Sequence, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PaletteColor {
    Bg,
    Gray,
    Blue,
    Green,
    Yellow,
    Orange,
    Red,
    Purple,
    Pink,
}

impl PaletteColor {
    pub fn is_bg_color(&self) -> bool {
        *self == Self::Bg
    }

    pub fn is_colorized(&self) -> bool {
        !matches!(self, Self::Bg | Self::Gray)
    }
}

impl FromStr for PaletteColor {
    type Err = ();
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Ok(match value {
            "bg" => Self::Bg,
            "gray" => Self::Gray,
            "blue" => Self::Blue,
            "green" => Self::Green,
            "yellow" => Self::Yellow,
            "orange" => Self::Orange,
            "red" => Self::Red,
            "purple" => Self::Purple,
            "pink" => Self::Pink,
            _ => return Err(()),
        })
    }
}

#[derive(Debug, Clone)]
pub struct BasePalette {
    pub actual_mode: ActualThemeMode,
    pub color_table: StaticMap<PaletteColor, Srgb>,
    pub score: f32,
}

impl From<WrapBasePalette> for BasePalette {
    fn from(v: WrapBasePalette) -> Self {
        let mut palette = Self {
            actual_mode: if v.dark {
                ActualThemeMode::Dark
            } else {
                ActualThemeMode::Light
            },
            color_table: v.color_table.map_values(|v| v.into()),
            score: 0.0,
        };
        palette.calc_full_score();
        palette
    }
}

impl BasePalette {
    pub fn new(base_rgb: &Srgb, color_theme: &ColorTheme, rng: &mut ThreadRng) -> Self {
        let (actual_mode, bg, fg) = generate_base(base_rgb, color_theme);

        let color_table = static_map! {
            PaletteColor::Bg => bg,
            PaletteColor::Gray => fg,
            // TODO: これで動いてるか確認する
            _ => fg.new_by_random_hue(rng),
        };

        let mut palette = Self {
            actual_mode,
            color_table,
            score: 0.0,
        };

        palette.calc_full_score();

        palette
    }

    pub fn export(&self, path: &Path) -> anyhow::Result<()> {
        let wrap_palette = WrapBasePalette::from(self.clone());
        let palette_str = serde_json::to_string(&wrap_palette)?;
        let mut palette_file = File::create(path)?;
        writeln!(palette_file, "{}", palette_str)?;
        Ok(())
    }

    pub fn load(path: &Path) -> anyhow::Result<Self> {
        let mut palette_file = File::open(path).expect("file not found");
        let mut palette_str = String::new();
        palette_file.read_to_string(&mut palette_str)?;
        let wrap_palette = serde_json::from_str::<WrapBasePalette>(&palette_str)?;
        let mut palette: Self = wrap_palette.into();
        palette.calc_full_score();
        Ok(palette)
    }

    pub fn renew(&mut self, change_palette_element: &[PaletteColor], rng: &mut ThreadRng) {
        let (l, chroma) = self.fg_average();
        let bg = Lch::from_color(self.color_table[PaletteColor::Bg]);
        let base_rgb = Srgb::from_color(Lch::new(l, chroma, bg.hue));

        let (actual_mode, bg, _) = generate_base(&base_rgb, &ColorTheme::Auto);
        self.actual_mode = actual_mode;

        for color in change_palette_element {
            if color.is_bg_color() {
                self.update_color(*color, bg);
            } else {
                let select_rgb = base_rgb.new_by_random_hue(rng);
                self.update_color(*color, select_rgb);
            }
        }
    }

    pub fn calc_full_score(&mut self) {
        // TODO: これが期待通りに動いているか確認する
        use itertools::Itertools;

        self.score = all::<PaletteColor>()
            .tuple_combinations()
            .map(|(a, b)| {
                let diff = self.color_table[a].compare(&self.color_table[b]);
                let p = if a.is_bg_color() { 42.0 } else { 21.0 };

                (diff / p).log10().min(0.0) * 1000000.0
            })
            .sum();

        let (l_ave, chroma_ave) = self.fg_average();

        let (l_point, chroma_point) = self
            .color_table
            .iter()
            .filter(|(k, _)| !k.is_bg_color())
            .fold((0.0, 0.0), |sum, (_, value)| {
                let lch = Lch::from_color(*value);
                (
                    sum.0 + ((lch.l - l_ave).abs() - 5.0).max(0.0).powf(2.0),
                    sum.1 + ((lch.chroma - chroma_ave).abs() - 5.0).max(0.0).powf(2.0),
                )
            });

        self.score -= l_point * 10000000.0 + chroma_point * 10000000.0;
    }

    pub fn get_color(&self, c: PaletteColor) -> Srgb {
        self.color_table[c]
    }

    pub fn update_color(&mut self, c: PaletteColor, color: Srgb) {
        self.color_table[c] = color;
        self.calc_full_score();
    }

    pub fn fg_average(&self) -> (f32, f32) {
        let n = all::<PaletteColor>().filter(|c| c.is_colorized()).count() as f32;

        self.color_table
            .iter()
            .filter(|(k, _)| !k.is_bg_color())
            .fold((0.0, 0.0), |sum, (_, value)| {
                let lch = Lch::from_color(*value);
                (sum.0 + lch.l / n, sum.1 + lch.chroma / n)
            })
    }
}
