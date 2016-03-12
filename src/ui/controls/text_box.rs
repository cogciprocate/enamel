// use super::{};
// use util;

use ui::{self, Shape2d, Element, ElementKind, KeyboardInputHandler};

pub struct TextBox;

impl TextBox {
    pub fn new(anchor_pos: [f32; 3], offset: (f32, f32), extra_width: f32,
                label: &str, color: [f32; 4], sub_text_string: &str, 
                key_handler: KeyboardInputHandler) 
            -> Element
    {
        let shape = Shape2d::hexagon_panel(1.0, extra_width, 0.0, color);

        Element::new(ElementKind::TextBox(TextBox), anchor_pos, [offset.0, offset.1, 0.0], shape)
            .text_string(label)
            .text_offset(((-extra_width / 2.0) - 1.5, 0.0))    
            .sub(TextField::new(anchor_pos, offset, extra_width, sub_text_string, key_handler))
    }
}


pub struct TextField;

impl TextField {
    pub fn new(anchor_pos: [f32; 3], offset: (f32, f32), width: f32, text_string: &str,
                key_handler: KeyboardInputHandler) -> Element
    {
        let color = [1.0, 1.0, 1.0, 1.0];
        let shape = Shape2d::rectangle(0.8, width + 2.4, -0.1, color);
        let text_offset = (-(shape.radii).0 + 0.16, 0.0);

        let new_offset = [
            offset.0 + 0.06,
            offset.1,
            0.0,
        ];

        Element::new(ElementKind::TextField, anchor_pos, new_offset, shape)        
            .border(0.05, ui::C_BLACK, false)
            .text_offset(text_offset)
            .text_string(text_string)
            .keyboard_input_handler(key_handler)
    }
}
