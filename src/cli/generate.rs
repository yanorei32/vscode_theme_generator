use std::{fs::create_dir_all, path::Path, str::FromStr};

use clap::{Args, ValueEnum};
use palette::Srgb;

use super::Cli;
use crate::{
    optimize::base_palette::optimize_base_palette,
    palette::{base_palette::BasePalette, full_palette::FullPalette},
    setting::Setting,
};

#[derive(Debug, Clone, Args)]

pub struct GenerateArgs {
    pub rgb: String,
    #[arg(short, long)]
    pub no_saturation_fg: bool,
    #[arg(short, long, default_value = "auto")]
    pub color_theme: ColorTheme,
}

pub struct ParseGenerateArgs {
    pub rgb: Srgb,
    pub no_saturation_fg: bool,
    pub color_theme: ColorTheme,
}

#[derive(Debug, Clone, ValueEnum, PartialEq)]
pub enum ColorTheme {
    Auto,
    Dark,
    Light,
}

impl TryFrom<GenerateArgs> for ParseGenerateArgs {
    type Error = anyhow::Error;

    fn try_from(v: GenerateArgs) -> Result<Self, Self::Error> {
        let rgb = Srgb::from_str(&v.rgb)?.into();
        Ok(Self {
            rgb,
            no_saturation_fg: v.no_saturation_fg,
            color_theme: v.color_theme,
        })
    }
}

impl Cli {
    pub fn generate(args: &GenerateArgs) -> anyhow::Result<()> {
        let args = ParseGenerateArgs::try_from(args.clone())?;
        let mut rng = rand::thread_rng();

        let path_prefix = Path::new(".vscode");
        create_dir_all(path_prefix)?;
        let palette_path = path_prefix.join("palette.json");
        let full_palette_path = path_prefix.join("full_palette.json");
        let setting_path = path_prefix.join("settings.json");

        let mut palette = BasePalette::new(&args.rgb, &args.color_theme, &mut rng);
        optimize_base_palette(&mut palette, &(2..9).collect(), 500, &mut rng)?;
        palette.export(&palette_path)?;

        let full_palette = FullPalette::from(palette);
        full_palette.export(&full_palette_path)?;

        let setting = Setting::new(&full_palette, args.no_saturation_fg);
        setting.export(&setting_path)?;
        Ok(())
    }
}
