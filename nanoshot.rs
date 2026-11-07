#![forbid(unsafe_code)]

use tray_icon::{Icon, TrayIcon, TrayIconBuilder, TrayIconEvent};
use winit::{
    application::ApplicationHandler,
    event_loop::{ActiveEventLoop, EventLoop, EventLoopProxy},
    window::WindowId,
    event::WindowEvent,
};
use std::{
    sync::OnceLock,
    fs,
    path::{Path, PathBuf},
    process::Command,
    time::{SystemTime, UNIX_EPOCH},
};

static CHAIN_CLICK_TIMES: u8 = 5;
static FOLDER: OnceLock<PathBuf> = OnceLock::new();

#[derive(Default)]
struct App(Option<TrayIcon>);

impl ApplicationHandler<TrayIconEvent> for App {
    fn window_event(&mut self, _: &ActiveEventLoop, _: WindowId, _: WindowEvent) {}

    fn resumed(&mut self, _: &ActiveEventLoop) {
        let img = image::open("icon.png").unwrap().into_rgba8();
        let (w, h) = img.dimensions();
        self.0 = Some(TrayIconBuilder::new()
                      .with_icon(Icon::from_rgba(img.into_raw(), w, h).unwrap())
                      .with_menu_on_right_click(false)
                      .with_tooltip("Click to capture region")
                      .build().unwrap());
    }

    fn user_event(&mut self, _: &ActiveEventLoop, event: TrayIconEvent) {

        if let TrayIconEvent::Click { button, button_state, .. } = event {
            if button_state == tray_icon::MouseButtonState::Up {
                match button {
                    tray_icon::MouseButton::Left => screenshot().unwrap(),
                    tray_icon::MouseButton::Right => chain_screenshot().unwrap(),
                    _ => {}
                }
            }
        }

    }
}

// 5 screenshots in a row - esc to exit
fn chain_screenshot() -> Result<(), Box<dyn std::error::Error>> {
    for _ in 0..CHAIN_CLICK_TIMES {
        screenshot()?;
    }
    Ok(())
}

// Take a screenshot
// NOTE: wraps /usr/sbin/screencapture
fn screenshot() -> Result<(), Box<dyn std::error::Error>> {

    let timestamp = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
    let path = Path::new(FOLDER.get().unwrap()).join(format!("Screenshot-{timestamp}.png"));

    let status = Command::new("/usr/sbin/screencapture")
        .arg("-i")
        .arg(&path)
        .status()?;

    if status.success() {
        Ok(())
    } else {
        Err("ERROR".into())
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {

    FOLDER.get_or_init(|| {
        let home = std::env::var_os("HOME")
            .map(PathBuf::from)
            .expect("HOME is not set");
        home.join("screenshots")
    });

    fs::create_dir_all(FOLDER.get().unwrap())?;

    println!("[+] nanoshot is running, saving screenshots to `{:?}'", FOLDER.get().unwrap());

    let el = EventLoop::<TrayIconEvent>::with_user_event().build().unwrap();
    let proxy: EventLoopProxy<_> = el.create_proxy();
    TrayIconEvent::set_event_handler(Some(move |e| { let _ = proxy.send_event(e); }));
    el.run_app(&mut App::default()).unwrap();
    Ok(())
}

// svg->png
// brew install librsvg
// rsvg-convert --width=32 --height=32 input.svg --output=icon.png
