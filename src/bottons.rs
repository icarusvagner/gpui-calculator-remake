use gpui::*;

use crate::{
    colors::{DAVY_GRAY, GRAY, SILVER_DARK, SILVER_LIGHT, WHITE},
    logic::{ButtonType, Operation},
};

pub enum ButtonVariant {
    Primary,
    Secondary,
    Neutral,
}

struct ButtonStyle {
    bg: u32,
    text_color: u32,
    hover_color: u32,
}

#[derive(IntoElement)]
pub struct Button {
    on_click: Box<dyn Fn(&MouseDownEvent, &mut Window, &mut App) + 'static>,
    base: Div,
    text: String,
    basis: f32,
    variant: ButtonVariant,
}

impl Button {
    pub fn on_click(
        mut self,
        handler: impl Fn(&MouseDownEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_click = Box::new(handler);
        self
    }

    fn get_variant_style(&self) -> ButtonStyle {
        match self.variant {
            ButtonVariant::Primary => ButtonStyle {
                bg: DAVY_GRAY,
                text_color: WHITE,
                hover_color: SILVER_LIGHT,
            },
            ButtonVariant::Secondary => ButtonStyle {
                bg: GRAY,
                text_color: WHITE,
                hover_color: SILVER_LIGHT,
            },
            ButtonVariant::Neutral => ButtonStyle {
                bg: SILVER_DARK,
                text_color: WHITE,
                hover_color: SILVER_LIGHT,
            },
        }
    }

    fn get_label(button_type: ButtonType) -> String {
        match button_type {
            ButtonType::Reset => "C".to_owned(),
            ButtonType::Sign => "+/-".to_owned(),
            ButtonType::Percent => "%".to_owned(),
            ButtonType::Comma => ",".to_owned(),
            ButtonType::Equal => "=".to_owned(),
            ButtonType::Number(num) => num.to_string(),
            ButtonType::Arithmetic(Operation::Division) => "/".to_owned(),
            ButtonType::Arithmetic(Operation::Multiply) => "x".to_owned(),
            ButtonType::Arithmetic(Operation::Addition) => "+".to_owned(),
            ButtonType::Arithmetic(Operation::Subtract) => "-".to_owned(),
        }
    }
}

impl Button {
    pub fn new(button_type: ButtonType, basis: f32, variant: ButtonVariant) -> Self {
        Self {
            on_click: Box::new(|_event, _window, _cx| {}),
            base: div(),
            text: Self::get_label(button_type),
            basis,
            variant,
        }
    }
}

impl RenderOnce for Button {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let style = self.get_variant_style();

        self.base
            .w_full()
            .cursor_pointer()
            .h(DefiniteLength::Fraction(0.176))
            .bg(rgb(style.bg))
            .text_color(rgb(style.text_color))
            .hover(|this| this.bg(rgb(style.hover_color)))
            .flex_basis(DefiniteLength::Fraction(self.basis))
            .flex()
            .rounded_lg()
            .items_center()
            .justify_center()
            .child(self.text)
            .on_mouse_down(MouseButton::Left, move |event, window, cx| {
                (&self.on_click)(event, window, cx)
            })
    }
}
