#![cfg_attr(feature = "strict", deny(missing_docs))]
#![cfg_attr(feature = "strict", deny(warnings))]
#![feature(try_from)]
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

mod application;
mod config;
mod error;
mod handler;
mod rust_rider;

use std::cell::RefCell;
use std::rc::Rc;

fn run() -> error::Result<()> {
  use error::ResultExt; // chain_err
  use std::convert::TryFrom; // try_from

  let config = config::Config::from_path_str("config.json").chain_err(|| {
    "Failed to create config"
  })?;
  let window = Rc::new(RefCell::new(
    piston_window::PistonWindow::try_from(&config).chain_err(
      || {
        "Failed to build window"
      },
    )?,
  ));

  let mut app = application::Application::<_, _>::new(window.clone());
  app
    .add_application_mode(
      "rust_rider",
      Box::new(rust_rider::GameMode::<_>::new(window.clone())),
    )
    .chain_err(|| "Failed to add rust rider application mode")?;
  app.set_active_application_mode("rust_rider").chain_err(
    || {
      "Failed to activate rust rider application mode"
    },
  )?;
  app.spin().chain_err(|| "Failed to spin")?;

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
