use std::{fs::File, io::Write, path::PathBuf};

use editor_token_color_customizations::EditorTokenColorCustomizations;
use serde::Serialize;
use workbench_color_customizations::WorkbenchColorCustomizations;

use crate::palette::{full_palette::FullPalette, wrap::wrap_full_palette::WrapFullPalette};

pub mod editor_token_color_customizations;
pub mod text_mate_rule;
pub mod workbench_color_customizations;

#[derive(Serialize)]
pub struct Setting {
    #[serde(rename = "workbench.colorCustomizations")]
    workbench_color_customizations: WorkbenchColorCustomizations,
    #[serde(rename = "editor.tokenColorCustomizations")]
    editor_token_color_customizations: EditorTokenColorCustomizations,
}

impl Setting {
    pub fn new(palette: &FullPalette, no_saturation_fg: bool) -> Self {
        let palette = WrapFullPalette::from(palette.clone());
        Self {
            workbench_color_customizations: WorkbenchColorCustomizations::new(
                &palette,
                no_saturation_fg,
            ),
            editor_token_color_customizations: EditorTokenColorCustomizations::new(
                &palette,
                no_saturation_fg,
            ),
        }
    }

    pub fn export(&self, path: &PathBuf) -> anyhow::Result<()> {
        let settinge_str = serde_json::to_string(&self)?;
        let mut setting_file = File::create(path)?;
        writeln!(setting_file, "{}", settinge_str)?;
        Ok(())
    }
}
