use std::mem::ManuallyDrop;
use ruminative::bevy::{NonSendMut, Query, With};
use ruminative::engine::tilemap_pipeline::{Cell, Transform};
use ruminative::imgui::{Context, igSetCurrentContext, Image, ImGuiContext, StyleVar, TextureId, Ui};

#[no_mangle]
pub fn move_right(
  mut query: Query<&mut Transform, With<Cell>>
) {
  for mut i in query.iter_mut() {
    i.y-=0.05;
  }
}

#[no_mangle]
pub fn move_left(
  mut query: Query<&mut Transform, With<Cell>>
) {
  for mut i in query.iter_mut() {
    i.x-=0.05;
  }
}
