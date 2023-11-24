use std::mem::ManuallyDrop;
use ruminative::bevy::{Entity, In, NonSendMut, Query, With};
use ruminative::engine::tilemap_pipeline::{Cell, Transform};
use ruminative::imgui::{Context, igSetCurrentContext, Image, ImGuiContext, StyleVar, TextureId, Ui};
use ruminative::winit::event::VirtualKeyCode;

#[no_mangle]
pub fn print(
  a: In<(Entity, VirtualKeyCode)>,
) {
  println!("{:?}", a.0.1);
}

#[no_mangle]
pub fn mov(
  a: In<(Entity, VirtualKeyCode)>,
  mut query: Query<&mut Transform, With<Cell>>
) {
  let (entity, kc) = a.0;
  let mut t = query.get_mut(a.0.0).unwrap();
  match kc {
    VirtualKeyCode::A => t.x-=0.1,
    VirtualKeyCode::D => t.x+=0.1,
    VirtualKeyCode::S => t.y+=0.1,
    VirtualKeyCode::W => t.y-=0.1,
    VirtualKeyCode::Z => {
      t.x = 0.0;
      t.y = 0.0;
    }
    _ => {}
  }
  // for mut i in query.iter_mut() {
  //   i.x-=a.0 as f32 / 10.0;
  // }
}
