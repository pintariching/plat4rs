use glam::Vec2;
use wgpu::{SurfaceError, VertexBufferLayout};
use winit::dpi::PhysicalSize;
use winit::event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;

mod camera;
mod game_state;
mod instance;
mod model;
mod resources;
mod state;
mod texture;

use state::State;

pub trait Vertex {
    fn desc<'a>() -> VertexBufferLayout<'a>;
}

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn from_virtual_keycode(keycode: &VirtualKeyCode) -> Option<Self> {
        match keycode {
            VirtualKeyCode::W | VirtualKeyCode::Up => Some(Self::Up),
            VirtualKeyCode::S | VirtualKeyCode::Down => Some(Self::Down),
            VirtualKeyCode::A | VirtualKeyCode::Left => Some(Self::Left),
            VirtualKeyCode::D | VirtualKeyCode::Right => Some(Self::Right),
            _ => None,
        }
    }

    pub fn to_vec2(&self) -> Vec2 {
        match self {
            Direction::Up => Vec2::Y,
            Direction::Down => Vec2::NEG_Y,
            Direction::Left => Vec2::X,
            Direction::Right => Vec2::NEG_X,
        }
    }
}

pub async fn run() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Plat4rs")
        .with_inner_size(PhysicalSize::new(800, 600))
        .build(&event_loop)
        .unwrap();

    let mut state = State::new(window).await;

    event_loop.run(move |event, _, control_flow| match event {
        Event::WindowEvent { window_id, event } if window_id == state.window().id() => {
            if !state.input(&event) {
                match event {
                    WindowEvent::CloseRequested
                    | WindowEvent::KeyboardInput {
                        input:
                            KeyboardInput {
                                state: ElementState::Pressed,
                                virtual_keycode: Some(VirtualKeyCode::Escape),
                                ..
                            },
                        ..
                    } => *control_flow = ControlFlow::Exit,
                    WindowEvent::Resized(physical_size) => state.resize(physical_size),
                    WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                        state.resize(*new_inner_size)
                    }
                    _ => (),
                }
            }
        }
        Event::RedrawRequested(window_id) if window_id == state.window().id() => {
            state.update();
            match state.render() {
                Ok(_) => (),
                // Reconfigure the surface if lost
                Err(SurfaceError::Lost) => state.resize(state.size),
                // The system is out of memory, we should probably quit
                Err(SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                // All other errors (Outdated, Timeout) should be resolved by the next frame
                Err(e) => eprintln!("{:?}", e),
            }
        }
        Event::MainEventsCleared => {
            state.game_state.update();

            // RedrawRequested will only trigger once, unless we manually
            // request it.
            state.window().request_redraw();
        }
        _ => (),
    })
}
