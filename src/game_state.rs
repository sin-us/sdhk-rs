extern crate glfw;
extern crate cgmath;

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
            WindowEvent::Key(key, _, Action::Press, _) | WindowEvent::Key(key, _, Action::Repeat, _) => {
                for rt in self.meshes.iter_mut() {
                    rt.process_key_pressed(key);
                };

                match key {
                    Key::W => self.camera.move_camera(CameraDirection::Forward), 
                    Key::S => self.camera.move_camera(CameraDirection::Back),
                    Key::A => { self.camera.move_camera(CameraDirection::Left);  },
                    Key::D => self.camera.move_camera(CameraDirection::Right),
                    Key::Left => self.camera.change_yaw(1.0),
                    Key::Right => self.camera.change_yaw(-1.0),
                    Key::Up => self.camera.change_pitch(1.0),
                    Key::Down => self.camera.change_pitch(-1.0),
                    Key::Space => (self.space_callback)(),
                    _ => {}
                }
            },
            WindowEvent::Scroll(_x,y) => self.camera.move_camera(if y > 0.0 { CameraDirection::Forward } else { CameraDirection::Back }),
            WindowEvent::CursorPos(x, y) => {
                println!("{} {}", x, y);
            },
            _ => {}
        }
    }
}
