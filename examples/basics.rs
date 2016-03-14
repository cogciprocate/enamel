//! This example demonstrates how Enamel would generally be used. 
//!
//! Two types are required to be declared. The first, `BackgroundCtl` in this
//! example, to carry information from event handling closures for various
//! interface elements. This type can be anything as long as it implements the
//! `Default` and `EventRemainder` traits.
//!
//! A second type, called `Background` here, acts as the background of the
//! window and represents 'everything else' having to do with user
//! interaction. If your program is a game, it would be the part of the game
//! behind any buttons and interface elements. If your program 

extern crate glium;
extern crate enamel;
#[macro_use] extern crate colorify;

use glium::{DisplayBuild, Surface};
use enamel::{Pane, Event, EventRemainder, UiRequest, TextBox, RectButton, HexButton, ElementState, 
    MouseButton, MouseScrollDelta, VirtualKeyCode, SetFocus};

/// This enum is used by our event handling closures to return useful
/// information and commands back to the main window. It can contain as many
/// custom variants as we need but must implement `Default` and
/// `EventRemainder` which are used by enamel.
///
// #[derive(Clone, Debug)]
pub enum BackgroundCtl {
    None,
    Closed,
    Input(Event),
    KeyboardInput(ElementState, u8, Option<VirtualKeyCode>),
    MouseMoved((i32, i32)),
    MouseWheel(MouseScrollDelta),
    MouseInput(ElementState, MouseButton),
    SetMouseFocus(bool),
    TextEntered(String),
}

impl Default for BackgroundCtl {
    fn default() -> BackgroundCtl {
        BackgroundCtl::None
    }
}

impl EventRemainder for BackgroundCtl {
    fn closed() -> Self {
        BackgroundCtl::Closed
    }

    fn input(event: Event) -> Self {
        BackgroundCtl::Input(event)
    }

    // fn keyboard_input(st: ElementState, key: u8, v_code: Option<VirtualKeyCode>) -> Self {
    //     BackgroundCtl::KeyboardInput(st, key, v_code)
    // }

    // fn mouse_moved(pos: (i32, i32)) -> Self {
    //     BackgroundCtl::MouseMoved(pos)
    // }

    // fn mouse_wheel(delta: MouseScrollDelta) -> Self {
    //     BackgroundCtl::MouseWheel(delta)
    // }

    // fn mouse_input(st: ElementState, btn: MouseButton) -> Self {
    //     BackgroundCtl::MouseInput(st, btn)
    // }
        
    fn set_mouse_focus(focus: bool) -> Self {
        BackgroundCtl::SetMouseFocus(focus)
    }
}


/// This represents a 'background' which we would use to know when know when
/// the mouse was not hovering over any interface elements. 
///
/// If any mouse or keyboard events happen when no interface elements have
/// focus, they would be sent here.
///
pub struct Background {
	pub mouse_pos: (i32, i32),
	pub closed: bool,
}

impl<'a> Background {
	fn new() -> Background {
	    Background {
	    	mouse_pos: (0, 0),
	    	closed: false,
	    }
	}

	fn handle_event_remainder(&mut self, rdr: BackgroundCtl) {
	    match rdr {
	        BackgroundCtl::None => (),
            BackgroundCtl::Input(e) => { match e {
                Event::KeyboardInput(st, key, vkc) => 
                    println!("Key: {} ({:?}) has been {:?}.", key, enamel::ui::map_vkc(vkc), st),
                Event::MouseMoved(pos) => self.handle_mouse_moved(pos),
                Event::MouseWheel(delta) => self.handle_mouse_wheel(delta),
                Event::MouseInput(st, btn) => self.handle_mouse_input(st, btn),
                Event::Touch(touch) => println!("Touch recieved: {:?}", touch),
                _ => (),
            } }
            // BackgroundCtl::KeyboardInput()
	        BackgroundCtl::MouseWheel(delta) => self.handle_mouse_wheel(delta),
	        BackgroundCtl::MouseInput(st, btn) => self.handle_mouse_input(st, btn),
	        BackgroundCtl::MouseMoved(pos) => self.handle_mouse_moved(pos),
	        BackgroundCtl::TextEntered(s) => printlnc!(royal_blue: "String entered: '{}'.", &s),
	        BackgroundCtl::Closed => { 
                self.closed = true;
                printlnc!(yellow_bold: "Exiting.");
            },
	        _ => (),           
	    }
	}

	fn handle_mouse_wheel(&self, scroll_delta: MouseScrollDelta) {
	    let (hrz, vrt) = match scroll_delta {
	        MouseScrollDelta::LineDelta(h, v) => (h, v),
	        MouseScrollDelta::PixelDelta(x, y) => (x, y),
	    };
	    printlnc!(green: "Mouse wheel scrolled by: horizontal: {}, vertical: {}", hrz, vrt)
	}

	fn handle_mouse_moved(&mut self, pos: (i32, i32)) {
	    // println!("Mouse has moved to: ({}, {})", pos.0, pos.1)
	    self.mouse_pos = pos;
	}

	fn handle_mouse_input(&self, btn_st: ElementState, btn: MouseButton) {
	    match btn {
	        MouseButton::Left => { match btn_st {
                ElementState::Pressed => 
                    printlnc!(teal: "Left mouse button pressed on background."),
                ElementState::Released => 
                    printlnc!(cyan: "Left mouse button released on background."),
            } },
            MouseButton::Right => { match btn_st {
                ElementState::Pressed => 
                    printlnc!(magenta: "Right mouse button pressed on background."),
                ElementState::Released => 
                    printlnc!(purple: "Right mouse button released on background."),
            } },
	        _ => (),
	    }
	}
}

