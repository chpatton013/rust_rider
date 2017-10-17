extern crate graphics;
extern crate nalgebra;
extern crate piston;
extern crate piston_window;

use std::collections::VecDeque;

use handler;

enum EditMode {
  Insert,
  Select,
}

type Point = nalgebra::Point2<f64>;
type Vector = nalgebra::Vector2<f64>;

fn draw_line_segment<G>(
  point1: &Point,
  point2: &Point,
  context: &piston_window::Context,
  graphics: &mut G,
) where
  G: graphics::Graphics,
{
  // Bring context.trans() and context.orient() into scope.
  use self::graphics::Transformed;
  let tangent = point2 - point1;
  let width = nalgebra::distance(point1, point2);
  let height = 4.0;
  piston_window::rectangle(
    BLACK,
    [0.0, 0.0, width, height],
    context
      .trans(point1.x, point1.y - height / 2.0)
      .orient(tangent.x, tangent.y)
      .transform,
    graphics,
  );
}

struct LineSegment {
  point1: Point,
  point2: Point,
}

impl LineSegment {
  pub fn new(point1: Point, point2: Point) -> LineSegment {
    LineSegment {
      point1: point1,
      point2: point2,
    }
  }

  pub fn draw<G>(&self, context: &piston_window::Context, graphics: &mut G)
  where
    G: graphics::Graphics,
  {
    draw_line_segment(&self.point1, &self.point2, context, graphics);
  }
}

const BLACK: piston_window::types::Color = [0.0, 0.0, 0.0, 1.0];
const GREEN: piston_window::types::Color = [0.0, 1.0, 0.0, 1.0];
const BLUE: piston_window::types::Color = [0.0, 0.0, 1.0, 1.0];

/// The game-state of the Rust Rider game. The state should act as the save data
/// for a resumable session of the game.
pub struct State {
  edit_mode: EditMode,
  line_segments: VecDeque<LineSegment>,
  active_line_segment: Option<Point>,
  mouse_position: Point,
}

impl State {
  /// Create a State with default values for a new game.
  pub fn new() -> State {
    State {
      edit_mode: EditMode::Insert,
      line_segments: VecDeque::new(),
      active_line_segment: None,
      mouse_position: Point::new(0.0, 0.0),
    }
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
               _button_args: &piston_window::ButtonArgs) {
  }

  fn on_controller_axis(
    &mut self,
    _event: &E,
    _controller_axis_args: &piston_window::ControllerAxisArgs) {}

  fn on_mouse_cursor(&mut self, _event: &E, position: &[f64; 2]) {
    self.state.mouse_position = Point::new(position[0], position[1]);
  }

  fn on_mouse_relative(&mut self, _event: &E, _relative: &[f64; 2]) {}

  fn on_mouse_scroll(&mut self, _event: &E, _scroll: &[f64; 2]) {}

  fn on_press(&mut self, _event: &E, button: &piston_window::Button) {
    match button {
      &piston_window::Button::Keyboard(key) => match key {
        piston_window::Key::LShift | piston_window::Key::RShift => {
          self.state.edit_mode = EditMode::Select;
        },
        _ => {},
      },
      &piston_window::Button::Mouse(mouse_button) => match mouse_button {
        piston_window::MouseButton::Left => {
          self.state.active_line_segment = Some(self.state.mouse_position);
        },
        _ => {},
      },
      _ => {},
    }
  }

  fn on_release(&mut self, _event: &E, button: &piston_window::Button) {
    match button {
      &piston_window::Button::Keyboard(key) => match key {
        piston_window::Key::LShift | piston_window::Key::RShift => {
          self.state.edit_mode = EditMode::Insert;
        },
        _ => {},
      },
      &piston_window::Button::Mouse(mouse_button) => match mouse_button {
        piston_window::MouseButton::Left => {
          match self.state.active_line_segment {
            Some(point1) => {
              self.state.line_segments.push_back(
                LineSegment::new(point1, self.state.mouse_position));
              self.state.active_line_segment = None;
            }
            None => {}
          }
        },
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
    // Bring draw_size() into scope.
    use piston_window::Window;
    let window_size = self.window.size();

    self.window.draw_2d(event, |context, graphics| {
      let edit_bar_color = match state.edit_mode {
        EditMode::Insert => GREEN,
        EditMode::Select => BLUE,
      };
      let edit_bar_width = window_size.width;
      let edit_bar_height = 20;
      let edit_bar_x_offset = 0;
      let edit_bar_y_offset = window_size.height - edit_bar_height;

      piston_window::clear([1.0; 4], graphics);
      piston_window::rectangle(
        edit_bar_color,
        [
          edit_bar_x_offset as f64,
          edit_bar_y_offset as f64,
          edit_bar_width as f64,
          edit_bar_height as f64,
        ],
        context.transform,
        graphics,
      );

      match state.active_line_segment {
        Some(point1) => {
          draw_line_segment(&point1, &state.mouse_position, &context, graphics);
        }
        None => {}
      }

      for line in state.line_segments.iter() {
        line.draw(&context, graphics);
      }
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
