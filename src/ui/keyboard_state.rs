use glium::glutin::{ElementState, VirtualKeyCode};

pub struct KeyboardState {
    pub shift: bool,
    pub control: bool,
    pub alt: bool,
}

impl KeyboardState {
    pub fn new() -> KeyboardState {
        KeyboardState {
            shift: false,
            control: false,
            alt: false,
        }
    }

    pub fn update(&mut self, key_state: ElementState, vk_code: Option<VirtualKeyCode>) {
        use glium::glutin::VirtualKeyCode::*;

        if let Some(vkc) = vk_code {
            match vkc {
                LShift | RShift => self.shift = map_state(key_state),
                LControl | RControl => self.control = map_state(key_state),
                LAlt | RAlt => self.alt = map_state(key_state),
                _ => (),
            }
        }
    }
}

fn map_state(key_state: ElementState) -> bool {
    match key_state {
        ElementState::Pressed => true,
        ElementState::Released => false,
    }
}
