use std::path::Path;

mod palette;
mod setting;

pub use palette::*;
pub use setting::*;

pub trait ExportExt {
    fn export(&self, path: &Path) -> anyhow::Result<()>;
}

pub trait LoadExt {
    fn load(path: &Path) -> anyhow::Result<Self> where Self: Sized;
}
