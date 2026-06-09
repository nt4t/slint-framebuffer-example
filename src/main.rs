use evdev::Device;
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
    width: usize,
    height: usize,
    bytes_per_pixel: usize,
    render_buffer: RefCell<Vec<Rgb565Pixel>>,
}

impl FramebufferPlatform {
    fn new(fb: Framebuffer) -> Self {
        let size = fb.get_size();
        let width = size.0 as usize;
        let height = size.1 as usize;
        let bytes_per_pixel = fb.get_bytes_per_pixel() as usize;
        let window = MinimalSoftwareWindow::new(RepaintBufferType::ReusedBuffer);
        window.set_size(PhysicalSize::new(size.0, size.1));
        Self {
            window,
            fb,
            width,
            height,
            bytes_per_pixel,
            render_buffer: RefCell::new(vec![Rgb565Pixel::default(); width * height]),
        }
    }

    fn render_to_fb(&self) {
        let mut frame = self.fb.map().unwrap();
        let buf = self.render_buffer.borrow();

        if self.bytes_per_pixel == 2 {
            let fb_pixels = frame.as_mut_ptr() as *mut u16;
            unsafe {
                for i in 0..std::cmp::min(buf.len(), self.width * self.height) {
                    *fb_pixels.add(i) = buf[i].0;
                }
            }
        } else {
            let fb_pixels = frame.as_mut_ptr() as *mut u32;
            let fb_len = (self.width * self.height * self.bytes_per_pixel) / self.bytes_per_pixel;
            unsafe {
                for i in 0..std::cmp::min(buf.len(), fb_len) {
                    let p = buf[i].0;
                    let r = ((p & 0xF800) >> 8) as u8;
                    let g = ((p & 0x07E0) >> 3) as u8;
                    let b = ((p & 0x001F) << 3) as u8;
                    *fb_pixels.add(i) = (u32::from(r) << 16) | (u32::from(g) << 8) | u32::from(b);
                }
            }
        }
    }
}

#[derive(Clone)]
struct PlatformWrapper {
    inner: Rc<FramebufferPlatform>,
}

impl Platform for PlatformWrapper {
    fn create_window_adapter(&self) -> Result<Rc<dyn slint::platform::WindowAdapter>, slint::PlatformError> {
        Ok(self.inner.window.clone())
    }
}

fn process_keyboard_events(ui: &AppWindow) {
    if let Ok(mut device) = Device::open("/dev/input/event0") {
        if let Err(e) = device.fetch_events() {
            eprintln!("Error fetching keyboard events: {}", e);
            return;
        }

        for input_event in device.fetch_events().unwrap() {
            let key_code = input_event.code();
            let value = input_event.value();

            eprintln!("KEY: code={} value={}", key_code, value);

            if value == 1 {
                match key_code {
                    103 | 25 | 17 => {
                        eprintln!("UP pressed (code={})", key_code);
                        let current = ui.get_selected_index();
                        eprintln!("Current index: {}", current);
                        if current > 0 {
                            ui.set_selected_index(current - 1);
                            eprintln!("Set index to: {}", current - 1);
                        }
                    }
                    108 | 16 | 31 => {
                        eprintln!("DOWN pressed (code={})", key_code);
                        let current = ui.get_selected_index();
                        eprintln!("Current index: {}", current);
                        if current < 4 {
                            ui.set_selected_index(current + 1);
                            eprintln!("Set index to: {}", current + 1);
                        }
                    }
                    28 => {
                        eprintln!("ENTER pressed");
                        let current = ui.get_selected_index();
                        eprintln!("Item selected: {}", current);
                        ui.invoke_item_selected(current);
                    }
                    _ => {}
                }
            }
        }
    }
}

