#![cfg_attr(feature = "strict", deny(missing_docs))]
#![cfg_attr(feature = "strict", deny(warnings))]

//! Rust Rider
//!
//! An exploratory clone of Line Rider.
//!
//! Rust Rider is a sandbox-style game where players build courses for their
//! character to ride. The player's only building block in creating their
//! courses are line segments. These segments can be of any length, orientation,
//! or quantity. Together they comprise a course that the character, propelled
//! by gravity, can ride.

extern crate piston_window;

mod handler;
mod rust_rider;

const DEFAULT_WINDOW_TITLE: &str = "Rust Rider";
const DEFAULT_WINDOW_SIZE: piston_window::Size = piston_window::Size {
  width: 640,
  height: 480,
};
const DEFAULT_WINDOW_SAMPLES: u8 = 0;
const DEFAULT_WINDOW_FULLSCREEN: bool = false;
const DEFAULT_WINDOW_EXIT_ON_ESC: bool = false;
const DEFAULT_WINDOW_VSYNC: bool = false;
const DEFAULT_WINDOW_SRGB: bool = false;
const DEFAULT_WINDOW_RESIZABLE: bool = false;
const DEFAULT_WINDOW_DECORATED: bool = false;
const DEFAULT_WINDOW_CONTROLLERS: bool = false;

fn make_window_settings() -> piston_window::WindowSettings {
  let window_settings = piston_window::WindowSettings::new(
    DEFAULT_WINDOW_TITLE,
    DEFAULT_WINDOW_SIZE,
  ).samples(DEFAULT_WINDOW_SAMPLES)
    .fullscreen(DEFAULT_WINDOW_FULLSCREEN)
    .exit_on_esc(DEFAULT_WINDOW_EXIT_ON_ESC)
    .vsync(DEFAULT_WINDOW_VSYNC)
    .opengl(piston_window::OpenGL::V3_2)
    .srgb(DEFAULT_WINDOW_SRGB)
    .resizable(DEFAULT_WINDOW_RESIZABLE)
    .decorated(DEFAULT_WINDOW_DECORATED)
    .controllers(DEFAULT_WINDOW_CONTROLLERS);

  window_settings
}

const DEFAULT_EVENT_MAX_FPS: u64 = 60;
const DEFAULT_EVENT_UPS: u64 = 120;
const DEFAULT_EVENT_UPS_RESET: u64 = 2;
const DEFAULT_EVENT_SWAP_BUFFERS: bool = true;
const DEFAULT_EVENT_BENCH_MODE: bool = false;
const DEFAULT_EVENT_LAZY: bool = false;

fn make_event_settings() -> piston_window::EventSettings {
  piston_window::EventSettings {
    max_fps: DEFAULT_EVENT_MAX_FPS,
    ups: DEFAULT_EVENT_UPS,
    ups_reset: DEFAULT_EVENT_UPS_RESET,
    swap_buffers: DEFAULT_EVENT_SWAP_BUFFERS,
    bench_mode: DEFAULT_EVENT_BENCH_MODE,
    lazy: DEFAULT_EVENT_LAZY,
  }
}

fn main() {
  let window_settings = make_window_settings();
  let event_settings = make_event_settings();

  let mut window: piston_window::PistonWindow =
    window_settings.build().unwrap_or_else(|error| {
      panic!("Failed to build PistonWindow: {}", error)
    });
  {
    // Bring set_event_settings() into scope.
    use piston_window::EventLoop;
    window.set_event_settings(event_settings);
  }

  // Infer rust_rider::GameMode's implicit type argument from window.
  rust_rider::GameMode::<_>::new(&mut window).spin();
}
