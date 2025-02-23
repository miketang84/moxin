use crate::chat::chat_line::*;
use crate::chat::model_selector::ModelSelectorAction;
use crate::data::store::Store;
use crate::my_models::downloaded_files_table::DownloadedFileAction;
use makepad_widgets::*;
use moxin_protocol::data::{DownloadedFile, FileID};

use super::model_selector::ModelSelectorWidgetExt;

live_design! {
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;

    import crate::shared::styles::*;
    import crate::shared::widgets::*;
    import makepad_draw::shader::std::*;

    import crate::chat::model_selector::ModelSelector;
    import crate::chat::chat_line::ChatLine;

    ICON_PROMPT = dep("crate://self/resources/icons/prompt.svg")
    ICON_STOP = dep("crate://self/resources/icons/stop.svg")
    ICON_JUMP_TO_BOTTOM = dep("crate://self/resources/icons/jump_to_bottom.svg")

    ChatAgentAvatar = <RoundedView> {
        width: 20,
        height: 20,

        show_bg: true,
        draw_bg: {
            color: #444D9A
        }

        align: {x: 0.5, y: 0.5},

        avatar_label = <Label> {
            width: Fit,
            height: Fit,
            draw_text:{
                text_style: <BOLD_FONT>{font_size: 10},
                color: #fff,
            }
            text: "P"
        }
    }

    UserChatLine = <ChatLine> {
        avatar_section = {
            <Image> {
                source: dep("crate://self/resources/images/chat_user_icon.png"),
                width: 20,
                height: 20,
            }
        }
    }

    ModelChatLine = <ChatLine> {
        avatar_section = {
            <ChatAgentAvatar> {}
        }
    }

    JumpToButtom = <View> {
        width: Fill,
        height: Fill,
        align: {x: 0.5, y: 1.0},

        jump_to_bottom = <CircleView> {
            width: 34,
            height: 34,
            align: {x: 0.5, y: 0.5},
            margin: {bottom: 10},

            cursor: Hand,

            show_bg: true,

            draw_bg: {
                radius: 14.0,
                color: #fff,
                border_width: 1.0,
                border_color: #EAECF0,
            }

            <Icon> {
                padding: 0,
                // These margins are used to center the icon inside the circle
                // Not sure why the icon is not centered by default
                margin: { top: 6, right: 4 },
                draw_icon: {
                    svg_file: (ICON_JUMP_TO_BOTTOM),
                    fn get_color(self) -> vec4 {
                        return #1C1B1F;
                    }
                }
                icon_walk: {width: 12, height: 12}
            }
        }
    }

    ChatPromptInput = <RoundedView> {
        width: Fill,
        height: Fit,

        show_bg: true,
        draw_bg: {
            color: #fff
        }

        padding: {top: 6, bottom: 6, left: 4, right: 10}

        spacing: 4,
        align: {x: 0.0, y: 1.0},

        draw_bg: {
            radius: 2.0,
            border_color: #D0D5DD,
            border_width: 1.0,
        }

        prompt = <MoxinTextInput> {
            width: Fill,
            height: Fit,

            empty_message: "Enter a message"
            draw_bg: {
                color: #fff
            }
            draw_text: {
                text_style:<REGULAR_FONT>{font_size: 10},

                instance prompt_enabled: 0.0
                fn get_color(self) -> vec4 {
                    return mix(
                        #D0D5DD,
                        #000,
                        self.prompt_enabled
                    )
                }
            }
        }

        prompt_icon = <RoundedView> {
            width: 28,
            height: 28,
            show_bg: true,
            draw_bg: {
                color: #D0D5DD
            }

            cursor: Hand,

            padding: {right: 4},
            margin: {bottom: 2},
            align: {x: 0.5, y: 0.5},

            icon_send = <View> {
                width: Fit,
                height: Fit,
                <Icon> {
                    draw_icon: {
                        svg_file: (ICON_PROMPT),
                        fn get_color(self) -> vec4 {
                            return #fff;
                        }
                    }
                    icon_walk: {width: 12, height: 12}
                }
            }
            icon_stop = <View> {
                width: Fit,
                height: Fit,
                visible: false,

                <Icon> {
                    draw_icon: {
                        svg_file: (ICON_STOP),
                        fn get_color(self) -> vec4 {
                            return #fff;
                        }
                    }
                    icon_walk: {width: 12, height: 12}
                }
            }
        }
    }

    ChatPanel = {{ChatPanel}} {
        width: Fill,
        height: Fill,
        margin: {top: 0, left: 20, right: 20, bottom: 20},

        flow: Overlay,

        no_downloaded_model = <View> {
            width: Fill,
            height: Fill,

            flow: Down,
            align: {x: 0.5, y: 0.5},

            <View> {
                width: Fill,
                height: Fill,
                flow: Down,
                spacing: 30,
                align: {x: 0.5, y: 0.5},

                <Label> {
                    draw_text: {
                        text_style: <REGULAR_FONT>{font_size: 12},
                        color: #667085
                    }
                    text: "You haven’t downloaded any models yet."
                }
                go_to_discover_button = <RoundedView> {
                    width: Fit,
                    height: Fit,
                    cursor: Arrow,

                    draw_bg: { color: #fff, border_color: #D0D5DD, border_width: 1}

                    button_label = <Label> {
                        margin: {top: 14, right: 12, bottom: 14, left: 12}
                        text: "Go To Discover"
                        draw_text: {
                            text_style: <BOLD_FONT>{font_size: 12},
                            fn get_color(self) -> vec4 {
                                return #087443;
                            }
                        }
                    }
                }
            }

            <View> {
                width: Fill, height: Fit
                flow: Down,
                align: {x: 0.5, y: 0.5},
                no_downloaded_model_prompt_input = <ChatPromptInput> {}
            }

        }

        no_model = <View> {
            width: Fill,
            height: Fill,

            flow: Down,
            align: {x: 0.5, y: 0.5},

            <View> {
                width: Fill,
                height: Fill,
                flow: Down,
                spacing: 30,
                align: {x: 0.5, y: 0.5},

                <Icon> {
                    draw_icon: {
                        svg_file: dep("crate://self/resources/icons/chat.svg"),
                        color: #D0D5DD
                    }
                    icon_walk: {width: 128, height: 128}
                }

                <Label> {
                    draw_text: {
                        text_style: <REGULAR_FONT>{font_size: 14},
                        color: #667085
                    }
                    text: "Start chatting by choosing a model from above"
                }
            }

            <View> {
                width: Fill, height: Fit
                flow: Down,
                align: {x: 0.5, y: 0.5},
                no_model_prompt_input = <ChatPromptInput> {}
            }

        }

        empty_conversation = <View> {
            visible: false,

            width: Fill,
            height: Fill,

            flow: Down,
            spacing: 30,
            align: {x: 0.5, y: 0.5},

            <ChatAgentAvatar> {}
            <Label> {
                draw_text: {
                    text_style: <REGULAR_FONT>{font_size: 14},
                    color: #101828
                }
                text: "How can I help you?"
            }
        }

        main = <View> {
            visible: false

            width: Fill,
            height: Fill,

            margin: { top: 86 }
            spacing: 4,
            flow: Down,

            <View> {
                width: Fill,
                height: Fill,

                flow: Overlay
                chat = <PortalList> {
                    width: Fill,
                    height: Fill,

                    drag_scrolling: false,
                    auto_tail: true,

                    UserChatLine = <UserChatLine> {}
                    ModelChatLine = <ModelChatLine> {}
                }

                <JumpToButtom> {}
            }

            main_prompt_input = <ChatPromptInput> {}
        }

        model_selector = <ModelSelector> {}
    }
}

#[derive(PartialEq)]
enum ChatPanelState {
    Unload {
        downloaded_model_empty: bool,
    },
    Idle,
    Streaming {
        auto_scroll_pending: bool,
        auto_scroll_cancellable: bool,
    },
}

impl Default for ChatPanelState {
    fn default() -> ChatPanelState {
        ChatPanelState::Unload {
            downloaded_model_empty: true,
        }
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct ChatPanel {
    #[deref]
    view: View,

    #[rust]
    state: ChatPanelState,
}

impl Widget for ChatPanel {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
        self.widget_match_event(cx, event, scope);

        if let Event::Signal = event {
            let store = scope.data.get_mut::<Store>().unwrap();

            match self.state {
                ChatPanelState::Streaming {
                    auto_scroll_pending,
                    auto_scroll_cancellable: _,
                } => {
                    self.state = ChatPanelState::Streaming {
                        auto_scroll_pending,
                        auto_scroll_cancellable: true,
                    };

                    let still_streaming = store.current_chat.as_ref().unwrap().is_streaming;
                    if still_streaming {
                        if auto_scroll_pending {
                            self.scroll_messages_to_bottom(cx);
                        }
                    } else {
                        // Scroll to the bottom when streaming is done
                        self.scroll_messages_to_bottom(cx);
                        self.state = ChatPanelState::Idle;
                    }

                    self.update_prompt_input(cx);

                    // Redraw because we expect to see new or updated chat entries
                    self.redraw(cx);
                }
                ChatPanelState::Unload {
                    downloaded_model_empty: _,
                } => self.unload_model(cx, store),
                _ => {}
            }
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        let store = scope.data.get_mut::<Store>().unwrap();

        let (chat_history, model_filename, initial_letter) =
            store
                .current_chat
                .as_ref()
                .map_or((vec![], "".to_string(), "".to_string()), |chat| {
                    let model_filename = chat.model_filename.clone();
                    let initial_letter = model_filename
                        .chars()
                        .next()
                        .unwrap_or_default()
                        .to_uppercase()
                        .to_string();
                    (chat.messages.clone(), model_filename, initial_letter)
                });

        let chats_count = chat_history.len();

        if chats_count == 0 {
            self.view(id!(empty_conversation))
                .label(id!(avatar_label))
                .set_text(initial_letter.as_str());
        }

        while let Some(view_item) = self.view.draw_walk(cx, scope, walk).step() {
            if let Some(mut list) = view_item.as_portal_list().borrow_mut() {
                list.set_item_range(cx, 0, chats_count);
                while let Some(item_id) = list.next_visible_item(cx) {
                    if item_id < chats_count {
                        let chat_line_data = &chat_history[item_id];

                        let item;
                        let mut chat_line_item;
                        if chat_line_data.is_assistant() {
                            item = list.item(cx, item_id, live_id!(ModelChatLine)).unwrap();
                            chat_line_item = item.as_chat_line();
                            chat_line_item.set_role(&model_filename);
                            chat_line_item.set_regenerate_enabled(false);
                            chat_line_item.set_avatar_text(&initial_letter);
                        } else {
                            item = list.item(cx, item_id, live_id!(UserChatLine)).unwrap();
                            chat_line_item = item.as_chat_line();
                            chat_line_item.set_role("You");
                            chat_line_item.set_regenerate_enabled(true);
                        };

                        chat_line_item.set_message_text(cx, &chat_line_data.content);
                        chat_line_item.set_message_id(chat_line_data.id);

                        // Disable actions for the last chat line when model is streaming
                        if matches!(self.state, ChatPanelState::Streaming { .. })
                            && item_id == chats_count - 1
                        {
                            chat_line_item.set_actions_enabled(cx, false);
                        } else {
                            chat_line_item.set_actions_enabled(cx, true);
                        }

                        item.draw_all(cx, &mut Scope::empty());
                    }
                }
            }
        }

        DrawStep::done()
    }
}

impl WidgetMatchEvent for ChatPanel {
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions, scope: &mut Scope) {
        let widget_uid = self.widget_uid();

        for action in actions {
            match action.as_widget_action().cast() {
                ModelSelectorAction::Selected(downloaded_file) => {
                    let store = scope.data.get_mut::<Store>().unwrap();
                    self.load_model(store, downloaded_file);
                }
                _ => {}
            }

            match action.as_widget_action().cast() {
                DownloadedFileAction::StartChat(file_id) => {
                    let store = scope.data.get_mut::<Store>().unwrap();
                    let downloaded_file = store
                        .downloaded_files
                        .iter()
                        .find(|file| file.file.id == file_id)
                        .expect("Attempted to start chat with a no longer existing file")
                        .clone();
                    self.load_model(store, downloaded_file);
                }
                _ => {}
            }

            match action.as_widget_action().cast() {
                ChatLineAction::Delete(id) => {
                    let store = scope.data.get_mut::<Store>().unwrap();
                    store.delete_chat_message(id);
                    self.redraw(cx);
                }
                ChatLineAction::Edit(id, updated, regenerate) => {
                    let store = scope.data.get_mut::<Store>().unwrap();
                    store.edit_chat_message(id, updated, regenerate);
                    self.redraw(cx);
                }
                _ => {}
            }

            match action.as_widget_action().cast() {
                ChatPanelAction::UnloadIfActive(file_id) => {
                    let store = scope.data.get_mut::<Store>().unwrap();
                    if store
                        .current_chat
                        .as_ref()
                        .map_or(false, |chat| chat.file_id == file_id)
                    {
                        self.unload_model(cx, store);
                        store.current_chat = None;
                        store.eject_model().expect("Failed to eject model");
                    }
                }
                _ => {}
            }
        }

        self.jump_to_bottom_actions(cx, actions, scope);

        match self.state {
            ChatPanelState::Idle => {
                self.handle_prompt_input_actions(cx, actions, scope);
            }
            ChatPanelState::Streaming {
                auto_scroll_pending: _,
                auto_scroll_cancellable,
            } => {
                let list = self.portal_list(id!(chat));
                if auto_scroll_cancellable && list.scrolled(actions) {
                    // Cancel auto-scrolling if the user scrolls up
                    self.state = ChatPanelState::Streaming {
                        auto_scroll_pending: false,
                        auto_scroll_cancellable,
                    };
                }

                if let Some(fe) = self.view(id!(prompt_icon)).finger_up(&actions) {
                    if fe.was_tap() {
                        let store = scope.data.get_mut::<Store>().unwrap();
                        store.cancel_chat_streaming();
                    }
                }
            }
            _ => {}
        }

        if let Some(fe) = self
            .view(id!(no_downloaded_model.go_to_discover_button))
            .finger_up(actions)
        {
            if fe.was_tap() {
                cx.widget_action(widget_uid, &scope.path, ChatPanelAction::NavigateToDiscover);
            }
        }
    }
}

impl ChatPanel {
    fn jump_to_bottom_actions(&mut self, cx: &mut Cx, actions: &Actions, scope: &mut Scope) {
        if let Some(fe) = self.view(id!(jump_to_bottom)).finger_up(actions) {
            if fe.was_tap() {
                self.scroll_messages_to_bottom(cx);
                self.redraw(cx);
            }
        }

        let jump_to_bottom = self.view(id!(jump_to_bottom));
        match self.state {
            ChatPanelState::Streaming {
                auto_scroll_pending: true,
                ..
            } => {
                // We avoid to show this button when the list is auto-scrolling upon
                // receiving a new message. Otherwise, the button flicks.
                jump_to_bottom.set_visible(false);
            }
            ChatPanelState::Idle | ChatPanelState::Streaming { .. } => {
                let store = scope.data.get_mut::<Store>().unwrap();
                let has_messages = store
                    .current_chat
                    .as_ref()
                    .map_or(false, |chat| chat.messages.len() > 0);

                let list = self.portal_list(id!(chat));
                jump_to_bottom.set_visible(has_messages && list.further_items_bellow_exist());
            }
            ChatPanelState::Unload {
                downloaded_model_empty: _,
            } => {
                jump_to_bottom.set_visible(false);
            }
        }
    }

    fn update_prompt_input(&mut self, cx: &mut Cx) {
        match self.state {
            ChatPanelState::Idle => {
                self.enable_or_disable_prompt_input(cx);
                self.show_prompt_input_send_icon(cx);
            }
            ChatPanelState::Streaming {
                auto_scroll_pending: _,
                auto_scroll_cancellable: _,
            } => {
                let prompt_input = self.text_input(id!(main_prompt_input.prompt));
                prompt_input.apply_over(
                    cx,
                    live! {
                        draw_text: { prompt_enabled: 0.0 }
                    },
                );
                self.show_prompt_input_stop_icon(cx);
            }
            ChatPanelState::Unload {
                downloaded_model_empty: _,
            } => {}
        }
    }

    fn enable_or_disable_prompt_input(&mut self, cx: &mut Cx) {
        let prompt_input = self.text_input(id!(main_prompt_input.prompt));
        let enable = if prompt_input.text().len() > 0 {
            1.0
        } else {
            0.0
        };

        prompt_input.apply_over(
            cx,
            live! {
                draw_text: { prompt_enabled: (enable) }
            },
        );
    }

    fn show_prompt_input_send_icon(&mut self, cx: &mut Cx) {
        self.view(id!(prompt_icon)).apply_over(
            cx,
            live! {
                icon_send = { visible: true }
                icon_stop = { visible: false }
            },
        );
        let prompt_input = self.text_input(id!(prompt));
        if prompt_input.text().len() > 0 {
            self.enable_prompt_input_icon(cx);
        } else {
            self.disable_prompt_input_icon(cx);
        }
    }

    fn show_prompt_input_stop_icon(&mut self, cx: &mut Cx) {
        self.view(id!(prompt_icon)).apply_over(
            cx,
            live! {
                icon_send = { visible: false }
                icon_stop = { visible: true }
            },
        );
        self.enable_prompt_input_icon(cx);
    }

    fn enable_prompt_input_icon(&mut self, cx: &mut Cx) {
        let enabled_color = vec3(0.0, 0.0, 0.0);
        self.view(id!(prompt_icon)).apply_over(
            cx,
            live! {
                draw_bg: {
                    color: (enabled_color)
                }
            },
        );
    }

    fn disable_prompt_input_icon(&mut self, cx: &mut Cx) {
        let disabled_color = vec3(0.816, 0.835, 0.867); // #D0D5DD
        self.view(id!(prompt_icon)).apply_over(
            cx,
            live! {
                draw_bg: {
                    color: (disabled_color)
                }
            },
        );
    }

    fn scroll_messages_to_bottom(&mut self, cx: &mut Cx) {
        let mut list = self.portal_list(id!(chat));
        list.smooth_scroll_to_end(cx, 10, 80.0);
    }

    fn load_model(&mut self, store: &mut Store, downloaded_file: DownloadedFile) {
        self.state = ChatPanelState::Idle;
        self.view(id!(main)).set_visible(true);
        self.view(id!(empty_conversation)).set_visible(true);
        self.view(id!(no_model)).set_visible(false);
        self.view(id!(no_downloaded_model)).set_visible(false);

        store.load_model(&downloaded_file.file);
    }

    fn unload_model(&mut self, cx: &mut Cx, store: &mut Store) {
        let downloaded_model_empty = store.downloaded_files.is_empty();
        self.state = ChatPanelState::Unload {
            downloaded_model_empty,
        };

        self.view(id!(main)).set_visible(false);
        self.view(id!(empty_conversation)).set_visible(false);

        match self.state {
            ChatPanelState::Unload {
                downloaded_model_empty: true,
            } => {
                self.view(id!(no_downloaded_model)).set_visible(true);
                self.view(id!(no_model)).set_visible(false);
            }
            ChatPanelState::Unload {
                downloaded_model_empty: false,
            } => {
                self.view(id!(no_model)).set_visible(true);
                self.view(id!(no_downloaded_model)).set_visible(false)
            }
            _ => {}
        }

        self.model_selector(id!(model_selector)).deselect(cx);
        self.view.redraw(cx);
    }

    fn handle_prompt_input_actions(&mut self, cx: &mut Cx, actions: &Actions, scope: &mut Scope) {
        let prompt_input = self.text_input(id!(main_prompt_input.prompt));

        if let Some(_text) = prompt_input.changed(actions) {
            self.update_prompt_input(cx);
        }

        if let Some(fe) = self.view(id!(prompt_icon)).finger_up(&actions) {
            if fe.was_tap() {
                self.send_message(cx, scope, prompt_input.text());
            }
        }

        if let Some(prompt) = prompt_input.returned(actions) {
            self.send_message(cx, scope, prompt);
        }
    }

    fn send_message(&mut self, cx: &mut Cx, scope: &mut Scope, prompt: String) {
        if prompt.trim().is_empty() {
            return;
        }

        self.show_prompt_input_stop_icon(cx);
        let store = scope.data.get_mut::<Store>().unwrap();
        store.send_chat_message(prompt.clone());

        let prompt_input = self.text_input(id!(main_prompt_input.prompt));
        prompt_input.set_text_and_redraw(cx, "");
        prompt_input.set_cursor(0, 0);
        self.update_prompt_input(cx);

        self.view(id!(empty_conversation)).set_visible(false);

        // Scroll to the bottom when the message is sent
        self.scroll_messages_to_bottom(cx);

        self.state = ChatPanelState::Streaming {
            auto_scroll_pending: true,
            auto_scroll_cancellable: false,
        };
    }
}

#[derive(Clone, DefaultNone, Debug)]
pub enum ChatPanelAction {
    UnloadIfActive(FileID),
    NavigateToDiscover,
    None,
}
