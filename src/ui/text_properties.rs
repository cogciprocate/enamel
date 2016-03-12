#![allow(dead_code)]
use glium_text::{TextSystem, FontTexture, TextDisplay};
use ui::{TextAlign};

pub const DEFAULT_TEXT_COLOR: (f32, f32, f32, f32) = (0.01, 0.01, 0.01, 1.0);
pub const TEXT_BASE_SCALE: f32 = 0.39;


pub struct ElementText {
    pub string: String,
    pub color: (f32, f32, f32, f32),
    pub base_scale: f32,
    pub element_offset: (f32, f32),
    pub align: TextAlign,
    pub raw_width: f32,    
    pub cur_scale: (f32, f32),
    pub cur_center_pos: (f32, f32),
}

impl ElementText {
    pub fn new(new_str: &str) -> ElementText {
        let mut string = String::with_capacity(64);
        string.push_str(new_str);

        ElementText {
            string: string,
            color: DEFAULT_TEXT_COLOR,
            base_scale: TEXT_BASE_SCALE,
            element_offset: (0.0, 0.0),
            align: TextAlign::Center,
            raw_width: 0.0,
            cur_scale: (0.0, 0.0), 
            cur_center_pos: (0.0, 0.0),
        }
    }

    pub fn color(mut self, color: (f32, f32, f32, f32)) -> ElementText {
        self.color = color;
        self
    }

    pub fn matrix(&self) -> [[f32; 4]; 4] {
        [    
            [self.cur_scale.0, 0.0, 0.0, 0.0,],
            [0.0, self.cur_scale.1, 0.0, 0.0,],
            [0.0, 0.0, 1.0, 0.0,],
            [    
                self.cur_center_pos.0, 
                self.cur_center_pos.1, 
                0.0, 1.0f32,
            ],     
        ]
    }

    pub fn set_raw_width(&mut self, ts: &TextSystem, ft: &FontTexture) {
        let text_display = TextDisplay::new(ts, ft, &self.string);
        self.raw_width = text_display.get_width();
    }

    pub fn get_color(&self) -> (f32, f32, f32, f32) {
        self.color
    }

    // pub fn get_string(&self) -> &str {
    //     &self.string
    // }
}

