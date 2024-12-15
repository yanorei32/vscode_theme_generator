use palette::{color_difference::Ciede2000, FromColor, Lch, Srgb, WithAlpha};
use rand::{rngs::ThreadRng, Rng};

use crate::model::{Color, ColorMap, HexStr, Theme as T, ThemeDetectionPolicy as TD};

const BLACK: Srgb = Srgb::new(0.0, 0.0, 0.0);
const WHITE: Srgb = Srgb::new(1.0, 1.0, 1.0);

pub trait SrgbExt {
    fn compare(&self, other: &Self) -> f32;
    fn new_with_hue(&self, hue: f32) -> Self;
    fn new_by_random_hue(&self, rng: &mut ThreadRng) -> Self;
    fn theme_color_for(&self, color_theme: &TD) -> (T, Srgb, Srgb);
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

    fn theme_color_for(&self, theme_detection_policy: &TD) -> (T, Srgb, Srgb) {
        let self_lch = Lch::from_color(*self);
        let mid = Srgb::from_color(Lch::new(50.0, 50.0, self_lch.hue));
        let darken = Srgb::from_color(Lch::new(10.0, 10.0, self_lch.hue));
        let whiten = Srgb::from_color(Lch::new(95.0, 5.0, self_lch.hue));

        let (theme, bg, fg, is_default_theme) = if BLACK.compare(self) < 10.5 {
            match theme_detection_policy {
                TD::Auto | TD::Dark => (T::Dark, *self, mid, false),
                TD::Light => (T::Light, whiten, mid, false),
            }
        } else if WHITE.compare(self) < 10.5 {
            match theme_detection_policy {
                TD::Dark => (T::Dark, darken, mid, false),
                TD::Auto | TD::Light => (T::Light, *self, mid, false),
            }
        } else {
            let theme = match theme_detection_policy {
                TD::Auto if 42.0 < darken.compare(self) => T::Dark,
                TD::Auto | TD::Light => T::Light,
                TD::Dark => T::Dark,
            };

            match theme {
                T::Dark => (theme, darken, *self, true),
                T::Light => (theme, whiten, *self, true),
            }
        };

        println!("select {theme} (default: {is_default_theme}) theme");
        (theme, bg, fg)
    }
}

pub trait ColorMapExt {
    fn base_color(&self) -> Srgb;
    fn fg_color_avg_luminouse_chroma(&self) -> (f32, f32);
}

impl ColorMapExt for ColorMap {
    fn base_color(&self) -> Srgb {
        let (l, chroma) = self.fg_color_avg_luminouse_chroma();
        let bg = Lch::from_color(self[Color::Bg]);

        Srgb::from_color(Lch::new(l, chroma, bg.hue))
    }

    fn fg_color_avg_luminouse_chroma(&self) -> (f32, f32) {
        let n = enum_iterator::all::<Color>()
            .filter(|c| !c.is_bg_color())
            .count() as f32;

        self.iter()
            .filter_map(|(k, v)| (!k.is_bg_color()).then_some(v))
            .map(|c| Lch::from_color(*c))
            .map(|c| (c.l / n, c.chroma / n))
            .fold((0.0, 0.0), |(l_acc, c_acc), (l, c)| (l_acc + l, c_acc + c))
    }
}

pub trait ReplaceAlphaExt {
    fn alpha(&self, alpha: f32) -> Self;
}

impl ReplaceAlphaExt for HexStr {
    fn alpha(&self, alpha: f32) -> Self {
        let alpha = (u8::MAX as f32 * alpha) as u8;
        HexStr(self.0.color.with_alpha(alpha))
    }
}
