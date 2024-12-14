use linearize::StaticMap;
use palette::{color_difference::Ciede2000, FromColor, Lch, Srgb, WithAlpha};
use rand::{rngs::ThreadRng, Rng};

use crate::{
    cli::generate::ColorTheme as CT,
    model::{ActualThemeMode as AT, Color, HexStr},
};

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

pub trait ReplaceAlphaExt {
    fn alpha(&self, alpha: f32) -> Self;
}

impl ReplaceAlphaExt for HexStr {
    fn alpha(&self, alpha: f32) -> Self {
        let alpha = (u8::MAX as f32 * alpha) as u8;
        HexStr(self.0.color.with_alpha(alpha))
    }
}

pub trait ColorMapExt {
    fn base_color(&self) -> Srgb;
    fn fg_color_luminouse_chroma(&self) -> (f32, f32);
}

impl ColorMapExt for StaticMap<Color, Srgb> {
    fn base_color(&self) -> Srgb {
        let (l, chroma) = self.fg_color_luminouse_chroma();
        let bg = Lch::from_color(self[Color::Bg]);

        Srgb::from_color(Lch::new(l, chroma, bg.hue))
    }

    fn fg_color_luminouse_chroma(&self) -> (f32, f32) {
        let n = enum_iterator::all::<Color>()
            .filter(|c| !c.is_bg_color())
            .count() as f32;

        self.iter()
            .filter(|(k, _)| !k.is_bg_color())
            .fold((0.0, 0.0), |(l, chroma), (_, value)| {
                let lch = Lch::from_color(*value);
                (l + lch.l / n, chroma + lch.chroma / n)
            })
    }
}
