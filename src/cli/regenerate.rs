use std::{fs::create_dir_all, path::Path};

use clap::Args;

use crate::{
    io::{ExportExt, LoadExt, Setting},
    model::Color,
    optimize::optimize_color_map,
    palette::{BasePalette, FullPalette},
    Cli,
};

#[derive(Debug, Clone, Args)]

pub struct RegenerateArgs {
    #[clap(short, long)]
    pub fixs: Vec<Color>,
    #[arg(short, long)]
    pub no_saturation_fg: bool,
}

impl Cli {
    pub fn regenerate(args: &RegenerateArgs) -> anyhow::Result<()> {
        let mut rng = rand::thread_rng();

        let path_prefix = Path::new(".vscode");
        create_dir_all(path_prefix)?;

        let palette_path = path_prefix.join("palette.json");

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
        full_palette.export(&path_prefix.join("full_palette.json"))?;

        let setting = Setting::new(&full_palette, args.no_saturation_fg);
        setting.export(&path_prefix.join("settings.json"))?;

        Ok(())
    }
}
