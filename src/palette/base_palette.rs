use std::{fs::File, io::Write, path::Path};

use linearize::{static_map, StaticMap};
use palette::Srgb;
use rand::rngs::ThreadRng;

use crate::{
    cli::generate::ColorTheme,
    model::{ActualThemeMode, Color, ColorMap},
    schema::BasePaletteFile,
    util::{ColorMapExt, SrgbExt},
};

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
        let palette = BasePaletteFile::from(self.clone());
        let palette = serde_json::to_string(&palette)?;
        File::create(path)?.write_all(palette.as_bytes())?;
        Ok(())
    }

    pub fn load(path: &Path) -> anyhow::Result<Self> {
        let palette = std::fs::read_to_string(path)?;
        let palette: BasePaletteFile = serde_json::from_str(&palette)?;
        Ok(palette.into())
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

impl From<BasePaletteFile> for BasePalette {
    fn from(v: BasePaletteFile) -> Self {
        let actual_mode = if v.dark {
            ActualThemeMode::Dark
        } else {
            ActualThemeMode::Light
        };

        let color_map = v.color_map.map_values(|v| v.0.color.into());

        Self::from_parts(actual_mode, color_map)
    }
}
