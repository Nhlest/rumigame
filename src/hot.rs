#[hot_lib_reloader::hot_module(dylib = "lib")]
pub mod hot_lib {
  use ruminative::bevy::*;
  use ruminative::engine::tilemap_pipeline::*;
  use ruminative::winit::event::VirtualKeyCode;
  hot_functions_from_file!("lib/src/lib.rs");
}
