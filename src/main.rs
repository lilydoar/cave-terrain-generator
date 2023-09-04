use color::{BLACK, RED, WHITE};
use log::info;
use pixels::{Pixels, SurfaceTexture};
use render::{
    clear_frame, render_grid, render_scalar_field, render_terrain, render_terrain_grid, set_row,
};
use terrain::Terrain;
use winit::{
    dpi::{LogicalSize, PhysicalPosition},
    event::{ElementState, Event, MouseButton, WindowEvent},
    event_loop::EventLoop,
    window::WindowBuilder,
};

mod color;
mod render;
mod terrain;

const SCREEN_ASPECT_RATIO: f64 = 1.0;
const SCREEN_WIDTH: usize = 900;
const SCREEN_HEIGHT: usize = (SCREEN_WIDTH as f64 / SCREEN_ASPECT_RATIO) as usize;
const PIXEL_SIZE: usize = 1;

fn main() {
    env_logger::init();

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
    let grid_size = SCREEN_WIDTH / world_width;

    let mut terrain = Terrain::new(world_width, world_height);

    let mut mouse_pos = PhysicalPosition::new(0.0, 0.0);

    // Event loop
    event_loop.run(move |event, _, control_flow| match event {
        Event::MainEventsCleared => {
            window.request_redraw();
        }
        Event::RedrawRequested(_) => {
            let frame = pixels.frame_mut();

            clear_frame(frame, &WHITE);
            render_terrain(frame, &terrain, &BLACK);
            render_terrain_grid(frame, &terrain, &RED);

            pixels.render().unwrap();
        }
        Event::WindowEvent {
            window_id: _,
            event: window_event,
        } => match window_event {
            WindowEvent::CloseRequested => control_flow.set_exit(),
            WindowEvent::CursorMoved { position, .. } => {
                mouse_pos = position;
            }
            WindowEvent::MouseInput { state, button, .. } => {
                if button == MouseButton::Left && state == ElementState::Pressed {
                    let (pixel_col, pixel_row) = pixels
                        .window_pos_to_pixel(mouse_pos.into())
                        .unwrap_or_else(|pos| pixels.clamp_pixel_pos(pos));

                    let grid_row = pixel_row / grid_size;
                    let grid_col = pixel_col / grid_size;

                    let old_scalar = terrain.scalar_field[grid_row][grid_col];

                    let mut new_scalar = old_scalar - 0.6;
                    if new_scalar < 0.0 {
                        new_scalar = 0.0;
                    }

                    terrain.modify_scalar_field(grid_row, grid_col, new_scalar);
                }
            }
            _ => {}
        },
        _ => {}
    });
}
