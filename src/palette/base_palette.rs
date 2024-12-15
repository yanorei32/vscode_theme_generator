use rand::rngs::ThreadRng;

use crate::{
    model::{Color, ColorMap, Theme, ThemeDetectionPolicy},
    util::{ColorMapExt, SrgbExt},
};

#[derive(Debug, Clone, Copy)]
pub struct BasePalette {
    theme: Theme,
    color_map: ColorMap,
}

impl BasePalette {
    pub fn new(theme: Theme, color_map: ColorMap) -> Self {
        Self { theme, color_map }
    }

    // TODO: まだデータに足が生えて歩きだしてる。
    pub fn randomize_colors(&self, renew_targets: &[Color], rng: &mut ThreadRng) -> Self {
        let base = self.color_map.base_color();
        let (theme, bg, _) = base.theme_color_for(ThemeDetectionPolicy::Auto);

        let mut color_map = self.color_map.clone();

        for &target in renew_targets {
            if target.is_bg_color() {
                color_map[target] = bg;
            } else {
                color_map[target] = base.new_by_random_hue(rng);
            }
        }

        Self::new(theme, color_map)
    }

    pub fn theme(&self) -> Theme {
        self.theme
    }

    pub fn take(self) -> (Theme, ColorMap) {
        (self.theme, self.color_map)
    }
}
