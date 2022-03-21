mod game;
mod gfx;
mod march;
mod types;
mod uniforms;
mod world;
mod world_loader;

extern crate derive_more;

#[macro_use]
extern crate glium;
extern crate nalgebra as na;

use glium::{glutin, index::PrimitiveType};

use notify::{watcher, RecursiveMode, Watcher};
use std::sync::mpsc::channel;
use std::thread;
use std::time::{Duration, SystemTime};

use game::Game;
use gfx::{DenseCartesianRenderer, DenseCartesianUniforms};
use uniforms::AsGPUResource;

fn main() {
    // Initialize watcher which monitors the shader files
    let (sender, receiver) = channel();
    let mut watcher = watcher(sender, Duration::ZERO).unwrap();
    watcher
        .watch(gfx::SHADER_PATH_NAME, RecursiveMode::Recursive)
        .unwrap();

    // Initialize event loop with shader file monitor
    let event_loop = glutin::event_loop::EventLoop::with_user_event();
    let fs_event_proxy: glutin::event_loop::EventLoopProxy<notify::DebouncedEvent> =
        event_loop.create_proxy();
    thread::spawn(move || loop {
        if let Ok(event) = receiver.recv() {
            fs_event_proxy
                .send_event(event)
                .expect("Notify failed for some reason");
        }
    });

    // Initialize display
    let wb = glutin::window::WindowBuilder::new();
    let cb = glutin::ContextBuilder::new().with_vsync(true);
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();

    // Initialize vertex buffer for triangle which covers the screen
    // This triangle is passed through the fragment shader, where ray marching is performed on each
    // pixel
    let vertex_buffer = {
        glium::VertexBuffer::new(
            &display,
            &[
                gfx::attrib::Vertex {
                    position: [-3.0, -1.0],
                    color: [0.0, 1.0, 0.0],
                },
                gfx::attrib::Vertex {
                    position: [3.0, -1.0],
                    color: [0.0, 0.0, 1.0],
                },
                gfx::attrib::Vertex {
                    position: [0.0, 4.0],
                    color: [1.0, 0.0, 0.0],
                },
            ],
        )
        .unwrap()
    };
    let index_buffer =
        glium::IndexBuffer::new(&display, PrimitiveType::TrianglesList, &[0, 1, 2]).unwrap();

    let mut program = gfx::load_shader(&display, "shader").unwrap();
    let mut game = Game::new();
    let mut renderer = gfx::DenseCartesianRenderer {
        uniforms: DenseCartesianUniforms {
            sdf: game.world.sdf.as_gpu_resource(&display),
            voxels: game.world.voxels.as_gpu_resource(&display),
        },
    };
    let mut window_focused = false;

    // TODO: Add FPS counter
    // the main loop
    event_loop.run(move |event, _, control_flow| {
        let old_time = game.begin_time + game.time_elapsed;
        let windowed_context = display.gl_window();
        let window = windowed_context.window();

        match event {
            // Window event
            glutin::event::Event::WindowEvent { event, .. } => match event {
                // Window closed
                glutin::event::WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                    return;
                }

                // Window resized
                glutin::event::WindowEvent::Resized { .. } => {
                    renderer.draw(&display, &vertex_buffer, &index_buffer, &program, &game);
                    return;
                }

                // Keyboard input
                glutin::event::WindowEvent::KeyboardInput { input, .. } => {
                    game.keyboard_input(input);
                }

                // Window focused
                glutin::event::WindowEvent::Focused(focused) => {
                    window_focused = focused;
                    window.set_cursor_visible(!focused);
                    window
                        .set_cursor_grab(focused)
                        .expect("Could not grab cursor");
                }

                _ => return,
            },

            // Mouse input
            glutin::event::Event::DeviceEvent {
                device_id: _,
                event,
            } => match event {
                glutin::event::DeviceEvent::MouseMotion { delta } => {
                    if window_focused {
                        game.mouse_input((delta.0 as f32, delta.1 as f32));
                    }
                }
                _ => (),
            },

            // Shader modified, reload
            glutin::event::Event::UserEvent(notify::DebouncedEvent::Write(_)) => {
                match gfx::load_shader(&display, "shader") {
                    Ok(new_program) => {
                        println!("Successfully loaded new shader.");
                        program = new_program;
                    }

                    Err(err) => {
                        eprintln!("\nError loading new shader: {}", err);
                    }
                }
                return;
            }

            glutin::event::Event::MainEventsCleared => {
                game.tick();
                renderer.draw(&display, &vertex_buffer, &index_buffer, &program, &game);
                let time_delta = SystemTime::now().duration_since(old_time).unwrap();
                //println!("Time Delta: {:?}", time_delta);
                game.mouse_delta = (0.0, 0.0);
                *control_flow = glutin::event_loop::ControlFlow::Poll
            }

            _ => return,
        };
    });
}
