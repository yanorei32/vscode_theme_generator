use serde::Serialize;

use crate::{color::wrap::wrap_srgb::WrapSrgb, palette::wrap::wrap_full_palette::WrapFullPalette};

#[derive(Serialize)]
pub struct WorkbenchColorCustomizations {
    // #[serde(rename = "focusBorder")]
    // pub focus_border: WrapSrgb,
    // #[serde(rename = "foreground")]
    // pub foreground: WrapSrgb,
    // #[serde(rename = "descriptionForeground")]
    // pub description_foreground: WrapSrgb,
    // #[serde(rename = "errorForeground")]
    // pub error_foreground: WrapSrgb,
    // #[serde(rename = "textLink.foreground")]
    // pub text_link_foreground: WrapSrgb,
    // #[serde(rename = "textLink.activeForeground")]
    // pub text_link_active_foreground: WrapSrgb,
    // #[serde(rename = "textBlockQuote.background")]
    // pub text_block_quote_background: WrapSrgb,
    // #[serde(rename = "textBlockQuote.border")]
    // pub text_block_quote_border: WrapSrgb,
    // #[serde(rename = "textCodeBlock.background")]
    // pub text_code_block_background: WrapSrgb,
    // #[serde(rename = "textPreformat.foreground")]
    // pub text_preformat_foreground: WrapSrgb,
    // #[serde(rename = "textSeparator.foreground")]
    // pub text_separator_foreground: WrapSrgb,
    // #[serde(rename = "button.background")]
    // pub button_background: WrapSrgb,
    // #[serde(rename = "button.foreground")]
    // pub button_foreground: WrapSrgb,
    // #[serde(rename = "button.hoverBackground")]
    // pub button_hover_background: WrapSrgb,
    // #[serde(rename = "button.secondaryBackground")]
    // pub button_secondary_background: WrapSrgb,
    // #[serde(rename = "button.secondaryForeground")]
    // pub button_secondary_foreground: WrapSrgb,
    // #[serde(rename = "button.secondaryHoverBackground")]
    // pub button_secondary_hover_background: WrapSrgb,
    // #[serde(rename = "checkbox.background")]
    // pub checkbox_background: WrapSrgb,
    // #[serde(rename = "checkbox.border")]
    // pub checkbox_border: WrapSrgb,
    // #[serde(rename = "dropdown.background")]
    // pub dropdown_background: WrapSrgb,
    // #[serde(rename = "dropdown.border")]
    // pub dropdown_border: WrapSrgb,
    // #[serde(rename = "dropdown.foreground")]
    // pub dropdown_foreground: WrapSrgb,
    // #[serde(rename = "dropdown.listBackground")]
    // pub dropdown_list_background: WrapSrgb,
    // #[serde(rename = "input.background")]
    // pub input_background: WrapSrgb,
    // #[serde(rename = "input.border")]
    // pub input_border: WrapSrgb,
    // #[serde(rename = "input.foreground")]
    // pub input_foreground: WrapSrgb,
    // #[serde(rename = "input.placeholderForeground")]
    // pub input_placeholder_foreground: WrapSrgb,
    // #[serde(rename = "badge.foreground")]
    // pub badge_foreground: WrapSrgb,
    // #[serde(rename = "badge.background")]
    // pub badge_background: WrapSrgb,
    // #[serde(rename = "progressBar.background")]
    // pub progress_bar_background: WrapSrgb,
    // #[serde(rename = "titleBar.activeForeground")]
    // pub title_bar_active_foreground: WrapSrgb,
    // #[serde(rename = "titleBar.activeBackground")]
    // pub title_bar_active_background: WrapSrgb,
    // #[serde(rename = "titleBar.inactiveForeground")]
    // pub title_bar_inactive_foreground: WrapSrgb,
    // #[serde(rename = "titleBar.inactiveBackground")]
    // pub title_bar_inactive_background: WrapSrgb,
    // #[serde(rename = "titleBar.border")]
    // pub title_bar_border: WrapSrgb,
    // #[serde(rename = "activityBar.foreground")]
    // pub activity_bar_foreground: WrapSrgb,
    // #[serde(rename = "activityBar.inactiveForeground")]
    // pub activity_bar_inactive_foreground: WrapSrgb,
    // #[serde(rename = "activityBar.background")]
    // pub activity_bar_background: WrapSrgb,
    // #[serde(rename = "activityBarBadge.foreground")]
    // pub activity_bar_badge_foreground: WrapSrgb,
    // #[serde(rename = "activityBarBadge.background")]
    // pub activity_bar_badge_background: WrapSrgb,
    // #[serde(rename = "activityBar.activeBorder")]
    // pub activity_bar_active_border: WrapSrgb,
    // #[serde(rename = "activityBar.border")]
    // pub activity_bar_border: WrapSrgb,
    // #[serde(rename = "sideBar.foreground")]
    // pub side_bar_foreground: WrapSrgb,
    // #[serde(rename = "sideBar.background")]
    // pub side_bar_background: WrapSrgb,
    // #[serde(rename = "sideBar.border")]
    // pub side_bar_border: WrapSrgb,
    // #[serde(rename = "sideBarTitle.foreground")]
    // pub side_bar_title_foreground: WrapSrgb,
    // #[serde(rename = "sideBarSectionHeader.foreground")]
    // pub side_bar_section_header_foreground: WrapSrgb,
    // #[serde(rename = "sideBarSectionHeader.background")]
    // pub side_bar_section_header_background: WrapSrgb,
    // #[serde(rename = "sideBarSectionHeader.border")]
    // pub side_bar_section_header_border: WrapSrgb,
    // #[serde(rename = "list.hoverForeground")]
    // pub list_hover_foreground: WrapSrgb,
    // #[serde(rename = "list.inactiveSelectionForeground")]
    // pub list_inactive_selection_foreground: WrapSrgb,
    // #[serde(rename = "list.activeSelectionForeground")]
    // pub list_active_selection_foreground: WrapSrgb,
    // #[serde(rename = "list.hoverBackground")]
    // pub list_hover_background: WrapSrgb,
    // #[serde(rename = "list.inactiveSelectionBackground")]
    // pub list_inactive_selection_background: WrapSrgb,
    // #[serde(rename = "list.activeSelectionBackground")]
    // pub list_active_selection_background: WrapSrgb,
    // #[serde(rename = "list.inactiveFocusBackground")]
    // pub list_inactive_focus_background: WrapSrgb,
    // #[serde(rename = "list.focusBackground")]
    // pub list_focus_background: WrapSrgb,
    // #[serde(rename = "tree.indentGuidesStroke")]
    // pub tree_indent_guides_stroke: WrapSrgb,
    // #[serde(rename = "notificationCenterHeader.foreground")]
    // pub notification_center_header_foreground: WrapSrgb,
    // #[serde(rename = "notificationCenterHeader.background")]
    // pub notification_center_header_background: WrapSrgb,
    // #[serde(rename = "notifications.foreground")]
    // pub notifications_foreground: WrapSrgb,
    // #[serde(rename = "notifications.background")]
    // pub notifications_background: WrapSrgb,
    // #[serde(rename = "notifications.border")]
    // pub notifications_border: WrapSrgb,
    // #[serde(rename = "notificationsErrorIcon.foreground")]
    // pub notifications_error_icon_foreground: WrapSrgb,
    // #[serde(rename = "notificationsWarningIcon.foreground")]
    // pub notifications_warning_icon_foreground: WrapSrgb,
    // #[serde(rename = "notificationsInfoIcon.foreground")]
    // pub notifications_info_icon_foreground: WrapSrgb,
    // #[serde(rename = "pickerGroup.border")]
    // pub picker_group_border: WrapSrgb,
    // #[serde(rename = "pickerGroup.foreground")]
    // pub picker_group_foreground: WrapSrgb,
    // #[serde(rename = "quickInput.background")]
    // pub quick_input_background: WrapSrgb,
    // #[serde(rename = "quickInput.foreground")]
    // pub quick_input_foreground: WrapSrgb,
    // #[serde(rename = "statusBar.foreground")]
    // pub status_bar_foreground: WrapSrgb,
    // #[serde(rename = "statusBar.background")]
    // pub status_bar_background: WrapSrgb,
    // #[serde(rename = "statusBar.border")]
    // pub status_bar_border: WrapSrgb,
    // #[serde(rename = "statusBar.noFolderBackground")]
    // pub status_bar_no_folder_background: WrapSrgb,
    // #[serde(rename = "statusBar.debuggingBackground")]
    // pub status_bar_debugging_background: WrapSrgb,
    // #[serde(rename = "statusBar.debuggingForeground")]
    // pub status_bar_debugging_foreground: WrapSrgb,
    // #[serde(rename = "statusBarItem.prominentBackground")]
    // pub status_bar_item_prominent_background: WrapSrgb,
    // #[serde(rename = "statusBarItem.remoteForeground")]
    // pub status_bar_item_remote_foreground: WrapSrgb,
    // #[serde(rename = "statusBarItem.remoteBackground")]
    // pub status_bar_item_remote_background: WrapSrgb,
    #[serde(rename = "editorGroupHeader.tabsBackground")]
    pub editor_group_header_tabs_background: WrapSrgb,
    #[serde(rename = "editorGroupHeader.tabsBorder")]
    pub editor_group_header_tabs_border: WrapSrgb,
    #[serde(rename = "editorGroup.border")]
    pub editor_group_border: WrapSrgb,
    // #[serde(rename = "tab.activeForeground")]
    // pub tab_active_foreground: WrapSrgb,
    // #[serde(rename = "tab.inactiveForeground")]
    // pub tab_inactive_foreground: WrapSrgb,
    // #[serde(rename = "tab.inactiveBackground")]
    // pub tab_inactive_background: WrapSrgb,
    // #[serde(rename = "tab.activeBackground")]
    // pub tab_active_background: WrapSrgb,
    // #[serde(rename = "tab.hoverBackground")]
    // pub tab_hover_background: WrapSrgb,
    // #[serde(rename = "tab.unfocusedHoverBackground")]
    // pub tab_unfocused_hover_background: WrapSrgb,
    // #[serde(rename = "tab.border")]
    // pub tab_border: WrapSrgb,
    // #[serde(rename = "tab.unfocusedActiveBorderTop")]
    // pub tab_unfocused_active_border_top: WrapSrgb,
    // #[serde(rename = "tab.activeBorder")]
    // pub tab_active_border: WrapSrgb,
    // #[serde(rename = "tab.unfocusedActiveBorder")]
    // pub tab_unfocused_active_border: WrapSrgb,
    // #[serde(rename = "tab.activeBorderTop")]
    // pub tab_active_border_top: WrapSrgb,
    // #[serde(rename = "breadcrumb.foreground")]
    // pub breadcrumb_foreground: WrapSrgb,
    // #[serde(rename = "breadcrumb.focusForeground")]
    // pub breadcrumb_focus_foreground: WrapSrgb,
    // #[serde(rename = "breadcrumb.activeSelectionForeground")]
    // pub breadcrumb_active_selection_foreground: WrapSrgb,
    // #[serde(rename = "breadcrumbPicker.background")]
    // pub breadcrumb_picker_background: WrapSrgb,
    #[serde(rename = "editor.foreground")]
    pub editor_foreground: WrapSrgb,
    #[serde(rename = "editor.background")]
    pub editor_background: WrapSrgb,
    #[serde(rename = "editorWidget.background")]
    pub editor_widget_background: WrapSrgb,
    #[serde(rename = "editor.foldBackground")]
    pub editor_fold_background: WrapSrgb,
    #[serde(rename = "editor.lineHighlightBackground")]
    pub editor_line_highlight_background: WrapSrgb,
    #[serde(rename = "editorLineNumber.foreground")]
    pub editor_line_number_foreground: WrapSrgb,
    #[serde(rename = "editorLineNumber.activeForeground")]
    pub editor_line_number_active_foreground: WrapSrgb,
    #[serde(rename = "editorIndentGuide.background")]
    pub editor_indent_guide_background: WrapSrgb,
    #[serde(rename = "editorIndentGuide.activeBackground")]
    pub editor_indent_guide_active_background: WrapSrgb,
    #[serde(rename = "editorWhitespace.foreground")]
    pub editor_whitespace_foreground: WrapSrgb,
    #[serde(rename = "editorCursor.foreground")]
    pub editor_cursor_foreground: WrapSrgb,
    #[serde(rename = "editorError.foreground")]
    pub editor_error_foreground: WrapSrgb,
    #[serde(rename = "editorWarning.foreground")]
    pub editor_warning_foreground: WrapSrgb,
    #[serde(rename = "editor.findMatchBackground")]
    pub editor_find_match_background: WrapSrgb,
    #[serde(rename = "editor.findMatchHighlightBackground")]
    pub editor_find_match_highlight_background: WrapSrgb,
    #[serde(rename = "editor.linkedEditingBackground")]
    pub editor_linked_editing_background: WrapSrgb,
    #[serde(rename = "editor.inactiveSelectionBackground")]
    pub editor_inactive_selection_background: WrapSrgb,
    #[serde(rename = "editor.selectionBackground")]
    pub editor_selection_background: WrapSrgb,
    #[serde(rename = "editor.selectionHighlightBackground")]
    pub editor_selection_highlight_background: WrapSrgb,
    #[serde(rename = "editor.selectionHighlightBorder")]
    pub editor_selection_highlight_border: WrapSrgb,
    #[serde(rename = "editor.wordHighlightBackground")]
    pub editor_word_highlight_background: WrapSrgb,
    #[serde(rename = "editor.wordHighlightStrongBackground")]
    pub editor_word_highlight_strong_background: WrapSrgb,
    #[serde(rename = "editor.wordHighlightBorder")]
    pub editor_word_highlight_border: WrapSrgb,
    #[serde(rename = "editor.wordHighlightStrongBorder")]
    pub editor_word_highlight_strong_border: WrapSrgb,
    #[serde(rename = "editorBracketMatch.background")]
    pub editor_bracket_match_background: WrapSrgb,
    #[serde(rename = "editorBracketMatch.border")]
    pub editor_bracket_match_border: WrapSrgb,
    // #[serde(rename = "editorGutter.modifiedBackground")]
    // pub editor_gutter_modified_background: WrapSrgb,
    // #[serde(rename = "editorGutter.addedBackground")]
    // pub editor_gutter_added_background: WrapSrgb,
    // #[serde(rename = "editorGutter.deletedBackground")]
    // pub editor_gutter_deleted_background: WrapSrgb,
    // #[serde(rename = "diffEditor.insertedTextBackground")]
    // pub diff_editor_inserted_text_background: WrapSrgb,
    // #[serde(rename = "diffEditor.removedTextBackground")]
    // pub diff_editor_removed_text_background: WrapSrgb,
    // #[serde(rename = "scrollbar.shadow")]
    // pub scrollbar_shadow: WrapSrgb,
    // #[serde(rename = "scrollbarSlider.background")]
    // pub scrollbar_slider_background: WrapSrgb,
    // #[serde(rename = "scrollbarSlider.hoverBackground")]
    // pub scrollbar_slider_hover_background: WrapSrgb,
    // #[serde(rename = "scrollbarSlider.activeBackground")]
    // pub scrollbar_slider_active_background: WrapSrgb,
    // #[serde(rename = "editorOverviewRuler.border")]
    // pub editor_overview_ruler_border: WrapSrgb,
    // #[serde(rename = "panel.background")]
    // pub panel_background: WrapSrgb,
    // #[serde(rename = "panel.border")]
    // pub panel_border: WrapSrgb,
    // #[serde(rename = "panelTitle.activeBorder")]
    // pub panel_title_active_border: WrapSrgb,
    // #[serde(rename = "panelTitle.activeForeground")]
    // pub panel_title_active_foreground: WrapSrgb,
    // #[serde(rename = "panelTitle.inactiveForeground")]
    // pub panel_title_inactive_foreground: WrapSrgb,
    // #[serde(rename = "panelInput.border")]
    // pub panel_input_border: WrapSrgb,
    // #[serde(rename = "terminal.foreground")]
    // pub terminal_foreground: WrapSrgb,
    // #[serde(rename = "terminal.tab.activeBorder")]
    // pub terminal_tab_active_border: WrapSrgb,
    // #[serde(rename = "terminalCursor.background")]
    // pub terminal_cursor_background: WrapSrgb,
    // #[serde(rename = "terminalCursor.foreground")]
    // pub terminal_cursor_foreground: WrapSrgb,
    // #[serde(rename = "terminal.ansiBrightWhite")]
    // pub terminal_ansi_bright_white: WrapSrgb,
    // #[serde(rename = "terminal.ansiWhite")]
    // pub terminal_ansi_white: WrapSrgb,
    // #[serde(rename = "terminal.ansiBrightBlack")]
    // pub terminal_ansi_bright_black: WrapSrgb,
    // #[serde(rename = "terminal.ansiBlack")]
    // pub terminal_ansi_black: WrapSrgb,
    // #[serde(rename = "terminal.ansiBlue")]
    // pub terminal_ansi_blue: WrapSrgb,
    // #[serde(rename = "terminal.ansiBrightBlue")]
    // pub terminal_ansi_bright_blue: WrapSrgb,
    // #[serde(rename = "terminal.ansiGreen")]
    // pub terminal_ansi_green: WrapSrgb,
    // #[serde(rename = "terminal.ansiBrightGreen")]
    // pub terminal_ansi_bright_green: WrapSrgb,
    // #[serde(rename = "terminal.ansiCyan")]
    // pub terminal_ansi_cyan: WrapSrgb,
    // #[serde(rename = "terminal.ansiBrightCyan")]
    // pub terminal_ansi_bright_cyan: WrapSrgb,
    // #[serde(rename = "terminal.ansiRed")]
    // pub terminal_ansi_red: WrapSrgb,
    // #[serde(rename = "terminal.ansiBrightRed")]
    // pub terminal_ansi_bright_red: WrapSrgb,
    // #[serde(rename = "terminal.ansiMagenta")]
    // pub terminal_ansi_magenta: WrapSrgb,
    // #[serde(rename = "terminal.ansiBrightMagenta")]
    // pub terminal_ansi_bright_magenta: WrapSrgb,
    // #[serde(rename = "terminal.ansiYellow")]
    // pub terminal_ansi_yellow: WrapSrgb,
    // #[serde(rename = "terminal.ansiBrightYellow")]
    // pub terminal_ansi_bright_yellow: WrapSrgb,
    #[serde(rename = "editorBracketHighlight.foreground1")]
    pub editor_bracket_highlight_foreground1: WrapSrgb,
    #[serde(rename = "editorBracketHighlight.foreground2")]
    pub editor_bracket_highlight_foreground2: WrapSrgb,
    #[serde(rename = "editorBracketHighlight.foreground3")]
    pub editor_bracket_highlight_foreground3: WrapSrgb,
    #[serde(rename = "editorBracketHighlight.foreground4")]
    pub editor_bracket_highlight_foreground4: WrapSrgb,
    #[serde(rename = "editorBracketHighlight.foreground5")]
    pub editor_bracket_highlight_foreground5: WrapSrgb,
    #[serde(rename = "editorBracketHighlight.foreground6")]
    pub editor_bracket_highlight_foreground6: WrapSrgb,
    // #[serde(rename = "debugToolBar.background")]
    // pub debug_tool_bar_background: WrapSrgb,
    // #[serde(rename = "editor.stackFrameHighlightBackground")]
    // pub editor_stack_frame_highlight_background: WrapSrgb,
    // #[serde(rename = "editor.focusedStackFrameHighlightBackground")]
    // pub editor_focused_stack_frame_highlight_background: WrapSrgb,
    // #[serde(rename = "peekViewEditor.matchHighlightBackground")]
    // pub peek_view_editor_match_highlight_background: WrapSrgb,
    // #[serde(rename = "peekViewResult.matchHighlightBackground")]
    // pub peek_view_result_match_highlight_background: WrapSrgb,
    // #[serde(rename = "peekViewEditor.background")]
    // pub peek_view_editor_background: WrapSrgb,
    // #[serde(rename = "peekViewResult.background")]
    // pub peek_view_result_background: WrapSrgb,
    // #[serde(rename = "settings.headerForeground")]
    // pub settings_header_foreground: WrapSrgb,
    // #[serde(rename = "settings.modifiedItemIndicator")]
    // pub settings_modified_item_indicator: WrapSrgb,
    // #[serde(rename = "welcomePage.buttonBackground")]
    // pub welcome_page_button_background: WrapSrgb,
    // #[serde(rename = "welcomePage.buttonHoverBackground")]
    // pub welcome_page_button_hover_background: WrapSrgb,
}

