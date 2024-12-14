use serde::Serialize;

use super::text_mate_rule::TextMateRule;
use crate::{io::palette::FullPaletteExportable, model::Color};

#[derive(Serialize)]
pub(super) struct EditorTokenColorCustomizations {
    #[serde(rename = "textMateRules")]
    text_mate_rules: Vec<TextMateRule>,
}

impl EditorTokenColorCustomizations {
    pub(super) fn new(palette: &FullPaletteExportable, _no_saturation_fg: bool) -> Self {
        Self {
            text_mate_rules: vec![
                TextMateRule::new(
                    &[
                        "comment",
                        "punctuation.definition.comment",
                        "string.comment",
                    ],
                    None,
                    Some(palette.color_map[Color::Gray][0]),
                ),
                TextMateRule::new(
                    &[
                        "constant",
                        "entity.name.constant",
                        "variable.other.constant",
                        "variable.other.enummember",
                        "variable.language",
                    ],
                    None,
                    Some(palette.color_map[Color::Blue][3]),
                ),
                TextMateRule::new(
                    &["entity", "entity.name"],
                    None,
                    Some(palette.color_map[Color::Purple][3]),
                ),
                TextMateRule::new(
                    &["variable.parameter.function"],
                    None,
                    Some(palette.color_map[Color::Gray][2]),
                ),
                TextMateRule::new(
                    &["entity.name.tag"],
                    None,
                    Some(palette.color_map[Color::Green][3]),
                ),
                TextMateRule::new(&["keyword"], None, Some(palette.color_map[Color::Red][3])),
                TextMateRule::new(
                    &["storage", "storage.type"],
                    None,
                    Some(palette.color_map[Color::Red][3]),
                ),
                TextMateRule::new(
                    &[
                        "storage.modifier.package",
                        "storage.modifier.import",
                        "storage.type.java",
                    ],
                    None,
                    Some(palette.color_map[Color::Gray][2]),
                ),
                TextMateRule::new(
                    &[
                        "string",
                        "punctuation.definition.string",
                        "string punctuation.section.embedded source",
                    ],
                    None,
                    Some(palette.color_map[Color::Blue][4]),
                ),
                TextMateRule::new(&["support"], None, Some(palette.color_map[Color::Blue][3])),
                TextMateRule::new(
                    &["meta.property-name"],
                    None,
                    Some(palette.color_map[Color::Blue][3]),
                ),
                TextMateRule::new(
                    &["variable"],
                    None,
                    Some(palette.color_map[Color::Orange][3]),
                ),
                TextMateRule::new(
                    &["variable.other"],
                    None,
                    Some(palette.color_map[Color::Gray][2]),
                ),
                TextMateRule::new(
                    &["invalid.broken"],
                    None,
                    Some(palette.color_map[Color::Red][3]),
                ),
                TextMateRule::new(
                    &["invalid.deprecated"],
                    None,
                    Some(palette.color_map[Color::Red][3]),
                ),
                TextMateRule::new(
                    &["invalid.illegal"],
                    None,
                    Some(palette.color_map[Color::Red][3]),
                ),
                TextMateRule::new(
                    &["invalid.unimplemented"],
                    None,
                    Some(palette.color_map[Color::Red][3]),
                ),
                TextMateRule::new(
                    &["carriage-return"],
                    Some("italic underline"),
                    Some(palette.color_map[Color::Gray][0]),
                ),
                TextMateRule::new(
                    &["message.error"],
                    None,
                    Some(palette.color_map[Color::Red][3]),
                ),
                TextMateRule::new(
                    &["string variable"],
                    None,
                    Some(palette.color_map[Color::Blue][3]),
                ),
                TextMateRule::new(
                    &["source.regexp", "string.regexp"],
                    None,
                    Some(palette.color_map[Color::Blue][4]),
                ),
                TextMateRule::new(
                    &[
                        "string.regexp.character-class",
                        "string.regexp constant.character.escape",
                        "string.regexp source.ruby.embedded",
                        "string.regexp string.regexp.arbitrary-repitition",
                    ],
                    None,
                    Some(palette.color_map[Color::Blue][4]),
                ),
                TextMateRule::new(
                    &["string.regexp constant.character.escape"],
                    None,
                    Some(palette.color_map[Color::Green][3]),
                ),
                TextMateRule::new(
                    &["support.constant"],
                    None,
                    Some(palette.color_map[Color::Blue][3]),
                ),
                TextMateRule::new(
                    &["support.variable"],
                    None,
                    Some(palette.color_map[Color::Blue][3]),
                ),
                TextMateRule::new(
                    &["punctuation.definition.list.begin.markdown"],
                    None,
                    Some(palette.color_map[Color::Orange][3]),
                ),
                TextMateRule::new(
                    &["markup.heading", "markup.heading entity.name"],
                    Some("bold"),
                    Some(palette.color_map[Color::Blue][3]),
                ),
                TextMateRule::new(
                    &["markup.quote"],
                    None,
                    Some(palette.color_map[Color::Green][3]),
                ),
                TextMateRule::new(
                    &["markup.italic"],
                    Some("italic"),
                    Some(palette.color_map[Color::Gray][2]),
                ),
                TextMateRule::new(
                    &["markup.bold"],
                    Some("bold"),
                    Some(palette.color_map[Color::Gray][2]),
                ),
                TextMateRule::new(&["markup.underline"], Some("underline"), None),
                TextMateRule::new(&["markup.strikethrough"], Some("strikethrough"), None),
                TextMateRule::new(
                    &["markup.inline.raw"],
                    None,
                    Some(palette.color_map[Color::Blue][3]),
                ),
                TextMateRule::new(
                    &[
                        "markup.deleted",
                        "meta.diff.header.from-file",
                        "punctuation.definition.deleted",
                    ],
                    None,
                    Some(palette.color_map[Color::Red][3]),
                ),
                TextMateRule::new(
                    &[
                        "markup.inserted",
                        "meta.diff.header.to-file",
                        "punctuation.definition.inserted",
                    ],
                    None,
                    Some(palette.color_map[Color::Green][3]),
                ),
                TextMateRule::new(
                    &["markup.changed", "punctuation.definition.changed"],
                    None,
                    Some(palette.color_map[Color::Orange][3]),
                ),
                TextMateRule::new(
                    &["markup.ignored", "markup.untracked"],
                    None,
                    Some(palette.color_map[Color::Gray][0]),
                ),
                TextMateRule::new(
                    &["meta.diff.range"],
                    Some("bold"),
                    Some(palette.color_map[Color::Purple][3]),
                ),
                TextMateRule::new(
                    &["meta.diff.header"],
                    None,
                    Some(palette.color_map[Color::Blue][3]),
                ),
                TextMateRule::new(
                    &["meta.separator"],
                    Some("bold"),
                    Some(palette.color_map[Color::Blue][3]),
                ),
                TextMateRule::new(
                    &["meta.output"],
                    None,
                    Some(palette.color_map[Color::Blue][3]),
                ),
                TextMateRule::new(
                    &[
                        "brackethighlighter.tag",
                        "brackethighlighter.curly",
                        "brackethighlighter.round",
                        "brackethighlighter.square",
                        "brackethighlighter.angle",
                        "brackethighlighter.quote",
                    ],
                    None,
                    Some(palette.color_map[Color::Gray][3]),
                ),
                TextMateRule::new(
                    &["brackethighlighter.unmatched"],
                    None,
                    Some(palette.color_map[Color::Red][3]),
                ),
                TextMateRule::new(
                    &["constant.other.reference.link", "string.other.link"],
                    Some("underline"),
                    Some(palette.color_map[Color::Blue][4]),
                ),
            ],
        }
    }
}
