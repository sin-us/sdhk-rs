extern crate glfw;
extern crate gl;
extern crate cgmath;

use camera::Camera;
use std::sync::mpsc::Receiver;
use std::marker::PhantomData;

use glfw::{ Glfw, Window, WindowEvent, WindowMode };
use glfw::{Context, Key, Action};

pub trait Game {
    fn camera(&self) -> &Camera;
    fn process_event(&mut self, event: WindowEvent);
    fn update(&mut self, time: f64);
    fn render(&self);
}

pub struct GameWindow<G: Game> {
    window: Window,
    events: Receiver<(f64, WindowEvent)>,
    glfw: Glfw,
    phantom_data: PhantomData<G>
}

impl <G> GameWindow<G> where G: Game {
    pub fn create(width: u32, height: u32, title: &str, mode: WindowMode) -> GameWindow<G> {
        let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
        glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
        glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));
        glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));

        let (mut window, events) = glfw.create_window(width, height, title, mode)
            .expect("Failed to create GLFW window");

        window.make_current();
        window.set_key_polling(true);
        window.set_mouse_button_polling(true);
        window.set_cursor_pos_polling(true);
        window.set_cursor_enter_polling(true);
        window.set_scroll_polling(true);
        window.set_framebuffer_size_polling(true);

        // gl: load all OpenGL function pointers
        gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

        GameWindow {
            window: window,
            events: events,
            glfw: glfw,
            phantom_data: PhantomData
        }
    }

    pub fn render(&mut self, game: &mut G) {
        unsafe {
            gl::Enable(gl::DEPTH_TEST);
            gl::Enable(gl::BLEND);
            gl::Enable(gl::CULL_FACE);

            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
        }

        while !self.window.should_close() {
            game.update(self.glfw.get_time());
            self.process_events(game);

            unsafe {
                gl::ClearColor(0.0, 0.0, 0.01, 1.0);
                gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

                game.render();
            }

            self.window.swap_buffers();
            self.glfw.poll_events();
        }
    }

    pub fn process_events(&mut self, game: &mut G) {
        for (_, event) in glfw::flush_messages(&self.events) {
            match event {
                glfw::WindowEvent::FramebufferSize(width, height) => unsafe { gl::Viewport(0, 0, width, height) },
                glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => self.window.set_should_close(true),
                _ => game.process_event(event)
            }
        }
    }
}