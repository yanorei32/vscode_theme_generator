use palette::{color_difference::Ciede2000, FromColor, Lch, Srgb};
use rand::{rngs::ThreadRng, Rng};

use crate::cli::generate::ColorTheme;

pub fn generate_random_color(base_rgb: Srgb, rng: &mut ThreadRng) -> Srgb {
    let hue = rng.gen_range(0.0..360.0);
    generate_color(base_rgb, hue)
}

pub fn generate_color(base_rgb: Srgb, hue: f32) -> Srgb {
    let base_lch = Lch::from_color(base_rgb);
    Srgb::from_color(Lch::new(base_lch.l, base_lch.chroma, hue))
}

pub fn compare(this: &Srgb, other: &Srgb) -> f32 {
    let this = Lch::from_color(*this);
    let other = Lch::from_color(*other);
    this.difference(other)
}

pub fn generate_base(base_rgb: &Srgb, color_theme: &ColorTheme) -> (bool, Srgb, Srgb) {
    let black = Srgb::new(0.0, 0.0, 0.0);
    let white: palette::rgb::Rgb = Srgb::new(1.0, 1.0, 1.0);
    let base_lch = Lch::from_color(*base_rgb);
    if compare(&black, base_rgb) < 10.5 {
        let (dark, bg, fg) = match color_theme {
            ColorTheme::Auto | ColorTheme::Dark => (
                true,
                *base_rgb,
                Srgb::from_color(Lch::new(50.0, 50.0, base_lch.hue)),
            ),
            ColorTheme::Light => (
                false,
                Srgb::from_color(Lch::new(95.0, 5.0, base_lch.hue)),
                Srgb::from_color(Lch::new(50.0, 50.0, base_lch.hue)),
            ),
        };
        if dark {
            println!("select dark default theme");
        } else {
            println!("select light default theme");
        }
        return (dark, bg, fg);
    }
    if compare(&white, base_rgb) < 10.5 {
        let (dark, bg, fg) = match color_theme {
            ColorTheme::Dark => (
                true,
                Srgb::from_color(Lch::new(10.0, 10.0, base_lch.hue)),
                Srgb::from_color(Lch::new(50.0, 50.0, base_lch.hue)),
            ),
            ColorTheme::Auto | ColorTheme::Light => (
                false,
                *base_rgb,
                Srgb::from_color(Lch::new(50.0, 50.0, base_lch.hue)),
            ),
        };
        if dark {
            println!("select dark default theme");
        } else {
            println!("select light default theme");
        }
        return (dark, bg, fg);
    }

    let base_lch: Lch = Lch::from_color(*base_rgb);
    let bg = Srgb::from_color(Lch::new(
        base_lch.l.min(10.0),
        base_lch.chroma.min(10.0),
        base_lch.hue,
    ));
    let dark = match color_theme {
        ColorTheme::Auto => 42.0 < compare(&bg, base_rgb),
        ColorTheme::Dark => true,
        ColorTheme::Light => false,
    };

    let bg = if dark {
        println!("select dark theme");
        bg
    } else {
        println!("select light theme");
        Srgb::from_color(Lch::new(
            base_lch.l.max(95.0),
            base_lch.chroma.min(5.0),
            base_lch.hue,
        ))
    };
    let fg = *base_rgb;
    (dark, bg, fg)
}
