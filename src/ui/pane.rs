#![allow(dead_code, unused_variables)]
// use std::ops::{Deref};
use glium_text::{self, TextSystem, FontTexture, TextDisplay};
use glium::backend::glutin_backend::GlutinFacade;
use glium::{self, VertexBuffer, IndexBuffer, Program, DrawParameters, Surface};
use glium::vertex::{EmptyInstanceAttributes as EIAttribs};
use glium::glutin::{ElementState, MouseButton, Event, VirtualKeyCode};
// use window::{Window, };
use ui::{self, Vertex, Element, MouseState, KeyboardState, UiRequest, EventRemainder};

const TWOSR3: f32 = 1.15470053838;
const DEFAULT_UI_SCALE: f32 = 0.9;

pub struct Pane<'d, R> where R: EventRemainder {
    vbo: Option<VertexBuffer<Vertex>>,
    ibo: Option<IndexBuffer<u16>>,
    elements: Vec<Element<R>>,
    program: Program,
    params: DrawParameters<'d>,
    display: &'d GlutinFacade,
    scale: f32,
    text_system: TextSystem,
    font_texture: FontTexture,
    mouse_state: MouseState,
    keybd_state: KeyboardState,
    mouse_focused: Option<usize>,
    keybd_focused: Option<usize>,
}

impl<'d, R> Pane<'d, R> where R: EventRemainder {
    pub fn new(display: &'d GlutinFacade) -> Pane<'d, R> {
        let scale = DEFAULT_UI_SCALE;
        let vbo = None;
        let ibo = None;

        // Create program:
        let program = Program::from_source(display, vertex_shader_src, fragment_shader_src, None).unwrap();

        // Draw parameters:
        let params = DrawParameters {
            depth: glium::Depth {
                test: glium::DepthTest::IfLessOrEqual,
                write: true,
                .. Default::default()
            },
            // backface_culling: glium::draw_parameters::BackfaceCullingMode::CullCounterClockwise,
            .. Default::default()
        };

        // Glium text renderer:
        let text_system = TextSystem::new(display);

        // Text font:
        let font_size = 24;
        let font_texture = FontTexture::new(display, &include_bytes!(
                // "/home/nick/projects/vibi/assets/fonts/nanum/NanumBarunGothic.ttf"
                "/home/nick/projects/vibi/assets/fonts/NotoSans/NotoSans-Bold.ttf"
            )[..], font_size).unwrap();

        Pane { 
            vbo: vbo,
            ibo: ibo,
            elements: Vec::new(),
            program: program,
            params: params,
            display: display,
            scale: scale,
            text_system: text_system,
            font_texture: font_texture,
            mouse_state: MouseState::new(),
            keybd_state: KeyboardState::new(),
            mouse_focused: None,
            keybd_focused: None,
        }
    }

    pub fn element(mut self, element: Element<R>) -> Pane<'d, R> {
        if self.vbo.is_some() || self.ibo.is_some() { 
            panic!("Ui::element(): [FIXME]: Cannot (yet) add element after initialization.") 
        }

        self.elements.push(element);
        self
    }

    pub fn init(mut self) -> Pane<'d, R> {
        let mut vertices: Vec<Vertex> = Vec::new();
        let mut indices: Vec<u16> = Vec::new();

        for element in self.elements.iter_mut() {
            element.set_text_width(&self.text_system, &self.font_texture);

            indices.extend_from_slice(&element.indices(vertices.len() as u16));

            // let vertices_idz = vertices.len();

            vertices.extend_from_slice(&element.vertices(
                self.display.get_framebuffer_dimensions(), self.scale,
            ));            
        }

        self.vbo = Some(VertexBuffer::dynamic(self.display, &vertices).unwrap());
        self.ibo = Some(IndexBuffer::new(self.display, glium::index::PrimitiveType::TrianglesList, 
            &indices).unwrap());