impl WorkbenchColorCustomizations {
    pub fn new(palette: &WrapFullPalette, no_saturation_fg: bool) -> Self {
        let fg = if no_saturation_fg {
            &palette.fg
        } else {
            &palette.gray
        };
        Self {
            // focus_border: palette.blue[2],
            // foreground: fg[3],
            // description_foreground: palette.gray[2],
            // error_foreground: palette.red[3],

            // text_link_foreground: palette.blue[3],
            // text_link_active_foreground: palette.blue[3],
            // text_block_quote_background: palette.bg[0],
            // text_block_quote_border: palette.gray[1],
            // text_code_block_background: palette.bg[0],
            // text_preformat_foreground: palette.gray[3],
            // text_separator_foreground: palette.gray[1],

            // button_background: palette.bg[0],
            // button_foreground: fg[4],
            // button_hover_background: palette.bg[1],

            // button_secondary_background: palette.bg[1],
            // button_secondary_foreground: fg[3],
            // button_secondary_hover_background: palette.bg[2],

            // checkbox_background: palette.bg[1],
            // checkbox_border: palette.gray[3],

            // dropdown_background: palette.bg[1],
            // dropdown_border: palette.gray[3],
            // dropdown_foreground: fg[3],
            // dropdown_list_background: palette.bg[0],

            // input_background: palette.bg[1],
            // input_border: palette.gray[3],
            // input_foreground: fg[3],
            // input_placeholder_foreground: fg[2],

            // badge_foreground: fg[3],
            // badge_background: palette.bg[1],

            // progress_bar_background: palette.blue[2],

            // title_bar_active_foreground: fg[3],
            // title_bar_active_background: palette.bg[2],
            // title_bar_inactive_foreground: fg[2],
            // title_bar_inactive_background: palette.bg[1],
            // title_bar_border: palette.gray[3],

            // activity_bar_foreground: fg[3],
            // activity_bar_inactive_foreground: palette.gray[2],
            // activity_bar_background: palette.bg[2],
            // activity_bar_badge_foreground: fg[3],
            // activity_bar_badge_background: palette.bg[2],
            // activity_bar_active_border: palette.gray[2],
            // activity_bar_border: palette.gray[3],

            // side_bar_foreground: fg[3],
            // side_bar_background: palette.bg[3],
            // side_bar_border: palette.gray[0],
            // side_bar_title_foreground: fg[3],
            // side_bar_section_header_foreground: fg[3],
            // side_bar_section_header_background: palette.bg[3],
            // side_bar_section_header_border: palette.gray[3],

            // list_hover_foreground: fg[3],
            // list_inactive_selection_foreground: fg[3],
            // list_active_selection_foreground: fg[3],
            // list_hover_background: palette.bg[3],
            // list_inactive_selection_background: palette.bg[2],
            // list_active_selection_background: palette.bg[1],
            // list_inactive_focus_background: palette.bg[0],
            // list_focus_background: palette.bg[4],

            // tree_indent_guides_stroke: palette.gray[1],

            // notification_center_header_foreground: fg[2],
            // notification_center_header_background: palette.bg[0],
            // notifications_foreground: fg[2],
            // notifications_background: palette.bg[2],
            // notifications_border: palette.gray[3],
            // notifications_error_icon_foreground: palette.red[0],
            // notifications_warning_icon_foreground: palette.yellow[2],
            // notifications_info_icon_foreground: palette.blue[3],

            // picker_group_border: palette.gray[1],
            // picker_group_foreground: fg[2],

            // quick_input_background: palette.bg[0],
            // quick_input_foreground: fg[2],

            // status_bar_foreground: fg[2],
            // status_bar_background: palette.bg[2],
            // status_bar_border: palette.gray[2],
            // status_bar_no_folder_background: palette.bg[0],
            // status_bar_debugging_background: palette.bg[2],
            // status_bar_debugging_foreground: palette.gray[2],
            // status_bar_item_prominent_background: palette.gray[3],
            // status_bar_item_remote_foreground: fg[3],
            // status_bar_item_remote_background: palette.bg[0],
            editor_group_header_tabs_background: palette.bg[1],
            editor_group_header_tabs_border: palette.gray[2],
            editor_group_border: palette.gray[2],

            // tab_active_foreground: fg[2],
            // tab_inactive_foreground: fg[3],
            // tab_inactive_background: palette.bg[2],
            // tab_active_background: palette.bg[4],
            // tab_hover_background: palette.bg[3],
            // tab_unfocused_hover_background: palette.bg[4],
            // tab_border: palette.gray[2],
            // tab_unfocused_active_border_top: palette.gray[2],
            // tab_active_border: palette.gray[0],
            // tab_unfocused_active_border: palette.gray[0],
            // tab_active_border_top: palette.gray[1],

            // breadcrumb_foreground: fg[3],
            // breadcrumb_focus_foreground: fg[2],
            // breadcrumb_active_selection_foreground: palette.gray[4],
            // breadcrumb_picker_background: palette.bg[2],
            editor_foreground: fg[2],
            editor_background: palette.bg[2],
            editor_widget_background: palette.bg[3],
            editor_fold_background: palette.bg[4].alpha(0.5),
            editor_line_highlight_background: palette.bg[4],
            editor_line_number_foreground: fg[1],
            editor_line_number_active_foreground: fg[2],
            editor_indent_guide_background: palette.bg[0],
            editor_indent_guide_active_background: palette.bg[1],
            editor_whitespace_foreground: fg[1],
            editor_cursor_foreground: fg[3],
            editor_error_foreground: palette.red[3],
            editor_warning_foreground: palette.yellow[3],

            editor_find_match_background: palette.bg[4],
            editor_find_match_highlight_background: palette.bg[0].alpha(0.5),
            editor_linked_editing_background: palette.bg[4],
            editor_inactive_selection_background: palette.bg[4].alpha(0.5),
            editor_selection_background: palette.bg[4],
            editor_selection_highlight_background: palette.bg[4].alpha(0.5),
            editor_selection_highlight_border: palette.gray[0],
            editor_word_highlight_background: palette.bg[4].alpha(0.5),
            editor_word_highlight_strong_background: palette.bg[4].alpha(0.5),
            editor_word_highlight_border: palette.gray[0],
            editor_word_highlight_strong_border: palette.gray[0],
            editor_bracket_match_background: palette.bg[4],
            editor_bracket_match_border: palette.gray[0],

            // editor_gutter_modified_background: palette.blue[2],
            // editor_gutter_added_background: palette.green[1],
            // editor_gutter_deleted_background: palette.red[2],

            // diff_editor_inserted_text_background: generate_color(palette.bg[0].into(), 270.0)
            //     .into(),
            // diff_editor_removed_text_background: generate_color(palette.bg[0].into(), 90.0).into(),
            // scrollbar_shadow: palette.bg[4],
            // scrollbar_slider_background: palette.bg[2],
            // scrollbar_slider_hover_background: palette.bg[1],
            // scrollbar_slider_active_background: palette.bg[0],
            // editor_overview_ruler_border: palette.gray[2],

            // panel_background: palette.bg[2],
            // panel_border: palette.gray[2],
            // panel_title_active_border: palette.gray[0],
            // panel_title_active_foreground: palette.gray[0],
            // panel_title_inactive_foreground: palette.gray[4],
            // panel_input_border: palette.gray[0],
            // terminal_foreground: fg[3],
            // terminal_tab_active_border: palette.gray[4],
            // terminal_cursor_background: palette.bg[2],
            // terminal_cursor_foreground: palette.gray[2],

            // terminal_ansi_bright_white: fg[4],
            // terminal_ansi_white: fg[4],
            // terminal_ansi_bright_black: fg[0],
            // terminal_ansi_black: fg[0],
            // terminal_ansi_blue: palette.blue[2],
            // terminal_ansi_bright_blue: palette.blue[2],
            // terminal_ansi_green: palette.green[2],
            // terminal_ansi_bright_green: palette.green[2],
            // terminal_ansi_cyan: palette.blue[2],
            // terminal_ansi_bright_cyan: palette.blue[2],
            // terminal_ansi_red: palette.red[2],
            // terminal_ansi_bright_red: palette.red[2],
            // terminal_ansi_magenta: palette.purple[2],
            // terminal_ansi_bright_magenta: palette.purple[2],
            // terminal_ansi_yellow: palette.yellow[2],
            // terminal_ansi_bright_yellow: palette.yellow[2],
            editor_bracket_highlight_foreground1: palette.blue[3],
            editor_bracket_highlight_foreground2: palette.orange[3],
            editor_bracket_highlight_foreground3: palette.purple[3],
            editor_bracket_highlight_foreground4: palette.blue[3],
            editor_bracket_highlight_foreground5: palette.orange[3],
            editor_bracket_highlight_foreground6: palette.purple[3],
            // debug_tool_bar_background: palette.bg[2],
            // editor_stack_frame_highlight_background: palette.bg[3],
            // editor_focused_stack_frame_highlight_background: palette.bg[3],

            // peek_view_editor_match_highlight_background: palette.bg[3],
            // peek_view_result_match_highlight_background: palette.bg[4],
            // peek_view_editor_background: palette.bg[0],
            // peek_view_result_background: palette.bg[1],

            // settings_header_foreground: fg[2],
            // settings_modified_item_indicator: palette.blue[2],
            // welcome_page_button_background: palette.bg[0],
            // welcome_page_button_hover_background: palette.bg[1],
        }
    }
}
