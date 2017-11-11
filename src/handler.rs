extern crate piston_window;

use error;

/// An interface describing all the different input-events that can be handled.
pub trait InputHandler {
  fn on_button<Event: piston_window::GenericEvent>(
    &mut self,
    event: &Event,
    button_args: &piston_window::ButtonArgs,
  ) -> error::Result<()>;

  fn on_controller_axis<Event: piston_window::GenericEvent>(
    &mut self,
    event: &Event,
    controller_axis_args: &piston_window::ControllerAxisArgs,
  ) -> error::Result<()>;

  fn on_mouse_cursor<Event: piston_window::GenericEvent>(
    &mut self,
    event: &Event,
    position: &[f64; 2],
  ) -> error::Result<()>;

  fn on_mouse_relative<Event: piston_window::GenericEvent>(
    &mut self,
    event: &Event,
    relative: &[f64; 2],
  ) -> error::Result<()>;

  fn on_mouse_scroll<Event: piston_window::GenericEvent>(
    &mut self,
    event: &Event,
    scroll: &[f64; 2],
  ) -> error::Result<()>;

  fn on_press<Event: piston_window::GenericEvent>(
    &mut self,
    event: &Event,
    button: &piston_window::Button,
  ) -> error::Result<()>;

  fn on_release<Event: piston_window::GenericEvent>(
    &mut self,
    event: &Event,
    button: &piston_window::Button,
  ) -> error::Result<()>;

  fn on_text<Event: piston_window::GenericEvent>(
    &mut self,
    event: &Event,
    text: &String,
  ) -> error::Result<()>;

  fn on_touch<Event: piston_window::GenericEvent>(
    &mut self,
    event: &Event,
    touch_args: &piston_window::TouchArgs,
  ) -> error::Result<()>;
}

/// An interface describing all the different update-events that can be handled.
pub trait UpdateHandler {
  fn on_idle<Event: piston_window::GenericEvent>(
    &mut self,
    event: &Event,
    idle_args: &piston_window::IdleArgs,
  ) -> error::Result<()>;

  fn on_update<Event: piston_window::GenericEvent>(
    &mut self,
    event: &Event,
    update_args: &piston_window::UpdateArgs,
  ) -> error::Result<()>;
}

/// An interface describing all the different window-events that can be handled.
pub trait WindowHandler {
  fn on_after_render<Event: piston_window::GenericEvent>(
    &mut self,
    event: &Event,
    after_render_args: &piston_window::AfterRenderArgs,
  ) -> error::Result<()>;

  fn on_close<Event: piston_window::GenericEvent>(
    &mut self,
    event: &Event,
    close_args: &piston_window::CloseArgs,
  ) -> error::Result<()>;

  fn on_cursor<Event: piston_window::GenericEvent>(
    &mut self,
    event: &Event,
    cursor: bool,
  ) -> error::Result<()>;

  fn on_focus<Event: piston_window::GenericEvent>(
    &mut self,
    event: &Event,
    focus: bool,
  ) -> error::Result<()>;

  fn on_render<Event: piston_window::GenericEvent>(
    &mut self,
    event: &Event,
    render_args: &piston_window::RenderArgs,
  ) -> error::Result<()>;

  fn on_resize<Event: piston_window::GenericEvent>(
    &mut self,
    event: &Event,
    size: &[u32; 2],
  ) -> error::Result<()>;
}

/// An interface that dispatches events to more specific handlers.
pub trait EventHandler: InputHandler + UpdateHandler + WindowHandler {
  fn on_event<Event: piston_window::GenericEvent>(
    &mut self,
    event: &Event,
  ) -> error::Result<()> {
    // Dispatch input events to InputHandler functions.
    if let Some(button) = event.button_args() {
      return self.on_button::<Event>(&event, &button);
    }
    if let Some(controller_axis) = event.controller_axis_args() {
      return self.on_controller_axis::<Event>(&event, &controller_axis);
    }
    if let Some(mouse_cursor) = event.mouse_cursor_args() {
      return self.on_mouse_cursor::<Event>(&event, &mouse_cursor);
    }
    if let Some(mouse_relative) = event.mouse_relative_args() {
      return self.on_mouse_relative::<Event>(&event, &mouse_relative);
    }
    if let Some(mouse_scroll) = event.mouse_scroll_args() {
      return self.on_mouse_scroll::<Event>(&event, &mouse_scroll);
    }
    if let Some(press) = event.press_args() {
      return self.on_press::<Event>(&event, &press);
    }
    if let Some(release) = event.release_args() {
      return self.on_release::<Event>(&event, &release);
    }
    if let Some(text) = event.text_args() {
      return self.on_text::<Event>(&event, &text);
    }
    if let Some(touch) = event.touch_args() {
      return self.on_touch::<Event>(&event, &touch);
    }

    // Dispatch update events to UpdateHandler functions.
    if let Some(idle) = event.idle_args() {
      return self.on_idle::<Event>(&event, &idle);
    }
    if let Some(update) = event.update_args() {
      return self.on_update::<Event>(&event, &update);
    }

    // Dispatch window events to WindowHandler functions.
    if let Some(after_render) = event.after_render_args() {
      return self.on_after_render::<Event>(&event, &after_render);
    }
    if let Some(close) = event.close_args() {
      return self.on_close::<Event>(&event, &close);
    }
    if let Some(cursor) = event.cursor_args() {
      return self.on_cursor::<Event>(&event, cursor);
    }
    if let Some(focus) = event.focus_args() {
      return self.on_focus::<Event>(&event, focus);
    }
    if let Some(render) = event.render_args() {
      return self.on_render::<Event>(&event, &render);
    }
    if let Some(resize) = event.resize_args() {
      return self.on_resize::<Event>(&event, &resize);
    }

    Err(error::Error::from("Failed to discover event type"))
  }
}