        self
    }

    /// Recalculates positions of vertices and updates any other properties such as color.
    // [FIXME]: Make something which doesn't need to rewrite every vertex. 
    //             Perhaps add an optional element index parameter.
    pub fn refresh_vertices(&mut self) {
        match self.vbo {
            Some(ref mut vbo) => {
                let mut vertices: Vec<Vertex> = Vec::with_capacity(vbo.len());

                for element in self.elements.iter_mut() {

                    vertices.extend_from_slice(&element.vertices(
                        self.display.get_framebuffer_dimensions(), self.scale,
                    ));
                }

                vbo.write(&vertices);
            },

            None => panic!("Ui::resize(): Cannot refresh until Ui has been \
                initialized with .init()"),
        }
    }

    pub fn handle_event(&mut self, event: Event) -> R {
        // use glium::glutin::Event::{Closed, Resized, KeyboardInput, MouseInput, MouseMoved};
        // use glium::glutin::ElementState::{Released, Pressed};

        match event {
            Event::Closed => {                    
                // window.close_pending = true;
                // EventRemainder::Closed
                R::closed()
            },
            Event::Resized(..) => {
                self.refresh_vertices();
                // R::default()
                R::default()
            },
            Event::KeyboardInput(key_state, _, vk_code) => {
                // [WINDOW REMOVED]:
                // self.handle_keyboard_input(key_state, vk_code, window)
                let res = self.handle_keyboard_input(key_state, vk_code);
                println!("PANE::HANDLE_EVENT(): returning: {:?}", res);
                res
            },
            Event::MouseInput(state, button) => {
                self.mouse_state.update_button(button, state);

                // [WINDOW REMOVED]:
                // self.handle_mouse_input(state, button, window);
                self.handle_mouse_input(state, button)
            },
            Event::MouseMoved(p) => {
                self.mouse_state.update_position(p);
                // [WINDOW REMOVED]:
                // window.handle_mouse_moved(&self.mouse_state);
                // EventRemainder::MousePosition(p.0, p.1)
                R::mouse_moved(p)
            },
            Event::MouseWheel(delta) => {
                // let _ = touch_phase;
                // [WINDOW REMOVED]:
                // window.handle_mouse_wheel(scroll_delta);
                // EventRemainder::MouseWheel(scroll_delta)
                R::mouse_wheel(delta)
            },
            _ => R::default()
        }
    }
    
    // [WINDOW REMOVED]:
    // fn handle_keyboard_input(&mut self, key_state: ElementState, vk_code: Option<VirtualKeyCode>,
    //             window: &mut Window) 
    // {
    fn handle_keyboard_input(&mut self, key_state: ElementState, vk_code: Option<VirtualKeyCode>) 
            -> R
    {
        // Update keyboard state (modifiers, etc.):
        self.keybd_state.update(key_state, vk_code);

        // Handle any hotkey combinations which may have occurred:
        if self.keybd_state.control {
            // if let ElementState::Pressed = key_state {
            //     if let Some(vkc) = vk_code {        
            //         // use glium::glutin::VirtualKeyCode::*;            
            //         match vkc {
            //             // [WINDOW REMOVED]:
            //             // Q => window.close_pending = true,
            //             VirtualKeyCode::Q => EventRemainder::Closed,
            //             _ => R::default(),
            //         }
            //     } else {
            //         R::default()
            //     }
            // } else {
            //     R::default()
            // }

            // 'Control' is down:
            match key_state {
                ElementState::Pressed => {
                    match vk_code {
                        Some(vkc) => {
                            match vkc {
                                VirtualKeyCode::Q => R::closed(),
                                _ => R::default(),
                            }
                        },
                        None => R::default(),
                    }
                },
                _ => R::default(),
            }
        } else {
            // No modifiers:
            // Pass input to the element that has keyboard focus, if any:
            if let Some(ele_idx) = self.keybd_focused {
                // [WINDOW REMOVED]:
                // self.elements[ele_idx].handle_keyboard_input(key_state, vk_code, &self.keybd_state, window);
                let (request, remainder) = self.elements[ele_idx].handle_keyboard_input(
                    key_state, vk_code, &self.keybd_state);
                println!("PANE::HANDLE_KEYBOARD_INPUT(): returning: {:?}", remainder);
                remainder
            } else {
                R::default()
            }
        }
    }

    fn handle_mouse_input(&mut self, state: ElementState, button: MouseButton) -> R {
        // let mut event_result = R::default();

        match self.mouse_focused {
            Some(ele_idx) => {
                // [WINDOW REMOVED]:
                // match self.elements[ele_idx].handle_mouse_input(state, button, window) {
                let (request, remainder) = self.elements[ele_idx].handle_mouse_input(state, button);

                // If element returns a request we can handle, return
                // `R::default()`. Otherwise, return the original
                // request.
                match request.clone() {
                    UiRequest::KeyboardFocus(on_off) => {
                        if on_off {                             
                            self.keybd_focused = Some(ele_idx);
                            self.elements[ele_idx].set_keybd_focus(true);
                        } else {
                            self.keybd_focused = None;
                            self.elements[ele_idx].set_keybd_focus(false);
                        }

                        self.refresh_vertices();
                        R::default()
                    },
                    UiRequest::Redraw => {
                        self.refresh_vertices();
                        R::default()
                    },
                    _ => remainder,
                }
            },
            None => {
                // Clear keyboard focus:
                self.keybd_focused = match self.keybd_focused {
                    Some(ele_idx) => {
                        self.elements[ele_idx].set_keybd_focus(false);
                        self.refresh_vertices();
                        None
                    },
                    None => None,
                };

                R::default()
            }
        }

        // event_result
        // self.refresh_vertices();

        // println!("    Keyboard Focus: {:?}", self.keybd_focused);
    }

    // [DEPRICATED]
    // fn handle_mouse_scroll(&mut self, scroll_delta: MouseScrollDelta, window: &mut Window) {
    //     match scroll_delta {
    //         MouseScrollDelta::LineDelta(v, h) => window.scroll(v, h),
    //         MouseScrollDelta::PixelDelta(x, y) => println!("vibi: Pixel delta recieved: ({}, {})", x, y),
    //     }
    // }

    pub fn draw<S: Surface>(&mut self, target: &mut S) {
        if self.vbo.is_none() || self.ibo.is_none() { 
            panic!("Ui::draw(): Buffers not initialized.") 
        }

        let model_color = ui::C_ORANGE;

        // Uniforms:
        let uniforms = uniform! {
            u_model_color: model_color,
        };

        // Update mouse focus:
        self.update_mouse_focus(target.get_dimensions());        

        // Draw elements:
        target.draw((self.vbo.as_ref().unwrap(), EIAttribs { len: 1 }), self.ibo.as_ref().unwrap(), 
            &self.program, &uniforms, &self.params).unwrap();

        // Draw element text:
        for element in self.elements.iter() {
            element.draw_text(&self.text_system, target, &self.font_texture);

            let text_display = TextDisplay::new(&self.text_system, &self.font_texture, 
                element.get_text());

            glium_text::draw(&text_display, &self.text_system, target, 
                element.text_matrix(), element.text().get_color());
        }
    }

    pub fn update_mouse_focus(&mut self, surface_dims: (u32, u32)) {
        // Update elements:
        if !self.mouse_state.is_stale() {
            // Determine which element has mouse focus (by index):
            let newly_focused = self.focused_element_idx(surface_dims);

            if newly_focused != self.mouse_focused {
                // No longer focused.
                if let Some(idx) = self.mouse_focused {
                    self.elements[idx].set_mouse_focus(false);
                }

                // Newly focused.
                if let Some(idx) = newly_focused {
                    self.elements[idx].set_mouse_focus(true);
                }

                self.mouse_focused = newly_focused;

                // [FIXME]: Make something which doesn't need to rewrite every vertex.
                self.refresh_vertices();
            }
        }
    }

    fn focused_element_idx(&mut self, surface_dims: (u32, u32)) -> Option<usize> {
        let mut idx = 0;

        for element in self.elements.iter_mut() {
            if element.has_mouse_focus(self.mouse_state.surface_position(surface_dims)) {
                // println!("Element [{}] has focus.", idx);
                return Some(idx);
            }

            idx += 1;
        } 

        None
    }

    pub fn mouse_state(&self) -> &MouseState {
        &self.mouse_state
    }

    pub fn set_input_stale(&mut self) {
        self.mouse_state.set_stale();
    }

    pub fn input_is_stale(&self) -> bool {
        self.mouse_state.is_stale()
    }  
}



// Vertex Shader:
#[allow(non_upper_case_globals)]
static vertex_shader_src: &'static str = r#"
    #version 330

    in vec3 position;
    in vec4 color;
    in vec2 xy_normal;

    out vec4 v_color;

    void main() {
        gl_Position = vec4(position, 1.0);

        v_color = color;
    };
"#;
        

// Fragment Shader:
#[allow(non_upper_case_globals)]
static fragment_shader_src: &'static str = r#"
    #version 330

    in vec4 v_color;

    out vec4 color;

    void main() {
        color = v_color;
    };
"#;
