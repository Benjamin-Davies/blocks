use winit::{event_loop::EventLoop, window::WindowBuilder};

use blocks_renderer::State;

#[pollster::main]
async fn main() {
    let event_loop = EventLoop::new().unwrap();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    let mut state = State::new(&window, wgpu::Backends::GL).await;

    state.run(event_loop).unwrap();
}
