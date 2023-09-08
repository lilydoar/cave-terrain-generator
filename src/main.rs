use color::RED;
use pixels::{Pixels, SurfaceTexture};
use render::{clear_frame, render_terrain, render_terrain_grid};
use terrain::Terrain;
use winit::{
    dpi::{LogicalSize, PhysicalPosition},
    event::{ElementState, Event, MouseButton, VirtualKeyCode, WindowEvent},
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

    // Input
    let mut mouse_pos = PhysicalPosition::new(0.0, 0.0);
    let mut mouse_pressed = false;

    // World
    let world_width = 40;
    let world_height = 40;
    let grid_size = SCREEN_WIDTH / world_width;
    let mut terrain = Terrain::new(world_width, world_height);

    let mut add_mode = true;
    let mut show_grid = false;

    // Event loop
    event_loop.run(move |event, _, control_flow| match event {
        Event::MainEventsCleared => {
            if mouse_pressed {
                let (pixel_col, pixel_row) = pixels
                    .window_pos_to_pixel(mouse_pos.into())
                    .unwrap_or_else(|pos| pixels.clamp_pixel_pos(pos));

                let grid_row = pixel_row / grid_size;
                let grid_col = pixel_col / grid_size;

                if add_mode {
                    terrain.modify_scalar_field(grid_row, grid_col, 1.0);
                } else {
                    terrain.modify_scalar_field(grid_row, grid_col, 0.0);
                }
            }

            window.request_redraw();
        }
        Event::RedrawRequested(_) => {
            let frame = pixels.frame_mut();

            // Battleship gray: [135, 142, 136, 255]
            // Liver: [114, 87, 82, 255]
            // Sepia: [93, 58, 0, 255]
            // Van Dyke: [71, 45, 35, 255]

            clear_frame(frame, &[135, 142, 136, 255]);

            render_terrain(frame, &terrain, &[71, 45, 35, 255]);

            if show_grid {
                render_terrain_grid(frame, &terrain, &RED);
            }

            pixels.render().unwrap();
        }
        Event::WindowEvent {
            event: window_event,
            ..
        } => match window_event {
            WindowEvent::CloseRequested => control_flow.set_exit(),
            WindowEvent::CursorMoved { position, .. } => {
                mouse_pos = position;
            }
            WindowEvent::MouseInput { state, button, .. } => {
                mouse_pressed = button == MouseButton::Left && state == ElementState::Pressed;
            }
            WindowEvent::KeyboardInput { input, .. } => {
                if input.state != ElementState::Pressed {
                    return;
                }

                match input.virtual_keycode {
                    Some(VirtualKeyCode::A) => add_mode = true,
                    Some(VirtualKeyCode::S) => add_mode = false,
                    Some(VirtualKeyCode::G) => show_grid = !show_grid,
                    _ => {}
                }
            }
            _ => {}
        },
        _ => {}
    });
}
