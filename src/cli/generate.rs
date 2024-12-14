use std::{fs::create_dir_all, path::Path};

use clap::{Args, ValueEnum};
use palette::Srgb;

use crate::{
    cli::Cli,
    io::{Setting, ExportExt},
    model::Color,
    optimize::optimize_color_map,
    palette::{BasePalette, FullPalette},
};

#[derive(Debug, Clone, Args)]

pub struct GenerateArgs {
    pub rgb: Srgb<u8>,
    #[arg(short, long)]
    pub no_saturation_fg: bool,
    #[arg(short, long, default_value = "auto")]
    pub color_theme: ColorTheme,
}

#[derive(Debug, Clone, ValueEnum, PartialEq)]
pub enum ColorTheme {
    Auto,
    Dark,
    Light,
}

impl Cli {
    pub fn generate(args: &GenerateArgs) -> anyhow::Result<()> {
        let mut rng = rand::thread_rng();

        let path_prefix = Path::new(".vscode");
        create_dir_all(path_prefix)?;
        let palette_path = path_prefix.join("palette.json");
        let full_palette_path = path_prefix.join("full_palette.json");
        let setting_path = path_prefix.join("settings.json");

        let palette = BasePalette::new(&args.rgb.into(), &args.color_theme, &mut rng);

        let (actual_mode, color_map) = palette.take();

        let color_map = optimize_color_map(
            &color_map,
            &enum_iterator::all::<Color>()
                .filter(|v| v.is_colorized())
                .collect::<Vec<_>>(),
            100,
            &mut rng,
        );

        let palette = BasePalette::from_parts(actual_mode, color_map);
        palette.export(&palette_path)?;

        let full_palette = FullPalette::from(palette);
        full_palette.export(&full_palette_path)?;

        let setting = Setting::new(&full_palette, args.no_saturation_fg);
        setting.export(&setting_path)?;
        Ok(())
    }
}
