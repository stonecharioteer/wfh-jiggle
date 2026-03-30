use log::{debug, info, warn};
use mouse_rs::{types::Point, Mouse};
use std::{env, io, num::NonZeroU64, thread, time::Duration};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "jiggle",
    about = "A tiny program that jiggles your mouse every \
    few seconds so that your computer doesn't get locked.\
    Use `RUST_LOG=debug` to output logs."
)]
struct Opt {
    /// Seconds to wait before moving your mouse.
    /// default: 10s, can also be set by using the `JIGGLE_SLEEP`
    /// environment variable.
    seconds: Option<NonZeroU64>,
}

#[cfg(target_os = "macos")]
const PLATFORM_HINT: Option<&str> =
    Some("On macOS, grant Accessibility permission to the terminal or app running jiggle.");
#[cfg(target_os = "linux")]
const PLATFORM_HINT: Option<&str> = Some(
    "On Linux, install libxdo and run in an X11 session (Wayland may block synthetic mouse input).",
);
#[cfg(target_os = "windows")]
const PLATFORM_HINT: Option<&str> = None;
#[cfg(not(any(target_os = "macos", target_os = "linux", target_os = "windows")))]
const PLATFORM_HINT: Option<&str> = None;

fn resolve_sleep_duration(opt: &Opt) -> u64 {
    if let Some(seconds) = opt.seconds {
        return seconds.get();
    }

    if let Ok(value) = env::var("JIGGLE_SLEEP") {
        match value.parse::<u64>().ok().and_then(NonZeroU64::new) {
            Some(seconds) => return seconds.get(),
            None => {
                warn!(
                    "Ignoring invalid JIGGLE_SLEEP value '{value}'. Expected a non-zero positive integer."
                );
            }
        }
    }

    10
}

fn jiggle_once(mouse_controller: &Mouse) -> Result<(), Box<dyn std::error::Error>> {
    let old_position = mouse_controller.get_position()?;

    for new_position in [
        Point {
            x: old_position.x.saturating_add(1),
            y: old_position.y.saturating_add(1),
        },
        Point {
            x: old_position.x.saturating_sub(1),
            y: old_position.y.saturating_sub(1),
        },
    ] {
        if mouse_controller
            .move_to(new_position.x, new_position.y)
            .is_ok()
        {
            return mouse_controller.move_to(old_position.x, old_position.y);
        }
    }

    Err(io::Error::new(
        io::ErrorKind::Other,
        "unable to move mouse cursor in either direction",
    )
    .into())
}

fn main() {
    let opt = Opt::from_args();
    env_logger::init();
    debug!("Jiggle started.");

    let mouse_controller = Mouse::new();
    let sleep_duration = resolve_sleep_duration(&opt);

    info!("Jiggling every {sleep_duration}s.");
    let mut printed_platform_hint = false;
    loop {
        debug!("Moving the mouse a tiny bit.");
        if let Err(err) = jiggle_once(&mouse_controller) {
            warn!("Could not jiggle mouse: {err}");
            if !printed_platform_hint {
                if let Some(hint) = PLATFORM_HINT {
                    warn!("{hint}");
                }
                printed_platform_hint = true;
            }
        }
        debug!("Sleeping for {sleep_duration}s.");
        thread::sleep(Duration::from_secs(sleep_duration));
    }
}
