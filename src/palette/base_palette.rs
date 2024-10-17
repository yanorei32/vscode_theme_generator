use std::{fs::File, io::Write, path::PathBuf};

use palette::{FromColor, Lch, Srgb};
use rand::rngs::ThreadRng;

use crate::{
    cli::generate::ColorTheme,
    color::util::{compare, generate_base, generate_random_color},
};

use super::wrap::wrap_base_palette::WrapBasePalette;

#[derive(Debug, Clone, Copy)]
pub struct BasePalette {
    pub dark: bool,
    pub bg: Srgb,
    pub gray: Srgb,
    pub blue: Srgb,
    pub green: Srgb,
    pub yellow: Srgb,
    pub orange: Srgb,
    pub red: Srgb,
    pub purple: Srgb,
    pub pink: Srgb,
    pub score: f32,
}

impl BasePalette {
    pub fn new(base_rgb: &Srgb, color_theme: &ColorTheme, rng: &mut ThreadRng) -> Self {
        let (dark, bg, fg) = generate_base(base_rgb, color_theme);

        let gray = fg;
        let blue = generate_random_color(fg, rng);
        let green = generate_random_color(fg, rng);
        let yellow = generate_random_color(fg, rng);
        let orange = generate_random_color(fg, rng);
        let red = generate_random_color(fg, rng);
        let purple = generate_random_color(fg, rng);
        let pink = generate_random_color(fg, rng);

        let mut palette = Self {
            dark,
            bg,
            gray,
            blue,
            green,
            yellow,
            orange,
            red,
            purple,
            pink,
            score: 0.0,
        };
        palette.calc_full_score();
        palette
    }

    pub fn export(&self, path: &PathBuf) -> anyhow::Result<()> {
        let wrap_palette: WrapBasePalette = (*self).into();
        let palette_str = serde_json::to_string(&wrap_palette)?;
        let mut palette_file = File::create(path)?;
        writeln!(palette_file, "{}", palette_str)?;
        Ok(())
    }

    pub fn calc_full_score(&mut self) {
        self.score = 0.0;

        for color_i in 0..9 {
            for color_j in color_i + 1..9 {
                let p = if color_i == 0 { 42.0 } else { 21.0 };
                self.score += (compare(&self.get_color(color_i), &self.get_color(color_j)) / p)
                    .log10()
                    .min(0.0)
                    * 1000000.0;
            }
        }
        let (l_ave, chroma_ave) = (1..9).fold((0.0, 0.0), |sum, idx: usize| {
            let lch = Lch::from_color(self.get_color(idx));
            (sum.0 + lch.l / 8.0, sum.1 + lch.chroma / 8.0)
        });
        let (l_point, chroma_point) = (1..9).fold((0.0, 0.0), |sum, idx: usize| {
            let lch = Lch::from_color(self.get_color(idx));
            (
                sum.0 + ((lch.l - l_ave).abs() - 5.0).max(0.0).powf(2.0),
                sum.1 + ((lch.chroma - chroma_ave).abs() - 5.0).max(0.0).powf(2.0),
            )
        });
        self.score -= l_point * 10000000.0 + chroma_point * 10000000.0;
    }

    pub fn get_color(&self, idx: usize) -> Srgb {
        let rgbs = [
            &self.bg,
            &self.gray,
            &self.blue,
            &self.green,
            &self.yellow,
            &self.orange,
            &self.red,
            &self.purple,
            &self.pink,
        ];
        *rgbs[idx]
    }

    fn set_color(&mut self, idx: usize, color: Srgb) {
        let rgbs = [
            &mut self.bg,
            &mut self.gray,
            &mut self.blue,
            &mut self.green,
            &mut self.yellow,
            &mut self.orange,
            &mut self.red,
            &mut self.purple,
            &mut self.pink,
        ];
        *rgbs[idx] = color
    }

    pub fn update_color(&mut self, idx: usize, color: Srgb) {
        self.set_color(idx, color);
        self.calc_full_score();
    }

    pub fn average(&self) -> (f32, f32) {
        let (l, chroma) = (1..9).fold((0.0, 0.0), |sum, idx: usize| {
            let lch = Lch::from_color(self.get_color(idx));
            (sum.0 + lch.l / 8.0, sum.1 + lch.chroma / 8.0)
        });
        (l, chroma)
    }
}
