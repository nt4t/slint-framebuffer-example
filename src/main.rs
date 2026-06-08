use linuxfb::{Framebuffer, set_terminal_mode, TerminalMode};
use std::cell::RefCell;
use std::rc::Rc;
use std::time::Duration;
use slint::{
    PhysicalSize,
    platform::{
        Platform,
        software_renderer::{
            MinimalSoftwareWindow,
            Rgb565Pixel,
            RepaintBufferType,
        },
    },
};

slint::include_modules!();

struct FramebufferPlatform {
    window: Rc<MinimalSoftwareWindow>,
    fb: Framebuffer,
    stride: usize,
    bpp: usize,
    width: usize,
    height: usize,
    render_buffer: RefCell<Vec<Rgb565Pixel>>,
}

impl FramebufferPlatform {
    fn new(fb: Framebuffer) -> Self {
        let size = fb.get_size();
        let bpp = fb.get_bytes_per_pixel() as usize;
        let width = size.0 as usize;
        let stride = width * bpp;
        let window = MinimalSoftwareWindow::new(RepaintBufferType::ReusedBuffer);
        window.set_size(PhysicalSize::new(size.0, size.1));
        Self {
            window,
            fb,
            stride,
            bpp,
            width,
            height: size.1 as usize,
            render_buffer: RefCell::new(vec![Rgb565Pixel::default(); width * size.1 as usize]),
        }
    }
}

impl Platform for FramebufferPlatform {
    fn create_window_adapter(&self) -> Result<Rc<dyn slint::platform::WindowAdapter>, slint::PlatformError> {
        Ok(self.window.clone())
    }

    fn run_event_loop(&self) -> Result<(), slint::PlatformError> {
        loop {
            slint::platform::update_timers_and_animations();

            self.window.draw_if_needed(|renderer| {
                let mut frame = self.fb.map().unwrap();
                {
                    let mut buf = self.render_buffer.borrow_mut();
                    buf.clear();
                    buf.resize(self.width * self.height, Rgb565Pixel::default());
                    renderer.render(&mut buf, self.width);
                }
                let fb_pixels = unsafe { frame.as_mut_ptr() as *mut u32 };
                let fb_len = (self.width * self.height * self.bpp) / 4;
                let buf = self.render_buffer.borrow();
                unsafe {
                    for i in 0..std::cmp::min(buf.len(), fb_len) {
                        let p = buf[i].0;
                        let r = ((p & 0xF800) >> 8) as u8;
                        let g = ((p & 0x07E0) >> 3) as u8;
                        let b = ((p & 0x001F) << 3) as u8;
                        *fb_pixels.add(i) = (u32::from(r) << 16) | (u32::from(g) << 8) | u32::from(b);
                    }
                }
            });

            if !self.window.has_active_animations() {
                std::thread::sleep(slint::platform::duration_until_next_timer_update().unwrap_or(Duration::from_secs(1)));
            }
        }
    }
}

fn main() -> Result<(), slint::PlatformError> {
    // TODO: adjust these values to match your system:

    // Path of the current TTY. Used to switch the terminal to graphics mode
    // and back to text mode.
    let tty_path = "/dev/tty0";
    let fb_path = "/dev/fb0";

    // Switch back to text mode when terminating
    ctrlc::set_handler(move || {
        let tty = std::fs::File::open(tty_path).unwrap();
        set_terminal_mode(&tty, TerminalMode::Text).expect("switch to text mode");
        std::process::exit(1);
    }).expect("install signal handlers");

    // Instruct slint to use the FramebufferPlatform
    slint::platform::set_platform(Box::new(
        FramebufferPlatform::new(
            Framebuffer::new(fb_path).expect("open framebuffer")
        )
    )).expect("set platform");

    // Switch terminal to graphics mode
    let tty = std::fs::File::open(tty_path).expect("open TTY");
    set_terminal_mode(&tty, TerminalMode::Graphics).expect("switch to graphics mode");
    drop(tty);

    let ui = AppWindow::new()?;

    let ui_handle = ui.as_weak();
    ui.on_request_increase_value(move || {
        let ui = ui_handle.unwrap();
        ui.set_counter(ui.get_counter() + 1);
    });

    ui.run()
}
