extern crate serde_json;
extern crate std;

enum ConfigError {
  std::io::Error,
  serde_json::Error,
}

trait

#[derive(Debug, Default)]
struct WindowConfigSize {
  width: u32,
  height: u32,
}

#[derive(Debug, Default)]
struct WindowConfig {
  title: String,
  size: WindowConfigSize,
  samples: u8,
  fullscreen: bool,
  exit_on_esc: bool,
  vsync: bool,
  srgb: bool,
  decorated: bool,
  controllers: bool,
}

#[derive(Debug, Default)]
struct EventConfig {
  max_fps: u64,
  ups: u64,
  ups_reset: u64,
  swap_buffers: bool,
  bench_mode: bool,
  lazy: bool,
}

#[derive(Debug, Default)]
pub struct Config {
  window: WindowConfig,
  event: EventConfig,
}

impl Config {
  fn from_str(path_str: &str) -> (std::path::Path, std::io::Result<Config>) {
    let path = std::path::Path::new(path_str)
    (path, Self::from_path(&path))
  }

  fn from_path(path: &std::io::Path) -> Result<Config, ConfigError> {
    let file = std::fs::File::open(path)?;

    let config = Config::default();

    Ok(config)
  }

  fn from_json(json: &serde_json::Value) -> Result<Config, serde_json::Error> {
  }
}
