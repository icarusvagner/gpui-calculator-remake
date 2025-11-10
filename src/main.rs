use gpui::*;

use crate::root::Root;

mod bottons;
mod colors;
mod consts;
mod display;
mod logic;
mod root;

actions!(calculator, [Quit]);

fn main() {
    Application::new().run(|cx: &mut App| {
        cx.activate(true);
        cx.on_action(|_: &Quit, cx| cx.quit());
        cx.bind_keys([KeyBinding::new("cmd-q", Quit, None)]);
        cx.set_menus(vec![Menu {
            name: "Calculator".into(),
            items: vec![MenuItem::action("Quit", Quit)],
        }]);

        let bounds = Bounds::centered(None, size(px(300.), px(450.)), cx);

        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |_, cx| cx.new(|cx| Root::new(cx)),
        )
        .unwrap();
    });
}
