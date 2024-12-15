use std::{fs::create_dir_all, path::Path};

use clap::Args;
use palette::Srgb;

use crate::{
    cli::Cli,
    io::{ExportExt, Setting},
    model::{BasePalette, Color, ColorMap, FullPalette, SrgbColorMapExt, ThemeDetectionStrategy},
    optimize::OptimizerExt,
    util::SrgbExt,
};

#[derive(Debug, Clone, Args)]
pub struct GenerateArgs {
    pub rgb: Srgb<u8>,

    #[arg(short, long)]
    pub no_saturation_ui: bool,

    // TODO: ここのリネーム
    #[arg(short, long, default_value = "auto")]
    pub color_theme: ThemeDetectionStrategy,
}

impl Cli {
    pub fn generate(args: &GenerateArgs) -> anyhow::Result<()> {
        let mut rng = rand::thread_rng();

        let path_prefix = Path::new(".vscode");
        create_dir_all(path_prefix)?;
        let palette_path = path_prefix.join("palette.json");
        let full_palette_path = path_prefix.join("full_palette.json");
        let setting_path = path_prefix.join("settings.json");

        let base = Srgb::from(args.rgb);

        let (theme, bg, gray) = base.theme_color_for(args.color_theme);

        // Don't optimize non-colored colors by default, likes Color::Gray
        let pre_optimizing_targets: Vec<_> = Color::colorized_iter().collect();

        let color_map = ColorMap::random_generate_by_color(bg, gray, &mut rng)
            .optimize(&pre_optimizing_targets, &mut rng);

        let palette = BasePalette::new(theme, color_map);

        let full_palette = FullPalette::from(&palette);
        let setting = Setting::new(&full_palette, args.no_saturation_ui);

        palette.export(&palette_path)?;
        full_palette.export(&full_palette_path)?;
        setting.export(&setting_path)?;

        Ok(())
    }
}
