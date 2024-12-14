use serde::Serialize;

use crate::{
    model::{Color, HexStr},
    util::ReplaceAlphaExt,
    schema::palette::FullPaletteFile,
};

#[derive(Serialize)]
pub struct WorkbenchColorCustomizations {
    #[serde(rename = "focusBorder")]
    pub focus_border: HexStr,
    #[serde(rename = "foreground")]
    pub foreground: HexStr,
    #[serde(rename = "descriptionForeground")]
    pub description_foreground: HexStr,
    #[serde(rename = "errorForeground")]
    pub error_foreground: HexStr,
    #[serde(rename = "textLink.foreground")]
    pub text_link_foreground: HexStr,
    #[serde(rename = "textLink.activeForeground")]
    pub text_link_active_foreground: HexStr,
    #[serde(rename = "textBlockQuote.background")]
    pub text_block_quote_background: HexStr,
    #[serde(rename = "textBlockQuote.border")]
    pub text_block_quote_border: HexStr,
    #[serde(rename = "textCodeBlock.background")]
    pub text_code_block_background: HexStr,
    #[serde(rename = "textPreformat.foreground")]
    pub text_preformat_foreground: HexStr,
    #[serde(rename = "textSeparator.foreground")]
    pub text_separator_foreground: HexStr,
    #[serde(rename = "button.background")]
    pub button_background: HexStr,
    #[serde(rename = "button.foreground")]
    pub button_foreground: HexStr,
    #[serde(rename = "button.hoverBackground")]
    pub button_hover_background: HexStr,
    #[serde(rename = "button.secondaryBackground")]
    pub button_secondary_background: HexStr,
    #[serde(rename = "button.secondaryForeground")]
    pub button_secondary_foreground: HexStr,
    #[serde(rename = "button.secondaryHoverBackground")]
    pub button_secondary_hover_background: HexStr,
    #[serde(rename = "checkbox.background")]
    pub checkbox_background: HexStr,
    #[serde(rename = "checkbox.border")]
    pub checkbox_border: HexStr,
    #[serde(rename = "dropdown.background")]
    pub dropdown_background: HexStr,
    #[serde(rename = "dropdown.border")]
    pub dropdown_border: HexStr,
    #[serde(rename = "dropdown.foreground")]
    pub dropdown_foreground: HexStr,
    #[serde(rename = "dropdown.listBackground")]
    pub dropdown_list_background: HexStr,
    #[serde(rename = "input.background")]
    pub input_background: HexStr,
    #[serde(rename = "input.border")]
    pub input_border: HexStr,
    #[serde(rename = "input.foreground")]
    pub input_foreground: HexStr,
    #[serde(rename = "input.placeholderForeground")]
    pub input_placeholder_foreground: HexStr,
    #[serde(rename = "badge.foreground")]
    pub badge_foreground: HexStr,
    #[serde(rename = "badge.background")]
    pub badge_background: HexStr,
    #[serde(rename = "progressBar.background")]
    pub progress_bar_background: HexStr,
    #[serde(rename = "titleBar.activeForeground")]
    pub title_bar_active_foreground: HexStr,
    #[serde(rename = "titleBar.activeBackground")]
    pub title_bar_active_background: HexStr,
    #[serde(rename = "titleBar.inactiveForeground")]
    pub title_bar_inactive_foreground: HexStr,
    #[serde(rename = "titleBar.inactiveBackground")]
    pub title_bar_inactive_background: HexStr,
    #[serde(rename = "titleBar.border")]
    pub title_bar_border: HexStr,
    #[serde(rename = "activityBar.foreground")]
    pub activity_bar_foreground: HexStr,
    #[serde(rename = "activityBar.inactiveForeground")]
    pub activity_bar_inactive_foreground: HexStr,
    #[serde(rename = "activityBar.background")]
    pub activity_bar_background: HexStr,
    #[serde(rename = "activityBarBadge.foreground")]
    pub activity_bar_badge_foreground: HexStr,
    #[serde(rename = "activityBarBadge.background")]
    pub activity_bar_badge_background: HexStr,
    #[serde(rename = "activityBar.activeBorder")]
    pub activity_bar_active_border: HexStr,
    #[serde(rename = "activityBar.border")]
    pub activity_bar_border: HexStr,
    #[serde(rename = "sideBar.foreground")]
    pub side_bar_foreground: HexStr,
    #[serde(rename = "sideBar.background")]
    pub side_bar_background: HexStr,
    #[serde(rename = "sideBar.border")]
    pub side_bar_border: HexStr,
    #[serde(rename = "sideBarTitle.foreground")]
    pub side_bar_title_foreground: HexStr,
    #[serde(rename = "sideBarSectionHeader.foreground")]
    pub side_bar_section_header_foreground: HexStr,
    #[serde(rename = "sideBarSectionHeader.background")]
    pub side_bar_section_header_background: HexStr,
    #[serde(rename = "sideBarSectionHeader.border")]
    pub side_bar_section_header_border: HexStr,
    #[serde(rename = "list.hoverForeground")]
    pub list_hover_foreground: HexStr,
    #[serde(rename = "list.inactiveSelectionForeground")]
    pub list_inactive_selection_foreground: HexStr,
    #[serde(rename = "list.activeSelectionForeground")]
    pub list_active_selection_foreground: HexStr,
    #[serde(rename = "list.hoverBackground")]
    pub list_hover_background: HexStr,
    #[serde(rename = "list.inactiveSelectionBackground")]
    pub list_inactive_selection_background: HexStr,
    #[serde(rename = "list.activeSelectionBackground")]
    pub list_active_selection_background: HexStr,
    #[serde(rename = "list.inactiveFocusBackground")]
    pub list_inactive_focus_background: HexStr,
    #[serde(rename = "list.focusBackground")]
    pub list_focus_background: HexStr,
    #[serde(rename = "tree.indentGuidesStroke")]
    pub tree_indent_guides_stroke: HexStr,
    #[serde(rename = "notificationCenterHeader.foreground")]
    pub notification_center_header_foreground: HexStr,
    #[serde(rename = "notificationCenterHeader.background")]
    pub notification_center_header_background: HexStr,
    #[serde(rename = "notifications.foreground")]
    pub notifications_foreground: HexStr,
    #[serde(rename = "notifications.background")]
    pub notifications_background: HexStr,
    #[serde(rename = "notifications.border")]
    pub notifications_border: HexStr,
    #[serde(rename = "notificationsErrorIcon.foreground")]
    pub notifications_error_icon_foreground: HexStr,
    #[serde(rename = "notificationsWarningIcon.foreground")]
    pub notifications_warning_icon_foreground: HexStr,
    #[serde(rename = "notificationsInfoIcon.foreground")]
    pub notifications_info_icon_foreground: HexStr,
    #[serde(rename = "pickerGroup.border")]
    pub picker_group_border: HexStr,
    #[serde(rename = "pickerGroup.foreground")]
    pub picker_group_foreground: HexStr,
    #[serde(rename = "quickInput.background")]
    pub quick_input_background: HexStr,
    #[serde(rename = "quickInput.foreground")]
    pub quick_input_foreground: HexStr,
    #[serde(rename = "statusBar.foreground")]
    pub status_bar_foreground: HexStr,
    #[serde(rename = "statusBar.background")]
    pub status_bar_background: HexStr,
    #[serde(rename = "statusBar.border")]
    pub status_bar_border: HexStr,
    #[serde(rename = "statusBar.noFolderBackground")]
    pub status_bar_no_folder_background: HexStr,
    #[serde(rename = "statusBar.debuggingBackground")]
    pub status_bar_debugging_background: HexStr,
    #[serde(rename = "statusBar.debuggingForeground")]
    pub status_bar_debugging_foreground: HexStr,
    #[serde(rename = "statusBarItem.prominentBackground")]
    pub status_bar_item_prominent_background: HexStr,
    #[serde(rename = "statusBarItem.remoteForeground")]
    pub status_bar_item_remote_foreground: HexStr,
    #[serde(rename = "statusBarItem.remoteBackground")]
    pub status_bar_item_remote_background: HexStr,
    #[serde(rename = "editorGroupHeader.tabsBackground")]
    pub editor_group_header_tabs_background: HexStr,
    #[serde(rename = "editorGroupHeader.tabsBorder")]
    pub editor_group_header_tabs_border: HexStr,
    #[serde(rename = "editorGroup.border")]
    pub editor_group_border: HexStr,
    #[serde(rename = "tab.activeForeground")]
    pub tab_active_foreground: HexStr,
    #[serde(rename = "tab.inactiveForeground")]
    pub tab_inactive_foreground: HexStr,
    #[serde(rename = "tab.inactiveBackground")]
    pub tab_inactive_background: HexStr,
    #[serde(rename = "tab.activeBackground")]
    pub tab_active_background: HexStr,
    #[serde(rename = "tab.hoverBackground")]
    pub tab_hover_background: HexStr,
    #[serde(rename = "tab.unfocusedHoverBackground")]
    pub tab_unfocused_hover_background: HexStr,
    #[serde(rename = "tab.border")]
    pub tab_border: HexStr,
    #[serde(rename = "tab.unfocusedActiveBorderTop")]
    pub tab_unfocused_active_border_top: HexStr,
    #[serde(rename = "tab.activeBorder")]
    pub tab_active_border: HexStr,
    #[serde(rename = "tab.unfocusedActiveBorder")]
    pub tab_unfocused_active_border: HexStr,
    #[serde(rename = "tab.activeBorderTop")]
    pub tab_active_border_top: HexStr,
    #[serde(rename = "breadcrumb.foreground")]
    pub breadcrumb_foreground: HexStr,
    #[serde(rename = "breadcrumb.focusForeground")]
    pub breadcrumb_focus_foreground: HexStr,
    #[serde(rename = "breadcrumb.activeSelectionForeground")]
    pub breadcrumb_active_selection_foreground: HexStr,
    #[serde(rename = "breadcrumbPicker.background")]
    pub breadcrumb_picker_background: HexStr,
    #[serde(rename = "editor.foreground")]
    pub editor_foreground: HexStr,
    #[serde(rename = "editor.background")]
    pub editor_background: HexStr,
    #[serde(rename = "editorWidget.background")]
    pub editor_widget_background: HexStr,
    #[serde(rename = "editor.foldBackground")]
    pub editor_fold_background: HexStr,
    #[serde(rename = "editor.lineHighlightBackground")]
    pub editor_line_highlight_background: HexStr,
    #[serde(rename = "editorLineNumber.foreground")]
    pub editor_line_number_foreground: HexStr,
    #[serde(rename = "editorLineNumber.activeForeground")]
    pub editor_line_number_active_foreground: HexStr,
    #[serde(rename = "editorIndentGuide.background")]
    pub editor_indent_guide_background: HexStr,
    #[serde(rename = "editorIndentGuide.activeBackground")]
    pub editor_indent_guide_active_background: HexStr,
    #[serde(rename = "editorWhitespace.foreground")]
    pub editor_whitespace_foreground: HexStr,
    #[serde(rename = "editorCursor.foreground")]
    pub editor_cursor_foreground: HexStr,
    #[serde(rename = "editorError.foreground")]
    pub editor_error_foreground: HexStr,
    #[serde(rename = "editorWarning.foreground")]
    pub editor_warning_foreground: HexStr,
    #[serde(rename = "editor.findMatchBackground")]
    pub editor_find_match_background: HexStr,
    #[serde(rename = "editor.findMatchHighlightBackground")]
    pub editor_find_match_highlight_background: HexStr,
    #[serde(rename = "editor.linkedEditingBackground")]
    pub editor_linked_editing_background: HexStr,
    #[serde(rename = "editor.inactiveSelectionBackground")]
    pub editor_inactive_selection_background: HexStr,
    #[serde(rename = "editor.selectionBackground")]
    pub editor_selection_background: HexStr,
    #[serde(rename = "editor.selectionHighlightBackground")]
    pub editor_selection_highlight_background: HexStr,
    #[serde(rename = "editor.selectionHighlightBorder")]
    pub editor_selection_highlight_border: HexStr,
    #[serde(rename = "editor.wordHighlightBackground")]
    pub editor_word_highlight_background: HexStr,
    #[serde(rename = "editor.wordHighlightStrongBackground")]
    pub editor_word_highlight_strong_background: HexStr,
    #[serde(rename = "editor.wordHighlightBorder")]
    pub editor_word_highlight_border: HexStr,
    #[serde(rename = "editor.wordHighlightStrongBorder")]
    pub editor_word_highlight_strong_border: HexStr,
    #[serde(rename = "editorBracketMatch.background")]
    pub editor_bracket_match_background: HexStr,
    #[serde(rename = "editorBracketMatch.border")]
    pub editor_bracket_match_border: HexStr,
    #[serde(rename = "editorGutter.modifiedBackground")]
    pub editor_gutter_modified_background: HexStr,
    #[serde(rename = "editorGutter.addedBackground")]
    pub editor_gutter_added_background: HexStr,
    #[serde(rename = "editorGutter.deletedBackground")]
    pub editor_gutter_deleted_background: HexStr,
    // #[serde(rename = "diffEditor.insertedTextBackground")]
    // pub diff_editor_inserted_text_background: WrapSrgb,
    // #[serde(rename = "diffEditor.removedTextBackground")]
    // pub diff_editor_removed_text_background: WrapSrgb,
    #[serde(rename = "scrollbar.shadow")]
    pub scrollbar_shadow: HexStr,
    #[serde(rename = "scrollbarSlider.background")]
    pub scrollbar_slider_background: HexStr,
    #[serde(rename = "scrollbarSlider.hoverBackground")]
    pub scrollbar_slider_hover_background: HexStr,
    #[serde(rename = "scrollbarSlider.activeBackground")]
    pub scrollbar_slider_active_background: HexStr,
    #[serde(rename = "editorOverviewRuler.border")]
    pub editor_overview_ruler_border: HexStr,
    #[serde(rename = "panel.background")]
    pub panel_background: HexStr,
    #[serde(rename = "panel.border")]
    pub panel_border: HexStr,
    #[serde(rename = "panelTitle.activeBorder")]
    pub panel_title_active_border: HexStr,
    #[serde(rename = "panelTitle.activeForeground")]
    pub panel_title_active_foreground: HexStr,
    #[serde(rename = "panelTitle.inactiveForeground")]
    pub panel_title_inactive_foreground: HexStr,
    #[serde(rename = "panelInput.border")]
    pub panel_input_border: HexStr,
    #[serde(rename = "terminal.foreground")]
    pub terminal_foreground: HexStr,
    #[serde(rename = "terminal.tab.activeBorder")]
    pub terminal_tab_active_border: HexStr,
    #[serde(rename = "terminalCursor.background")]
    pub terminal_cursor_background: HexStr,
    #[serde(rename = "terminalCursor.foreground")]
    pub terminal_cursor_foreground: HexStr,
    #[serde(rename = "terminal.ansiBrightWhite")]
    pub terminal_ansi_bright_white: HexStr,
    #[serde(rename = "terminal.ansiWhite")]
    pub terminal_ansi_white: HexStr,
    #[serde(rename = "terminal.ansiBrightBlack")]
    pub terminal_ansi_bright_black: HexStr,
    #[serde(rename = "terminal.ansiBlack")]
    pub terminal_ansi_black: HexStr,
    #[serde(rename = "terminal.ansiBlue")]
    pub terminal_ansi_blue: HexStr,
    #[serde(rename = "terminal.ansiBrightBlue")]
    pub terminal_ansi_bright_blue: HexStr,
    #[serde(rename = "terminal.ansiGreen")]
    pub terminal_ansi_green: HexStr,
    #[serde(rename = "terminal.ansiBrightGreen")]
    pub terminal_ansi_bright_green: HexStr,
    #[serde(rename = "terminal.ansiCyan")]
    pub terminal_ansi_cyan: HexStr,
    #[serde(rename = "terminal.ansiBrightCyan")]
    pub terminal_ansi_bright_cyan: HexStr,
    #[serde(rename = "terminal.ansiRed")]
    pub terminal_ansi_red: HexStr,
    #[serde(rename = "terminal.ansiBrightRed")]
    pub terminal_ansi_bright_red: HexStr,
    #[serde(rename = "terminal.ansiMagenta")]
    pub terminal_ansi_magenta: HexStr,
    #[serde(rename = "terminal.ansiBrightMagenta")]
    pub terminal_ansi_bright_magenta: HexStr,
    #[serde(rename = "terminal.ansiYellow")]
    pub terminal_ansi_yellow: HexStr,
    #[serde(rename = "terminal.ansiBrightYellow")]
    pub terminal_ansi_bright_yellow: HexStr,
    #[serde(rename = "editorBracketHighlight.foreground1")]
    pub editor_bracket_highlight_foreground1: HexStr,
    #[serde(rename = "editorBracketHighlight.foreground2")]
    pub editor_bracket_highlight_foreground2: HexStr,
    #[serde(rename = "editorBracketHighlight.foreground3")]
    pub editor_bracket_highlight_foreground3: HexStr,
    #[serde(rename = "editorBracketHighlight.foreground4")]
    pub editor_bracket_highlight_foreground4: HexStr,
    #[serde(rename = "editorBracketHighlight.foreground5")]
    pub editor_bracket_highlight_foreground5: HexStr,
    #[serde(rename = "editorBracketHighlight.foreground6")]
    pub editor_bracket_highlight_foreground6: HexStr,
    #[serde(rename = "debugToolBar.background")]
    pub debug_tool_bar_background: HexStr,
    #[serde(rename = "editor.stackFrameHighlightBackground")]
    pub editor_stack_frame_highlight_background: HexStr,
    #[serde(rename = "editor.focusedStackFrameHighlightBackground")]
    pub editor_focused_stack_frame_highlight_background: HexStr,
    #[serde(rename = "peekViewEditor.matchHighlightBackground")]
    pub peek_view_editor_match_highlight_background: HexStr,
    #[serde(rename = "peekViewResult.matchHighlightBackground")]
    pub peek_view_result_match_highlight_background: HexStr,
    #[serde(rename = "peekViewEditor.background")]
    pub peek_view_editor_background: HexStr,
    #[serde(rename = "peekViewResult.background")]
    pub peek_view_result_background: HexStr,
    #[serde(rename = "settings.headerForeground")]
    pub settings_header_foreground: HexStr,
    #[serde(rename = "settings.modifiedItemIndicator")]
    pub settings_modified_item_indicator: HexStr,
    #[serde(rename = "welcomePage.buttonBackground")]
    pub welcome_page_button_background: HexStr,
    #[serde(rename = "welcomePage.buttonHoverBackground")]
    pub welcome_page_button_hover_background: HexStr,
}

