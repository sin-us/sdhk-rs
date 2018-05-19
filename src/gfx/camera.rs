extern crate cgmath;
use cgmath::prelude::*;
use cgmath::{ Vector3, Point3, Matrix4, Deg, Quaternion };

#[allow(dead_code)]
pub enum CameraDirection {
	Up,
    Down,
    Left,
    Right,
    Forward,
    Back
}

#[allow(dead_code)]
#[derive(PartialEq)]
pub enum CameraType {
    Free,
    Orbit { step: f32 }
}

pub struct Camera {
    camera_type: CameraType,

    viewport_x: u32,
    viewport_y: u32,

    window_width: u32,
    window_height: u32,

    aspect: f32,
    field_of_view: f32,
    near_clip: f32,
    far_clip: f32,

    camera_scale: f32,
    camera_yaw: f32,
    camera_pitch: f32,

    max_pitch_rate: f32,
    max_yaw_rate: f32,
    move_camera: bool,

    camera_position: Vector3<f32>,
    camera_position_delta: Vector3<f32>,
    camera_look_at: Vector3<f32>,
    camera_forward: Vector3<f32>,

    camera_up: Vector3<f32>,
    mouse_position: Vector3<f32>,

    projection: Matrix4<f32>,
    view: Matrix4<f32>,
    model: Matrix4<f32>,
    mvp: Matrix4<f32>,
}

impl Camera {

    pub fn new() -> Camera {
        Camera {
            camera_type: CameraType::Free,

            camera_up: Vector3::new(0.0, 1.0, 0.0),
            field_of_view: 45.0,
            camera_position_delta: Vector3::new(0.0, 0.0, 0.0),
            camera_scale: 0.5,
            max_pitch_rate: 5.0,
            max_yaw_rate: 5.0,
            move_camera: false,

            viewport_x: 0,
            viewport_y: 0,

            window_width: 0,
            window_height: 0,

            aspect: 0.0,
            near_clip: 0.1,
            far_clip: 1000.0,

            camera_yaw: 0.0,
            camera_pitch: 0.0,

            camera_position: Vector3::new(0.0, 0.0, 0.0),
            camera_look_at: Vector3::new(0.0, 0.0, 0.0),
            camera_forward:  Vector3::new(0.0, 0.0, 0.0),

            mouse_position:  Vector3::new(0.0, 0.0, 0.0),

            projection: Matrix4::zero(),
            view: Matrix4::zero(),
            model: Matrix4::zero(),
            mvp: Matrix4::zero(),
        }
    }

    pub fn set_type(&mut self, camera_type: CameraType) {
        self.camera_type = camera_type;
    }

    pub fn reset(&mut self) {
        self.camera_up = Vector3::new(0.0, 1.0, 0.0);
    }

    pub fn update(&mut self) {
        match self.camera_type {
            CameraType::Free => {
                self.camera_forward = (self.camera_look_at - self.camera_position).normalize();

                let camera_right = self.camera_forward.cross(self.camera_up);

                let pitch_quat = Quaternion::from_axis_angle(camera_right, Deg(self.camera_pitch));
                let yaw_quat = Quaternion::from_axis_angle(self.camera_up, Deg(self.camera_yaw));
                
                let rotation = (pitch_quat * yaw_quat).normalize();

                self.camera_forward = rotation.rotate_vector(self.camera_forward);
                self.camera_position += self.camera_position_delta;

                self.camera_look_at = self.camera_position + self.camera_forward * 1.0;
                
                //damping for smooth camera
                self.camera_yaw *= 0.5;
                self.camera_pitch *= 0.5;
                self.camera_position_delta = self.camera_position_delta * 0.8;
                
                //compute the MVP
                self.view = Matrix4::look_at(Point3::from_vec(self.camera_position), Point3::from_vec(self.camera_look_at), self.camera_up);
                self.model = Matrix4::from_value(1.0);
                self.mvp = self.projection * self.view * self.model;
            },
            CameraType::Orbit { step: _ } => {
                self.camera_forward = (self.camera_look_at - self.camera_position).normalize();

                let camera_right = self.camera_forward.cross(self.camera_up);

                let rotation_x = Quaternion::from_axis_angle(camera_right, Deg(self.camera_position_delta.x));
                let rotation_y = Quaternion::from_axis_angle(self.camera_up, Deg(self.camera_position_delta.y));
                let rotation = rotation_x * rotation_y;
                self.camera_position = rotation.rotate_vector(self.camera_position);
                self.camera_position.z += self.camera_position_delta.z;
                self.camera_forward = rotation.rotate_vector(self.camera_forward);
                self.camera_up = rotation.rotate_vector(self.camera_up);

                //damping for smooth camera
                self.camera_yaw *= 0.5;
                self.camera_pitch *= 0.5;
                self.camera_position_delta = self.camera_position_delta * 0.8;
                
                //compute the MVP
                self.view = Matrix4::look_at(Point3::from_vec(self.camera_position), Point3::from_vec(self.camera_look_at), self.camera_up);
                self.model = Matrix4::from_value(1.0);
                self.mvp = self.projection * self.view * self.model;
            }
        }
        
    }

    pub fn set_position(&mut self, pos: Vector3<f32>) {
        self.camera_position = pos;
    }

    pub fn set_look_at(&mut self, pos: Vector3<f32>) {
        self.camera_look_at = pos;
    }

