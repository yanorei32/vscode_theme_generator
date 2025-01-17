use serde::Serialize;

use crate::palette::wrap::wrap_full_palette::WrapFullPalette;

use super::text_mate_rule::TextMateRule;

#[derive(Serialize)]
pub struct EditorTokenColorCustomizations {
    #[serde(rename = "textMateRules")]
    pub text_mate_rules: Vec<TextMateRule>,
}

impl EditorTokenColorCustomizations {
    pub fn new(palette: &WrapFullPalette, _no_saturation_fg: bool) -> Self {
        Self {
            text_mate_rules: vec![
                TextMateRule::new(
                    vec![
                        "comment",
                        "punctuation.definition.comment",
                        "string.comment",
                    ],
                    None,
                    Some(palette.gray[0]),
                ),
                TextMateRule::new(
                    vec![
                        "constant",
                        "entity.name.constant",
                        "variable.other.constant",
                        "variable.other.enummember",
                        "variable.language",
                    ],
                    None,
                    Some(palette.blue[3]),
                ),
                TextMateRule::new(vec!["entity", "entity.name"], None, Some(palette.purple[3])),
                TextMateRule::new(
                    vec!["variable.parameter.function"],
                    None,
                    Some(palette.gray[2]),
                ),
                TextMateRule::new(vec!["entity.name.tag"], None, Some(palette.green[3])),
                TextMateRule::new(vec!["keyword"], None, Some(palette.red[3])),
                TextMateRule::new(vec!["storage", "storage.type"], None, Some(palette.red[3])),
                TextMateRule::new(
                    vec![
                        "storage.modifier.package",
                        "storage.modifier.import",
                        "storage.type.java",
                    ],
                    None,
                    Some(palette.gray[2]),
                ),
                TextMateRule::new(
                    vec![
                        "string",
                        "punctuation.definition.string",
                        "string punctuation.section.embedded source",
                    ],
                    None,
                    Some(palette.blue[4]),
                ),
                TextMateRule::new(vec!["support"], None, Some(palette.blue[3])),
                TextMateRule::new(vec!["meta.property-name"], None, Some(palette.blue[3])),
                TextMateRule::new(vec!["variable"], None, Some(palette.orange[3])),
                TextMateRule::new(vec!["variable.other"], None, Some(palette.gray[2])),
                TextMateRule::new(vec!["invalid.broken"], None, Some(palette.red[3])),
                TextMateRule::new(vec!["invalid.deprecated"], None, Some(palette.red[3])),
                TextMateRule::new(vec!["invalid.illegal"], None, Some(palette.red[3])),
                TextMateRule::new(vec!["invalid.unimplemented"], None, Some(palette.red[3])),
                TextMateRule::new(
                    vec!["carriage-return"],
                    Some("italic underline"),
                    Some(palette.gray[0]),
                ),
                TextMateRule::new(vec!["message.error"], None, Some(palette.red[3])),
                TextMateRule::new(vec!["string variable"], None, Some(palette.blue[3])),
                TextMateRule::new(
                    vec!["source.regexp", "string.regexp"],
                    None,
                    Some(palette.blue[4]),
                ),
                TextMateRule::new(
                    vec![
                        "string.regexp.character-class",
                        "string.regexp constant.character.escape",
                        "string.regexp source.ruby.embedded",
                        "string.regexp string.regexp.arbitrary-repitition",
                    ],
                    None,
                    Some(palette.blue[4]),
                ),
                TextMateRule::new(
                    vec!["string.regexp constant.character.escape"],
                    None,
                    Some(palette.green[3]),
                ),
                TextMateRule::new(vec!["support.constant"], None, Some(palette.blue[3])),
                TextMateRule::new(vec!["support.variable"], None, Some(palette.blue[3])),
                TextMateRule::new(
                    vec!["punctuation.definition.list.begin.markdown"],
                    None,
                    Some(palette.orange[3]),
                ),
                TextMateRule::new(
                    vec!["markup.heading", "markup.heading entity.name"],
                    Some("bold"),
                    Some(palette.blue[3]),
                ),
                TextMateRule::new(vec!["markup.quote"], None, Some(palette.green[3])),
                TextMateRule::new(vec!["markup.italic"], Some("italic"), Some(palette.gray[2])),
                TextMateRule::new(vec!["markup.bold"], Some("bold"), Some(palette.gray[2])),
                TextMateRule::new(vec!["markup.underline"], Some("underline"), None),
                TextMateRule::new(vec!["markup.strikethrough"], Some("strikethrough"), None),
                TextMateRule::new(vec!["markup.inline.raw"], None, Some(palette.blue[3])),
                TextMateRule::new(
                    vec![
                        "markup.deleted",
                        "meta.diff.header.from-file",
                        "punctuation.definition.deleted",
                    ],
                    None,
                    Some(palette.red[3]),
                ),
                TextMateRule::new(
                    vec![
                        "markup.inserted",
                        "meta.diff.header.to-file",
                        "punctuation.definition.inserted",
                    ],
                    None,
                    Some(palette.green[3]),
                ),
                TextMateRule::new(
                    vec!["markup.changed", "punctuation.definition.changed"],
                    None,
                    Some(palette.orange[3]),
                ),
                TextMateRule::new(
                    vec!["markup.ignored", "markup.untracked"],
                    None,
                    Some(palette.gray[0]),
                ),
                TextMateRule::new(
                    vec!["meta.diff.range"],
                    Some("bold"),
                    Some(palette.purple[3]),
                ),
                TextMateRule::new(vec!["meta.diff.header"], None, Some(palette.blue[3])),
                TextMateRule::new(vec!["meta.separator"], Some("bold"), Some(palette.blue[3])),
                TextMateRule::new(vec!["meta.output"], None, Some(palette.blue[3])),
                TextMateRule::new(
                    vec![
                        "brackethighlighter.tag",
                        "brackethighlighter.curly",
                        "brackethighlighter.round",
                        "brackethighlighter.square",
                        "brackethighlighter.angle",
                        "brackethighlighter.quote",
                    ],
                    None,
                    Some(palette.gray[3]),
                ),
                TextMateRule::new(
                    vec!["brackethighlighter.unmatched"],
                    None,
                    Some(palette.red[3]),
                ),
                TextMateRule::new(
                    vec!["constant.other.reference.link", "string.other.link"],
                    Some("underline"),
                    Some(palette.blue[4]),
                ),
            ],
        }
    }
}
