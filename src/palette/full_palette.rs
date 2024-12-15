use linearize::StaticMap;
use palette::{FromColor as _, Lch, Srgb};

use crate::{
    model::{Theme, Color},
    palette::BasePalette,
    util::ColorMapExt,
};

#[derive(Debug, Clone)]
pub struct FullPalette {
    pub theme: Theme,

    pub fg: [Srgb; 5],
    pub color_map: StaticMap<Color, [Srgb; 5]>,
}

fn make_variant(rgb: Srgb, theme: Theme, double_width: bool) -> [Srgb; 5] {
    let lch = Lch::from_color(rgb);
    let width_cut = if double_width { 1.0 } else { 2.0 };
    let width = lch.l.min(100.0 - lch.l) / width_cut;

    let mut ls = [
        lch.l + width,
        lch.l + width / 2.0,
        lch.l,
        lch.l - width / 2.0,
        lch.l - width,
    ];

    if theme.dark() {
        ls.reverse();
    }

    ls.map(|l| Srgb::from_color(Lch::new(l, lch.chroma, lch.hue)))
}

impl From<BasePalette> for FullPalette {
    fn from(v: BasePalette) -> Self {
        let (actual_mode, color_map) = v.take();
        let fg = Srgb::from_color(Lch::new(color_map.fg_color_avg_luminouse_chroma().0, 0.0, 0.0));

        Self {
            theme: actual_mode,
            fg: make_variant(fg, actual_mode, true),
            color_map: color_map.map(|k, c| make_variant(c, actual_mode, k == Color::Bg)),
        }
    }
}
