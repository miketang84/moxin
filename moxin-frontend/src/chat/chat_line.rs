use makepad_widgets::markdown::MarkdownWidgetExt;
use crate::chat::chat_line_loading::ChatLineLoadingWidgetExt;
use makepad_widgets::*;

use makepad_markdown::parse_markdown;

live_design! {
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;

    import makepad_draw::shader::std::*;
    import crate::shared::styles::*;
    import crate::shared::widgets::*;
    import crate::shared::resource_imports::*;
    import crate::chat::chat_line_loading::ChatLineLoading;

    ICON_EDIT = dep("crate://self/resources/icons/edit.svg")
    ICON_DELETE = dep("crate://self/resources/icons/delete.svg")

    ChatLineEditButton = <RoundedView> {
        width: 56,
        height: 31,
        align: {x: 0.5, y: 0.5}
        spacing: 6,

        cursor: Hand,

        draw_bg: { color: #099250 }

        button_label = <Label> {
            draw_text: {
                text_style: <REGULAR_FONT>{font_size: 9},
                fn get_color(self) -> vec4 {
                    return #fff;
                }
            }
        }
    }

    SaveButton = <ChatLineEditButton> {
        button_label = {
            text: "Save"
        }
    }

    SaveAndRegerateButton = <ChatLineEditButton> {
        width: 130,
        button_label = {
            text: "Save & Regenerate"
        }
    }

    CancelButton = <ChatLineEditButton> {
        draw_bg: { border_color: #D0D5DD, border_width: 1.0, color: #fff }

        button_label = {
            draw_text: {
                text_style: <REGULAR_FONT>{font_size: 9},
                fn get_color(self) -> vec4 {
                    return #000;
                }
            }
            text: "Cancel"
        }
    }

    MESSAGE_TEXT_COLOR = #000
    TEXT_HEIGHT_FACTOR = 1.3
    LINE_SPACING = 8.0
    BLOCK_LINE_SPACING = 12.0

    MessageText = <Markdown>{
        padding: 0,
        line_spacing: (LINE_SPACING),
        paragraph_spacing: 20.0,
        width: Fill, height: Fit,
        font_size: 10.0,
        draw_normal: {
            color: (MESSAGE_TEXT_COLOR),
            text_style: { height_factor: (TEXT_HEIGHT_FACTOR), line_spacing: (LINE_SPACING) }
        }
        draw_italic: {
            color: (MESSAGE_TEXT_COLOR),
            text_style: { height_factor: (TEXT_HEIGHT_FACTOR), line_spacing: (LINE_SPACING) }
        }
        draw_bold: {
            color: (MESSAGE_TEXT_COLOR),
            text_style: { height_factor: (TEXT_HEIGHT_FACTOR), line_spacing: (LINE_SPACING) }
        }
        draw_bold_italic: {
            color: (MESSAGE_TEXT_COLOR),
            text_style: { height_factor: (TEXT_HEIGHT_FACTOR), line_spacing: (LINE_SPACING) }
        }
        draw_fixed: {
            color: (MESSAGE_TEXT_COLOR),
            text_style: { height_factor: (TEXT_HEIGHT_FACTOR), line_spacing: (LINE_SPACING) }
        }
        draw_block: {
            line_color: (MESSAGE_TEXT_COLOR)
            sep_color: (#EDEDED)
            quote_bg_color: (#EDEDED)
            quote_fg_color: (#969696)
            block_color: (#EDEDED)
            code_color: (#EDEDED)
        }
        list_item_layout: { line_spacing: 5.0, padding: {left: 10.0, right:10, top: 6.0, bottom: 0}, }
        list_item_walk:{margin:0, height:Fit, width:Fill}
        code_layout: { line_spacing: (BLOCK_LINE_SPACING), padding: {top: 10.0, bottom: 10.0}}
        quote_layout: { line_spacing: (BLOCK_LINE_SPACING), padding: {top: 10.0, bottom: 10.0}}
    }

    EditTextInput = <MoxinTextInput> {
        width: Fill,
        height: Fit,
        padding: 0,
        empty_message: ""

        draw_bg: {
            color: #fff
        }
        draw_text: {
            text_style:<REGULAR_FONT>{height_factor: (1.3*1.3), font_size: 10},
            word: Wrap,

            instance prompt_enabled: 0.0
            fn get_color(self) -> vec4 {
                return #000;
            }
        }
    }

    ChatLineBody = <View> {
        width: Fill,
        height: Fit,
        spacing: 5,
        flow: Down,

        <View> {
            height: 20,
            align: {x: 0.0, y: 0.5},

            role = <Label> {
                width: Fit,
                height: Fit,
                draw_text:{
                    text_style: <BOLD_FONT>{font_size: 10},
                    color: #000
                }
            }
        }

        <View> {
            width: Fill,
            height: Fit,
            flow: Down,
            padding: {top: 12, bottom: 12},
            align: {x: 0.5, y: 0.0},

            input_container = <View> {
                visible: false,
                width: Fill,
                height: Fit,
                input = <EditTextInput> {
                }
            }

            loading_container = <View> {
                width: Fill,
                height: Fit,
                loading = <ChatLineLoading> {}
            }

            markdown_message_container = <View> {
                width: Fill,
                height: Fit,
                markdown_message = <MessageText> {}
            }

            plain_text_message_container = <View> {
                width: Fill,
                height: Fit,
                plain_text_message = <Label> {
                    width: Fill,
                    height: Fit,
                    draw_text: {
                        text_style: <REGULAR_FONT>{height_factor: (1.3*1.3), font_size: 10},
                        color: #000
                    }
                }
            }

            edit_buttons = <View> {
                visible: false,
                width: Fit,
                height: Fit,
                margin: {top: 10},
                spacing: 6,
                save = <SaveButton> {}
                save_and_regenerate = <SaveAndRegerateButton> {}
                cancel = <CancelButton> {}
            }
        }
    }

    ChatLineActionButton = <Button> {
        width: 14
        height: 14
        draw_icon: {
            fn get_color(self) -> vec4 {
                return #BDBDBD;
            }
        }
        padding: 0,
        icon_walk: {width: 14, height: 14}
        draw_bg: {
            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                return sdf.result
            }
        }
        text: ""
    }

    ChatLine = {{ChatLine}} {
        padding: {top: 10, bottom: 3},
        width: Fill,
        height: Fit,

       // cursor: Default,

        avatar_section = <View> {
            width: Fit,
            height: Fit,
            margin: {left: 20, right: 20},
        }

        main_section = <View> {
            width: Fill,
            height: Fit,

            flow: Down,
            spacing: 8,

            body_section = <ChatLineBody> {}

            actions_section = <View> {
                width: Fill,
                height: 16,
                actions = <View> {
                    width: Fill,
                    height: Fit,
                    visible: false,
                    spacing: 6,

                    copy_button = <ChatLineActionButton> {
                        draw_icon: { svg_file: (ICON_COPY) }
                    }
                    edit_button = <ChatLineActionButton> {
                        draw_icon: { svg_file: (ICON_EDIT) }
                    }
                    delete_button = <ChatLineActionButton> {
                        draw_icon: { svg_file: (ICON_DELETE) }
                    }
                }
            }
        }

    }
}

#[derive(Clone, DefaultNone, Debug)]
pub enum ChatLineAction {
    Delete(usize),
    Edit(usize, String, bool),
    None,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub enum ChatLineState {
    #[default]
    Editable,
    NotEditable,
    OnEdit,
}

#[derive(Live, LiveHook, Widget)]
pub struct ChatLine {
    #[deref]
    view: View,

    #[rust]
    message_id: usize,

    #[rust]
    edition_state: ChatLineState,

    #[rust]
    hovered: bool,
}

impl Widget for ChatLine {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
        self.widget_match_event(cx, event, scope);

        // Current Makepad's processing of the hover events is not enough
        // in our case because it collapes the hover state of the
        // children widgets (specially, the text input widget). So, we rely
        // on this basic mouse over calculation to show the actions buttons.
        if matches!(self.edition_state, ChatLineState::Editable) {
            if let Event::MouseMove(e) = event {
                let hovered = self.view.area().rect(cx).contains(e.abs);
                if self.hovered != hovered {
                    self.hovered = hovered;
                    self.view(id!(actions_section.actions)).set_visible(hovered);
                    self.redraw(cx);
                }
            }
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}

impl WidgetMatchEvent for ChatLine {
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions, scope: &mut Scope) {
        match self.edition_state {
            ChatLineState::Editable => self.handle_editable_actions(cx, actions, scope),
            ChatLineState::OnEdit => self.handle_on_edit_actions(cx, actions, scope),
            ChatLineState::NotEditable => {}
        }
    }
}

impl ChatLine {
    pub fn set_edit_mode(&mut self, cx: &mut Cx, enabled: bool) {
        self.edition_state = if enabled {
            ChatLineState::OnEdit
        } else {
            ChatLineState::Editable
        };

        self.view(id!(actions_section.actions)).set_visible(false);
        self.view(id!(edit_buttons)).set_visible(enabled);
        self.view(id!(input_container)).set_visible(enabled);
        self.show_or_hide_message_label(cx, !enabled);

        self.redraw(cx);
    }

    pub fn show_or_hide_message_label(&mut self, cx: &mut Cx, show: bool) {
        let text = self.text_input(id!(input)).text();
        let to_markdown = parse_markdown(&text);
        let is_plain_text = to_markdown.nodes.len() <= 3;

        self.view(id!(plain_text_message_container))
            .set_visible(show && is_plain_text);
        self.view(id!(markdown_message_container))
            .set_visible(show && !is_plain_text);

        self.redraw(cx);
    }

    pub fn handle_editable_actions(&mut self, cx: &mut Cx, actions: &Actions, scope: &mut Scope) {
        if self.button(id!(delete_button)).clicked(&actions) {
            let widget_id = self.view.widget_uid();
            cx.widget_action(
                widget_id,
                &scope.path,
                ChatLineAction::Delete(self.message_id),
            );
        }

        if self.button(id!(edit_button)).clicked(&actions) {
            self.set_edit_mode(cx, true);
        }

        if self.button(id!(copy_button)).clicked(&actions) {
            let text_to_copy = self.text_input(id!(input)).text();
            cx.copy_to_clipboard(&text_to_copy);
        }
    }

    pub fn handle_on_edit_actions(&mut self, cx: &mut Cx, actions: &Actions, scope: &mut Scope) {
        if let Some(fe) = self.view(id!(save)).finger_up(&actions) {
            if fe.was_tap() {
                let updated_message = self.text_input(id!(input)).text();

                // Do not allow to have empty messages for now.
                // TODO We should disable Save button when the message is empty. 
                if !updated_message.trim().is_empty() {
                    let widget_id = self.view.widget_uid();
                    cx.widget_action(
                        widget_id,
                        &scope.path,
                        ChatLineAction::Edit(self.message_id, updated_message, false),
                    );
                }

                self.set_edit_mode(cx, false);
            }
        }

        if let Some(fe) = self.view(id!(save_and_regenerate)).finger_up(&actions) {
            if fe.was_tap() {
                let updated_message = self.text_input(id!(input)).text();

                // TODO We should disable Save and Regenerate button when the message is empty. 
                if !updated_message.trim().is_empty() {
                    let widget_id = self.view.widget_uid();
                    cx.widget_action(
                        widget_id,
                        &scope.path,
                        ChatLineAction::Edit(self.message_id, updated_message, true),
                    );
                }

                self.set_edit_mode(cx, false);
            }
        }

        if let Some(fe) = self.view(id!(cancel)).finger_up(&actions) {
            if fe.was_tap() {
                self.set_edit_mode(cx, false);
            }
        }
    }
}

impl ChatLineRef {
    pub fn set_role(&mut self, text: &str) {
        let Some(mut inner) = self.borrow_mut() else {
            return;
        };
        inner.label(id!(role)).set_text(text);
    }

    pub fn set_avatar_text(&mut self, text: &str) {
        let Some(mut inner) = self.borrow_mut() else {
            return;
        };
        inner.label(id!(avatar_label)).set_text(text);
    }

    pub fn set_message_text(&mut self, cx: &mut Cx, text: &str) {
        let Some(mut inner) = self.borrow_mut() else {
            return;
        };

        match inner.edition_state {
            ChatLineState::Editable | ChatLineState::NotEditable => {
                inner.text_input(id!(input)).set_text(text.trim());
                inner.label(id!(plain_text_message)).set_text(text.trim());
                inner.markdown(id!(markdown_message)).set_text(text.trim());

                // We know only AI assistant messages could be empty, so it is never
                // displayed in user's chat lines.
                let show_loading = text.trim().is_empty();
                inner.view(id!(loading_container)).set_visible(show_loading);

                let mut loading_widget = inner.chat_line_loading(id!(loading_container.loading));
                if show_loading {
                    loading_widget.animate(cx);
                } else {
                    loading_widget.stop_animation();
                }

                inner.show_or_hide_message_label(cx, true);
            }
            ChatLineState::OnEdit => {}
        }
    }

    pub fn set_message_id(&mut self, message_id: usize) {
        let Some(mut inner) = self.borrow_mut() else {
            return;
        };
        inner.message_id = message_id;
    }

    pub fn set_actions_enabled(&mut self, _cx: &mut Cx, enabled: bool) {
        let Some(mut inner) = self.borrow_mut() else {
            return;
        };

        if enabled {
            if inner.edition_state == ChatLineState::NotEditable {
                inner.edition_state = ChatLineState::Editable;
            }
        } else {
            inner.edition_state = ChatLineState::NotEditable;
            inner.view(id!(actions_section.actions)).set_visible(false);
        }
    }

    pub fn set_regenerate_enabled(&mut self, enabled: bool) {
        let Some(mut inner) = self.borrow_mut() else {
            return;
        };
        inner.view(id!(save_and_regenerate)).set_visible(enabled);
    }
}
