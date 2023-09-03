use color::WHITE;
use pixels::{Pixels, SurfaceTexture};
use render::{render_grid, render_scalar_field};
use terrain::Terrain;
use winit::{
    dpi::LogicalSize,
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    window::WindowBuilder,
};

mod color;
mod render;
mod terrain;

const SCREEN_ASPECT_RATIO: f64 = 4.0 / 3.0;
const SCREEN_WIDTH: usize = 512;
const SCREEN_HEIGHT: usize = (SCREEN_WIDTH as f64 / SCREEN_ASPECT_RATIO) as usize;
const PIXEL_SIZE: usize = 2;

fn main() {
    // Rendering
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_inner_size(LogicalSize::new(
            (SCREEN_WIDTH * PIXEL_SIZE) as f64,
            (SCREEN_HEIGHT * PIXEL_SIZE) as f64,
        ))
        .with_resizable(false)
        .build(&event_loop)
        .unwrap();

    let mut pixels = Pixels::new(
        SCREEN_WIDTH as u32,
        SCREEN_HEIGHT as u32,
        SurfaceTexture::new(
            window.inner_size().width,
            window.inner_size().height,
            &window,
        ),
    )
    .unwrap();

    // World
    let world_width = 10;
    let world_height = 10;
    let terrain = Terrain::new(world_width, world_height);

    event_loop.run(move |event, _, control_flow| match event {
        Event::MainEventsCleared => {
            window.request_redraw();
        }
        Event::RedrawRequested(_) => {
            let frame = pixels.frame_mut();

            // render_grid(frame, 50, &WHITE);
            render_scalar_field(frame, &terrain);

            pixels.render().unwrap();
        }
        Event::WindowEvent {
            window_id: _,
            event: window_event,
        } => match window_event {
            WindowEvent::CloseRequested => control_flow.set_exit(),
            _ => {}
        },
        _ => {}
    });
}
