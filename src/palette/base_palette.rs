use linearize::static_map;
use palette::Srgb;
use rand::rngs::ThreadRng;

use crate::{
    cli::generate::ColorTheme,
    model::{ActualThemeMode, Color, ColorMap},
    util::{ColorMapExt, SrgbExt},
};

#[derive(Debug, Clone)]
pub struct BasePalette {
    actual_mode: ActualThemeMode,
    color_map: ColorMap,
}

impl BasePalette {
    pub fn new(base_rgb: &Srgb, color_theme: &ColorTheme, rng: &mut ThreadRng) -> Self {
        let (actual_mode, bg, fg) = base_rgb.theme_color_for(color_theme);

        // TODO: これで動いてるか確認する
        let color_map = static_map! {
            Color::Bg => bg,
            Color::Gray => fg,
            _ => fg.new_by_random_hue(rng),
        };

        Self::from_parts(actual_mode, color_map)
    }

    pub fn from_parts(actual_mode: ActualThemeMode, color_map: ColorMap) -> Self {
        Self {
            actual_mode,
            color_map,
        }
    }

    pub fn renew_colors(&self, targets: &[Color], rng: &mut ThreadRng) -> Self {
        let base = self.color_map.base_color();
        let (actual_mode, bg, _) = base.theme_color_for(&ColorTheme::Auto);

        let mut color_map = self.color_map.clone();

        for color in targets {
            if color.is_bg_color() {
                color_map[*color] = bg;
            } else {
                color_map[*color] = base.new_by_random_hue(rng);
            }
        }

        Self::from_parts(actual_mode, color_map)
    }

    pub fn dark(&self) -> bool {
        self.actual_mode == ActualThemeMode::Dark
    }

    pub fn take(self) -> (ActualThemeMode, ColorMap) {
        (self.actual_mode, self.color_map)
    }
}
