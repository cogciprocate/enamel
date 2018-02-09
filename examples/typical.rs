//! This example demonstrates how Enamel would generally be used.
//!
//! One trait must be implemented to use custom handler closures. In this
//! example we've implemented it on the enum, `BackgroundCtl` but it can be
//! implemented on a struct just as well. This type carries information from
//! our custom defined event handling closures. It can do anything you want as
//! long as it implements the `Default` and `EventRemainder` traits. `Default`
//! should set a variant or a flag indicating that no action should be taken.
//!


extern crate glium;
extern crate enamel;
#[macro_use] extern crate colorify;

use glium::{glutin, Surface};
use enamel::{ui, Pane, Event, WindowEvent, EventRemainder, UiRequest, TextBox, RectButton,
    HexButton, ElementState, MouseButton, MouseScrollDelta};
use enamel::ui::C_ORANGE as ORANGE;


/// This enum is used by our event handling closures to return useful
/// information and commands back to the main window.
///
/// This can contain as many custom variants as we need (and doesn't even need
/// to be an enum) but must implement `Default` and `EventRemainder` which are
/// used by enamel to send information about events not handled by custom
/// closures.
///
pub enum BackgroundCtl {
    None,
    Event(Event),
    Start,
    Stop,
    Text(String),
    Close,
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
                Event::WindowEvent { window_id: _, event: win_event } => {
                    match win_event {
                        WindowEvent::KeyboardInput { device_id: _, input } => {
                            println!("Key: 0x{:02X} ({:?}) has been {:?}.", input.scancode,
                                enamel::ui::map_vkc(input.virtual_keycode), input.state);
                        },
                        WindowEvent::CursorMoved { device_id: _, position, modifiers: _ } => {
                            self.handle_mouse_moved(position);
                        }
                        WindowEvent::MouseWheel { device_id: _, delta, phase: _, modifiers: _ } => {
                            self.handle_mouse_wheel(delta);
                        }
                        WindowEvent::MouseInput { device_id: _, state, button, modifiers: _ } => {
                            self.handle_mouse_input(state, button)
                        }
                        WindowEvent::Touch(touch) => println!("Touch recieved: {:?}", touch),
                        WindowEvent::Closed => self.handle_closed(),
                        _ => (),
                    }
                }
                _ => (),
            } }
	        BackgroundCtl::Text(s) => printlnc!(royal_blue: "String entered: '{}'.", &s),
            BackgroundCtl::Start => printlnc!(lime: "Starting something!"),
            BackgroundCtl::Stop => printlnc!(red: "Stopping everything!"),
            BackgroundCtl::Close => self.handle_closed(),
	    }
	}

	fn handle_mouse_wheel(&self, scroll_delta: MouseScrollDelta) {
	    let (hrz, vrt) = match scroll_delta {
	        MouseScrollDelta::LineDelta(h, v) => (h, v),
	        MouseScrollDelta::PixelDelta(x, y) => (x, y),
	    };
	    printlnc!(green: "Mouse wheel scrolled by: horizontal: {}, vertical: {}", hrz, vrt)
	}

	fn handle_mouse_moved(&mut self, pos: (f64, f64)) {
	    // println!("Mouse has moved to: ({}, {})", pos.0, pos.1)
	    self.mouse_pos = (pos.0 as i32, pos.1 as i32);
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
    let mut events_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new()
        .with_dimensions(600, 800)
        .with_title("Button Sample".to_string());
    let context = glutin::ContextBuilder::new()
        .with_depth_buffer(24)
        .with_vsync(true)
        .with_multisampling(8);
    let display = glium::Display::new(window, context, &events_loop).unwrap();

    // Primary user interface elements:
    let mut ui = Pane::new(&display)
        .element(HexButton::new(ui::BOTTOM_RIGHT, (-0.57, 0.37), 1.8, "Previous", ORANGE)
            .mouse_event_handler(Box::new(|_, _| {
            	println!("This button doesn't do much.");
                (UiRequest::None, BackgroundCtl::None)
            }))
        )
        .element(HexButton::new(ui::BOTTOM_RIGHT, (-0.20, 0.37), 1.8, "Next", ORANGE)
            .mouse_event_handler(Box::new(|_, _| {
            	println!("This button does less than the one next to it.");
                (UiRequest::None, BackgroundCtl::None)
            }))
        )
        .element(TextBox::new(ui::BOTTOM_RIGHT, (-0.385, 0.27), 4.45, "Text:", ORANGE, "")
            .keyboard_event_handler(Box::new(|key_st, vk_code, kb_st, text_string| {
                enamel::ui::key_into_string(key_st, vk_code, kb_st, text_string);
                (UiRequest::None, BackgroundCtl::Text(text_string.clone()))
            }))
            .mouse_event_handler(Box::new(|_, _| {
            	println!("TextBox clicked and now has keyboard focus.");
                (UiRequest::KeyboardFocus(true), BackgroundCtl::None)
            }))

        )
        .element(RectButton::new(ui::BOTTOM_RIGHT, (-0.57, 0.17), 4.8, "Start", ORANGE)
            .mouse_event_handler(Box::new(|_, _| {
            	printlnc!(lime_bold: "Start clicked!");
                (UiRequest::None, BackgroundCtl::Start)
            }))
        )
        .element(RectButton::new(ui::BOTTOM_RIGHT, (-0.20, 0.17), 4.8, "Stop", ORANGE)
            .mouse_event_handler(Box::new(|_, _| {
                printlnc!(red_bold: "Stop clicked!");
                (UiRequest::None, BackgroundCtl::Stop)
            }))
        )
        .element(RectButton::new(ui::BOTTOM_RIGHT, (-0.20, 0.07), 4.8, "Exit", ORANGE)
            .mouse_event_handler(Box::new(|_, _| {
                printlnc!(yellow_bold: "Exit clicked!");
                (UiRequest::None, BackgroundCtl::Close)
            }))
        )
        .init();

    // This can be whatever we want as long as it implements `SetMouseFocus`:
    let mut background = Background::new();

    printlnc!(white: "Enamel 'typical' example running. Press the 'Exit' button to quit.");

    loop {
        // Create draw target and clear color and depth:
        let mut target = display.draw();
        target.clear_color_and_depth((0.03, 0.03, 0.05, 1.0), 1.0);

        // Check input events:
        events_loop.poll_events(|ev| {
            background.handle_event_remainder(ui.handle_event(ev));
        });

        // Draw UI:
        ui.draw(&mut target);

        // Swap buffers:
        target.finish().unwrap();

        if background.closed { break; }
    }
}