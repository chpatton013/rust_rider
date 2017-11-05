#![cfg_attr(feature = "strict", deny(missing_docs))]
#![cfg_attr(feature = "strict", deny(warnings))]
#![recursion_limit = "1024"]

//! Rust Rider
//!
//! An exploratory clone of Line Rider.
//!
//! Rust Rider is a sandbox-style game where players build courses for their
//! character to ride. The player's only building block in creating their
//! courses are line segments. These segments can be of any length, orientation,
//! or quantity. Together they comprise a course that the character, propelled
//! by gravity, can ride.

#[macro_use]
extern crate error_chain;
extern crate piston_window;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

mod config;
mod error;
mod handler;
mod rust_rider;

fn run() -> error::Result<()> {
  use error::ResultExt;

  let config = config::Config::from_path_str("config.json")
    .chain_err(|| "Failed to create config")?;
  let window_settings = piston_window::WindowSettings::from(&config);
  let event_settings = piston_window::EventSettings::from(&config);

  let mut window: piston_window::PistonWindow =
    window_settings.build().unwrap_or_else(|error| {
      panic!("Failed to build PistonWindow: {}", error)
    });
  {
    use piston_window::EventLoop; // set_event_settings
    window.set_event_settings(event_settings);
  }

  // Infer rust_rider::GameMode's implicit type argument from window.
  rust_rider::GameMode::<_>::new(&mut window).spin();

  Ok(())
}

fn main() {
  if let Err(ref e) = run() {
    use std::io::Write; // writeln
    use error_chain::ChainedError; // display_chain

    writeln!(std::io::stderr(), "{}", e.display_chain())
      .expect("Error writing to stderr");
    std::process::exit(1);
  }
}
