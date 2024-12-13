use palette::{color_difference::Ciede2000, FromColor, Lch, Srgb};
use rand::{rngs::ThreadRng, Rng};

use crate::cli::generate::ColorTheme;
use crate::model::*;

pub trait SrgbExt {
    fn compare(&self, other: &Self) -> f32;
    fn new_with_hue(&self, hue: f32) -> Self;
    fn new_by_random_hue(&self, rng: &mut ThreadRng) -> Self;
}

impl SrgbExt for Srgb {
    fn compare(&self, other: &Self) -> f32 {
        let this = Lch::from_color(*self);
        let other = Lch::from_color(*other);
        this.difference(other)
    }

    fn new_with_hue(&self, hue: f32) -> Self {
        let base_lch = Lch::from_color(*self);
        Self::from_color(Lch::new(base_lch.l, base_lch.chroma, hue))
    }

    fn new_by_random_hue(&self, rng: &mut ThreadRng) -> Self {
        let hue = rng.gen_range(0.0..360.0);
        self.new_with_hue(hue)
    }
}

pub fn generate_base(base_rgb: &Srgb, color_theme: &ColorTheme) -> (ActualThemeMode, Srgb, Srgb) {
    let base_lch = Lch::from_color(*base_rgb);

    let black = Srgb::new(0.0, 0.0, 0.0);
    if black.compare(base_rgb) < 10.5 {
        let (actual_mode, bg, fg) = match color_theme {
            ColorTheme::Auto | ColorTheme::Dark => (
                ActualThemeMode::Dark,
                *base_rgb,
                Srgb::from_color(Lch::new(50.0, 50.0, base_lch.hue)),
            ),
            ColorTheme::Light => (
                ActualThemeMode::Light,
                Srgb::from_color(Lch::new(95.0, 5.0, base_lch.hue)),
                Srgb::from_color(Lch::new(50.0, 50.0, base_lch.hue)),
            ),
        };
        println!("select {actual_mode} default theme");
        return (actual_mode, bg, fg);
    }

    let white = Srgb::new(1.0, 1.0, 1.0);
    if white.compare(base_rgb) < 10.5 {
        let (actual_mode, bg, fg) = match color_theme {
            ColorTheme::Dark => (
                ActualThemeMode::Dark,
                Srgb::from_color(Lch::new(10.0, 10.0, base_lch.hue)),
                Srgb::from_color(Lch::new(50.0, 50.0, base_lch.hue)),
            ),
            ColorTheme::Auto | ColorTheme::Light => (
                ActualThemeMode::Light,
                *base_rgb,
                Srgb::from_color(Lch::new(50.0, 50.0, base_lch.hue)),
            ),
        };

        println!("select {actual_mode} default theme");
        return (actual_mode, bg, fg);
    }

    let bg = Srgb::from_color(Lch::new(
        base_lch.l.min(10.0),
        base_lch.chroma.min(10.0),
        base_lch.hue,
    ));

    let actual_mode = match color_theme {
        ColorTheme::Auto if 42.0 < bg.compare(base_rgb) => ActualThemeMode::Dark,
        ColorTheme::Auto => ActualThemeMode::Light,
        ColorTheme::Dark => ActualThemeMode::Dark,
        ColorTheme::Light => ActualThemeMode::Light,
    };

    println!("select {actual_mode} theme");

    let bg = if actual_mode == ActualThemeMode::Dark {
        bg
    } else {
        Srgb::from_color(Lch::new(
            base_lch.l.max(95.0),
            base_lch.chroma.min(5.0),
            base_lch.hue,
        ))
    };

    let fg = *base_rgb;
    (actual_mode, bg, fg)
}
