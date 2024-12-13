use serde::Serialize;

use crate::{
    color::{Color, SrgbA},
    palette::wrap::wrap_full_palette::WrapFullPalette,
};

#[derive(Serialize)]
pub struct WorkbenchColorCustomizations {
    #[serde(rename = "focusBorder")]
    pub focus_border: SrgbA,
    #[serde(rename = "foreground")]
    pub foreground: SrgbA,
    #[serde(rename = "descriptionForeground")]
    pub description_foreground: SrgbA,
    #[serde(rename = "errorForeground")]
    pub error_foreground: SrgbA,
    #[serde(rename = "textLink.foreground")]
    pub text_link_foreground: SrgbA,
    #[serde(rename = "textLink.activeForeground")]
    pub text_link_active_foreground: SrgbA,
    #[serde(rename = "textBlockQuote.background")]
    pub text_block_quote_background: SrgbA,
    #[serde(rename = "textBlockQuote.border")]
    pub text_block_quote_border: SrgbA,
    #[serde(rename = "textCodeBlock.background")]
    pub text_code_block_background: SrgbA,
    #[serde(rename = "textPreformat.foreground")]
    pub text_preformat_foreground: SrgbA,
    #[serde(rename = "textSeparator.foreground")]
    pub text_separator_foreground: SrgbA,
    #[serde(rename = "button.background")]
    pub button_background: SrgbA,
    #[serde(rename = "button.foreground")]
    pub button_foreground: SrgbA,
    #[serde(rename = "button.hoverBackground")]
    pub button_hover_background: SrgbA,
    #[serde(rename = "button.secondaryBackground")]
    pub button_secondary_background: SrgbA,
    #[serde(rename = "button.secondaryForeground")]
    pub button_secondary_foreground: SrgbA,
    #[serde(rename = "button.secondaryHoverBackground")]
    pub button_secondary_hover_background: SrgbA,
    #[serde(rename = "checkbox.background")]
    pub checkbox_background: SrgbA,
    #[serde(rename = "checkbox.border")]
    pub checkbox_border: SrgbA,
    #[serde(rename = "dropdown.background")]
    pub dropdown_background: SrgbA,
    #[serde(rename = "dropdown.border")]
    pub dropdown_border: SrgbA,
    #[serde(rename = "dropdown.foreground")]
    pub dropdown_foreground: SrgbA,
    #[serde(rename = "dropdown.listBackground")]
    pub dropdown_list_background: SrgbA,
    #[serde(rename = "input.background")]
    pub input_background: SrgbA,
    #[serde(rename = "input.border")]
    pub input_border: SrgbA,
    #[serde(rename = "input.foreground")]
    pub input_foreground: SrgbA,
    #[serde(rename = "input.placeholderForeground")]
    pub input_placeholder_foreground: SrgbA,
    #[serde(rename = "badge.foreground")]
    pub badge_foreground: SrgbA,
    #[serde(rename = "badge.background")]
    pub badge_background: SrgbA,
    #[serde(rename = "progressBar.background")]
    pub progress_bar_background: SrgbA,
    #[serde(rename = "titleBar.activeForeground")]
    pub title_bar_active_foreground: SrgbA,
    #[serde(rename = "titleBar.activeBackground")]
    pub title_bar_active_background: SrgbA,
    #[serde(rename = "titleBar.inactiveForeground")]
    pub title_bar_inactive_foreground: SrgbA,
    #[serde(rename = "titleBar.inactiveBackground")]
    pub title_bar_inactive_background: SrgbA,
    #[serde(rename = "titleBar.border")]
    pub title_bar_border: SrgbA,
    #[serde(rename = "activityBar.foreground")]
    pub activity_bar_foreground: SrgbA,
    #[serde(rename = "activityBar.inactiveForeground")]
    pub activity_bar_inactive_foreground: SrgbA,
    #[serde(rename = "activityBar.background")]
    pub activity_bar_background: SrgbA,
    #[serde(rename = "activityBarBadge.foreground")]
    pub activity_bar_badge_foreground: SrgbA,
    #[serde(rename = "activityBarBadge.background")]
    pub activity_bar_badge_background: SrgbA,
    #[serde(rename = "activityBar.activeBorder")]
    pub activity_bar_active_border: SrgbA,
    #[serde(rename = "activityBar.border")]
    pub activity_bar_border: SrgbA,
    #[serde(rename = "sideBar.foreground")]
    pub side_bar_foreground: SrgbA,
    #[serde(rename = "sideBar.background")]
    pub side_bar_background: SrgbA,
    #[serde(rename = "sideBar.border")]
    pub side_bar_border: SrgbA,
    #[serde(rename = "sideBarTitle.foreground")]
    pub side_bar_title_foreground: SrgbA,
    #[serde(rename = "sideBarSectionHeader.foreground")]
    pub side_bar_section_header_foreground: SrgbA,
    #[serde(rename = "sideBarSectionHeader.background")]
    pub side_bar_section_header_background: SrgbA,
    #[serde(rename = "sideBarSectionHeader.border")]
    pub side_bar_section_header_border: SrgbA,
    #[serde(rename = "list.hoverForeground")]
    pub list_hover_foreground: SrgbA,
    #[serde(rename = "list.inactiveSelectionForeground")]
    pub list_inactive_selection_foreground: SrgbA,
    #[serde(rename = "list.activeSelectionForeground")]
    pub list_active_selection_foreground: SrgbA,
    #[serde(rename = "list.hoverBackground")]
    pub list_hover_background: SrgbA,
    #[serde(rename = "list.inactiveSelectionBackground")]
    pub list_inactive_selection_background: SrgbA,
    #[serde(rename = "list.activeSelectionBackground")]
    pub list_active_selection_background: SrgbA,
    #[serde(rename = "list.inactiveFocusBackground")]
    pub list_inactive_focus_background: SrgbA,
    #[serde(rename = "list.focusBackground")]
    pub list_focus_background: SrgbA,
    #[serde(rename = "tree.indentGuidesStroke")]
    pub tree_indent_guides_stroke: SrgbA,
    #[serde(rename = "notificationCenterHeader.foreground")]
    pub notification_center_header_foreground: SrgbA,
    #[serde(rename = "notificationCenterHeader.background")]
    pub notification_center_header_background: SrgbA,
    #[serde(rename = "notifications.foreground")]
    pub notifications_foreground: SrgbA,
    #[serde(rename = "notifications.background")]
    pub notifications_background: SrgbA,
    #[serde(rename = "notifications.border")]
    pub notifications_border: SrgbA,
    #[serde(rename = "notificationsErrorIcon.foreground")]
    pub notifications_error_icon_foreground: SrgbA,
    #[serde(rename = "notificationsWarningIcon.foreground")]
    pub notifications_warning_icon_foreground: SrgbA,
    #[serde(rename = "notificationsInfoIcon.foreground")]
    pub notifications_info_icon_foreground: SrgbA,
    #[serde(rename = "pickerGroup.border")]
    pub picker_group_border: SrgbA,
    #[serde(rename = "pickerGroup.foreground")]
    pub picker_group_foreground: SrgbA,
    #[serde(rename = "quickInput.background")]
    pub quick_input_background: SrgbA,
    #[serde(rename = "quickInput.foreground")]
    pub quick_input_foreground: SrgbA,
    #[serde(rename = "statusBar.foreground")]
    pub status_bar_foreground: SrgbA,
    #[serde(rename = "statusBar.background")]
    pub status_bar_background: SrgbA,
    #[serde(rename = "statusBar.border")]
    pub status_bar_border: SrgbA,
    #[serde(rename = "statusBar.noFolderBackground")]
    pub status_bar_no_folder_background: SrgbA,
    #[serde(rename = "statusBar.debuggingBackground")]
    pub status_bar_debugging_background: SrgbA,
    #[serde(rename = "statusBar.debuggingForeground")]
    pub status_bar_debugging_foreground: SrgbA,
    #[serde(rename = "statusBarItem.prominentBackground")]
    pub status_bar_item_prominent_background: SrgbA,
    #[serde(rename = "statusBarItem.remoteForeground")]
    pub status_bar_item_remote_foreground: SrgbA,
    #[serde(rename = "statusBarItem.remoteBackground")]
    pub status_bar_item_remote_background: SrgbA,
    #[serde(rename = "editorGroupHeader.tabsBackground")]
    pub editor_group_header_tabs_background: SrgbA,
    #[serde(rename = "editorGroupHeader.tabsBorder")]
    pub editor_group_header_tabs_border: SrgbA,
    #[serde(rename = "editorGroup.border")]
    pub editor_group_border: SrgbA,
    #[serde(rename = "tab.activeForeground")]
    pub tab_active_foreground: SrgbA,
    #[serde(rename = "tab.inactiveForeground")]
    pub tab_inactive_foreground: SrgbA,
    #[serde(rename = "tab.inactiveBackground")]
    pub tab_inactive_background: SrgbA,
    #[serde(rename = "tab.activeBackground")]
    pub tab_active_background: SrgbA,
    #[serde(rename = "tab.hoverBackground")]
    pub tab_hover_background: SrgbA,
    #[serde(rename = "tab.unfocusedHoverBackground")]
    pub tab_unfocused_hover_background: SrgbA,
    #[serde(rename = "tab.border")]
    pub tab_border: SrgbA,
    #[serde(rename = "tab.unfocusedActiveBorderTop")]
    pub tab_unfocused_active_border_top: SrgbA,
    #[serde(rename = "tab.activeBorder")]
    pub tab_active_border: SrgbA,
    #[serde(rename = "tab.unfocusedActiveBorder")]
    pub tab_unfocused_active_border: SrgbA,
    #[serde(rename = "tab.activeBorderTop")]
    pub tab_active_border_top: SrgbA,
    #[serde(rename = "breadcrumb.foreground")]
    pub breadcrumb_foreground: SrgbA,
    #[serde(rename = "breadcrumb.focusForeground")]
    pub breadcrumb_focus_foreground: SrgbA,
    #[serde(rename = "breadcrumb.activeSelectionForeground")]
    pub breadcrumb_active_selection_foreground: SrgbA,
    #[serde(rename = "breadcrumbPicker.background")]
    pub breadcrumb_picker_background: SrgbA,
    #[serde(rename = "editor.foreground")]
    pub editor_foreground: SrgbA,
    #[serde(rename = "editor.background")]
    pub editor_background: SrgbA,
    #[serde(rename = "editorWidget.background")]
    pub editor_widget_background: SrgbA,
    #[serde(rename = "editor.foldBackground")]
    pub editor_fold_background: SrgbA,
    #[serde(rename = "editor.lineHighlightBackground")]
    pub editor_line_highlight_background: SrgbA,
    #[serde(rename = "editorLineNumber.foreground")]
    pub editor_line_number_foreground: SrgbA,
    #[serde(rename = "editorLineNumber.activeForeground")]
    pub editor_line_number_active_foreground: SrgbA,
    #[serde(rename = "editorIndentGuide.background")]
    pub editor_indent_guide_background: SrgbA,
    #[serde(rename = "editorIndentGuide.activeBackground")]
    pub editor_indent_guide_active_background: SrgbA,
    #[serde(rename = "editorWhitespace.foreground")]
    pub editor_whitespace_foreground: SrgbA,
    #[serde(rename = "editorCursor.foreground")]
    pub editor_cursor_foreground: SrgbA,
    #[serde(rename = "editorError.foreground")]
    pub editor_error_foreground: SrgbA,
    #[serde(rename = "editorWarning.foreground")]
    pub editor_warning_foreground: SrgbA,
    #[serde(rename = "editor.findMatchBackground")]
    pub editor_find_match_background: SrgbA,
    #[serde(rename = "editor.findMatchHighlightBackground")]
    pub editor_find_match_highlight_background: SrgbA,
    #[serde(rename = "editor.linkedEditingBackground")]
    pub editor_linked_editing_background: SrgbA,
    #[serde(rename = "editor.inactiveSelectionBackground")]
    pub editor_inactive_selection_background: SrgbA,
    #[serde(rename = "editor.selectionBackground")]
    pub editor_selection_background: SrgbA,
    #[serde(rename = "editor.selectionHighlightBackground")]
    pub editor_selection_highlight_background: SrgbA,
    #[serde(rename = "editor.selectionHighlightBorder")]
    pub editor_selection_highlight_border: SrgbA,
    #[serde(rename = "editor.wordHighlightBackground")]
    pub editor_word_highlight_background: SrgbA,
    #[serde(rename = "editor.wordHighlightStrongBackground")]
    pub editor_word_highlight_strong_background: SrgbA,
    #[serde(rename = "editor.wordHighlightBorder")]
    pub editor_word_highlight_border: SrgbA,
    #[serde(rename = "editor.wordHighlightStrongBorder")]
    pub editor_word_highlight_strong_border: SrgbA,
    #[serde(rename = "editorBracketMatch.background")]
    pub editor_bracket_match_background: SrgbA,
    #[serde(rename = "editorBracketMatch.border")]
    pub editor_bracket_match_border: SrgbA,
    #[serde(rename = "editorGutter.modifiedBackground")]
    pub editor_gutter_modified_background: SrgbA,
    #[serde(rename = "editorGutter.addedBackground")]
    pub editor_gutter_added_background: SrgbA,
    #[serde(rename = "editorGutter.deletedBackground")]
    pub editor_gutter_deleted_background: SrgbA,
    // #[serde(rename = "diffEditor.insertedTextBackground")]
    // pub diff_editor_inserted_text_background: WrapSrgb,
    // #[serde(rename = "diffEditor.removedTextBackground")]
    // pub diff_editor_removed_text_background: WrapSrgb,
    #[serde(rename = "scrollbar.shadow")]
    pub scrollbar_shadow: SrgbA,
    #[serde(rename = "scrollbarSlider.background")]
    pub scrollbar_slider_background: SrgbA,
    #[serde(rename = "scrollbarSlider.hoverBackground")]
    pub scrollbar_slider_hover_background: SrgbA,
    #[serde(rename = "scrollbarSlider.activeBackground")]
    pub scrollbar_slider_active_background: SrgbA,
    #[serde(rename = "editorOverviewRuler.border")]
    pub editor_overview_ruler_border: SrgbA,
    #[serde(rename = "panel.background")]
    pub panel_background: SrgbA,
    #[serde(rename = "panel.border")]
    pub panel_border: SrgbA,
    #[serde(rename = "panelTitle.activeBorder")]
    pub panel_title_active_border: SrgbA,
    #[serde(rename = "panelTitle.activeForeground")]
    pub panel_title_active_foreground: SrgbA,
    #[serde(rename = "panelTitle.inactiveForeground")]
    pub panel_title_inactive_foreground: SrgbA,
    #[serde(rename = "panelInput.border")]
    pub panel_input_border: SrgbA,
    #[serde(rename = "terminal.foreground")]
    pub terminal_foreground: SrgbA,
    #[serde(rename = "terminal.tab.activeBorder")]
    pub terminal_tab_active_border: SrgbA,
    #[serde(rename = "terminalCursor.background")]
    pub terminal_cursor_background: SrgbA,
    #[serde(rename = "terminalCursor.foreground")]
    pub terminal_cursor_foreground: SrgbA,
    #[serde(rename = "terminal.ansiBrightWhite")]
    pub terminal_ansi_bright_white: SrgbA,
    #[serde(rename = "terminal.ansiWhite")]
    pub terminal_ansi_white: SrgbA,
    #[serde(rename = "terminal.ansiBrightBlack")]
    pub terminal_ansi_bright_black: SrgbA,
    #[serde(rename = "terminal.ansiBlack")]
    pub terminal_ansi_black: SrgbA,
    #[serde(rename = "terminal.ansiBlue")]
    pub terminal_ansi_blue: SrgbA,
    #[serde(rename = "terminal.ansiBrightBlue")]
    pub terminal_ansi_bright_blue: SrgbA,
    #[serde(rename = "terminal.ansiGreen")]
    pub terminal_ansi_green: SrgbA,
    #[serde(rename = "terminal.ansiBrightGreen")]
    pub terminal_ansi_bright_green: SrgbA,
    #[serde(rename = "terminal.ansiCyan")]
    pub terminal_ansi_cyan: SrgbA,
    #[serde(rename = "terminal.ansiBrightCyan")]
    pub terminal_ansi_bright_cyan: SrgbA,
    #[serde(rename = "terminal.ansiRed")]
    pub terminal_ansi_red: SrgbA,
    #[serde(rename = "terminal.ansiBrightRed")]
    pub terminal_ansi_bright_red: SrgbA,
    #[serde(rename = "terminal.ansiMagenta")]
    pub terminal_ansi_magenta: SrgbA,
    #[serde(rename = "terminal.ansiBrightMagenta")]
    pub terminal_ansi_bright_magenta: SrgbA,
    #[serde(rename = "terminal.ansiYellow")]
    pub terminal_ansi_yellow: SrgbA,
    #[serde(rename = "terminal.ansiBrightYellow")]
    pub terminal_ansi_bright_yellow: SrgbA,
    #[serde(rename = "editorBracketHighlight.foreground1")]
    pub editor_bracket_highlight_foreground1: SrgbA,
    #[serde(rename = "editorBracketHighlight.foreground2")]
    pub editor_bracket_highlight_foreground2: SrgbA,
    #[serde(rename = "editorBracketHighlight.foreground3")]
    pub editor_bracket_highlight_foreground3: SrgbA,
    #[serde(rename = "editorBracketHighlight.foreground4")]
    pub editor_bracket_highlight_foreground4: SrgbA,
    #[serde(rename = "editorBracketHighlight.foreground5")]
    pub editor_bracket_highlight_foreground5: SrgbA,
    #[serde(rename = "editorBracketHighlight.foreground6")]
    pub editor_bracket_highlight_foreground6: SrgbA,
    #[serde(rename = "debugToolBar.background")]
    pub debug_tool_bar_background: SrgbA,
    #[serde(rename = "editor.stackFrameHighlightBackground")]
    pub editor_stack_frame_highlight_background: SrgbA,
    #[serde(rename = "editor.focusedStackFrameHighlightBackground")]
    pub editor_focused_stack_frame_highlight_background: SrgbA,
    #[serde(rename = "peekViewEditor.matchHighlightBackground")]
    pub peek_view_editor_match_highlight_background: SrgbA,
    #[serde(rename = "peekViewResult.matchHighlightBackground")]
    pub peek_view_result_match_highlight_background: SrgbA,
    #[serde(rename = "peekViewEditor.background")]
    pub peek_view_editor_background: SrgbA,
    #[serde(rename = "peekViewResult.background")]
    pub peek_view_result_background: SrgbA,
    #[serde(rename = "settings.headerForeground")]
    pub settings_header_foreground: SrgbA,
    #[serde(rename = "settings.modifiedItemIndicator")]
    pub settings_modified_item_indicator: SrgbA,
    #[serde(rename = "welcomePage.buttonBackground")]
    pub welcome_page_button_background: SrgbA,
    #[serde(rename = "welcomePage.buttonHoverBackground")]
    pub welcome_page_button_hover_background: SrgbA,
}

