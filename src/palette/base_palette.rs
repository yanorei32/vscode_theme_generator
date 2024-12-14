use std::{
    fs::File,
    io::{Read, Write},
    path::Path,
};

use linearize::{static_map, StaticMap};
use palette::Srgb;
use rand::rngs::ThreadRng;

use crate::{
    cli::generate::ColorTheme,
    model::{ActualThemeMode, Color, ColorMap},
    util::{ColorMapExt, SrgbExt},
};

use super::wrap::wrap_base_palette::WrapBasePalette;

#[derive(Debug, Clone)]
pub struct BasePalette {
    actual_mode: ActualThemeMode,
    color_map: StaticMap<Color, Srgb>,
}

impl BasePalette {
    pub fn new(base_rgb: &Srgb, color_theme: &ColorTheme, rng: &mut ThreadRng) -> Self {
        let (actual_mode, bg, fg) = base_rgb.theme_color_for(color_theme);

        // TODO: これで動いてるか確認する
        let color_table = static_map! {
            Color::Bg => bg,
            Color::Gray => fg,
            _ => fg.new_by_random_hue(rng),
        };

        Self::from_parts(actual_mode, color_table)
    }

    pub fn from_parts(actual_mode: ActualThemeMode, color_map: StaticMap<Color, Srgb>) -> Self {
        Self {
            actual_mode,
            color_map,
        }
    }

    pub fn export(&self, path: &Path) -> anyhow::Result<()> {
        let wrap_palette = WrapBasePalette::from(self.clone());
        let palette_str = serde_json::to_string(&wrap_palette)?;
        let mut palette_file = File::create(path)?;
        writeln!(palette_file, "{}", palette_str)?;
        Ok(())
    }

    pub fn load(path: &Path) -> anyhow::Result<Self> {
        let mut palette_file = File::open(path).expect("file not found");
        let mut palette_str = String::new();
        palette_file.read_to_string(&mut palette_str)?;
        let wrap_palette = serde_json::from_str::<WrapBasePalette>(&palette_str)?;
        Ok(wrap_palette.into())
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

impl From<WrapBasePalette> for BasePalette {
    fn from(v: WrapBasePalette) -> Self {
        let actual_mode = if v.dark {
            ActualThemeMode::Dark
        } else {
            ActualThemeMode::Light
        };

        let color_map = v.color_map.map_values(|v| v.0.color.into());

        Self::from_parts(actual_mode, color_map)
    }
}
