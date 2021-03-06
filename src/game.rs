#[allow(unused_imports)]
use glium::{glutin, Surface};
use std::f32::consts::PI;

use na::vector;
//use na::{Rotation3, Vector4};

use std::time::{Duration, SystemTime};

//use glium::texture::integral_texture3d::IntegralTexture3d;

use crate::world::{Space, WORLD_DIM};

use std::fs::File;


pub struct Game {
    pub begin_time: SystemTime,
    pub time_elapsed: Duration,
    pub keyboard: KeyboardState,
    pub mouse_delta: (f32, f32),
    pub camera: Camera,
    pub world: Space,
    //pub world_uniforms: Dense
    pub walk_speed: f32,
}

impl Game {
    const STRAFE_FACTOR: f32 = 0.5;
    const MOUSE_SPEED: f32 = 1.0;

    pub fn new() -> Game {
        let mut file = File::open("res/levels/test.gox").unwrap();
        Game {
            begin_time: SystemTime::now(),
            time_elapsed: Duration::from_millis(0),
            camera: Camera {
                pos: vector![1.0+(WORLD_DIM.0 / 16) as f32, 1.0+(WORLD_DIM.1 / 16) as f32, 1.0+(WORLD_DIM.2 / 16) as f32],
                yaw: 0.0,
                pitch: 0.0,
                dir: vector![0.0, 0.0, 1.0],
                forward: vector![0.0, 0.0, 1.0],
                right: vector![1.0, 0.0, 0.0],
            },
            keyboard: KeyboardState {
                ..Default::default()
            },
            mouse_delta: (0.0, 0.0),
            world: Space::from_gox(WORLD_DIM, &mut file),
            walk_speed: 1.0,
        }
    }

    // TODO: Make a better keyboard input handler (onKeyPressed, onKeyReleased, onKeyHold, etc...)
    pub fn tick(&mut self) {
        self.time_elapsed = SystemTime::now().duration_since(self.begin_time).unwrap();
        self.camera.set_rotation(
            self.camera.yaw + (self.mouse_delta.0 as f32) * 1e-3 * Game::MOUSE_SPEED,
            self.camera.pitch - (self.mouse_delta.1 as f32) * 1e-3 * Game::MOUSE_SPEED,
        );

        if is_key_pressed(self.keyboard.front) {
            self.camera.pos += self.walk_speed * self.camera.dir;
        } else if is_key_pressed(self.keyboard.back) {
            self.camera.pos -= self.walk_speed * self.camera.dir;
        }

        if is_key_pressed(self.keyboard.right) {
            self.camera.pos += self.walk_speed * self.camera.right * Game::STRAFE_FACTOR;
        } else if is_key_pressed(self.keyboard.left) {
            self.camera.pos -= self.walk_speed * self.camera.right * Game::STRAFE_FACTOR;
        }

        if is_key_pressed(self.keyboard.walk) {
            self.walk_speed = 0.5;
        } else {
            self.walk_speed = 1.0;
        }

    }

    // callback for mouse input event
    pub fn mouse_input(&mut self, delta: (f32, f32)) {
        self.mouse_delta = delta;
    }

    // callback for keyboard input event
    pub fn keyboard_input(&mut self, input: glutin::event::KeyboardInput) {
        let key = match input.virtual_keycode {
            Some(key) => key,
            None => return,
        };
        update_key_state(&mut self.keyboard, key, input.state);
    }
}

pub struct Camera {
    pub pos: na::Vector3<f32>,

    // Forward direction is -z
    //
    //               +
    //   - yaw +   pitch
    //               -
    //

    pub yaw: f32,
    pub pitch: f32,
    pub dir: na::Vector3<f32>,
    pub forward: na::Vector3<f32>,
    pub right: na::Vector3<f32>,
}

impl Camera {
    fn set_rotation(&mut self, yaw: f32, pitch: f32) {
        self.yaw = yaw % (2.0 * PI);
        self.pitch = pitch.clamp(-89.9 * (PI / 180.0), 89.9 * (PI / 180.0));
        self.forward = vector![self.yaw.sin(), 0.0, self.yaw.cos()];
        self.dir = self.pitch.cos() * self.forward + self.pitch.sin() * vector![0.0, 1.0, 0.0];
        self.right = vector![self.yaw.cos(), 0.0, -self.yaw.sin()];
    }
}

pub struct KeyboardState {
    pub front: glutin::event::ElementState,
    pub back: glutin::event::ElementState,
    pub left: glutin::event::ElementState,
    pub right: glutin::event::ElementState,
    pub walk: glutin::event::ElementState,
}

impl Default for KeyboardState {
    fn default() -> KeyboardState {
        KeyboardState {
            front: glutin::event::ElementState::Released,
            back: glutin::event::ElementState::Released,
            left: glutin::event::ElementState::Released,
            right: glutin::event::ElementState::Released,
            walk: glutin::event::ElementState::Released,
        }
    }
}

pub fn is_key_pressed(state: glutin::event::ElementState) -> bool {
    return state == glutin::event::ElementState::Pressed;
}

pub fn update_key_state(
    kb_state: &mut KeyboardState,
    keycode: glutin::event::VirtualKeyCode,
    key_state: glutin::event::ElementState,
) {
    match keycode {
        // W
        glutin::event::VirtualKeyCode::W => kb_state.front = key_state,

        // A
        glutin::event::VirtualKeyCode::A => kb_state.left = key_state,

        // S
        glutin::event::VirtualKeyCode::S => kb_state.back = key_state,

        // D
        glutin::event::VirtualKeyCode::D => kb_state.right = key_state,

        // LShift
        glutin::event::VirtualKeyCode::LShift => kb_state.walk = key_state,

        // Other
        _ => (),
    }
}
