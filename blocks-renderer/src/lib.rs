use clock::Clock;
use wgpu::util::DeviceExt;
use winit::{
    dpi::PhysicalPosition,
    error::EventLoopError,
    event::{DeviceEvent, ElementState, Event, KeyEvent, MouseButton, TouchPhase, WindowEvent},
    event_loop::EventLoop,
    keyboard::{KeyCode, PhysicalKey},
    window::{CursorGrabMode, Window},
};

use blocks_game::Game;

pub mod clock;

mod camera;
mod texture;
mod voxel_renderer;

const CORNFLOWER_BLUE: wgpu::Color = wgpu::Color {
    r: 0.4,
    g: 0.6,
    b: 0.9,
    a: 1.0,
};

const MOUSE_SENSITIVITY: f32 = 0.1;

pub struct State<'a, C: Clock> {
    surface: wgpu::Surface<'a>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: winit::dpi::PhysicalSize<u32>,
    pub manual_size: bool,
    window: &'a Window,
    camera: camera::Camera,
    camera_buffer: wgpu::Buffer,
    camera_bind_group: wgpu::BindGroup,
    depth_texture: texture::Texture,
    voxel_renderer: voxel_renderer::VoxelRenderer,
    game: Game,
    clock: C,
    last_frame: C::Instant,
    cursor_grabbed: bool,
    last_touch_location: PhysicalPosition<f64>,
}

