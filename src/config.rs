extern crate piston_window;
extern crate serde_json;
extern crate std;

use error;

fn default_window_size_width() -> u32 {
  1600
}
fn default_window_size_height() -> u32 {
  1000
}
fn default_window_samples() -> u8 {
  0
}
fn default_window_fullscreen() -> bool {
  false
}
fn default_window_exit_on_esc() -> bool {
  false
}
fn default_window_vsync() -> bool {
  false
}
fn default_window_srgb() -> bool {
  false
}
fn default_window_resizable() -> bool {
  false
}
fn default_window_decorated() -> bool {
  false
}
fn default_window_controllers() -> bool {
  false
}
fn default_event_max_fps() -> u64 {
  60
}
fn default_event_ups() -> u64 {
  120
}
fn default_event_ups_reset() -> u64 {
  2
}
fn default_event_swap_buffers() -> bool {
  true
}
fn default_event_bench_mode() -> bool {
  false
}
fn default_event_lazy() -> bool {
  false
}

#[derive(Debug, Default, Serialize, Deserialize)]
struct WindowConfigSize {
  #[serde(default = "default_window_size_width")]
  width: u32,
  #[serde(default = "default_window_size_height")]
  height: u32,
}

#[derive(Debug, Default, Serialize, Deserialize)]
struct WindowConfig {
  title: String,
  #[serde(default)]
  size: WindowConfigSize,
  #[serde(default = "default_window_samples")]
  samples: u8,
  #[serde(default = "default_window_fullscreen")]
  fullscreen: bool,
  #[serde(default = "default_window_exit_on_esc")]
  exit_on_esc: bool,
  #[serde(default = "default_window_vsync")]
  vsync: bool,
  #[serde(default = "default_window_srgb")]
  srgb: bool,
  #[serde(default = "default_window_resizable")]
  resizable: bool,
  #[serde(default = "default_window_decorated")]
  decorated: bool,
  #[serde(default = "default_window_controllers")]
  controllers: bool,
}

#[derive(Debug, Default, Serialize, Deserialize)]
struct EventConfig {
  #[serde(default = "default_event_max_fps")]
  max_fps: u64,
  #[serde(default = "default_event_ups")]
  ups: u64,
  #[serde(default = "default_event_ups_reset")]
  ups_reset: u64,
  #[serde(default = "default_event_swap_buffers")]
  swap_buffers: bool,
  #[serde(default = "default_event_bench_mode")]
  bench_mode: bool,
  #[serde(default = "default_event_lazy")]
  lazy: bool,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Config {
  #[serde(default)]
  window: WindowConfig,
  #[serde(default)]
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

impl<'config> From<&'config Config> for piston_window::WindowSettings {
  fn from(config: &'config Config) -> piston_window::WindowSettings {
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

impl<'config> From<&'config Config> for piston_window::EventSettings {
  fn from(config: &'config Config) -> piston_window::EventSettings {
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

impl<'config> std::convert::TryFrom<&'config Config>
  for piston_window::PistonWindow {
  type Error = error::Error;

  fn try_from(
    config: &'config Config,
  ) -> error::Result<piston_window::PistonWindow> {
    use piston_window::EventLoop; // set_event_settings

    let window_settings = piston_window::WindowSettings::from(config);
    let event_settings = piston_window::EventSettings::from(config);

    match window_settings.build::<piston_window::PistonWindow>() {
      Ok(mut window) => {
        window.set_event_settings(event_settings);
        Ok(window)
      },
      Err(message) => Err(error::Error::from(message)),
    }
  }
}
