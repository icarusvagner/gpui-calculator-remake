use gpui::*;

use crate::{
    bottons::{Button, ButtonVariant},
    colors::BG_COLOR,
    consts::BUTTONS,
    display::Display,
    logic::{ButtonType, Logic},
};

pub struct Root {
    pub logic: Logic,
    focus_handle: FocusHandle,
}

impl Root {
    pub fn new(cx: &mut App) -> Self {
        let logic = Logic::new();

        Self {
            logic,
            focus_handle: cx.focus_handle(),
        }
    }

    fn get_buttons(&self, cx: &mut Context<Self>) -> Vec<Button> {
        let mut buttons = Vec::new();

        for button_type in BUTTONS {
            let basis = match button_type {
                ButtonType::Equal => 0.47,
                _ => 0.225,
            };

            let variant = match button_type {
                ButtonType::Equal => ButtonVariant::Primary,
                ButtonType::Number(_) => ButtonVariant::Neutral,
                ButtonType::Comma => ButtonVariant::Neutral,
                _ => ButtonVariant::Secondary,
            };

            let button = Button::new(button_type.clone(), basis, variant).on_click(cx.listener(
                move |this, _, _, cx| {
                    this.logic.on_button_pressed(button_type.clone());
                    cx.notify();
                },
            ));

            buttons.push(button);
        }

        buttons
    }
}

impl Render for Root {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let display_val = self.logic.get_display_value();
        let buttons = self.get_buttons(cx);

        div()
            .track_focus(&self.focus_handle)
            .on_key_down(cx.listener(|this, event: &KeyDownEvent, _, cx| {
                this.logic.handle_key_input(&event.keystroke.key.as_str());
                cx.notify();
            }))
            .size_full()
            .flex()
            .flex_col()
            .bg(rgb(BG_COLOR))
            .text_lg()
            .child(cx.new(|_| Display::new(display_val)))
            .child(
                div()
                    .flex()
                    .flex_row()
                    .flex_wrap()
                    .items_center()
                    .justify_center()
                    .h(DefiniteLength::Fraction(0.80))
                    .py(DefiniteLength::Fraction(0.02))
                    .gap(DefiniteLength::Fraction(0.02))
                    .children(buttons),
            )
    }
}
