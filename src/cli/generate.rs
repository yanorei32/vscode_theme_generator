use std::str::FromStr;

use clap::Args;
use palette::Srgb;

use super::Cli;

#[derive(Debug, Clone, Args)]

pub struct GenerateArgs {
    pub rgb: String,
}

pub struct ParseGenerateArgs {
    pub rgb: Srgb,
}

impl TryFrom<GenerateArgs> for ParseGenerateArgs {
    type Error = anyhow::Error;

    fn try_from(v: GenerateArgs) -> Result<Self, Self::Error> {
        let rgb = Srgb::from_str(&v.rgb)?.into();
        Ok(Self { rgb })
    }
}

impl Cli {
    pub fn generate(args: &GenerateArgs) -> anyhow::Result<()> {
        let args = ParseGenerateArgs::try_from(args.clone())?;
        todo!();
        Ok(())
    }
}
