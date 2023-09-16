use pixels::{Pixels, SurfaceTexture};
use winit::dpi::PhysicalPosition;
use winit::{dpi::LogicalSize, event_loop::EventLoop, window::WindowBuilder};

pub struct Window {
    window: winit::window::Window,
    pub screen: Screen,
}

impl Window {
    pub fn new(
        screen_width: usize,
        screen_height: usize,
        pixel_size: usize,
    ) -> (Self, EventLoop<()>) {
        let event_loop = EventLoop::new();
        let window = WindowBuilder::new()
            .with_inner_size(LogicalSize::new(
                (screen_width * pixel_size) as f64,
                (screen_height * pixel_size) as f64,
            ))
            .with_resizable(false)
            .build(&event_loop)
            .unwrap();

        let screen = Screen::new(screen_width, screen_height, &window);

        (Self { window, screen }, event_loop)
    }

    pub fn request_redraw(&self) {
        self.window.request_redraw()
    }
}

pub struct Screen {
    width: usize,
    height: usize,
    aspect_ratio: f64,

    pixels: Pixels,
}

impl Screen {
    pub fn new(width: usize, height: usize, window: &winit::window::Window) -> Self {
        let pixels = Pixels::new(
            width as u32,
            height as u32,
            SurfaceTexture::new(
                window.inner_size().width,
                window.inner_size().height,
                &window,
            ),
        )
        .unwrap();

        let aspect_ratio = width as f64 / height as f64;

        Self {
            width,
            height,
            aspect_ratio,
            pixels,
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn frame_mut(&mut self) -> &mut [u8] {
        self.pixels.frame_mut()
    }

    pub fn draw_frame(&self) -> Result<(), pixels::Error> {
        self.pixels.render()
    }

    pub fn window_pos_to_pixel(&self, pos: PhysicalPosition<f64>) -> (usize, usize) {
        self.pixels
            .window_pos_to_pixel(pos.into())
            .unwrap_or_else(|pos| self.pixels.clamp_pixel_pos(pos))
    }
}
