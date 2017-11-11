extern crate piston_window;

use std::cell::RefCell;
use std::collections::BTreeMap;
use std::rc::Rc;

use error;
use handler;

pub struct Application<Window, EventHandler>
where
  Window: piston_window::Window,
  EventHandler: handler::EventHandler,
{
  window: Rc<RefCell<piston_window::PistonWindow<Window>>>,
  application_modes: BTreeMap<String, Box<EventHandler>>,
  active_application_mode: Option<String>,
}

impl<Window, EventHandler> Application<Window, EventHandler>
where
  Window: piston_window::Window,
  EventHandler: handler::EventHandler,
{
  /// Create a Application with default values.
  pub fn new(window: Rc<RefCell<piston_window::PistonWindow<Window>>>) -> Self {
    Application {
      window: window,
      application_modes: BTreeMap::new(),
      active_application_mode: None,
    }
  }

  pub fn add_application_mode(
    &mut self,
    name: &str,
    application_mode: Box<EventHandler>,
    ) -> error::Result<()> {
    match self.application_modes.insert(String::from(name), application_mode) {
      Some(_) => Err(error::Error::from(
          format!("Application mode {} already exists", name),
          )),
      None => Ok(()),
    }
  }

  pub fn remove_application_mode(
    &mut self,
    name: &str,
  ) -> error::Result<Box<EventHandler>> {
    match self.active_application_mode {
      Some(ref mut active_name) => {
        if name == active_name {
          return Err(error::Error::from(
              format!("Application mode {} is currently active", name),
              ));
        }
      },
      None => {},
    }

    match self.application_modes.remove(name) {
      Some(application_mode) => Ok(application_mode),
      None => Err(error::Error::from(
          format!("Application mode {} does not exist", name),
          )),
    }
  }

  pub fn get_application_mode(
    &mut self,
    name: &str,
  ) -> Option<&mut Box<EventHandler>> {
    self.application_modes.get_mut(name)
  }

  pub fn get_active_application_mode(
    &mut self,
  ) -> Option<&mut Box<EventHandler>> {
    match self.active_application_mode.to_owned() {
      Some(active_name) => self.get_application_mode(&active_name),
      None => None,
    }
  }

  pub fn set_active_application_mode(
    &mut self,
    name: &str,
  ) -> error::Result<&mut Box<EventHandler>> {
    match self.active_application_mode {
      Some(ref active_name) => {
        if name == active_name {
          return Err(error::Error::from(
              format!("Application mode {} is already active", name),
              ));
        }
      },
      None => {},
    }

    match self.get_application_mode(name) {
      Some(application_mode) => Ok(application_mode),
      None => Err(error::Error::from(
          format!("No application mode named {}", name),
          )),
    }
  }

  pub fn next(&mut self) -> Option<piston_window::Event> {
    self.window.borrow_mut().next()
  }

  /// Process events from window until termination.
  /// Returns early if active_application_mode is None at any time.
  pub fn spin(&mut self) -> error::Result<()> {
    if self.active_application_mode.is_none() {
      return Err(error::Error::from(
          "Cannot spin with no active application mode",
          ));
    }

    while let Some(event) = self.next() {
      match self.get_active_application_mode() {
        Some(application_mode) => {
          application_mode.on_event(&event)?;
        },
        None => {
          return Err(error::Error::from(
              "Cannot handle event without active application mode",
              ));
        },
      }
    }

    Ok(())
  }
}