fn read_keyboard_events(ui_weak: slint::Weak<AppWindow>) {
    let mut device = match Device::open("/dev/input/event0") {
        Ok(d) => d,
        Err(e) => {
            eprintln!("Failed to open /dev/input/event0: {}", e);
            return;
        }
    };

    eprintln!("Reading raw key presses from event device...");

    loop {
        match device.fetch_events() {
            Ok(events) => {
                for event in events {
                    if event.event_type() == EventType::KEY {
                        if event.value() == 1 {
                            let code = event.code();
                            eprintln!("Key pressed: {}", code);
                            match code {
                                17 => { // KEY_W
                                    eprintln!("KEY_W: trying to upgrade ui_weak");
                                    if let Some(ui) = ui_weak.clone().upgrade() {
                                        let current = ui.get_selected_index();
                                        eprintln!("KEY_W: current={}, upgrading succeeded", current);
                                        if current > 0 {
                                            ui.set_selected_index(current - 1);
                                            eprintln!("KEY_W: set to {}", current - 1);
                                        } else {
                                            eprintln!("KEY_W: at top, no change");
                                        }
                                    } else {
                                        eprintln!("KEY_W: upgrade failed!");
                                    }
                                }
                                31 => { // KEY_S
                                    eprintln!("KEY_S: trying to upgrade ui_weak");
                                    if let Some(ui) = ui_weak.clone().upgrade() {
                                        let current = ui.get_selected_index();
                                        eprintln!("KEY_S: current={}, upgrading succeeded", current);
                                        if current < 4 {
                                            ui.set_selected_index(current + 1);
                                            eprintln!("KEY_S: set to {}", current + 1);
                                        } else {
                                            eprintln!("KEY_S: at bottom, no change");
                                        }
                                    } else {
                                        eprintln!("KEY_S: upgrade failed!");
                                    }
                                }
                                28 => { // KEY_ENTER
                                    eprintln!("KEY_ENTER: trying to upgrade ui_weak");
                                    if let Some(ui) = ui_weak.clone().upgrade() {
                                        let index = ui.get_selected_index();
                                        eprintln!("KEY_ENTER: selected index={}, upgrading succeeded", index);
                                    } else {
                                        eprintln!("KEY_ENTER: upgrade failed!");
                                    }
                                }
                                _ => {}
                            }
                        }
                    }
                }
            }
            Err(e) => {
                eprintln!("Error fetching events: {}", e);
                break;
            }
        }
        std::thread::sleep(Duration::from_millis(10));
    }
}

fn main() -> Result<(), slint::PlatformError> {
    let tty_path = "/dev/tty0";
    let fb_path = "/dev/fb0";

    ctrlc::set_handler(move || {
        if let Ok(tty) = std::fs::File::open(tty_path) {
            let _ = set_terminal_mode(&tty, TerminalMode::Text);
        }
        std::process::exit(1);
    }).expect("install signal handlers");

    let fb = Framebuffer::new(fb_path).expect("open framebuffer");

    let platform = PlatformWrapper {
        inner: Rc::new(FramebufferPlatform::new(fb)),
    };

    slint::platform::set_platform(Box::new(platform.clone())).expect("set platform");

    if let Ok(tty) = std::fs::File::open(tty_path) {
        let _ = set_terminal_mode(&tty, TerminalMode::Graphics);
        drop(tty);
    }

    let ui = AppWindow::new()?;

    ui.on_item_selected(move |index| {
        eprintln!("Item selected callback: {}", index);
    });

    platform.inner.window.draw_if_needed(|renderer| {
        renderer.render(&mut *platform.inner.render_buffer.borrow_mut(), platform.inner.width);
    });
    platform.inner.render_to_fb();

    loop {
        slint::platform::update_timers_and_animations();

        process_keyboard_events(&ui);

        platform.inner.window.draw_if_needed(|renderer| {
            renderer.render(&mut *platform.inner.render_buffer.borrow_mut(), platform.inner.width);
        });

        platform.inner.render_to_fb();

        std::thread::sleep(Duration::from_millis(50));
    }
}
