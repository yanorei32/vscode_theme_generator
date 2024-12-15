use linearize::StaticCopyMap;
use palette::{FromColor, IntoColor, Lch, Srgb};

use crate::model::{BasePalette, Color, ColorMapExt, Theme};

#[derive(Debug, Clone, Copy)]
pub struct FullPalette {
    pub theme: Theme,

    pub fg: [Srgb; 5],
    pub color_map: StaticCopyMap<Color, [Srgb; 5]>,
}

fn make_variant<I: IntoColor<Lch>, O: FromColor<Lch>>(
    i: I,
    theme: Theme,
    double_width: bool,
) -> [O; 5] {
    let lch: Lch = i.into_color();

    let width_cut = if double_width { 1.0 } else { 2.0 };
    let width = lch.l.min(100.0 - lch.l) / width_cut;

    let mut variants = [
        lch.l + width,
        lch.l + width / 2.0,
        lch.l,
        lch.l - width / 2.0,
        lch.l - width,
    ];

    if theme.dark() {
        variants.reverse();
    }

    variants.map(|l| O::from_color(Lch::new(l, lch.chroma, lch.hue)))
}

impl From<BasePalette> for FullPalette {
    fn from(v: BasePalette) -> Self {
        let (actual_mode, color_map) = v.take();

        let fg = Srgb::from_color(Lch::new(
            color_map.fg_color_avg_luminouse_chroma().0,
            0.0,
            0.0,
        ));

        Self {
            theme: actual_mode,
            fg: make_variant(fg, actual_mode, true),
            color_map: color_map.map(|k, c| make_variant(c, actual_mode, k == Color::Bg)),
        }
    }
}
