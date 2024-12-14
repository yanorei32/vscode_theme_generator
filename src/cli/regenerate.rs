use std::{fs::create_dir_all, option, path::Path};

use clap::Args;

use crate::{
    model::Color,
    optimize::optimize_color_map,
    palette::{base_palette::BasePalette, full_palette::FullPalette},
    setting::Setting,
    Cli,
};

#[derive(Debug, Clone, Args)]

pub struct RegenerateArgs {
    #[clap(short, long)]
    pub fixs: Vec<String>,
    #[arg(short, long)]
    pub no_saturation_fg: bool,
}

pub struct ParseRegenerateArgs {
    pub fixs: Vec<Color>,
    pub no_saturation_fg: bool,
}

impl TryFrom<RegenerateArgs> for ParseRegenerateArgs {
    type Error = anyhow::Error;

    fn try_from(v: RegenerateArgs) -> Result<Self, Self::Error> {
        Ok(Self {
            fixs: v.fixs.iter().filter_map(|v| v.parse().ok()).collect(),
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

        let (actual_mode, color_map) = BasePalette::load(&palette_path)?
            .renew_colors(&args.fixs, &mut rng)
            .take();

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
