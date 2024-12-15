use linearize::{StaticCopyMap, static_copy_map};
use palette::{Srgb, Lch, FromColor};

use crate::{
    util::SrgbExt,
    model::Color,
};

pub type ColorMap<T> = StaticCopyMap<Color, T>;

pub trait SrgbColorMapExt {
    fn random_generate_by_color<R: rand::Rng, CGray: Into<Srgb> + Copy, CBg: Into<Srgb> + Copy>(
        bg_color: CBg,
        cgray_color: CGray,
        rng: &mut R,
    ) -> Self;

    fn base_color(&self) -> Srgb;
    fn fg_color_avg_luminouse_chroma(&self) -> (f32, f32);
}

impl SrgbColorMapExt for ColorMap<Srgb> {
    fn random_generate_by_color<R: rand::Rng, CGray: Into<Srgb> + Copy, CBg: Into<Srgb> + Copy>(
        bg: CBg,
        gray: CGray,
        rng: &mut R,
    ) -> Self {
        let gray = gray.into();
        let bg = bg.into();

        static_copy_map! {
            Color::Bg => bg,
            Color::Gray => gray,
            _ => gray.new_by_random_hue(rng),
        }
    }

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
