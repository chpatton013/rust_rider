extern crate graphics;
extern crate nalgebra;
extern crate piston;
extern crate piston_window;

use std::cell::RefCell;
use std::rc::Rc;

use error;
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
  use self::graphics::Transformed; // piston_window::Context.{trans,orient}

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
  line_segments: Vec<LineSegment>,
  active_line_segment: Option<Point>,
  mouse_position: Point,
}

impl State {
  /// Create a State with default values for a new game.
  pub fn new() -> State {
    State {
      edit_mode: EditMode::Insert,
      line_segments: Vec::new(),
      active_line_segment: None,
      mouse_position: Point::new(0.0, 0.0),
    }
  }
}

pub struct GameMode<Window>
where
  Window: piston_window::Window,
{
  state: State,
  window: Rc<RefCell<piston_window::PistonWindow<Window>>>,
}

/// How GameMode responds to input-events.
impl<Window> handler::InputHandler for GameMode<Window>
where Window: piston_window::Window,
{
  fn on_button<Event: piston_window::GenericEvent>(
    &mut self,
    _event: &Event,
    _button_args: &piston_window::ButtonArgs,
  ) -> error::Result<()> {
    Ok(())
  }

  fn on_controller_axis<Event: piston_window::GenericEvent>(
    &mut self,
    _event: &Event,
    _controller_axis_args: &piston_window::ControllerAxisArgs,
  ) -> error::Result<()> {
    Ok(())
  }

  fn on_mouse_cursor<Event: piston_window::GenericEvent>(
    &mut self,
    _event: &Event,
    position: &[f64; 2],
  ) -> error::Result<()> {
    self.state.mouse_position = Point::new(position[0], position[1]);

    Ok(())
  }

  fn on_mouse_relative<Event: piston_window::GenericEvent>(
    &mut self,
    _event: &Event,
    _relative: &[f64; 2],
  ) -> error::Result<()> {
    Ok(())
  }

  fn on_mouse_scroll<Event: piston_window::GenericEvent>(
    &mut self,
    _event: &Event,
    _scroll: &[f64; 2],
  ) -> error::Result<()> {
    Ok(())
  }

  fn on_press<Event: piston_window::GenericEvent>(
    &mut self,
    _event: &Event,
    button: &piston_window::Button,
  ) -> error::Result<()> {
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

    Ok(())
  }

  fn on_release<Event: piston_window::GenericEvent>(
    &mut self,
    _event: &Event,
    button: &piston_window::Button,
  ) -> error::Result<()> {
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
              self.state.line_segments.push(
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

    Ok(())
  }

  fn on_text<Event: piston_window::GenericEvent>(
    &mut self,
    _event: &Event,
    _text: &String,
  ) -> error::Result<()> {
    Ok(())
  }

  fn on_touch<Event: piston_window::GenericEvent>(
    &mut self,
    _event: &Event,
    _touch_args: &piston_window::TouchArgs,
  ) -> error::Result<()> {
    Ok(())
  }
}

/// How GameMode responds to update-events.
impl<Window> handler::UpdateHandler for GameMode<Window>
where Window: piston_window::Window,
{
  fn on_idle<Event: piston_window::GenericEvent>(
    &mut self,
    _event: &Event,
    _idle_args: &piston_window::IdleArgs,
  ) -> error::Result<()> {
    Ok(())
  }

  fn on_update<Event: piston_window::GenericEvent>(
    &mut self,
    _event: &Event,
    _update_args: &piston_window::UpdateArgs,
  ) -> error::Result<()> {
    Ok(())
  }
}

/// How GameMode responds to window-events.
impl<Window> handler::WindowHandler for GameMode<Window>
where Window: piston_window::OpenGLWindow,
{
  fn on_after_render<Event: piston_window::GenericEvent>(
    &mut self,
    _event: &Event,
    _after_render_args: &piston_window::AfterRenderArgs,
  ) -> error::Result<()> {
    Ok(())
  }

  fn on_close<Event: piston_window::GenericEvent>(
    &mut self,
    _event: &Event,
    _close_args: &piston_window::CloseArgs,
  ) -> error::Result<()> {
    Ok(())
  }

  fn on_cursor<Event: piston_window::GenericEvent>(
    &mut self,
    _event: &Event,
    _cursor: bool,
  ) -> error::Result<()> {
    Ok(())
  }

  fn on_focus<Event: piston_window::GenericEvent>(
    &mut self,
    _event: &Event,
    _focus: bool,
  ) -> error::Result<()> {
    Ok(())
  }

  fn on_render<Event: piston_window::GenericEvent>(
    &mut self,
    event: &Event,
    _render_args: &piston_window::RenderArgs,
  ) -> error::Result<()> {
    use piston_window::Window; // size

    // Borrow member references immutably before allowing self to be borrowed
    // mutably by self.window.draw_2d().
    let state = &self.state;
    let window_size = self.window.borrow().size();

    self.window.borrow_mut().draw_2d(event, |context, graphics| {
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

    Ok(())
  }

  fn on_resize<Event: piston_window::GenericEvent>(
    &mut self,
    _event: &Event,
    _size: &[u32; 2],
  ) -> error::Result<()> {
    Ok(())
  }
}

/// Inherit default implementation of EventHandler::on_event.
impl<Window> handler::EventHandler for GameMode<Window>
where Window: piston_window::OpenGLWindow,
{}

impl<Window> GameMode<Window>
where
  Window: piston_window::Window + piston_window::OpenGLWindow,
{
  /// Create a GameMode for a new game.
  pub fn new(
    window: Rc<RefCell<piston_window::PistonWindow<Window>>>,
  ) -> GameMode<Window> {
    GameMode::new_with_state(window, State::new())
  }

  /// Create a GameMode with an existing State.
  pub fn new_with_state(
    window: Rc<RefCell<piston_window::PistonWindow<Window>>>,
    state: State,
  ) -> GameMode<Window> {
    GameMode {
      window: window,
      state: state,
    }
  }
}
