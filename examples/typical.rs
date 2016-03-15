//! This example demonstrates how Enamel would generally be used. 
//!
//! Two traits must be implemented to use the enamel interfaces. The first,
//! (implemented by the enum, `BackgroundCtl`, in this example) carries
//! information from our custom defined event handling closures. This type can
//! be anything as long as it implements the `Default` and `EventRemainder`
//! traits.
//!
//! A second type (called `Background` here) acts as the background of the
//! window and represents 'everything else' having to do with your program. It
//! represents whatever layer or layers are 'behind' the user interface. This
//! type need only implement `SetMouseFocus`, which is a second way to set focus.
//!

extern crate glium;
extern crate enamel;
#[macro_use] extern crate colorify;

use glium::{DisplayBuild, Surface};
use enamel::{Pane, Event, EventRemainder, UiRequest, TextBox, RectButton, HexButton, ElementState, 
    MouseButton, MouseScrollDelta};

/// This enum is used by our event handling closures to return useful
/// information and commands back to the main window. 
///
/// This can contain as many custom variants as we need (and doesn't even need
/// to be an enum) but must implement `Default` and `EventRemainder` which are
/// used by enamel to send information about events not handled by custom
/// closures.
///
///
// #[derive(Clone, Debug)]
pub enum BackgroundCtl {
    None,
    Event(Event),
    Start,
    Stop,
    Text(String),
}

impl Default for BackgroundCtl {
    fn default() -> BackgroundCtl {
        BackgroundCtl::None
    }
}

impl EventRemainder for BackgroundCtl {
    fn event(event: Event) -> Self {
        BackgroundCtl::Event(event)
    }
}


/// This represents a 'background' which is sent any events that the interface
/// layer did not make use of. These 'leftover' events are represented by the
/// `EventRemainder` trait, which in this case we have implemented on an enum.
///
/// If any mouse or keyboard events happen when no interface elements make use
/// of them, they would be sent here.
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
            BackgroundCtl::Event(e) => { match e {
                Event::KeyboardInput(st, key, vkc) => 
                    println!("Key: 0x{:02X} ({:?}) has been {:?}.", key, enamel::ui::map_vkc(vkc), st),
                Event::MouseMoved(pos) => self.handle_mouse_moved(pos),
                Event::MouseWheel(delta) => self.handle_mouse_wheel(delta),
                Event::MouseInput(st, btn) => self.handle_mouse_input(st, btn),
                Event::Touch(touch) => println!("Touch recieved: {:?}", touch),
                Event::Closed => self.handle_closed(),
                _ => (),
            } }
	        BackgroundCtl::Text(s) => printlnc!(royal_blue: "String entered: '{}'.", &s),
            BackgroundCtl::Start => printlnc!(lime: "Starting something!"),
            BackgroundCtl::Stop => printlnc!(red: "Stopping everything!"),
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

    fn handle_closed(&mut self) {
        self.closed = true;
        printlnc!(yellow: "Exiting.");
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
                "Previous", enamel::ui::C_ORANGE)
            .mouse_event_handler(Box::new(|_, _| {
            	println!("This button doesn't do much.");
                (UiRequest::None, BackgroundCtl::None)
            }))
        )
        .element(HexButton::new([1.0, -1.0, 0.0], (-0.20, 0.37), 1.8, 
                "Next", enamel::ui::C_ORANGE)
            .mouse_event_handler(Box::new(|_, _| {
            	println!("This button does less than the one next to it.");
                (UiRequest::None, BackgroundCtl::None)
            }))
        )
        .element(TextBox::new([1.0, -1.0, 0.0], (-0.385, 0.27), 4.45, 
                "Text:", enamel::ui::C_ORANGE, "", Box::new(|key_st, vk_code, kb_st, text_string| {
                    enamel::ui::key_into_string(key_st, vk_code, kb_st, text_string);

                    (UiRequest::None, BackgroundCtl::Text(text_string.clone()))
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
                (UiRequest::None, BackgroundCtl::Start)
            }))
        )
        .element(RectButton::new([1.0, -1.0, 0.0], (-0.20, 0.17), 4.8, 
                "Stop", enamel::ui::C_ORANGE)
            .mouse_event_handler(Box::new(|_, _| {                     
                printlnc!(red_bold: "Stop clicked!");
                (UiRequest::None, BackgroundCtl::Stop)
            }))
        )
        .element(RectButton::new([1.0, -1.0, 0.0], (-0.20, 0.07), 4.8, 
                "Exit", enamel::ui::C_ORANGE)
            .mouse_event_handler(Box::new(|_, _| {
                printlnc!(yellow_bold: "Exit clicked!");
                (UiRequest::None, BackgroundCtl::Event(Event::Closed))
            }))
        )
        .init();

    // This can be whatever we want as long as it implements `SetMouseFocus`:
    let mut background = Background::new();

    printlnc!(white: "Enamel 'typical' example running. Press 'ctrl + q' or \
        push the 'Exit' button to quit.");

    loop {
        // Create draw target and clear color and depth:
        let mut target = display.draw();
        target.clear_color_and_depth((0.03, 0.03, 0.05, 1.0), 1.0);

        // Check input events:
        for ev in display.poll_events() {
            background.handle_event_remainder(ui.handle_event(ev));
        }

        // Draw UI:
        ui.draw(&mut target);

        // Swap buffers:
        target.finish().unwrap();

        if background.closed { break; }
    }
}