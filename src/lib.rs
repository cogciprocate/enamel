#![allow(dead_code)]

#[macro_use] extern crate glium;
extern crate glium_text;

mod ui;
// // [TEMP]:
// mod window;

pub mod util;

// // [TEMP]:
// pub use self::window::Window;

pub use self::ui::{Button, HexButton, TextBox, MouseState, KeyboardState, Element, ElementBorder, 
	ElementKind, ElementText, Pane, Shape2d, Vertex, CustomEventResult, MouseInputHandler, 
	KeyboardInputHandler, TextAlign, EventResult, HandlerOption};
