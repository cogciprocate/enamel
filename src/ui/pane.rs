
use glium_text_rusttype::{self, TextSystem, FontTexture, TextDisplay};
use glium::{self, VertexBuffer, IndexBuffer, Program, DrawParameters, Surface};
use glium::backend::glutin::Display;
use glium::vertex::{EmptyInstanceAttributes as EIAttribs};
use glium::glutin::{ElementState, MouseButton, Event, WindowEvent, VirtualKeyCode};
use ui::{self, Vertex, Element, MouseState, KeyboardState, UiRequest, EventRemainder};

// const TWOSR3: f32 = 1.15470053838;
const DEFAULT_UI_SCALE: f32 = 0.9;

pub struct Pane<'d, R> where R: EventRemainder {
    vbo: Option<VertexBuffer<Vertex>>,
    ibo: Option<IndexBuffer<u16>>,
    elements: Vec<Element<R>>,
    program: Program,
    params: DrawParameters<'d>,
    display: &'d Display,
    scale: f32,
    text_system: TextSystem,
    font_texture: FontTexture,
    mouse_state: MouseState,
    keybd_state: KeyboardState,
    mouse_focused: Option<usize>,
    keybd_focused: Option<usize>,
    surface_dims: (u32, u32),
}

impl<'d, R> Pane<'d, R> where R: EventRemainder {
    pub fn new(display: &'d Display) -> Pane<'d, R> {
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
        let font_size = 36;
        let font_texture = FontTexture::new(display, &include_bytes!(
                // "/home/nick/projects/vibi/assets/fonts/nanum/NanumBarunGothic.ttf"
                "assets/fonts/NotoSans/NotoSans-Bold.ttf"
            )[..], font_size, FontTexture::ascii_character_list()).unwrap();

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
            surface_dims: display.get_framebuffer_dimensions(),
        }
    }

    pub fn element(mut self, element: Element<R>) -> Pane<'d, R> {
        if self.vbo.is_some() || self.ibo.is_some() {
            panic!("Ui::element(): [FIXME]: Cannot [yet] add element after initialization.")
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

            vertices.extend_from_slice(&element.vertices(
                self.display.get_framebuffer_dimensions(), self.scale,
            ));
        }

        self.vbo = Some(VertexBuffer::dynamic(self.display, &vertices).unwrap());
        self.ibo = Some(IndexBuffer::new(self.display, glium::index::PrimitiveType::TrianglesList,
            &indices).unwrap());

