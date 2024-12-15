use std::{fs::create_dir_all, path::Path};

use clap::Args;
use palette::Srgb;

use crate::{
    cli::Cli,
    io::{ExportExt, Setting},
    model::{Color, ThemeDetectionPolicy},
    optimize::OptimizerExt,
    palette::{BasePalette, FullPalette},
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

        let palette = BasePalette::new(&args.rgb.into(), &args.color_theme, &mut rng)
            .optimize(&optimize_targets, &mut rng);

        palette.export(&palette_path)?;

        let full_palette = FullPalette::from(palette);
        full_palette.export(&full_palette_path)?;

        let setting = Setting::new(&full_palette, args.no_saturation_fg);
        setting.export(&setting_path)?;
        Ok(())
    }
}
