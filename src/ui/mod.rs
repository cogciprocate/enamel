#![allow(dead_code)]

mod element;
mod pane;
mod shape_2d;
mod text_properties;
mod vertex;
mod keyboard_state;
mod mouse_state;
mod controls;

pub use self::controls::{Button, HexButton, RectButton, TextBox};
pub use self::mouse_state::MouseState;
pub use self::keyboard_state::KeyboardState;
pub use self::element::{Element, ElementBorder, ElementKind, ElementText};
pub use self::pane::Pane;
pub use self::shape_2d::Shape2d;
pub use self::vertex::Vertex;
pub use self::traits::{CustomEventRemainder, EventRemainder, SetMouseFocus};
pub use self::aliases::{MouseEventHandler, KeyboardEventHandler};
pub use self::enums::{TextAlign, UiRequest, EventRemainderOld, HandlerOption};
pub use self::functions::{ key_into_string, map_vkc };

pub const TOP_LEFT: [f32; 3] = [-1.0, 1.0, 0.0];
pub const TOP_RIGHT: [f32; 3] = [1.0, 1.0, 0.0];
pub const BOTTOM_LEFT: [f32; 3] = [-1.0, -1.0, 0.0];
pub const BOTTOM_RIGHT: [f32; 3] = [1.0, -1.0, 0.0];

pub const C_PINK: [f32; 4] = [0.990, 0.490, 0.700, 1.0];
pub const C_ORANGE: [f32; 4] = [0.960, 0.400, 0.0, 1.0];
pub const C_DARK_ORANGE: [f32; 4] = [0.384, 0.080, 0.0, 1.0]; 
pub const C_BLUE: [f32; 4] = [0.204, 0.396, 0.643, 1.0];
pub const C_BLACK: [f32; 4] = [0.001, 0.001, 0.001, 1.0];
pub const SUBDEPTH: f32 = -0.015625;
pub const SUBSUBDEPTH: f32 = 0.000244140625;


mod traits {
    use std::fmt::{Debug, Formatter, Result as FmtResult};
    use std::default::Default;
    use glium::glutin::Event;

    // TODO: DEPRICATE:
    pub trait SetMouseFocus {
        fn set_mouse_focus(&mut self, bool);
    }

    pub trait EventRemainder: Default {
        fn event(Event) -> Self;
    }

    pub trait CustomEventRemainder: CustomEventRemainderClone + CustomEventRemainderDebug {}

    pub trait CustomEventRemainderClone {
        fn clone_box(&self) -> Box<CustomEventRemainder>;
    }

    pub trait CustomEventRemainderDebug {
        fn debug_fmt(&self, &mut Formatter) -> FmtResult;
    }

    impl<T> CustomEventRemainderClone for T where T: 'static + CustomEventRemainder + Clone {
        fn clone_box(&self) -> Box<CustomEventRemainder> {
            Box::new(self.clone())
        }
    }

    impl<T> CustomEventRemainderDebug for T where T: 'static + CustomEventRemainder + Debug {
        fn debug_fmt(&self, f: &mut Formatter) -> FmtResult {
            self.fmt(f)
        }
    }

    impl Clone for Box<CustomEventRemainder> {
        fn clone(&self) -> Box<CustomEventRemainder> {
            self.clone_box()
        }
    }

    impl Debug for Box<CustomEventRemainder> {
        fn fmt(&self, f: &mut Formatter) -> FmtResult {
            (*self).debug_fmt(f)
        }
    }
}


mod aliases {
    use glium::glutin::{ElementState, MouseButton, VirtualKeyCode};
    use ui::{UiRequest, KeyboardState};

    pub type MouseEventHandler<T> = Box<FnMut(ElementState, MouseButton) -> (UiRequest, T)>;

    pub type KeyboardEventHandler<T> = Box<FnMut(ElementState, Option<VirtualKeyCode>, &KeyboardState, 
        &mut String) -> (UiRequest, T)>;
}


mod structs {
    #![allow(dead_code)]
    use glium::glutin::{VirtualKeyCode};

    struct KeyCombo {
        keys: Vec<VirtualKeyCode>,
    }
}


mod enums {
    use std::fmt::{Debug, Formatter, Error};
    use ui::CustomEventRemainder;
    use glium::glutin::MouseScrollDelta;

    #[derive(Clone, PartialEq, Eq)]
    pub enum UiRequest {
        None,
        Refresh,
        KeyboardFocus(bool),
    }

    #[derive(Clone)]
    pub enum EventRemainderOld {
        None,
        Closed,
        MousePosition(i32, i32),
        MouseWheel(MouseScrollDelta),
        Custom(Box<CustomEventRemainder>),
    }

    pub enum HandlerOption<T> {
        None,
        Fn(T),
        Sub(usize),    
    }

    #[derive(Clone, PartialEq, Eq)]
    pub enum TextAlign {
        Center,
        Left,
        Right,
    }

    impl<T> HandlerOption<T> {
        pub fn is_some(&self) -> bool {
            if let &HandlerOption::None = self {
                false
            } else {
                true
            }
        }
    }

    impl<T> Debug for HandlerOption<T> {
        fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
            match self {
                &HandlerOption::Fn(_) => write!(f, "HandlerOption::Fn(_)"),
                &HandlerOption::Sub(idx) => write!(f, "HandlerOption::Sub({})", idx),
                &HandlerOption::None => write!(f, "HandlerOption::None"),
            }
        }
    }
}


mod functions {
    use glium::glutin::{VirtualKeyCode, ElementState};
    use ui::KeyboardState;

    pub fn key_into_string(key_state: ElementState, vk_code: Option<VirtualKeyCode>, kb_state: &KeyboardState, 
            string: &mut String) 
    {
        if let ElementState::Pressed = key_state {
            match vk_code {
                Some(VirtualKeyCode::Back) => {
                    string.pop();
                },

                _ => {
                    if let Some(mut c) = map_vkc(vk_code) {                    
                        if kb_state.shift { c = c.to_uppercase().next().unwrap_or(c); }
                        string.push(c);                
                    }
                },
            }
        }
    }

    // [FIXME]: TODO: 
    // - Consider using a hashmap? Could be more efficient.
    pub fn map_vkc(vkc: Option<VirtualKeyCode>) -> Option<char> {
        use glium::glutin::VirtualKeyCode::*;

        if let Some(vkc) = vkc { 
            match vkc {
                Key1 | Numpad0 => Some('1'),
                Key2 | Numpad1 => Some('2'),
                Key3 | Numpad2 => Some('3'),
                Key4 | Numpad3 => Some('4'),
                Key5 | Numpad4 => Some('5'),
                Key6 | Numpad5 => Some('6'),
                Key7 | Numpad6 => Some('7'),
                Key8 | Numpad7 => Some('8'),
                Key9 | Numpad8 => Some('9'),
                Key0 | Numpad9 => Some('0'),    
                A => Some('a'),
                B => Some('b'),
                C => Some('c'),
                D => Some('d'),
                E => Some('e'),
                F => Some('f'),
                G => Some('g'),
                H => Some('h'),
                I => Some('i'),
                J => Some('j'),
                K => Some('k'),
                L => Some('l'),
                M => Some('m'),
                N => Some('n'),
                O => Some('o'),
                P => Some('p'),
                Q => Some('q'),
                R => Some('r'),
                S => Some('s'),
                T => Some('t'),
                U => Some('u'),
                V => Some('v'),
                W => Some('w'),
                X => Some('x'),
                Y => Some('y'),
                Z => Some('z'),
                Space => Some(' '),
                _ => None
            }
        } else {
            None
        }
    }
}