    pub fn set_fov(&mut self, fov: f32) {
        self.field_of_view = fov;
        self.projection = cgmath::perspective(Deg(self.field_of_view), self.aspect, self.near_clip, self.far_clip);
    }

    pub fn set_viewport(&mut self, loc_x: u32, loc_y: u32, width: u32, height: u32) {
        self.viewport_x = loc_x;
        self.viewport_y = loc_y;
        self.window_width = width;
        self.window_height = height;
        //need to use doubles division here, it will not work otherwise and it is possible to get a zero aspect ratio with integer rounding
        self.aspect = width as f32 / height as f32;
        self.projection = cgmath::perspective(Deg(self.field_of_view), self.aspect, self.near_clip, self.far_clip);
    }

    pub fn set_clipping(&mut self, near_clip_distance: f32, far_clip_distance: f32) {
        self.near_clip = near_clip_distance;
        self.far_clip = far_clip_distance;
    }

    pub fn move_camera(&mut self, dir: CameraDirection) {
        match self.camera_type {
            CameraType::Free => {
                match dir {
                    CameraDirection::Up => self.camera_position_delta += self.camera_up * self.camera_scale,
                    CameraDirection::Down => self.camera_position_delta -= self.camera_up * self.camera_scale,

                    CameraDirection::Left => { self.camera_position_delta -= self.camera_forward.cross(self.camera_up) * self.camera_scale; },
                    CameraDirection::Right => self.camera_position_delta += self.camera_forward.cross(self.camera_up) * self.camera_scale,
                    
                    CameraDirection::Forward => self.camera_position_delta += self.camera_forward * self.camera_scale,
                    CameraDirection::Back => self.camera_position_delta -= self.camera_forward * self.camera_scale
                }
            },
            CameraType::Orbit { step } => {
                match dir {
                    CameraDirection::Up => { self.camera_position_delta -= Vector3::new(step, 0.0, 0.0); },
                    CameraDirection::Down => { self.camera_position_delta += Vector3::new(step, 0.0, 0.0); },

                    CameraDirection::Left => { self.camera_position_delta -= Vector3::new(0.0, step, 0.0); },
                    CameraDirection::Right => { self.camera_position_delta += Vector3::new(0.0, step, 0.0); },
                    
                    CameraDirection::Forward => { self.camera_position_delta += Vector3::new(0.0, 0.0, step); },
                    CameraDirection::Back => { self.camera_position_delta -= Vector3::new(0.0, 0.0, step); },
                }
            }
        }
        
    }

    pub fn change_pitch(&mut self, degrees: f32) {
        let mut degrees = degrees;
        //Check bounds with the max pitch rate so that we aren't moving too fast
        if degrees < -self.max_pitch_rate {
            degrees = -self.max_pitch_rate;
        } else if degrees > self.max_pitch_rate {
            degrees = self.max_pitch_rate;
        }
        self.camera_pitch += degrees;

        //Check bounds for the camera pitch
        if self.camera_pitch > 360.0 {
            self.camera_pitch -= 360.0;
        } else if self.camera_pitch < -360.0 {
            self.camera_pitch += 360.0;
        }
    }

    pub fn change_yaw(&mut self, degrees: f32) {
        let mut degrees = degrees;
        //Check bounds with the max heading rate so that we aren't moving too fast
        if degrees < -self.max_yaw_rate {
            degrees = -self.max_yaw_rate;
        } else if degrees > self.max_yaw_rate {
            degrees = self.max_yaw_rate;
        }

        //This controls how the heading is changed if the camera is pointed straight up or down
        //The heading delta direction changes
        if self.camera_pitch > 90.0 && self.camera_pitch < 270.0 || (self.camera_pitch < -90.0 && self.camera_pitch > -270.0) {
            self.camera_yaw -= degrees;
        } else {
            self.camera_yaw += degrees;
        }

        //Check bounds for the camera heading
        if self.camera_yaw > 360.0 {
            self.camera_yaw -= 360.0;
        } else if self.camera_yaw < -360.0 {
            self.camera_yaw += 360.0;
        }
    }

    pub fn move_2d(&mut self, x: u32, y: u32) {
        //compute the mouse delta from the previous mouse position
        let mouse_delta = self.mouse_position - Vector3::new(x as f32, y as f32, 0.0);
        //if the camera is moving, meaning that the mouse was clicked and dragged, change the pitch and heading
        if self.move_camera {
            self.change_yaw(0.08 * mouse_delta.x);
            self.change_pitch(0.08 * mouse_delta.y);
        }
        self.mouse_position = Vector3::new(x as f32, y as f32, 0.0);
    }

    pub fn get_pvm(&self) -> (Matrix4<f32>, Matrix4<f32>, Matrix4<f32>) {
        (self.projection, self.view, self.model)
    }

    pub fn create(pos: Vector3<f32>, front: Vector3<f32>, up: Vector3<f32>) -> Camera {
        let mut camera = Camera::new();
        camera.camera_position = pos;
        camera.camera_look_at = front;
        camera.camera_up = up;

        camera
    }

    pub fn create_default() -> Camera {
         Camera::create(Vector3::new(0.0, 0.0, -1.0), Vector3::new(0.0, 0.0, 0.0), Vector3::new(0.0, 1.0, 0.0))
    }
}