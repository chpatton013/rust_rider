extern crate piston_window;
extern crate serde_json;
extern crate std;

use error;

const DEFAULT_WINDOW_TITLE: &str = "Rust Rider";
const DEFAULT_WINDOW_WIDTH: u32 = 1600;
const DEFAULT_WINDOW_HEIGHT: u32 = 1000;
const DEFAULT_WINDOW_SAMPLES: u8 = 0;
const DEFAULT_WINDOW_FULLSCREEN: bool = false;
const DEFAULT_WINDOW_EXIT_ON_ESC: bool = false;
const DEFAULT_WINDOW_VSYNC: bool = false;
const DEFAULT_WINDOW_SRGB: bool = false;
const DEFAULT_WINDOW_RESIZABLE: bool = false;
const DEFAULT_WINDOW_DECORATED: bool = false;
const DEFAULT_WINDOW_CONTROLLERS: bool = false;

const DEFAULT_EVENT_MAX_FPS: u64 = 60;
const DEFAULT_EVENT_UPS: u64 = 120;
const DEFAULT_EVENT_UPS_RESET: u64 = 2;
const DEFAULT_EVENT_SWAP_BUFFERS: bool = true;
const DEFAULT_EVENT_BENCH_MODE: bool = false;
const DEFAULT_EVENT_LAZY: bool = false;

#[derive(Debug, Default, Serialize, Deserialize)]
struct WindowConfigSize {
  width: u32,
  height: u32,
}

#[derive(Debug, Default, Serialize, Deserialize)]
struct WindowConfig {
  title: String,
  size: WindowConfigSize,
  samples: u8,
  fullscreen: bool,
  exit_on_esc: bool,
  vsync: bool,
  srgb: bool,
  resizable: bool,
  decorated: bool,
  controllers: bool,
}

#[derive(Debug, Default, Serialize, Deserialize)]
struct EventConfig {
  max_fps: u64,
  ups: u64,
  ups_reset: u64,
  swap_buffers: bool,
  bench_mode: bool,
  lazy: bool,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Config {
  window: WindowConfig,
  event: EventConfig,
}

impl Config {
  pub fn from_path_str(path_str: &str) -> error::Result<Config> {
    Self::from_path(&std::path::Path::new(path_str))
  }

  pub fn from_path(path: &std::path::Path) -> error::Result<Config> {
    let file = std::fs::File::open(path)?;
    let config = serde_json::from_reader(file)?;
    Ok(config)
  }

  pub fn from_json_str(json_str: &str) -> error::Result<Config> {
    let config = serde_json::from_str(json_str)?;
    Ok(config)
  }
}

impl<'a> From<&'a Config> for piston_window::WindowSettings {
  fn from(config: &'a Config) -> piston_window::WindowSettings {
    let window_settings = piston_window::WindowSettings::new(
      config.window.title.as_str(),
      piston_window::Size {
        width: config.window.size.width,
        height: config.window.size.height,
      },
    ).samples(config.window.samples)
      .fullscreen(config.window.fullscreen)
      .exit_on_esc(config.window.exit_on_esc)
      .vsync(config.window.vsync)
      .opengl(piston_window::OpenGL::V3_2)
      .srgb(config.window.srgb)
      .resizable(config.window.resizable)
      .decorated(config.window.decorated)
      .controllers(config.window.controllers);

    window_settings
  }
}

impl<'a> From<&'a Config> for piston_window::EventSettings {
  fn from(config: &'a Config) -> piston_window::EventSettings {
    piston_window::EventSettings {
      max_fps: config.event.max_fps,
      ups: config.event.ups,
      ups_reset: config.event.ups_reset,
      swap_buffers: config.event.swap_buffers,
      bench_mode: config.event.bench_mode,
      lazy: config.event.lazy,
    }
  }
}
