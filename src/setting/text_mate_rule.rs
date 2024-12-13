use serde::Serialize;
use crate::color::SrgbA;

#[derive(Serialize)]
pub struct TextMateRule {
    pub scope: Vec<String>,
    pub settings: TextMateRuleSettings,
}

#[derive(Serialize)]
pub struct TextMateRuleSettings {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "fontStyle")]
    pub font_style: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub foreground: Option<SrgbA>,
}

impl TextMateRule {
    pub fn new(scope: Vec<&str>, font_style: Option<&str>, foreground: Option<SrgbA>) -> Self {
        let font_style = font_style.map(|font_style| font_style.to_string());
        Self {
            scope: scope.into_iter().map(|v| v.to_string()).collect(),
            settings: TextMateRuleSettings {
                font_style,
                foreground,
            },
        }
    }
}
