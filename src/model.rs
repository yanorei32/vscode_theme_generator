use std::fmt::{Display, Formatter, Error as FmtError};

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum ActualThemeMode {
    Dark,
    Light,
}

impl Display for ActualThemeMode {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), FmtError> {
        match self {
            Self::Dark => write!(f, "dark"),
            Self::Light => write!(f, "light"),
        }
    }
}