        self
    }

    pub fn draw<S>(&mut self, target: &mut S)
            where S: Surface
    {
        if self.vbo.is_none() || self.ibo.is_none() {
            panic!("Ui::draw(): Buffers not initialized.")
        }

        let model_color = ui::C_ORANGE;

        // Uniforms:
        let uniforms = uniform! {
            u_model_color: model_color,
        };

        self.surface_dims = target.get_dimensions();

        // Update mouse focus:
        self.update_mouse_focus();

        // Draw elements:
        target.draw((self.vbo.as_ref().unwrap(), EIAttribs { len: 1 }), self.ibo.as_ref().unwrap(),
            &self.program, &uniforms, &self.params).unwrap();

        // Draw element text:
        for element in self.elements.iter() {
            element.draw_text(&self.text_system, target, &self.font_texture);

            let text_display = TextDisplay::new(&self.text_system, &self.font_texture,
                element.get_text());

            glium_text_rusttype::draw(&text_display, &self.text_system, target,
                element.text_matrix(), element.text().get_color()).unwrap();
        }
    }

    pub fn handle_event(&mut self, event: Event) -> R {
        match event.clone() {
            Event::WindowEvent { window_id: _, event: win_event } => {
                match win_event {
                WindowEvent::Resized(..) => {
                    self.refresh_vertices();
                    R::event(event)
                },
                WindowEvent::KeyboardInput { device_id: _, input } => {
                    self.handle_keyboard_input(input.state, input.virtual_keycode, event)
                },
                WindowEvent::MouseInput { device_id: _, state, button, modifiers: _ } => {
                    self.mouse_state.set_button(button, state);
                    self.update_mouse_focus();
                    self.handle_mouse_input(state, button, event)
                },
                WindowEvent::CursorMoved { device_id: _, position, modifiers: _ } => {
                    self.mouse_state.update_position(position);
                    R::event(event)
                },
                WindowEvent::MouseWheel { device_id: _, delta: _, phase: _, modifiers: _ } => {
                    R::event(event)
                },
                _ => R::event(event),
                }
            }
            _ => R::event(event),
        }
    }

    fn handle_keyboard_input(&mut self, key_state: ElementState, vk_code: Option<VirtualKeyCode>,
                event: Event) -> R
    {
        // Update keyboard state (modifiers, etc.):
        self.keybd_state.update(key_state, vk_code);

        // Handle any hotkey combinations which may have occurred:
        if self.keybd_state.control {
            // 'Control' is down:
            match key_state {
                ElementState::Pressed => { match vk_code {
                    Some(vkc) => { match vkc {
                        VirtualKeyCode::Q => {
                            // R::event(Event::WindowEvent { window_id: 0,
                            //     event: WindowEvent::Closed })
                            R::event(event)
                        },
                        _ => R::event(event),
                    } },
                    None => R::event(event),
                } },
                _ => R::event(event),
            }
        } else {
            // No modifiers:
            // Pass input to the element that has keyboard focus, if any:
            if let Some(ele_idx) = self.keybd_focused {
                let (_request, remainder) = self.elements[ele_idx].handle_keyboard_input(
                    key_state, vk_code, &self.keybd_state, event);
                remainder
            } else {
                R::event(event)
            }
        }
    }

    fn handle_mouse_input(&mut self, state: ElementState, button: MouseButton, event: Event) -> R {
        // Determine if any elements currently have mouse focus and will be
        // handling the input event, if not, send up to the consumer.
        match self.mouse_focused {
            Some(ele_idx) => {
                let (request, remainder) = self.elements[ele_idx]
                    .handle_mouse_input(state, button, event);

                match request {
                    UiRequest::KeyboardFocus(on) => {
                        if on {
                            self.keybd_focused = Some(ele_idx);
                            self.elements[ele_idx].set_keybd_focus(true);
                        } else {
                            self.keybd_focused = None;
                            self.elements[ele_idx].set_keybd_focus(false);
                        }
                    },
                    UiRequest::Refresh => (),
                    _ => (),
                }
                self.refresh_vertices();
                remainder
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

                // Send the unhandled input event back up to the consumer:
                R::event(event)
            }
        }
    }

    pub fn update_mouse_focus(&mut self) {
        if self.mouse_state.any_pressed() { return; }

        // Update elements if the mouse has moved since last time
        if !self.mouse_state.is_stale() {
            // Determine which element has mouse focus (by index):
            let newly_focused = self.focused_element_idx();

            // If something new now has focus:
            if newly_focused != self.mouse_focused {
                // Tell previously focused element the bad news:
                match self.mouse_focused {
                    Some(idx) => self.elements[idx].set_mouse_focus(false),
                    None => /*background.set_mouse_focus(false)*/ (),
                }

                // Notify the newly focused that it is now in the spotlight:
                match newly_focused {
                    Some(idx) => self.elements[idx].set_mouse_focus(true),
                    None => /*background.set_mouse_focus(true)*/ (),
                }

                // Update for comparison next time:
                self.mouse_focused = newly_focused;

                // [FIXME]: Make something that doesn't need to rewrite every vertex.
                self.refresh_vertices();
            }
        }
    }

    fn focused_element_idx(&mut self) -> Option<usize> {
        let mut idx = 0;

        for element in self.elements.iter_mut() {
            if element.has_mouse_focus(self.mouse_state.surface_position(self.surface_dims)) {
                // println!("Element [{}] has focus.", idx);
                return Some(idx);
            }
            idx += 1;
        }
        None
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
    }

"#;


// Fragment Shader:
#[allow(non_upper_case_globals)]
static fragment_shader_src: &'static str = r#"
    #version 330

    in vec4 v_color;

    out vec4 color;

    void main() {
        color = v_color;
    }

"#;
