use std::{fs::create_dir_all, path::Path};

use clap::Args;

use super::Cli;
use crate::{
    optimize::base_palette::optimize_base_palette,
    palette::{base_palette::BasePalette, full_palette::FullPalette},
    setting::Setting,
};

#[derive(Debug, Clone, Args)]

pub struct RegenerateArgs {
    #[clap(short, long)]
    pub fixs: Vec<String>,
    #[arg(short, long)]
    pub no_saturation_fg: bool,
}

pub struct ParseRegenerateArgs {
    pub fixs: Vec<usize>,
    pub no_saturation_fg: bool,
}

impl TryFrom<RegenerateArgs> for ParseRegenerateArgs {
    type Error = anyhow::Error;

    fn try_from(v: RegenerateArgs) -> Result<Self, Self::Error> {
        Ok(Self {
            fixs: BasePalette::parse_id(&v.fixs),
            no_saturation_fg: v.no_saturation_fg,
        })
    }
}

impl Cli {
    pub fn regenerate(args: &RegenerateArgs) -> anyhow::Result<()> {
        let args = ParseRegenerateArgs::try_from(args.clone())?;
        let mut rng = rand::thread_rng();

        let path_prefix = Path::new(".vscode");
        create_dir_all(path_prefix)?;
        let palette_path = path_prefix.join("palette.json");
        let full_palette_path = path_prefix.join("full_palette.json");
        let setting_path = path_prefix.join("settings.json");

        let mut palette = BasePalette::load(&palette_path)?;
        if !args.fixs.is_empty() {
            palette.renew(&args.fixs, &mut rng);
            optimize_base_palette(&mut palette, args.fixs, 100, &mut rng)?;
        }
        palette.export(&palette_path)?;

        let full_palette = FullPalette::from(palette);
        full_palette.export(&full_palette_path)?;

        let setting = Setting::new(&full_palette, args.no_saturation_fg);
        setting.export(&setting_path)?;
        Ok(())
    }
}
