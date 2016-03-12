
use ui::{Shape2d, Element, ElementKind};

pub struct Button {
    is_depressed: bool,
}

impl Button {
    pub fn new() -> Button {
        Button { is_depressed: false }
    }

    // pub fn is_depressed(&self) -> bool {
    //     self.is_depressed
    // }

    // pub fn depress(&mut self, depress: bool) {
    //     self.is_depressed = depress
    // }
}


pub struct HexButton;

impl HexButton {
    pub fn new(anchor_pos: [f32; 3], offset: (f32, f32), extra_width: f32,
            text: &str, color: [f32; 4]) -> Element
    {
        let shape = Shape2d::hexagon_panel(1.0, extra_width, 0.0, color);

        Element::new(ElementKind::Button(Button::new()), anchor_pos, [offset.0, offset.1, 0.0], shape)
            .text_string(text)
    }
}
