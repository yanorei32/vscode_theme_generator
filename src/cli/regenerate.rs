use std::{fs::create_dir_all, path::Path};

use clap::Args;

use crate::{
    io::{ExportExt, LoadExt, Setting},
    model::{BasePalette, Color, SrgbColorMapExt, FullPalette},
    optimize::OptimizerExt,
    util::SrgbExt,
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

        let (theme, mut color_map) = BasePalette::load(&palette_path)?.take();

        //  TODO: 余計なColorMapに対する知識 BEGIN
        let base = color_map.base_color();

        for &target in &args.fixs {
            if target.is_bg_color() {
                let (_, bg, _) = base.theme_color_for(theme.into());
                color_map[target] = bg;
            } else {
                color_map[target] = base.new_by_random_hue(&mut rng);
            }
        }
        //  TODO: 余計なColorMapに対する知識 END

        let palette = BasePalette::new(theme, color_map).optimize(&args.fixs, &mut rng);

        let full_palette = FullPalette::from(&palette);

        let setting = Setting::new(&full_palette, args.no_saturation_fg);

        palette.export(&palette_path)?;
        full_palette.export(&path_prefix.join("full_palette.json"))?;
        setting.export(&path_prefix.join("settings.json"))?;

        Ok(())
    }
}
