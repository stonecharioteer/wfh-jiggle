use log::debug;
use std::{env, num::NonZeroU64, thread, time::Duration};
use structopt::StructOpt;

use mouse_rs::{types::Point, Mouse};

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

fn main() {
    // Get the current mouse location.
    let opt = Opt::from_args();
    env_logger::init();
    debug!("Jiggle started.");

    let mouse_controller = Mouse::new();
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
    loop {
        let old_position = mouse_controller.get_position().unwrap();
        let new_position = Point {
            x: old_position.x + 1,
            y: old_position.y + 1,
        };
        debug!("Moving the mouse a tiny bit.");
        _ = mouse_controller.move_to(new_position.x, new_position.y);
        _ = mouse_controller.move_to(old_position.x, old_position.y);
        debug!("Sleeping for {sleep_duration}s.");
        thread::sleep(Duration::new(sleep_duration, 0));
    }
}
