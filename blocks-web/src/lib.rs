use wasm_bindgen::prelude::*;
use winit::{event_loop::EventLoop, window::WindowBuilder};

use blocks_renderer::State;

#[wasm_bindgen(start)]
async fn run() {
    console_error_panic_hook::set_once();
    console_log::init_with_level(log::Level::Info).expect("Couldn't initialize logger");

    let event_loop = EventLoop::new().unwrap();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    #[cfg(target_arch = "wasm32")]
    {
        use winit::platform::web::WindowExtWebSys;
        web_sys::window()
            .and_then(|win| win.document())
            .and_then(|doc| {
                let canvas = web_sys::Element::from(window.canvas()?);
                doc.body()?.append_child(&canvas).ok()?;
                Some(())
            })
            .expect("Couldn't append canvas to document body.");
    }

    let mut state = State::new(&window, wgpu::Backends::GL, Clock::new()).await;

    let win = web_sys::window().unwrap();
    let w = win.inner_width().unwrap().as_f64().unwrap() as u32;
    let h = win.inner_height().unwrap().as_f64().unwrap() as u32;
    state.resize(winit::dpi::PhysicalSize::new(w, h));
    state.manual_size = true;

    state.run(event_loop).unwrap();
}

struct Clock {
    performance: web_sys::Performance,
}

impl Clock {
    fn new() -> Self {
        let window = web_sys::window().expect("Window should be available.");
        let performance = window
            .performance()
            .expect("Performance should be available.");
        Self { performance }
    }
}

impl blocks_renderer::clock::Clock for Clock {
    /// Time in milliseconds since the page started loading.
    type Instant = f64;

    fn now(&self) -> Self::Instant {
        self.performance.now()
    }

    fn seconds_elapsed(&self, start: Self::Instant, end: Self::Instant) -> f32 {
        let seconds = (end - start) / 1000.0;
        seconds as f32
    }
}
