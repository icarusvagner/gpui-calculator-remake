use gpui::*;

use crate::colors::{BG_COLOR, WHITE_SMOKE};

pub struct Display {
    value: f64,
}

impl Display {
    pub fn new(value: f64) -> Self {
        Self { value }
    }
}

impl Render for Display {
    fn render(
        &mut self,
        window: &mut gpui::Window,
        cx: &mut gpui::Context<Self>,
    ) -> impl gpui::IntoElement {
        div()
            .bg(rgb(BG_COLOR))
            .text_color(rgb(WHITE_SMOKE))
            .h(DefiniteLength::Fraction(0.2))
            .px_8()
            .w_full()
            .flex()
            .items_center()
            .justify_end()
            .child(self.value.to_string())
    }
}
