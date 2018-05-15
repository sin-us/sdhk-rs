extern crate glfw;
extern crate cgmath;

use gfx::vertex::Vertex;
use gfx::render_target::RenderTarget;
use gfx::camera::{Camera, CameraDirection};
use gfx::game_window::Game;

use glfw::{Key, Action, WindowEvent};

pub struct GameState<'a> {
    pub camera: Camera,
    pub meshes: Vec<Box<RenderTarget + 'a>>,
    space_callback: Box<Fn() + 'a>,
    last_frame: f64,
    delta_time: f64
}

impl<'a> GameState<'a> {
    pub fn new() -> GameState<'a> {
        GameState {
            camera: Camera::create_default(),
            meshes: Vec::new(),
            space_callback: Box::new(|| {}),
            last_frame: 0.0,
            delta_time: 0.0
        }
    }

    pub fn add_mesh(&mut self, mesh: Box<RenderTarget + 'a>) {
        self.meshes.push(mesh)
    }

    pub fn register_callback<CB: 'a + Fn()>(&mut self, callback: CB) {
        self.space_callback = Box::new(callback);
    }
}

impl<'a> Game for GameState<'a> {
    fn camera(&self) -> &Camera {
        &self.camera
    }

    fn update(&mut self, time: f64) {
        self.delta_time = time - self.last_frame;
        self.last_frame = time;

        self.camera.update();

        for rt in self.meshes.iter_mut() {
            rt.update(&self.camera, time as f32);
        }
    }

    fn render(&self) {
        for rt in self.meshes.iter() {
            rt.render();
        }
    }

    fn process_event(&mut self, event: WindowEvent) {
        println!("{:?}", event);

        match event {
            WindowEvent::Key(_, _, Action::Press, _) | WindowEvent::Key(_, _, Action::Repeat, _) => {
                match event {
                    WindowEvent::Key(Key::W, _, _, _) => self.camera.move_camera(CameraDirection::Forward), 
                    WindowEvent::Key(Key::S, _, _, _) => self.camera.move_camera(CameraDirection::Back),
                    WindowEvent::Key(Key::A, _, _, _) => { self.camera.move_camera(CameraDirection::Left);  },
                    WindowEvent::Key(Key::D, _, _, _) => self.camera.move_camera(CameraDirection::Right),
                    WindowEvent::Key(Key::Left, _, _, _) => self.camera.change_yaw(1.0),
                    WindowEvent::Key(Key::Right, _, _, _) => self.camera.change_yaw(-1.0),
                    WindowEvent::Key(Key::Up, _, _, _) => self.camera.change_pitch(1.0),
                    WindowEvent::Key(Key::Down, _, _, _) => self.camera.change_pitch(-1.0),
                    WindowEvent::Key(Key::Space, _, _, _) => (self.space_callback)(),
                    _ => {}
                }
            },
            WindowEvent::Scroll(x,y) => self.camera.move_camera(if y > 0.0 { CameraDirection::Forward } else { CameraDirection::Back }),
            WindowEvent::CursorPos(x, y) => {
                println!("{} {}", x, y);
            },
            _ => {}
        }
    }
}
