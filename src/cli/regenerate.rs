use super::Cli;
use crate::palette::base_palette::BasePalette;
use clap::Args;

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
        todo!();
        Ok(())
    }
}
