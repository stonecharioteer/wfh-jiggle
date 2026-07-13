use log::debug;
use std::{
    env,
    num::NonZeroU64,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread,
    time::Duration,
};
use structopt::StructOpt;

use mouse_rs::Mouse;
use tray_icon::{
    menu::{Menu, MenuEvent, MenuItem, PredefinedMenuItem},
    TrayIconBuilder,
};
use winit::{
    application::ApplicationHandler,
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
};

#[derive(Debug, StructOpt)]
#[structopt(
    name = "jiggle",
    about = "Jiggles your mouse to keep your Mac awake. Use `RUST_LOG=debug` for logs."
)]
struct Opt {
    /// Seconds between mouse movements (default: 10, or $JIGGLE_SLEEP)
    seconds: Option<NonZeroU64>,
}

fn make_icon(active: bool) -> tray_icon::Icon {
    const SIZE: u32 = 22;
    let cx = SIZE as f32 / 2.0;
    let cy = SIZE as f32 / 2.0;
    let r = cx - 1.5;
    // green when active, gray when paused
    let color: [u8; 4] = if active {
        [52, 211, 153, 255]
    } else {
        [150, 150, 150, 200]
    };
    let mut rgba = Vec::with_capacity((SIZE * SIZE * 4) as usize);
    for y in 0..SIZE {
        for x in 0..SIZE {
            let dx = x as f32 + 0.5 - cx;
            let dy = y as f32 + 0.5 - cy;
            if dx * dx + dy * dy <= r * r {
                rgba.extend_from_slice(&color);
            } else {
                rgba.extend_from_slice(&[0, 0, 0, 0]);
            }
        }
    }
    tray_icon::Icon::from_rgba(rgba, SIZE, SIZE).unwrap()
}

struct App {
    quit_id: tray_icon::menu::MenuId,
    toggle_id: tray_icon::menu::MenuId,
    toggle_item: MenuItem,
    paused: Arc<AtomicBool>,
    tray: tray_icon::TrayIcon,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, _: &ActiveEventLoop) {}

    fn window_event(
        &mut self,
        _: &ActiveEventLoop,
        _: winit::window::WindowId,
        _: winit::event::WindowEvent,
    ) {
    }

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        event_loop.set_control_flow(ControlFlow::WaitUntil(
            std::time::Instant::now() + Duration::from_millis(200),
        ));
        if let Ok(event) = MenuEvent::receiver().try_recv() {
            if event.id == self.quit_id {
                event_loop.exit();
            } else if event.id == self.toggle_id {
                let was_paused = self.paused.fetch_xor(true, Ordering::Relaxed);
                if was_paused {
                    // resuming
                    self.toggle_item.set_text("Pause");
                    self.tray.set_icon(Some(make_icon(true))).unwrap();
                    self.tray
                        .set_tooltip(Some("Jiggle — keeping your Mac awake"))
                        .unwrap();
                } else {
                    // pausing
                    self.toggle_item.set_text("Resume");
                    self.tray.set_icon(Some(make_icon(false))).unwrap();
                    self.tray.set_tooltip(Some("Jiggle — paused")).unwrap();
                }
            }
        }
    }
}

fn main() {
    let opt = Opt::from_args();
    env_logger::init();
    debug!("Jiggle started.");

    let sleep_duration = opt
        .seconds
        .or_else(|| {
            env::var("JIGGLE_SLEEP")
                .ok()
                .and_then(|var| var.parse().ok())
                .and_then(NonZeroU64::new)
        })
        .map(NonZeroU64::get)
        .unwrap_or(10);

    debug!("Sleep duration is set to {sleep_duration}s.");

    let paused = Arc::new(AtomicBool::new(false));
    let paused_thread = Arc::clone(&paused);

    thread::spawn(move || {
        let mouse = Mouse::new();
        let mut elapsed: u64 = 0;
        loop {
            thread::sleep(Duration::from_secs(1));
            if paused_thread.load(Ordering::Relaxed) {
                elapsed = 0;
                continue;
            }
            elapsed += 1;
            if elapsed >= sleep_duration {
                elapsed = 0;
                let old = mouse.get_position().unwrap();
                debug!("Moving the mouse a tiny bit.");
                mouse.move_to(old.x + 1, old.y + 1).unwrap();
                mouse.move_to(old.x, old.y).unwrap();
                debug!("Sleeping for {sleep_duration}s.");
            }
        }
    });

    let toggle_item = MenuItem::new("Pause", true, None);
    let quit_item = MenuItem::new("Quit Jiggle", true, None);
    let status_item = MenuItem::new(format!("Jiggling every {sleep_duration}s"), false, None);
    let menu = Menu::new();
    menu.append_items(&[
        &status_item,
        &PredefinedMenuItem::separator(),
        &toggle_item,
        &quit_item,
    ])
    .unwrap();
    let toggle_id = toggle_item.id().clone();
    let quit_id = quit_item.id().clone();

    let event_loop = EventLoop::new().unwrap();
    let tray = TrayIconBuilder::new()
        .with_menu(Box::new(menu))
        .with_tooltip("Jiggle — keeping your Mac awake")
        .with_icon(make_icon(true))
        .build()
        .unwrap();

    let mut app = App {
        quit_id,
        toggle_id,
        toggle_item,
        paused,
        tray,
    };
    event_loop.run_app(&mut app).unwrap();
}
