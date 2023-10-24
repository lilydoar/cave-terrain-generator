use color::{RED, WHITE};
use render::{clear_frame, render_string, render_terrain, render_terrain_grid};
use terrain::Terrain;
use window::Window;
use winit::{
    dpi::PhysicalPosition,
    event::{ElementState, Event, MouseButton, VirtualKeyCode, WindowEvent},
};

mod color;
mod font;
mod render;
mod terrain;
mod window;

fn main() {
    env_logger::init();

    // Rendering
    let screen_size = 1000;
    let (mut window, event_loop) = Window::new(screen_size, screen_size, 1);

    // Input
    let mut mouse_pos = PhysicalPosition::new(0.0, 0.0);
    let mut mouse_pressed = false;

    // World
    let world_width = 40;
    let world_height = 40;
    let grid_size = screen_size / world_width;
    let mut terrain = Terrain::new(world_width, world_height);

    let mut add_mode = true;
    let mut show_grid = false;

    let instruction_str = "a add mode\ns subtract mode\ng toggle grid\n";

    // Event loop
    event_loop.run(move |event, _, control_flow| match event {
        Event::MainEventsCleared => {
            if mouse_pressed {
                let (pixel_col, pixel_row) = window.screen.window_pos_to_pixel(mouse_pos);

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
            // Battleship gray: [135, 142, 136, 255]
            // Liver: [114, 87, 82, 255]
            // Sepia: [93, 58, 0, 255]
            // Van Dyke: [71, 45, 35, 255]

            clear_frame(&mut window.screen, &[135, 142, 136, 255]);

            render_terrain(&mut window.screen, &terrain, &[71, 45, 35, 255]);

            if show_grid {
                render_terrain_grid(&mut window.screen, &terrain, &RED);
            }

            render_string(&mut window.screen, instruction_str, 10, 10, 2, &WHITE);

            window.screen.draw_frame().unwrap();
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
