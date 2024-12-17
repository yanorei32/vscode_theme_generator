use std::{fs::create_dir_all, path::Path};

use clap::Args;

use crate::{
    io::{ExportExt, LoadExt, Setting},
    model::{BasePalette, Color, FullPalette},
    optimize::OptimizerExt,
    Cli,
};

#[derive(Debug, Clone, Args)]
pub struct RegenerateArgs {
    #[clap(short, long)]
    pub fixs: Vec<Color>,

    #[arg(short, long)]
    pub no_saturation_ui: bool,
}

impl Cli {
    pub fn regenerate(args: &RegenerateArgs) -> anyhow::Result<()> {
        let mut rng = rand::thread_rng();

        let path_prefix = Path::new(".vscode");
        create_dir_all(path_prefix)?;

        let palette_path = path_prefix.join("palette.json");

        let palette = BasePalette::load(&palette_path)?
            .regenerate_colors(&args.fixs, &mut rng)
            .optimize(&args.fixs, &mut rng);

        let full_palette = FullPalette::from(&palette);

        let setting = Setting::new(&full_palette, args.no_saturation_ui);

        palette.export(&palette_path)?;
        full_palette.export(&path_prefix.join("full_palette.json"))?;
        setting.export(&path_prefix.join("settings.json"))?;

        Ok(())
    }
}
