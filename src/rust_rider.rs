extern crate piston_window;

use handler;

const RED: piston_window::types::Color = [1.0, 0.0, 0.0, 1.0];
const BLUE: piston_window::types::Color = [0.0, 0.0, 1.0, 1.0];

/// The game-state of the Rust Rider game. The state should act as the save data
/// for a resumable session of the game.
pub struct State {
  box_color: piston_window::types::Color,
}

impl State {
  /// Create a State with default values for a new game.
  pub fn new() -> State {
    State { box_color: BLUE }
  }
}

pub struct GameMode<'w, W>
where
  W: 'w + piston_window::Window,
{
  state: State,
  window: &'w mut piston_window::PistonWindow<W>,
}

/// How GameMode responds to input-events.
impl<'w, E, W> handler::InputHandler<E> for GameMode<'w, W>
    where E: piston_window::GenericEvent,
          W: 'w + piston_window::Window,
{
  fn on_button(&mut self,
               _event: &E,
               _button_args: &piston_window::ButtonArgs) {}

  fn on_controller_axis(
      &mut self,
      _event: &E,
      _controller_axis_args: &piston_window::ControllerAxisArgs) {}

  fn on_mouse_cursor(&mut self, _event: &E, _position: &[f64; 2]) {}

  fn on_mouse_relative(&mut self, _event: &E, _relative: &[f64; 2]) {}

  fn on_mouse_scroll(&mut self, _event: &E, _scroll: &[f64; 2]) {}

  fn on_press(&mut self, _event: &E, button: &piston_window::Button) {
    match button {
      &piston_window::Button::Keyboard(key) => match key {
        piston_window::Key::Space => { self.state.box_color = RED },
        _ => {},
      },
      _ => {},
    }
  }

  fn on_release(&mut self, _event: &E, button: &piston_window::Button) {
    match button {
      &piston_window::Button::Keyboard(key) => match key {
        piston_window::Key::Space => { self.state.box_color = BLUE },
        _ => {},
      },
      _ => {},
    }
  }

  fn on_text(&mut self, _event: &E, _text: &String) {}

  fn on_touch(&mut self, _event: &E, _touch_args: &piston_window::TouchArgs) {}
}

/// How GameMode responds to update-events.
impl<'w, E, W> handler::UpdateHandler<E> for GameMode<'w, W>
    where E: piston_window::GenericEvent,
          W: 'w + piston_window::Window,
{
  fn on_idle(&mut self, _event: &E, _idle_args: &piston_window::IdleArgs) {}

  fn on_update(&mut self,
               _event: &E,
               _update_args: &piston_window::UpdateArgs) {}
}

/// How GameMode responds to window-events.
impl<'w, E, W> handler::WindowHandler<E> for GameMode<'w, W>
    where E: piston_window::GenericEvent,
          W: 'w + piston_window::OpenGLWindow,
{
  fn on_after_render(&mut self,
                     _event: &E,
                     _after_render_args: &piston_window::AfterRenderArgs) {}

  fn on_close(&mut self, _event: &E, _close_args: &piston_window::CloseArgs) {}

  fn on_cursor(&mut self, _event: &E, _cursor: bool) {}

  fn on_focus(&mut self, _event: &E, _focus: bool) {}

  fn on_render(&mut self, event: &E, _render_args: &piston_window::RenderArgs) {
    // Borrow member references immutably before allowing self to be borrowed
    // mutably by self.window.draw_2d().
    let state = &self.state;

    self.window.draw_2d(event, |context, graphics| {
      piston_window::clear([1.0; 4], graphics);
      piston_window::rectangle(state.box_color,
                               [0.0, 0.0, 100.0, 100.0],
                               context.transform,
                               graphics);
    });
  }

  fn on_resize(&mut self, _event: &E, _size: &[u32; 2]) {}
}

/// Inherit default implementation of EventHandler::on_event.
impl<'w, E, W> handler::EventHandler<E> for GameMode<'w, W>
    where E: piston_window::GenericEvent,
          W: 'w + piston_window::OpenGLWindow,
{}

impl<'w, W> GameMode<'w, W>
where
  W: 'w
    + piston_window::Window
    + piston_window::OpenGLWindow,
{
  /// Create a GameMode for a new game.
  pub fn new(
    window: &'w mut piston_window::PistonWindow<W>,
  ) -> GameMode<'w, W> {
    GameMode::new_with_state(window, State::new())
  }

  /// Create a GameMode with an existing State.
  pub fn new_with_state(
    window: &'w mut piston_window::PistonWindow<W>,
    state: State,
  ) -> GameMode<'w, W> {
    GameMode {
      window: window,
      state: state,
    }
  }

  /// Process events from window until termination.
  /// TODO: Move this into a Game struct so we can switch game modes.
  pub fn spin(&mut self) {
    while let Some(event) = self.window.next() {
      // Bring on_event() into scope.
      use handler::EventHandler;
      self.on_event(&event);
    }
  }
}
