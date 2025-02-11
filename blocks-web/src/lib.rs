use wasm_bindgen::prelude::*;
use winit::{event_loop::EventLoop, window::WindowBuilder};

use blocks::State;

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

    let mut state = State::new(&window, wgpu::Backends::GL).await;

    let win = web_sys::window().unwrap();
    let w = win.inner_width().unwrap().as_f64().unwrap() as u32;
    let h = win.inner_height().unwrap().as_f64().unwrap() as u32;
    state.resize(winit::dpi::PhysicalSize::new(w, h));
    state.manual_size = true;

    state.run(event_loop).unwrap();
}
