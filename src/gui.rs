use ruminative::bevy::NonSendMut;
use ruminative::imgui::Context;

pub fn gui(
  mut imgui: NonSendMut<Context>,
) {
  let ui = imgui.current_frame();
  ui.window("Test").build(|| {
    ui.button("a");
  }).unwrap();
}

pub fn location_gui(
  mut imgui: NonSendMut<Context>,
) {
  let ui = imgui.current_frame();
  ui.window("Location").build(|| {
  }).unwrap();
}