impl WorkbenchColorCustomizations {
    pub fn new(palette: &FullPaletteFile, no_saturation_fg: bool) -> Self {
        let fg = if no_saturation_fg {
            &palette.fg
        } else {
            &palette.color_map[Color::Gray]
        };
        Self {
            focus_border: palette.color_map[Color::Blue][2],
            foreground: fg[3],
            description_foreground: palette.color_map[Color::Gray][2],
            error_foreground: palette.color_map[Color::Red][3],

            text_link_foreground: palette.color_map[Color::Blue][3],
            text_link_active_foreground: palette.color_map[Color::Blue][3],
            text_block_quote_background: palette.color_map[Color::Bg][0],
            text_block_quote_border: palette.color_map[Color::Gray][1],
            text_code_block_background: palette.color_map[Color::Bg][0],
            text_preformat_foreground: palette.color_map[Color::Gray][3],
            text_separator_foreground: palette.color_map[Color::Gray][1],

            button_background: palette.color_map[Color::Bg][0],
            button_foreground: fg[4],
            button_hover_background: palette.color_map[Color::Bg][1],

            button_secondary_background: palette.color_map[Color::Bg][1],
            button_secondary_foreground: fg[3],
            button_secondary_hover_background: palette.color_map[Color::Bg][2],

            checkbox_background: palette.color_map[Color::Bg][1],
            checkbox_border: palette.color_map[Color::Gray][3],

            dropdown_background: palette.color_map[Color::Bg][1],
            dropdown_border: palette.color_map[Color::Gray][3],
            dropdown_foreground: fg[3],
            dropdown_list_background: palette.color_map[Color::Bg][0],

            input_background: palette.color_map[Color::Bg][1],
            input_border: palette.color_map[Color::Gray][3],
            input_foreground: fg[3],
            input_placeholder_foreground: fg[2],

            badge_foreground: fg[3],
            badge_background: palette.color_map[Color::Bg][1],

            progress_bar_background: palette.color_map[Color::Blue][2],

            title_bar_active_foreground: fg[3],
            title_bar_active_background: palette.color_map[Color::Bg][2],
            title_bar_inactive_foreground: fg[2],
            title_bar_inactive_background: palette.color_map[Color::Bg][1],
            title_bar_border: palette.color_map[Color::Gray][3],

            activity_bar_foreground: fg[3],
            activity_bar_inactive_foreground: palette.color_map[Color::Gray][2],
            activity_bar_background: palette.color_map[Color::Bg][2],
            activity_bar_badge_foreground: fg[3],
            activity_bar_badge_background: palette.color_map[Color::Bg][2],
            activity_bar_active_border: palette.color_map[Color::Gray][2],
            activity_bar_border: palette.color_map[Color::Gray][3],

            side_bar_foreground: fg[3],
            side_bar_background: palette.color_map[Color::Bg][3],
            side_bar_border: palette.color_map[Color::Gray][0],
            side_bar_title_foreground: fg[3],
            side_bar_section_header_foreground: fg[3],
            side_bar_section_header_background: palette.color_map[Color::Bg][3],
            side_bar_section_header_border: palette.color_map[Color::Gray][3],

            list_hover_foreground: fg[3],
            list_inactive_selection_foreground: fg[3],
            list_active_selection_foreground: fg[3],
            list_hover_background: palette.color_map[Color::Bg][3],
            list_inactive_selection_background: palette.color_map[Color::Bg][2],
            list_active_selection_background: palette.color_map[Color::Bg][1],
            list_inactive_focus_background: palette.color_map[Color::Bg][0],
            list_focus_background: palette.color_map[Color::Bg][4],

            tree_indent_guides_stroke: palette.color_map[Color::Gray][1],

            notification_center_header_foreground: fg[2],
            notification_center_header_background: palette.color_map[Color::Bg][0],
            notifications_foreground: fg[2],
            notifications_background: palette.color_map[Color::Bg][2],
            notifications_border: palette.color_map[Color::Gray][3],
            notifications_error_icon_foreground: palette.color_map[Color::Red][0],
            notifications_warning_icon_foreground: palette.color_map[Color::Yellow][2],
            notifications_info_icon_foreground: palette.color_map[Color::Blue][3],

            picker_group_border: palette.color_map[Color::Gray][1],
            picker_group_foreground: fg[2],

            quick_input_background: palette.color_map[Color::Bg][0],
            quick_input_foreground: fg[2],

            status_bar_foreground: fg[2],
            status_bar_background: palette.color_map[Color::Bg][2],
            status_bar_border: palette.color_map[Color::Gray][2],
            status_bar_no_folder_background: palette.color_map[Color::Bg][0],
            status_bar_debugging_background: palette.color_map[Color::Bg][2],
            status_bar_debugging_foreground: palette.color_map[Color::Gray][2],
            status_bar_item_prominent_background: palette.color_map[Color::Gray][3],
            status_bar_item_remote_foreground: fg[3],
            status_bar_item_remote_background: palette.color_map[Color::Bg][0],

            editor_group_header_tabs_background: palette.color_map[Color::Bg][1],
            editor_group_header_tabs_border: palette.color_map[Color::Gray][2],
            editor_group_border: palette.color_map[Color::Gray][2],

            tab_active_foreground: fg[2],
            tab_inactive_foreground: fg[3],
            tab_inactive_background: palette.color_map[Color::Bg][2],
            tab_active_background: palette.color_map[Color::Bg][4],
            tab_hover_background: palette.color_map[Color::Bg][3],
            tab_unfocused_hover_background: palette.color_map[Color::Bg][4],
            tab_border: palette.color_map[Color::Gray][2],
            tab_unfocused_active_border_top: palette.color_map[Color::Gray][2],
            tab_active_border: palette.color_map[Color::Gray][0],
            tab_unfocused_active_border: palette.color_map[Color::Gray][0],
            tab_active_border_top: palette.color_map[Color::Gray][1],

            breadcrumb_foreground: fg[3],
            breadcrumb_focus_foreground: fg[2],
            breadcrumb_active_selection_foreground: palette.color_map[Color::Gray][4],
            breadcrumb_picker_background: palette.color_map[Color::Bg][2],

            editor_foreground: fg[2],
            editor_background: palette.color_map[Color::Bg][2],
            editor_widget_background: palette.color_map[Color::Bg][3],
            editor_fold_background: palette.color_map[Color::Bg][4].alpha(0.5),
            editor_line_highlight_background: palette.color_map[Color::Bg][4],
            editor_line_number_foreground: fg[1],
            editor_line_number_active_foreground: fg[2],
            editor_indent_guide_background: palette.color_map[Color::Bg][0],
            editor_indent_guide_active_background: palette.color_map[Color::Bg][1],
            editor_whitespace_foreground: fg[1],
            editor_cursor_foreground: fg[3],
            editor_error_foreground: palette.color_map[Color::Red][3],
            editor_warning_foreground: palette.color_map[Color::Yellow][3],

            editor_find_match_background: palette.color_map[Color::Bg][4],
            editor_find_match_highlight_background: palette.color_map[Color::Bg][0].alpha(0.5),
            editor_linked_editing_background: palette.color_map[Color::Bg][4],
            editor_inactive_selection_background: palette.color_map[Color::Bg][4].alpha(0.5),
            editor_selection_background: palette.color_map[Color::Bg][4],
            editor_selection_highlight_background: palette.color_map[Color::Bg][4].alpha(0.5),
            editor_selection_highlight_border: palette.color_map[Color::Gray][0],
            editor_word_highlight_background: palette.color_map[Color::Bg][4].alpha(0.5),
            editor_word_highlight_strong_background: palette.color_map[Color::Bg][4].alpha(0.5),
            editor_word_highlight_border: palette.color_map[Color::Gray][0],
            editor_word_highlight_strong_border: palette.color_map[Color::Gray][0],
            editor_bracket_match_background: palette.color_map[Color::Bg][4],
            editor_bracket_match_border: palette.color_map[Color::Gray][0],

            editor_gutter_modified_background: palette.color_map[Color::Blue][2],
            editor_gutter_added_background: palette.color_map[Color::Green][1],
            editor_gutter_deleted_background: palette.color_map[Color::Red][2],

            // diff_editor_inserted_text_background: generate_color(palette.base_color_table[Color::Bg][0].into(), 270.0)
            //     .into(),
            // diff_editor_removed_text_background: generate_color(palette.base_color_table[Color::Bg][0].into(), 90.0).into(),
            scrollbar_shadow: palette.color_map[Color::Bg][4],
            scrollbar_slider_background: palette.color_map[Color::Bg][2],
            scrollbar_slider_hover_background: palette.color_map[Color::Bg][1],
            scrollbar_slider_active_background: palette.color_map[Color::Bg][0],
            editor_overview_ruler_border: palette.color_map[Color::Gray][2],

            panel_background: palette.color_map[Color::Bg][2],
            panel_border: palette.color_map[Color::Gray][2],
            panel_title_active_border: palette.color_map[Color::Gray][0],
            panel_title_active_foreground: palette.color_map[Color::Gray][0],
            panel_title_inactive_foreground: palette.color_map[Color::Gray][4],
            panel_input_border: palette.color_map[Color::Gray][0],

            terminal_foreground: fg[3],
            terminal_tab_active_border: palette.color_map[Color::Gray][4],
            terminal_cursor_background: palette.color_map[Color::Bg][2],
            terminal_cursor_foreground: palette.color_map[Color::Gray][2],

            terminal_ansi_bright_white: fg[4],
            terminal_ansi_white: fg[4],
            terminal_ansi_bright_black: fg[0],
            terminal_ansi_black: fg[0],
            terminal_ansi_blue: palette.color_map[Color::Blue][2],
            terminal_ansi_bright_blue: palette.color_map[Color::Blue][2],
            terminal_ansi_green: palette.color_map[Color::Green][2],
            terminal_ansi_bright_green: palette.color_map[Color::Green][2],
            terminal_ansi_cyan: palette.color_map[Color::Blue][2],
            terminal_ansi_bright_cyan: palette.color_map[Color::Blue][2],
            terminal_ansi_red: palette.color_map[Color::Red][2],
            terminal_ansi_bright_red: palette.color_map[Color::Red][2],
            terminal_ansi_magenta: palette.color_map[Color::Purple][2],
            terminal_ansi_bright_magenta: palette.color_map[Color::Purple][2],
            terminal_ansi_yellow: palette.color_map[Color::Yellow][2],
            terminal_ansi_bright_yellow: palette.color_map[Color::Yellow][2],

            editor_bracket_highlight_foreground1: palette.color_map[Color::Blue][3],
            editor_bracket_highlight_foreground2: palette.color_map[Color::Orange][3],
            editor_bracket_highlight_foreground3: palette.color_map[Color::Purple][3],
            editor_bracket_highlight_foreground4: palette.color_map[Color::Blue][3],
            editor_bracket_highlight_foreground5: palette.color_map[Color::Orange][3],
            editor_bracket_highlight_foreground6: palette.color_map[Color::Purple][3],

            debug_tool_bar_background: palette.color_map[Color::Bg][2],
            editor_stack_frame_highlight_background: palette.color_map[Color::Bg][3],
            editor_focused_stack_frame_highlight_background: palette.color_map[Color::Bg][3],

            peek_view_editor_match_highlight_background: palette.color_map[Color::Bg][3],
            peek_view_result_match_highlight_background: palette.color_map[Color::Bg][4],
            peek_view_editor_background: palette.color_map[Color::Bg][0],
            peek_view_result_background: palette.color_map[Color::Bg][1],

            settings_header_foreground: fg[2],
            settings_modified_item_indicator: palette.color_map[Color::Blue][2],
            welcome_page_button_background: palette.color_map[Color::Bg][0],
            welcome_page_button_hover_background: palette.color_map[Color::Bg][1],
        }
    }
}
