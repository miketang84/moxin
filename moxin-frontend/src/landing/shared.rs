use makepad_widgets::*;

live_design! {
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;

    import crate::shared::styles::*;
    import crate::shared::widgets::*;

    const MODEL_LINK_FONT_COLOR = #x155EEF

    ModelLink = <LinkLabel> {
        width: Fill,
        draw_text: {
            text_style: <REGULAR_FONT>{font_size: 9},
            fn get_color(self) -> vec4 {
                return mix(
                    mix(
                        MODEL_LINK_FONT_COLOR,
                        MODEL_LINK_FONT_COLOR,
                        self.hover
                    ),
                    MODEL_LINK_FONT_COLOR,
                    self.pressed
                )
            }
        }
    }

    ModelAttributeTag = <RoundedView> {
        width: Fit,
        height: Fit,
        padding: {top: 6, bottom: 6, left: 10, right: 10}

        spacing: 5,
        draw_bg: {
            instance radius: 2.0,
        }

        attr_name = <Label> {
            draw_text:{
                text_style: <REGULAR_FONT>{font_size: 9},
                color: #fff
            }
        }

        attr_value = <Label> {
            draw_text:{
                text_style: <BOLD_FONT>{font_size: 9},
                color: #fff
            }
        }
    }

    ModelAttributes = <View> {
        width: Fit,
        height: Fit,
        spacing: 10,

        model_size_tag = <ModelAttributeTag> {
            draw_bg: { color: #44899A },
            attr_name = { text: "Model Size" }
        }

        model_requires_tag = <ModelAttributeTag> {
            draw_bg: { color: #5CAA74 },
            attr_name = { text: "Requires" }
        }

        model_architecture_tag = <ModelAttributeTag> {
            draw_bg: { color: #A44EBB },
            attr_name = { text: "Architecture" }
        }
    }
}