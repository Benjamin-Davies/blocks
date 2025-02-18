use winit::{event_loop::EventLoop, window::WindowBuilder};

use blocks_renderer::State;

#[pollster::main]
async fn main() {
    let event_loop = EventLoop::new().unwrap();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    let mut state = State::new(&window, wgpu::Backends::GL, Clock).await;

    state.run(event_loop).unwrap();
}

struct Clock;

impl blocks_renderer::clock::Clock for Clock {
    type Instant = std::time::Instant;

    fn now(&self) -> Self::Instant {
        std::time::Instant::now()
    }

    fn seconds_elapsed(&self, start: Self::Instant, end: Self::Instant) -> f32 {
        let duration = end.duration_since(start);
        duration.as_secs_f32()
    }
}
