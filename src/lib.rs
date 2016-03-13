#![allow(dead_code)]

#[macro_use] extern crate glium;
extern crate glium_text;

pub mod ui;
// // [TEMP]:
// mod window;

pub mod util;

// // [TEMP]:
// pub use self::window::Window;

pub use self::ui::{Button, HexButton, TextBox, MouseState, KeyboardState, Element, ElementBorder, 
	ElementKind, ElementText, Pane, Shape2d, Vertex, CustomEventRemainder, MouseInputHandler, 
	KeyboardInputHandler, TextAlign, EventRemainder, HandlerOption, UiRequest};

pub use glium::glutin::{ElementState, MouseButton, MouseScrollDelta};