impl WorkbenchColorCustomizations {
    pub fn new(palette: &WrapFullPalette, no_saturation_fg: bool) -> Self {
        let fg = if no_saturation_fg {
            &palette.fg
        } else {
            &palette.color_table[Color::Gray]
        };
        Self {
            focus_border: palette.color_table[Color::Blue][2],
            foreground: fg[3],
            description_foreground: palette.color_table[Color::Gray][2],
            error_foreground: palette.color_table[Color::Red][3],

            text_link_foreground: palette.color_table[Color::Blue][3],
            text_link_active_foreground: palette.color_table[Color::Blue][3],
            text_block_quote_background: palette.color_table[Color::Bg][0],
            text_block_quote_border: palette.color_table[Color::Gray][1],
            text_code_block_background: palette.color_table[Color::Bg][0],
            text_preformat_foreground: palette.color_table[Color::Gray][3],
            text_separator_foreground: palette.color_table[Color::Gray][1],

            button_background: palette.color_table[Color::Bg][0],
            button_foreground: fg[4],
            button_hover_background: palette.color_table[Color::Bg][1],

            button_secondary_background: palette.color_table[Color::Bg][1],
            button_secondary_foreground: fg[3],
            button_secondary_hover_background: palette.color_table[Color::Bg][2],

            checkbox_background: palette.color_table[Color::Bg][1],
            checkbox_border: palette.color_table[Color::Gray][3],

            dropdown_background: palette.color_table[Color::Bg][1],
            dropdown_border: palette.color_table[Color::Gray][3],
            dropdown_foreground: fg[3],
            dropdown_list_background: palette.color_table[Color::Bg][0],

            input_background: palette.color_table[Color::Bg][1],
            input_border: palette.color_table[Color::Gray][3],
            input_foreground: fg[3],
            input_placeholder_foreground: fg[2],

            badge_foreground: fg[3],
            badge_background: palette.color_table[Color::Bg][1],

            progress_bar_background: palette.color_table[Color::Blue][2],

            title_bar_active_foreground: fg[3],
            title_bar_active_background: palette.color_table[Color::Bg][2],
            title_bar_inactive_foreground: fg[2],
            title_bar_inactive_background: palette.color_table[Color::Bg][1],
            title_bar_border: palette.color_table[Color::Gray][3],

            activity_bar_foreground: fg[3],
            activity_bar_inactive_foreground: palette.color_table[Color::Gray][2],
            activity_bar_background: palette.color_table[Color::Bg][2],
            activity_bar_badge_foreground: fg[3],
            activity_bar_badge_background: palette.color_table[Color::Bg][2],
            activity_bar_active_border: palette.color_table[Color::Gray][2],
            activity_bar_border: palette.color_table[Color::Gray][3],

            side_bar_foreground: fg[3],
            side_bar_background: palette.color_table[Color::Bg][3],
            side_bar_border: palette.color_table[Color::Gray][0],
            side_bar_title_foreground: fg[3],
            side_bar_section_header_foreground: fg[3],
            side_bar_section_header_background: palette.color_table[Color::Bg][3],
            side_bar_section_header_border: palette.color_table[Color::Gray][3],

            list_hover_foreground: fg[3],
            list_inactive_selection_foreground: fg[3],
            list_active_selection_foreground: fg[3],
            list_hover_background: palette.color_table[Color::Bg][3],
            list_inactive_selection_background: palette.color_table[Color::Bg][2],
            list_active_selection_background: palette.color_table[Color::Bg][1],
            list_inactive_focus_background: palette.color_table[Color::Bg][0],
            list_focus_background: palette.color_table[Color::Bg][4],

            tree_indent_guides_stroke: palette.color_table[Color::Gray][1],

            notification_center_header_foreground: fg[2],
            notification_center_header_background: palette.color_table[Color::Bg][0],
            notifications_foreground: fg[2],
            notifications_background: palette.color_table[Color::Bg][2],
            notifications_border: palette.color_table[Color::Gray][3],
            notifications_error_icon_foreground: palette.color_table[Color::Red][0],
            notifications_warning_icon_foreground: palette.color_table[Color::Yellow][2],
            notifications_info_icon_foreground: palette.color_table[Color::Blue][3],

            picker_group_border: palette.color_table[Color::Gray][1],
            picker_group_foreground: fg[2],

            quick_input_background: palette.color_table[Color::Bg][0],
            quick_input_foreground: fg[2],

            status_bar_foreground: fg[2],
            status_bar_background: palette.color_table[Color::Bg][2],
            status_bar_border: palette.color_table[Color::Gray][2],
            status_bar_no_folder_background: palette.color_table[Color::Bg][0],
            status_bar_debugging_background: palette.color_table[Color::Bg][2],
            status_bar_debugging_foreground: palette.color_table[Color::Gray][2],
            status_bar_item_prominent_background: palette.color_table[Color::Gray][3],
            status_bar_item_remote_foreground: fg[3],
            status_bar_item_remote_background: palette.color_table[Color::Bg][0],

            editor_group_header_tabs_background: palette.color_table[Color::Bg][1],
            editor_group_header_tabs_border: palette.color_table[Color::Gray][2],
            editor_group_border: palette.color_table[Color::Gray][2],

            tab_active_foreground: fg[2],
            tab_inactive_foreground: fg[3],
            tab_inactive_background: palette.color_table[Color::Bg][2],
            tab_active_background: palette.color_table[Color::Bg][4],
            tab_hover_background: palette.color_table[Color::Bg][3],
            tab_unfocused_hover_background: palette.color_table[Color::Bg][4],
            tab_border: palette.color_table[Color::Gray][2],
            tab_unfocused_active_border_top: palette.color_table[Color::Gray][2],
            tab_active_border: palette.color_table[Color::Gray][0],
            tab_unfocused_active_border: palette.color_table[Color::Gray][0],
            tab_active_border_top: palette.color_table[Color::Gray][1],

            breadcrumb_foreground: fg[3],
            breadcrumb_focus_foreground: fg[2],
            breadcrumb_active_selection_foreground: palette.color_table[Color::Gray][4],
            breadcrumb_picker_background: palette.color_table[Color::Bg][2],

            editor_foreground: fg[2],
            editor_background: palette.color_table[Color::Bg][2],
            editor_widget_background: palette.color_table[Color::Bg][3],
            editor_fold_background: palette.color_table[Color::Bg][4].alpha(0.5),
            editor_line_highlight_background: palette.color_table[Color::Bg][4],
            editor_line_number_foreground: fg[1],
            editor_line_number_active_foreground: fg[2],
            editor_indent_guide_background: palette.color_table[Color::Bg][0],
            editor_indent_guide_active_background: palette.color_table[Color::Bg][1],
            editor_whitespace_foreground: fg[1],
            editor_cursor_foreground: fg[3],
            editor_error_foreground: palette.color_table[Color::Red][3],
            editor_warning_foreground: palette.color_table[Color::Yellow][3],

            editor_find_match_background: palette.color_table[Color::Bg][4],
            editor_find_match_highlight_background: palette.color_table[Color::Bg][0]
                .alpha(0.5),
            editor_linked_editing_background: palette.color_table[Color::Bg][4],
            editor_inactive_selection_background: palette.color_table[Color::Bg][4]
                .alpha(0.5),
            editor_selection_background: palette.color_table[Color::Bg][4],
            editor_selection_highlight_background: palette.color_table[Color::Bg][4]
                .alpha(0.5),
            editor_selection_highlight_border: palette.color_table[Color::Gray][0],
            editor_word_highlight_background: palette.color_table[Color::Bg][4].alpha(0.5),
            editor_word_highlight_strong_background: palette.color_table[Color::Bg][4]
                .alpha(0.5),
            editor_word_highlight_border: palette.color_table[Color::Gray][0],
            editor_word_highlight_strong_border: palette.color_table[Color::Gray][0],
            editor_bracket_match_background: palette.color_table[Color::Bg][4],
            editor_bracket_match_border: palette.color_table[Color::Gray][0],

            editor_gutter_modified_background: palette.color_table[Color::Blue][2],
            editor_gutter_added_background: palette.color_table[Color::Green][1],
            editor_gutter_deleted_background: palette.color_table[Color::Red][2],

            // diff_editor_inserted_text_background: generate_color(palette.base_color_table[Color::Bg][0].into(), 270.0)
            //     .into(),
            // diff_editor_removed_text_background: generate_color(palette.base_color_table[Color::Bg][0].into(), 90.0).into(),
            scrollbar_shadow: palette.color_table[Color::Bg][4],
            scrollbar_slider_background: palette.color_table[Color::Bg][2],
            scrollbar_slider_hover_background: palette.color_table[Color::Bg][1],
            scrollbar_slider_active_background: palette.color_table[Color::Bg][0],
            editor_overview_ruler_border: palette.color_table[Color::Gray][2],

            panel_background: palette.color_table[Color::Bg][2],
            panel_border: palette.color_table[Color::Gray][2],
            panel_title_active_border: palette.color_table[Color::Gray][0],
            panel_title_active_foreground: palette.color_table[Color::Gray][0],
            panel_title_inactive_foreground: palette.color_table[Color::Gray][4],
            panel_input_border: palette.color_table[Color::Gray][0],

            terminal_foreground: fg[3],
            terminal_tab_active_border: palette.color_table[Color::Gray][4],
            terminal_cursor_background: palette.color_table[Color::Bg][2],
            terminal_cursor_foreground: palette.color_table[Color::Gray][2],

            terminal_ansi_bright_white: fg[4],
            terminal_ansi_white: fg[4],
            terminal_ansi_bright_black: fg[0],
            terminal_ansi_black: fg[0],
            terminal_ansi_blue: palette.color_table[Color::Blue][2],
            terminal_ansi_bright_blue: palette.color_table[Color::Blue][2],
            terminal_ansi_green: palette.color_table[Color::Green][2],
            terminal_ansi_bright_green: palette.color_table[Color::Green][2],
            terminal_ansi_cyan: palette.color_table[Color::Blue][2],
            terminal_ansi_bright_cyan: palette.color_table[Color::Blue][2],
            terminal_ansi_red: palette.color_table[Color::Red][2],
            terminal_ansi_bright_red: palette.color_table[Color::Red][2],
            terminal_ansi_magenta: palette.color_table[Color::Purple][2],
            terminal_ansi_bright_magenta: palette.color_table[Color::Purple][2],
            terminal_ansi_yellow: palette.color_table[Color::Yellow][2],
            terminal_ansi_bright_yellow: palette.color_table[Color::Yellow][2],

            editor_bracket_highlight_foreground1: palette.color_table[Color::Blue][3],
            editor_bracket_highlight_foreground2: palette.color_table[Color::Orange][3],
            editor_bracket_highlight_foreground3: palette.color_table[Color::Purple][3],
            editor_bracket_highlight_foreground4: palette.color_table[Color::Blue][3],
            editor_bracket_highlight_foreground5: palette.color_table[Color::Orange][3],
            editor_bracket_highlight_foreground6: palette.color_table[Color::Purple][3],

            debug_tool_bar_background: palette.color_table[Color::Bg][2],
            editor_stack_frame_highlight_background: palette.color_table[Color::Bg][3],
            editor_focused_stack_frame_highlight_background: palette.color_table[Color::Bg]
                [3],

            peek_view_editor_match_highlight_background: palette.color_table[Color::Bg][3],
            peek_view_result_match_highlight_background: palette.color_table[Color::Bg][4],
            peek_view_editor_background: palette.color_table[Color::Bg][0],
            peek_view_result_background: palette.color_table[Color::Bg][1],

            settings_header_foreground: fg[2],
            settings_modified_item_indicator: palette.color_table[Color::Blue][2],
            welcome_page_button_background: palette.color_table[Color::Bg][0],
            welcome_page_button_hover_background: palette.color_table[Color::Bg][1],
        }
    }
}