impl<'a> SetFocus for Background {
	fn set_mouse_focus(&mut self, focus: bool) {
		if focus {
			printlnc!(dark_grey: "Background now has mouse focus.");
		} else {
			printlnc!(dark_grey: "Background lost mouse focus.");
		}
	}
}


fn main() {
	// Glutin window:
    let display = glium::glutin::WindowBuilder::new()
        .with_depth_buffer(24)
        .with_dimensions(600, 800)
        .with_title("Button Sample".to_string())
        .with_multisampling(8)
        .with_vsync()
        .build_glium().unwrap();

    // Primary user interface elements:
    let mut ui = Pane::new(&display)
        .element(HexButton::new([1.0, -1.0, 0.0], (-0.57, 0.37), 1.8, 
                "View Output", enamel::ui::C_ORANGE)
            .mouse_event_handler(Box::new(|_, _| {
            	println!("This button doesn't do much.");
                (UiRequest::None, BackgroundCtl::None)
            }))
        )
        .element(HexButton::new([1.0, -1.0, 0.0], (-0.20, 0.37), 1.8, 
                "View All", enamel::ui::C_ORANGE)
            .mouse_event_handler(Box::new(|_, _| {
            	println!("This button does even less than the one next to it.");
                (UiRequest::None, BackgroundCtl::None)
            }))
        )
        .element(TextBox::new([1.0, -1.0, 0.0], (-0.385, 0.27), 4.45, 
                "Text:", enamel::ui::C_ORANGE, "1", Box::new(|key_st, vk_code, kb_st, text_string| {
                    enamel::ui::key_into_string(key_st, vk_code, kb_st, text_string);

                    (UiRequest::None, BackgroundCtl::TextEntered(text_string.clone()))
                } )
            )
            .mouse_event_handler(Box::new(|_, _| {
            	println!("TextBox clicked and now has keyboard focus.");
                (UiRequest::KeyboardFocus(true), BackgroundCtl::None)
            } ))

        )
        .element(RectButton::new([1.0, -1.0, 0.0], (-0.57, 0.17), 4.8, 
                "Start", enamel::ui::C_ORANGE)
            .mouse_event_handler(Box::new(|_, _| {
            	printlnc!(lime_bold: "Start clicked!");
                (UiRequest::None, BackgroundCtl::None)
            }))
        )
        .element(RectButton::new([1.0, -1.0, 0.0], (-0.20, 0.17), 4.8, 
                "Stop", enamel::ui::C_ORANGE)
            .mouse_event_handler(Box::new(|_, _| {                     
                printlnc!(red_bold: "Stop clicked!");
                (UiRequest::None, BackgroundCtl::None)
            }))
        )
        .element(RectButton::new([1.0, -1.0, 0.0], (-0.20, 0.07), 4.8, 
                "Exit", enamel::ui::C_ORANGE)
            .mouse_event_handler(Box::new(|_, _| {
                printlnc!(yellow: "Exit clicked!");
                (UiRequest::None, BackgroundCtl::Closed)
            }))
        )
        .init();

    // This can be whatever we want as long as it implements `SetFocus`:
    let mut background = Background::new();

    printlnc!(white: "Enamel 'basics' example running. Press 'ctrl + q' or \
        push the 'Exit' button to quit.");

    loop {
        // Create draw target and clear color and depth:
        let mut target = display.draw();
        target.clear_color_and_depth((0.03, 0.03, 0.05, 1.0), 1.0);

        // Check input events:
        for ev in display.poll_events() {
            let rmndr = ui.handle_event(ev, &mut background);
            background.handle_event_remainder(rmndr);
        }

        // Draw UI:
        ui.draw(&mut target, &mut background);

        // Swap buffers:
        target.finish().unwrap();

        if background.closed { break; }
    }
}