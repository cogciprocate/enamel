extern crate glium;
extern crate enamel;
#[macro_use] extern crate colorify;

use glium::{DisplayBuild, Surface};
use enamel::{Pane, EventRemainder, UiRequest, TextBox, RectButton, HexButton, ElementState, 
    MouseButton, MouseScrollDelta, SetFocus};

/// This enum is used by our event handling closures to return useful
/// information and commands back to the main window. It can contain as many
/// custom variants as we need but must implement `Default` and
/// `EventRemainder` which are used by enamel.
///
// #[derive(Clone, Debug)]
pub enum BackgroundCtl {
    None,
    Closed,
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
    fn closed() -> BackgroundCtl {
        BackgroundCtl::Closed
    }

    fn mouse_moved(pos: (i32, i32)) -> Self {
        BackgroundCtl::MouseMoved(pos)
    }

    fn mouse_wheel(delta: MouseScrollDelta) -> Self {
        BackgroundCtl::MouseWheel(delta)
    }

    fn mouse_input(state: ElementState, button: MouseButton) -> Self {
        BackgroundCtl::MouseInput(state, button)
    }
        
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

	fn handle_event_remainder(&mut self, rdr: BackgroundCtl) -> bool {
	    match rdr {
	        BackgroundCtl::None => (),
	        BackgroundCtl::MouseWheel(delta) => self.handle_mouse_wheel(delta),
	        BackgroundCtl::MouseInput(state, button) => self.handle_mouse_input(state, button),
	        BackgroundCtl::MouseMoved(pos) => self.handle_mouse_moved(pos),
	        BackgroundCtl::TextEntered(s) => printlnc!(royal_blue: "String entered: {}", &s),
	        BackgroundCtl::Closed => return true,
	        _ => (),           
	    }
	    false
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

	fn handle_mouse_input(&self, button_state: ElementState, button: MouseButton) {
	    match button {
	        MouseButton::Left => {
	            match button_state {
	                ElementState::Pressed => printlnc!(magenta: "Left mouse button pressed on background."),
	                ElementState::Released => printlnc!(purple: "Left mouse button released on background."),
	            }
	        },
	        _ => (),
	    }
	}
}

impl<'a> SetFocus for Background {
	fn set_mouse_focus(&mut self, focus: bool) {
		if focus {
			printlnc!(dark_grey: "Window background now has focus.");
		} else {
			printlnc!(dark_grey: "Window background lost focus.");
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
                "Text:", enamel::ui::C_ORANGE, "1", Box::new(|key_state, vk_code, kb_state, text_string| {
                    enamel::ui::key_into_string(key_state, vk_code, kb_state, text_string);

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
                printlnc!(yellow: "Exit button clicked.");
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
            background.closed = background.handle_event_remainder(ui.handle_event(ev));
        }

        // Draw UI:
        ui.draw(&mut target, &mut background);

        // Swap buffers:
        target.finish().unwrap();

        if background.closed { break; }
    }
}