impl<'a, C: Clock> State<'a, C> {
    pub async fn new(window: &'a Window, backends: wgpu::Backends, clock: C) -> Self {
        let size = window.inner_size();

        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
            backends,
            ..Default::default()
        });

        let surface = instance.create_surface(window).unwrap();

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .unwrap();

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    required_features: wgpu::Features::empty(),
                    required_limits: adapter.limits(),
                    label: None,
                    memory_hints: Default::default(),
                },
                None,
            )
            .await
            .unwrap();

        let surface_caps = surface.get_capabilities(&adapter);

        let surface_format = surface_caps
            .formats
            .iter()
            .find(|f| f.is_srgb())
            .copied()
            .unwrap_or(surface_caps.formats[0]);
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };

        let camera = camera::Camera::new(config.width as f32 / config.height as f32);

        let camera_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Camera Buffer"),
            contents: bytemuck::cast_slice(&[camera.build_view_projection_matrix()]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let camera_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
                label: Some("camera_bind_group_layout"),
            });

        let camera_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &camera_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: camera_buffer.as_entire_binding(),
            }],
            label: Some("camera_bind_group"),
        });

        let depth_texture =
            texture::Texture::create_depth_texture(&device, &config, "depth_texture");

        let voxel_renderer = voxel_renderer::VoxelRenderer::new(
            &device,
            &queue,
            &camera_bind_group_layout,
            config.format,
        );

        let game = Game::new();

        Self {
            surface,
            device,
            queue,
            config,
            size,
            manual_size: false,
            window,
            camera,
            camera_buffer,
            camera_bind_group,
            depth_texture,
            voxel_renderer,
            game,
            last_frame: clock.now(),
            clock,
            cursor_grabbed: false,
            last_touch_location: PhysicalPosition::new(0.0, 0.0),
        }
    }

    pub fn run(&mut self, event_loop: EventLoop<()>) -> Result<(), EventLoopError> {
        event_loop.run(move |event, control_flow| match event {
            Event::DeviceEvent { ref event, .. } => {
                self.device_input(event);
            }
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == self.window.id() => {
                if !self.input(event) {
                    match event {
                        WindowEvent::CloseRequested
                        | WindowEvent::KeyboardInput {
                            event:
                                KeyEvent {
                                    state: ElementState::Pressed,
                                    physical_key: PhysicalKey::Code(KeyCode::KeyQ),
                                    ..
                                },
                            ..
                        } => control_flow.exit(),
                        WindowEvent::Resized(physical_size) => {
                            if !self.manual_size {
                                self.resize(*physical_size);
                            }
                        }
                        WindowEvent::RedrawRequested => {
                            self.window.request_redraw();

                            self.update();
                            match self.render() {
                                Ok(_) => {}
                                Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => {
                                    self.resize(self.size);
                                }
                                Err(
                                    wgpu::SurfaceError::OutOfMemory | wgpu::SurfaceError::Other,
                                ) => {
                                    log::error!("OutOfMemory");
                                    control_flow.exit();
                                }
                                Err(wgpu::SurfaceError::Timeout) => {
                                    log::warn!("Surface timeout")
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        })
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width == 0 || new_size.height == 0 {
            return;
        }

        self.size = new_size;
        self.config.width = new_size.width;
        self.config.height = new_size.height;
        self.surface.configure(&self.device, &self.config);

        self.depth_texture =
            texture::Texture::create_depth_texture(&self.device, &self.config, "depth_texture");

        self.camera.aspect = new_size.width as f32 / new_size.height as f32;
        self.queue.write_buffer(
            &self.camera_buffer,
            0,
            bytemuck::cast_slice(&[self.camera.build_view_projection_matrix()]),
        );
    }

    fn input(&mut self, event: &WindowEvent) -> bool {
        match event {
            WindowEvent::MouseInput {
                button: MouseButton::Left,
                state: ElementState::Pressed,
                ..
            } => {
                self.window.set_cursor_grab(CursorGrabMode::Locked).unwrap();
                self.window.set_cursor_visible(false);
                self.cursor_grabbed = true;
                true
            }
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        state: ElementState::Pressed,
                        physical_key: PhysicalKey::Code(KeyCode::Escape),
                        ..
                    },
                ..
            } => {
                self.window.set_cursor_grab(CursorGrabMode::None).unwrap();
                self.window.set_cursor_visible(true);
                self.cursor_grabbed = false;
                true
            }
            WindowEvent::KeyboardInput { event, .. } => match event.physical_key {
                PhysicalKey::Code(KeyCode::KeyW) => {
                    match event.state {
                        ElementState::Pressed => self.game.player.walk_vector.z = 1.0,
                        ElementState::Released => self.game.player.walk_vector.z = 0.0,
                    }
                    true
                }
                PhysicalKey::Code(KeyCode::KeyA) => {
                    match event.state {
                        ElementState::Pressed => self.game.player.walk_vector.x = 1.0,
                        ElementState::Released => self.game.player.walk_vector.x = 0.0,
                    }
                    true
                }
                PhysicalKey::Code(KeyCode::KeyS) => {
                    match event.state {
                        ElementState::Pressed => self.game.player.walk_vector.z = -1.0,
                        ElementState::Released => self.game.player.walk_vector.z = 0.0,
                    }
                    true
                }
                PhysicalKey::Code(KeyCode::KeyD) => {
                    match event.state {
                        ElementState::Pressed => self.game.player.walk_vector.x = -1.0,
                        ElementState::Released => self.game.player.walk_vector.x = 0.0,
                    }
                    true
                }
                PhysicalKey::Code(KeyCode::Space) => match event.state {
                    ElementState::Pressed => {
                        self.game.player.jump();
                        true
                    }
                    _ => false,
                },
                _ => false,
            },
            WindowEvent::Touch(touch) => match touch.phase {
                TouchPhase::Started => {
                    self.last_touch_location = touch.location;
                    true
                }
                TouchPhase::Moved => {
                    let delta_x = touch.location.x - self.last_touch_location.x;
                    let delta_y = touch.location.y - self.last_touch_location.y;

                    self.game.player.head_angle.x += MOUSE_SENSITIVITY * delta_y as f32;
                    self.game.player.head_angle.y += MOUSE_SENSITIVITY * delta_x as f32;

                    self.last_touch_location = touch.location;
                    true
                }
                _ => false,
            },
            _ => false,
        }
    }

    pub fn device_input(&mut self, event: &DeviceEvent) -> bool {
        match event {
            DeviceEvent::MouseMotion { delta } if self.cursor_grabbed => {
                let &(delta_x, delta_y) = delta;

                self.game.player.head_angle.x -= MOUSE_SENSITIVITY * delta_y as f32;
                self.game.player.head_angle.y -= MOUSE_SENSITIVITY * delta_x as f32;

                true
            }
            _ => false,
        }
    }

    fn update(&mut self) {
        let this_frame = self.clock.now();
        let delta_time = self.clock.seconds_elapsed(self.last_frame, this_frame);

        self.game.update(delta_time);

        self.camera.update(&self.game.player);
        self.queue.write_buffer(
            &self.camera_buffer,
            0,
            bytemuck::cast_slice(&[self.camera.build_view_projection_matrix()]),
        );

        self.voxel_renderer.update(&self.device, &mut self.game);

        self.last_frame = this_frame;
    }

    fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        if self.size.width == 0 || self.size.height == 0 {
            return Ok(());
        }

        let output = self.surface.get_current_texture()?;

        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(CORNFLOWER_BLUE),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &self.depth_texture.view,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0),
                        store: wgpu::StoreOp::Store,
                    }),
                    stencil_ops: None,
                }),
                occlusion_query_set: None,
                timestamp_writes: None,
            });

            self.voxel_renderer
                .render(&mut render_pass, &self.camera_bind_group);
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}
