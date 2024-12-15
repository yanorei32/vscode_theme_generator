use std::{fs::create_dir_all, path::Path};

use clap::Args;
use palette::Srgb;

use crate::{
    cli::Cli,
    io::{ExportExt, Setting},
    model::{Color, ColorMap, ThemeDetectionPolicy},
    optimize::OptimizerExt,
    palette::{BasePalette, FullPalette},
    util::{ColorMapExt, SrgbExt},
};

#[derive(Debug, Clone, Args)]
pub struct GenerateArgs {
    pub rgb: Srgb<u8>,

    #[arg(short, long)]
    pub no_saturation_fg: bool,

    #[arg(short, long, default_value = "auto")]
    pub color_theme: ThemeDetectionPolicy,
}

impl Cli {
    pub fn generate(args: &GenerateArgs) -> anyhow::Result<()> {
        let mut rng = rand::thread_rng();

        let path_prefix = Path::new(".vscode");
        create_dir_all(path_prefix)?;
        let palette_path = path_prefix.join("palette.json");
        let full_palette_path = path_prefix.join("full_palette.json");
        let setting_path = path_prefix.join("settings.json");

        let optimize_targets: Vec<_> = Color::colorized_iter().collect();

        let base_color = Srgb::from(args.rgb);

        let (theme, bg, fg) = base_color.theme_color_for(args.color_theme);
        let color_map = ColorMap::random_generate_by_color(bg, fg, &mut rng);

        let palette = BasePalette::new(theme, color_map).optimize(&optimize_targets, &mut rng);
        palette.export(&palette_path)?;

        let full_palette = FullPalette::from(palette);
        full_palette.export(&full_palette_path)?;

        let setting = Setting::new(&full_palette, args.no_saturation_fg);
        setting.export(&setting_path)?;
        Ok(())
    }
}
