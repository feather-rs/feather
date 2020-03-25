//! Implements a GUI for performing assorted debugging tasks.
//! Currently only used for world generation.

use crate::worldgen;
use imgui_winit_support::{HiDpiMode, WinitPlatform};
use std::time::Instant;
use winit::dpi::LogicalSize;
use winit::event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::Window;

pub const UI_WIDTH: usize = 1920 / 2;
pub const UI_HEIGHT: usize = 1080 / 2;
pub const VIEWER_WIDTH: usize = 512;
pub const VIEWER_HEIGHT: usize = 512;

struct Context {
    window: Window,
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    sc_desc: wgpu::SwapChainDescriptor,
    sc: wgpu::SwapChain,
    platform: WinitPlatform,
    renderer: imgui_wgpu::Renderer,
    hidpi_factor: f64,
}

enum State {
    Worldgen(worldgen::devtools::State),
}

pub fn run() -> anyhow::Result<()> {
    let (mut context, event_loop, mut imgui) = init_context()?;
    let mut state = State::Worldgen(worldgen::devtools::State::default());

    let mut last_frame = Instant::now();

    let mut buffer = vec![0; VIEWER_HEIGHT * VIEWER_WIDTH];
    let mut display_window = minifb::Window::new(
        "Feather Dev Tools: Display",
        VIEWER_WIDTH,
        VIEWER_HEIGHT,
        minifb::WindowOptions::default(),
    )?;

    let mut is_first_run = true;
    event_loop.run(move |event, _, control_flow| {
        let mut buffer_changed = false;
        *control_flow = ControlFlow::Poll;
        match event {
            Event::WindowEvent {
                event: WindowEvent::ScaleFactorChanged { scale_factor, .. },
                ..
            } => {
                context.hidpi_factor = scale_factor;
            }
            Event::WindowEvent {
                event: WindowEvent::Resized(_),
                ..
            } => {
                let size = context.window.inner_size();

                context.sc_desc = wgpu::SwapChainDescriptor {
                    usage: wgpu::TextureUsage::OUTPUT_ATTACHMENT,
                    format: wgpu::TextureFormat::Bgra8Unorm,
                    width: size.width as u32,
                    height: size.height as u32,
                    present_mode: wgpu::PresentMode::Vsync,
                };

                context.sc = context
                    .device
                    .create_swap_chain(&context.surface, &context.sc_desc);
            }
            Event::WindowEvent {
                event:
                    WindowEvent::KeyboardInput {
                        input:
                            KeyboardInput {
                                virtual_keycode: Some(VirtualKeyCode::Escape),
                                state: ElementState::Pressed,
                                ..
                            },
                        ..
                    },
                ..
            }
            | Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                *control_flow = ControlFlow::Exit;
            }
            Event::MainEventsCleared => {
                context.window.request_redraw();
            }
            Event::RedrawEventsCleared => {
                last_frame = imgui.io_mut().update_delta_time(last_frame);

                let frame = context.sc.get_next_texture();
                context
                    .platform
                    .prepare_frame(imgui.io_mut(), &context.window)
                    .expect("failed to prepare frame");
                let ui = imgui.frame();

                {
                    match &mut state {
                        State::Worldgen(state) => {
                            if state.render(&ui, &mut buffer, is_first_run) {
                                buffer_changed = true;
                            }
                        }
                    }
                }

                let mut encoder: wgpu::CommandEncoder = context
                    .device
                    .create_command_encoder(&wgpu::CommandEncoderDescriptor { todo: 0 });

                context.platform.prepare_render(&ui, &context.window);
                context
                    .renderer
                    .render(ui.render(), &context.device, &mut encoder, &frame.view)
                    .expect("Rendering failed");

                context.queue.submit(&[encoder.finish()]);
            }
            _ => (),
        }

        context
            .platform
            .handle_event(imgui.io_mut(), &context.window, &event);

        if buffer_changed || is_first_run {
            display_window
                .update_with_buffer(&buffer, VIEWER_WIDTH, VIEWER_HEIGHT)
                .unwrap();
        }

        is_first_run = false;
    });
}

fn init_context() -> anyhow::Result<(Context, EventLoop<()>, imgui::Context)> {
    let event_loop = EventLoop::new();
    let hidpi_factor = 1.0;

    let window = Window::new(&event_loop)?;
    window.set_inner_size(LogicalSize {
        width: UI_WIDTH as f64,
        height: UI_HEIGHT as f64,
    });

    window.set_title(&format!(
        "Feather Developer Tools v{}",
        env!("CARGO_PKG_VERSION")
    ));

    let size = window.inner_size();

    let surface = wgpu::Surface::create(&window);

    let adapter = wgpu::Adapter::request(&wgpu::RequestAdapterOptions {
        power_preference: wgpu::PowerPreference::Default,
        backends: wgpu::BackendBit::PRIMARY,
    })
    .ok_or_else(|| anyhow::anyhow!("failed to select a suitable adapter"))?;

    let (device, mut queue) = adapter.request_device(&wgpu::DeviceDescriptor {
        extensions: wgpu::Extensions {
            anisotropic_filtering: false,
        },
        limits: Default::default(),
    });

    let sc_desc = wgpu::SwapChainDescriptor {
        usage: wgpu::TextureUsage::OUTPUT_ATTACHMENT,
        format: wgpu::TextureFormat::Bgra8Unorm,
        width: size.width as u32,
        height: size.height as u32,
        present_mode: wgpu::PresentMode::Vsync,
    };

    let sc = device.create_swap_chain(&surface, &sc_desc);

    let mut imgui = imgui::Context::create();

    let mut platform = WinitPlatform::init(&mut imgui);
    platform.attach_window(imgui.io_mut(), &window, HiDpiMode::Default);

    let renderer = imgui_wgpu::Renderer::new(
        &mut imgui,
        &device,
        &mut queue,
        sc_desc.format,
        Some(wgpu::Color {
            r: 0.2,
            g: 0.2,
            b: 0.2,
            a: 1.0,
        }),
    );

    Ok((
        Context {
            window,
            surface,
            device,
            queue,
            sc_desc,
            sc,
            platform,
            renderer,
            hidpi_factor,
        },
        event_loop,
        imgui,
    ))
}
