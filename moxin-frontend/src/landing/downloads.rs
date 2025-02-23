use crate::data::store::{DownloadInfoStatus, Store};
use makepad_widgets::*;

live_design! {
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;
    import makepad_draw::shader::std::*;

    import crate::shared::styles::*;
    import crate::shared::widgets::*;
    import crate::shared::icon::*;

    import crate::landing::download_item::DownloadItem;

    ICON_COLLAPSE = dep("crate://self/resources/icons/collapse.svg")

    CollapseButton = <View> {
        cursor: Hand
        width: Fit, height: Fit
        icon = <Icon> {
            draw_icon: {
                svg_file: (ICON_COLLAPSE)
                fn get_color(self) -> vec4 {
                    return #667085;
                }
            }
            icon_walk: {width: 18, height: Fit}
        }
    }

    Header = <View> {
        width: Fill,
        height: Fit,
        spacing: 25,
        padding: {right: 43},

        <Label> {
            margin: {right: 20.0},
            draw_text:{
                text_style: <BOLD_FONT>{font_size: 9},
                color: #000
            }
            text: "Model Downloads"
        }

        downloading_count = <Label> {
            draw_text:{
                text_style: <REGULAR_FONT>{font_size: 9},
                color: #099250
            }
            text: "1 downloading"
        }

        paused_count = <Label> {
            draw_text:{
                text_style: <REGULAR_FONT>{font_size: 9},
                color: #667085
            }
            text: "1 paused"
        }

        // <Label> {
        //     draw_text:{
        //         text_style: <REGULAR_FONT>{font_size: 9},
        //         color: #667085
        //     }
        //     text: "5 completed"
        // }

        <VerticalFiller> {}

        collapse = <CollapseButton> {}
    }

    Content = <View> {
        width: Fill,
        height: 350,

        list = <PortalList> {
            width: Fill,
            height: Fill,

            DownloadItem = <DownloadItem> {}
        }
    }

    Downloads = {{Downloads}} {
        width: Fill,
        height: Fit,
        flow: Down,

        show_bg: true,
        draw_bg: {
            color: #FCFCFD,
        }

        // TODO there is a better way to have only top-border?
        <Line> { draw_bg: { color: #EAECF0 }}
        <Header> {
            padding: {top: 20.0, bottom: 20.0, left: 43.0},
        }
        content = <Content> {
            height: 0,
            padding: {top: 12.0, bottom: 12.0, left: 43.0, right: 43.0},
        }

        animator: {
            content = {
                default: collapse,
                expand = {
                    redraw: true,
                    from: {all: Forward {duration: 0.3}}
                    ease: ExpDecay {d1: 0.80, d2: 0.97}
                    apply: {content = { height: 350.0 }}
                }
                collapse = {
                    redraw: true,
                    from: {all: Forward {duration: 0.3}}
                    ease: ExpDecay {d1: 0.80, d2: 0.97}
                    apply: {content = { height: 0.0 }}
                }
            }
        }

    }
}

#[derive(Live, LiveHook, Widget)]
pub struct Downloads {
    #[deref]
    view: View,

    #[animator]
    animator: Animator,
}

impl Widget for Downloads {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
        self.widget_match_event(cx, event, scope);

        if self.animator_handle_event(cx, event).must_redraw() {
            self.redraw(cx);
        }

        match event.hits(cx, self.view.area()) {
            Hit::FingerUp(fe) => {
                if fe.was_tap() {
                    self.toggle_collapse(cx);
                }
            }
            _ => {}
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        let store = scope.data.get::<Store>().unwrap();
        let current_downloads = store.current_downloads_info();
        let downloads_count = current_downloads.len();

        let download_count = current_downloads
            .iter()
            .filter(|d| matches!(d.status, DownloadInfoStatus::Downloading))
            .count();
        self.label(id!(downloading_count))
            .set_text(&format!("{} downloading", download_count));

        let paused_count = current_downloads
            .iter()
            .filter(|d| matches!(d.status, DownloadInfoStatus::Paused))
            .count();
        self.label(id!(paused_count))
            .set_text(&format!("{} paused", paused_count));

        while let Some(view_item) = self.view.draw_walk(cx, &mut Scope::empty(), walk).step() {
            if let Some(mut list) = view_item.as_portal_list().borrow_mut() {
                list.set_item_range(cx, 0, downloads_count);
                while let Some(item_id) = list.next_visible_item(cx) {
                    let item = list.item(cx, item_id, live_id!(DownloadItem)).unwrap();

                    if item_id < downloads_count {
                        let download = &current_downloads[item_id];
                        item.draw_all(cx, &mut Scope::with_data(&mut download.clone()));
                    }
                }
            }
        }

        DrawStep::done()
    }
}

impl WidgetMatchEvent for Downloads {
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions, _scope: &mut Scope) {
        if let Some(fe) = self.view(id!(collapse)).finger_up(&actions) {
            if fe.was_tap() {
                self.toggle_collapse(cx);
            }
        }
    }
}

impl Downloads {
    fn toggle_collapse(&mut self, cx: &mut Cx) {
        if self.animator.animator_in_state(cx, id!(content.collapse)) {
            self.animator_play(cx, id!(content.expand));
            self.set_collapse_button_open(cx, true)
        } else {
            self.animator_play(cx, id!(content.collapse));
            self.set_collapse_button_open(cx, false)
        }
    }

    fn set_collapse_button_open(&mut self, cx: &mut Cx, is_open: bool) {
        let rotation_angle = if is_open { 0.0 } else { 180.0 };
        self.icon(id!(collapse.icon))
        .apply_over(cx, 
            live! {
                draw_icon: { rotation_angle: (rotation_angle) }
            },
        );
    }
}
