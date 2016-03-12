#![allow(dead_code)]

mod element;
mod pane;
mod shape_2d;
mod text_properties;
mod vertex;
mod keyboard_state;
mod mouse_state;
mod controls;

pub use self::controls::{Button, HexButton, TextBox};
pub use self::mouse_state::MouseState;
pub use self::keyboard_state::KeyboardState;
pub use self::element::{Element, ElementBorder, ElementKind, ElementText};
pub use self::pane::Pane;
pub use self::shape_2d::Shape2d;
// pub use self::text_properties::TextProperties;
pub use self::vertex::Vertex;
pub use self::traits::{CustomEventResult};
pub use self::types::{MouseInputHandler, KeyboardInputHandler};
pub use self::enums::{TextAlign, EventResult, HandlerOption};
pub use self::functions::{ key_into_string };
// pub use self::traits::HandlerWindow;


pub use glium::glutin::{ElementState, MouseButton, MouseScrollDelta};

pub const C_PINK: [f32; 4] = [0.990, 0.490, 0.700, 1.0];
pub const C_ORANGE: [f32; 4] = [0.960, 0.400, 0.0, 1.0];
pub const C_DARK_ORANGE: [f32; 4] = [0.384, 0.080, 0.0, 1.0]; 
pub const C_BLUE: [f32; 4] = [0.204, 0.396, 0.643, 1.0];
pub const C_BLACK: [f32; 4] = [0.001, 0.001, 0.001, 1.0];
pub const SUBDEPTH: f32 = -0.015625;
pub const SUBSUBDEPTH: f32 = 0.000244140625;


// Implement this one day. Just a type which has some data for the handlers to use.
mod traits {
    // use std::fmt::Debug;
    // pub trait HandlerWindow {

    // }

    pub trait CustomEventResult: CustomEventResultClone {}

    pub trait CustomEventResultClone {
        fn clone_box(&self) -> Box<CustomEventResult>;
    }

    impl<T> CustomEventResultClone for T where T: 'static + CustomEventResult + Clone {
        fn clone_box(&self) -> Box<CustomEventResult> {
            Box::new(self.clone())
        }
    }

    impl Clone for Box<CustomEventResult> {
        fn clone(&self) -> Box<CustomEventResult> {
            self.clone_box()
        }
    }

    // trait Animal: AnimalClone {
    //     fn speak(&self);
    // }

    // // Splitting AnimalClone into its own trait allows us to provide a blanket
    // // implementation for all compatible types, without having to implement the
    // // rest of Animal.  In this case, we implement it for all types that have
    // // 'static lifetime (*i.e.* they don't contain non-'static pointers), and
    // // implement both Animal and Clone.  Don't ask me how the compiler resolves
    // // implementing AnimalClone for Animal when Animal requires AnimalClone; I
    // // have *no* idea why this works.
    // trait AnimalClone {
    //     fn clone_box(&self) -> Box<Animal>;
    // }

    // impl<T> AnimalClone for T where T: 'static + Animal + Clone {
    //     fn clone_box(&self) -> Box<Animal> {
    //         Box::new(self.clone())
    //     }
    // }

    // // We can now implement Clone manually by forwarding to clone_box.
    // impl Clone for Box<Animal> {
    //     fn clone(&self) -> Box<Animal> {
    //         self.clone_box()
    //     }
    // }

    // #[derive(Clone)]
    // struct Dog {
    //     name: String,
    // }

    // impl Dog {
    //     fn new(name: &str) -> Dog {
    //         return Dog { name: name.to_string() }
    //     }
    // }

    // impl Animal for Dog {
    //     fn speak(&self) {
    //         println!("{}: ruff, ruff!", self.name);
    //     }
    // }

    // #[derive(Clone)]
    // struct AnimalHouse {
    //     animal: Box<Animal>,
    // }

    // fn main() {
    //     let house = AnimalHouse { animal: Box::new(Dog::new("Bobby")) };
    //     let house2 = house.clone();
    //     house2.animal.speak();
    // }
}


mod types {
    use glium::glutin::{ElementState, MouseButton, VirtualKeyCode};
    use ui::{EventResult, KeyboardState};
    // [WINDOW REMOVED]:
    // use window::Window;

    // [WINDOW REMOVED]:
    // pub type MouseInputHandler = Box<FnMut(ElementState, MouseButton, 
    //     &mut Window) -> EventResult>;
    pub type MouseInputHandler = Box<FnMut(ElementState, MouseButton) -> EventResult>;

    // [WINDOW REMOVED]:
    // pub type KeyboardInputHandler = Box<FnMut(ElementState, Option<VirtualKeyCode>, &KeyboardState, &mut String,
    //     &mut Window) -> EventResult>;
    pub type KeyboardInputHandler = Box<FnMut(ElementState, Option<VirtualKeyCode>, &KeyboardState, 
        &mut String) -> EventResult>;
}


mod enums {
    use std::fmt::{Debug, Formatter, Error};
    use ui::CustomEventResult;
    use glium::glutin::MouseScrollDelta;

    pub enum TextAlign {
        Center,
        Left,
        Right,
    }

    #[derive(Clone)]
    pub enum EventResult {
        None,
        Closed,
        RequestKeyboardFocus(bool),
        RequestRedraw,
        MousePosition(i32, i32),
        MouseWheel(MouseScrollDelta),
        Custom(Box<CustomEventResult>),
    }

    pub enum HandlerOption<T> {
        None,
        Fn(T),
        Sub(usize),    
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
    // use glium::glutin::VirtualKeyCode::*;
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
