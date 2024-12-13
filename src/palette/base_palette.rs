use std::{
    fs::File,
    io::{Read, Write},
    path::PathBuf,
    str::FromStr,
};

use enum_iterator::{all, Sequence};
use enum_map::Enum;
use palette::{FromColor, Lch, Srgb};
use rand::rngs::ThreadRng;

use crate::{
    cli::generate::ColorTheme,
    color::util::{compare, generate_base, generate_random_color},
};

use super::wrap::wrap_base_palette::WrapBasePalette;

#[derive(Debug, Clone, Copy, Enum, Eq, PartialEq, Sequence)]
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
        match self {
            Self::Bg | Self::Gray => false,
            _ => true,
        }
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
    pub dark: bool,
    pub color_table: enum_map::EnumMap<PaletteColor, Srgb>,
    pub score: f32,
}

impl From<WrapBasePalette> for BasePalette {
    fn from(v: WrapBasePalette) -> Self {
        let mut palette = Self {
            dark: v.dark,
            color_table: enum_map::enum_map! {
                PaletteColor::Bg => v.bg.into() ,
                PaletteColor::Gray => v.gray.into(),
                PaletteColor::Blue => v.blue.into(),
                PaletteColor::Green => v.green.into(),
                PaletteColor::Yellow => v.yellow.into(),
                PaletteColor::Orange => v.orange.into(),
                PaletteColor::Red => v.red.into(),
                PaletteColor::Purple => v.purple.into(),
                PaletteColor::Pink => v.pink.into(),
            },
            score: 0.0,
        };
        palette.calc_full_score();
        palette
    }
}

impl BasePalette {
    pub fn new(base_rgb: &Srgb, color_theme: &ColorTheme, rng: &mut ThreadRng) -> Self {
        let (dark, bg, fg) = generate_base(base_rgb, color_theme);

        let color_table = enum_map::enum_map! {
            PaletteColor::Bg => bg,
            PaletteColor::Gray => fg,
            PaletteColor::Blue => generate_random_color(fg, rng),
            PaletteColor::Green => generate_random_color(fg, rng),
            PaletteColor::Yellow => generate_random_color(fg, rng),
            PaletteColor::Orange => generate_random_color(fg, rng),
            PaletteColor::Red => generate_random_color(fg, rng),
            PaletteColor::Purple => generate_random_color(fg, rng),
            PaletteColor::Pink => generate_random_color(fg, rng),
        };

        let mut palette = Self {
            dark,
            color_table,
            score: 0.0,
        };

        palette.calc_full_score();

        palette
    }

    pub fn export(&self, path: &PathBuf) -> anyhow::Result<()> {
        let wrap_palette = WrapBasePalette::from(self.clone());
        let palette_str = serde_json::to_string(&wrap_palette)?;
        let mut palette_file = File::create(path)?;
        writeln!(palette_file, "{}", palette_str)?;
        Ok(())
    }

    pub fn load(path: &PathBuf) -> anyhow::Result<Self> {
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
        let (dark, bg, _) = generate_base(&base_rgb, &ColorTheme::Auto);
        self.dark = dark;

        for color in change_palette_element {
            if color.is_bg_color() {
                self.update_color(*color, bg);
            } else {
                let select_rgb = generate_random_color(base_rgb, rng);
                self.update_color(*color, select_rgb);
            }
        }
    }

    pub fn calc_full_score(&mut self) {
        self.score = 0.0;

        for (nth, color_a) in all::<PaletteColor>().enumerate() {
            for color_b in all::<PaletteColor>().skip(nth + 1) {
                let p = if color_a.is_bg_color() { 42.0 } else { 21.0 };
                self.score += (compare(&self.get_color(color_a), &self.get_color(color_b)) / p)
                    .log10()
                    .min(0.0)
                    * 1000000.0;
            }
        }

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

    fn set_color(&mut self, c: PaletteColor, color: Srgb) {
        self.color_table[c] = color;
    }

    pub fn update_color(&mut self, c: PaletteColor, color: Srgb) {
        self.set_color(c, color);
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
