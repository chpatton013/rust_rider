extern crate piston_window;

use std::collections::BTreeMap;

use error;
use handler;

pub struct Application<'window, EventType, WindowType>
where
EventType: piston_window::GenericEvent,
WindowType: 'window + piston_window::Window,
{
  window: &'window mut piston_window::PistonWindow<WindowType>,
  application_modes: BTreeMap<String, Box<handler::EventHandler<EventType>>>,
  active_application_mode: Option<String>,
}

impl<'window, EventType, WindowType> Application<'window, EventType, WindowType>
where
EventType: piston_window::GenericEvent,
WindowType: 'window + piston_window::Window,
{
  /// Create a Application with default values.
  pub fn new(window: &'window mut piston_window::PistonWindow<WindowType>)
      -> Self
  {
    Application{
      window: window,
      application_modes: BTreeMap::new(),
      active_application_mode: None,
    }
  }

  fn get_active_application_mode(&mut self)
      -> Option<&mut Box<handler::EventHandler<EventType>>>
  {
    match self.active_application_mode {
      Some(active_name) => self.application_modes.get_mut(&active_name),
      None => None,
    }
  }

  pub fn add_application_mode(
    &mut self,
    name: String,
    application_mode: Box<handler::EventHandler<EventType>>)
      -> error::Result<()>
  {
    match self.application_modes.insert(name, application_mode) {
      Some(_) => error::Result::new(error::Error::from_kind(error::ErrorKind::Msg(format!("Application mode {} already exists", name)))),
      None => Ok(()),
    }
  }

  pub fn remove_application_mode(&mut self, name: &str)
      -> error::Result<Box<handler::EventHandler<EventType>>> {
    match self.active_application_mode {
      Some(active_name) => if name == active_name {
        return error::ErrorKind::Msg(format!("Application mode {} is currently active", name));
      },
      None => {},
    }

    match self.application_modes.remove(name) {
      Some(application_mode) => application_mode,
      None => error::ErrorKind::Msg(format!("Application mode {} does not exist", name)),
    }
  }

  pub fn set_active_application_mode(&mut self, name: &str) -> error::Result<()> {
    match self.active_application_mode {
      Some(active_name) => if name == active_name {
        return error::ErrorKind::Msg(format!("Application mode {} is already active", name));
      },
      None => {},
    }

    match self.application_modes.get_mut(name) {
      Some(&mut application_mode) => Ok(()),
      None => error::ErrorKind::Msg(format!("No application mode named {}", name)),
    }
  }

  /// Process events from window until termination.
  /// Returns early if active_application_mode is None at any time.
  pub fn spin(&mut self) -> error::Result<()> {
    if self.active_application_mode.is_none() {
      return error::ErrorKind::Msg("Cannot spin with no active application mode");
    }

    while let Some(event) = self.window.next() {
      match self.get_active_application_mode() {
        Some(application_mode) => {
          use handler::EventHandler; // on_event
          // TODO: on_event should return a result
          self.active_application_mode.on_event(&event);
        },
        None => {
          return error::ErrorKind::Msg("Cannot handle event without active application mode");
        },
      }
    }

    Ok(())
  }
}
