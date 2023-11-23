#[hot_lib_reloader::hot_module(dylib = "lib")]
pub mod hot_lib {
  use std::mem::ManuallyDrop;
  use ruminative::bevy::NonSendMut;
  use ruminative::imgui::{Context, igSetCurrentContext, Image, ImGuiContext, StyleVar, TextureId, Ui};
  hot_functions_from_file!("lib/src/lib.rs");
}
