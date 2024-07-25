wit_bindgen::generate!({
    world: "plugin",
});

use crate::flutter::renderer::widget;
use crate::flutter::renderer::widget::Widget;

struct Plugin;

impl Guest for Plugin {
    fn run(renderer: Renderer) -> Option<Error> {
        renderer.render(Widget::Col(widget::Col::new(
            vec![Widget::Text(widget::Text::new("test"))],
            None,
            None,
            None,
            None,
            None,
            None,
        )));

        Option::None
    }
}

export!(Plugin);
