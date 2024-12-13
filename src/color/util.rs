use palette::{color_difference::Ciede2000, FromColor, Lch, Srgb};
use rand::{rngs::ThreadRng, Rng};

use crate::cli::generate::ColorTheme as CT;
use crate::model::ActualThemeMode as AT;

const BLACK: Srgb = Srgb::new(0.0, 0.0, 0.0);
const WHITE: Srgb = Srgb::new(1.0, 1.0, 1.0);

pub trait SrgbExt {
    fn compare(&self, other: &Self) -> f32;
    fn new_with_hue(&self, hue: f32) -> Self;
    fn new_by_random_hue(&self, rng: &mut ThreadRng) -> Self;
    fn theme_color_for(&self, color_theme: &CT) -> (AT, Srgb, Srgb);
}

impl SrgbExt for Srgb {
    fn compare(&self, other: &Self) -> f32 {
        Lch::from_color(*self).difference(Lch::from_color(*other))
    }

    fn new_with_hue(&self, hue: f32) -> Self {
        let base_lch = Lch::from_color(*self);
        Self::from_color(Lch::new(base_lch.l, base_lch.chroma, hue))
    }

    fn new_by_random_hue(&self, rng: &mut ThreadRng) -> Self {
        let hue = rng.gen_range(0.0..360.0);
        self.new_with_hue(hue)
    }

    fn theme_color_for(&self, color_theme: &CT) -> (AT, Srgb, Srgb) {
        let self_lch = Lch::from_color(*self);
        let mid = Srgb::from_color(Lch::new(50.0, 50.0, self_lch.hue));
        let darken = Srgb::from_color(Lch::new(10.0, 10.0, self_lch.hue));
        let whiten = Srgb::from_color(Lch::new(95.0, 5.0, self_lch.hue));

        let (actual_mode, bg, fg, is_default_theme) = if BLACK.compare(self) < 10.5 {
            match color_theme {
                CT::Auto | CT::Dark => (AT::Dark, *self, mid, false),
                CT::Light => (AT::Light, whiten, mid, false),
            }
        } else if WHITE.compare(self) < 10.5 {
            match color_theme {
                CT::Dark => (AT::Dark, darken, mid, false),
                CT::Auto | CT::Light => (AT::Light, *self, mid, false),
            }
        } else {
            let actual_mode = match color_theme {
                CT::Auto if 42.0 < darken.compare(self) => AT::Dark,
                CT::Auto | CT::Light => AT::Light,
                CT::Dark => AT::Dark,
            };

            match actual_mode {
                AT::Dark => (actual_mode, darken, *self, true),
                AT::Light => (actual_mode, whiten, *self, true),
            }
        };

        println!("select {actual_mode} (default: {is_default_theme}) theme");
        (actual_mode, bg, fg)
    }
}
