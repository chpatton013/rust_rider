extern crate piston_window;

/// An interface describing all the different input-events that can be handled.
pub trait InputHandler<E>
where
  E: piston_window::GenericEvent,
{
  fn on_button(&mut self, event: &E, button_args: &piston_window::ButtonArgs);
  fn on_controller_axis(
    &mut self,
    event: &E,
    controller_axis_args: &piston_window::ControllerAxisArgs,
  );
  fn on_mouse_cursor(&mut self, event: &E, position: &[f64; 2]);
  fn on_mouse_relative(&mut self, event: &E, relative: &[f64; 2]);
  fn on_mouse_scroll(&mut self, event: &E, scroll: &[f64; 2]);
  fn on_press(&mut self, event: &E, button: &piston_window::Button);
  fn on_release(&mut self, event: &E, button: &piston_window::Button);
  fn on_text(&mut self, event: &E, text: &String);
  fn on_touch(&mut self, event: &E, touch_args: &piston_window::TouchArgs);
}

/// An interface describing all the different update-events that can be handled.
pub trait UpdateHandler<E>
where
  E: piston_window::GenericEvent,
{
  fn on_idle(&mut self, event: &E, idle_args: &piston_window::IdleArgs);
  fn on_update(&mut self, event: &E, update_args: &piston_window::UpdateArgs);
}

/// An interface describing all the different window-events that can be handled.
pub trait WindowHandler<E>
where
  E: piston_window::GenericEvent,
{
  fn on_after_render(
    &mut self,
    event: &E,
    after_render_args: &piston_window::AfterRenderArgs,
  );
  fn on_close(&mut self, event: &E, close_args: &piston_window::CloseArgs);
  fn on_cursor(&mut self, event: &E, cursor: bool);
  fn on_focus(&mut self, event: &E, focus: bool);
  fn on_render(&mut self, event: &E, render_args: &piston_window::RenderArgs);
  fn on_resize(&mut self, event: &E, size: &[u32; 2]);
}

/// An interface that dispatches events to more specific handlers.
pub trait EventHandler<E> : InputHandler<E> +
                            UpdateHandler<E> +
                            WindowHandler<E>
    where E: piston_window::GenericEvent,
{
  fn on_event(&mut self, event: &E) {
    // Dispatch input events to InputHandler functions.
    if let Some(button) = event.button_args() {
      self.on_button(&event, &button)
    }
    if let Some(controller_axis) = event.controller_axis_args() {
      self.on_controller_axis(&event, &controller_axis)
    }
    if let Some(mouse_cursor) = event.mouse_cursor_args() {
      self.on_mouse_cursor(&event, &mouse_cursor)
    }
    if let Some(mouse_relative) = event.mouse_relative_args() {
      self.on_mouse_relative(&event, &mouse_relative)
    }
    if let Some(mouse_scroll) = event.mouse_scroll_args() {
      self.on_mouse_scroll(&event, &mouse_scroll)
    }
    if let Some(press) = event.press_args() { self.on_press(&event, &press) }
    if let Some(release) = event.release_args() {
      self.on_release(&event, &release)
    }
    if let Some(text) = event.text_args() { self.on_text(&event, &text) }
    if let Some(touch) = event.touch_args() { self.on_touch(&event, &touch) }

    // Dispatch update events to UpdateHandler functions.
    if let Some(idle) = event.idle_args() { self.on_idle(&event, &idle) }
    if let Some(update) = event.update_args() {
      self.on_update(&event, &update)
    }

    // Dispatch window events to WindowHandler functions.
    if let Some(after_render) = event.after_render_args() {
      self.on_after_render(&event, &after_render)
    }
    if let Some(close) = event.close_args() { self.on_close(&event, &close) }
    if let Some(cursor) = event.cursor_args() { self.on_cursor(&event, cursor) }
    if let Some(focus) = event.focus_args() { self.on_focus(&event, focus) }
    if let Some(render) = event.render_args() {
      self.on_render(&event, &render)
    }
    if let Some(resize) = event.resize_args() {
      self.on_resize(&event, &resize)
    }
  }
}
