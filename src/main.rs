use std::ffi::CString;
use ruminative::bevy::{App, Update};
use ruminative::engine::{ASingleton, GameViewport, Resultat};
use ruminative::Ruminative;
use ruminative::systems::SystemStorage;

pub mod hot;


fn main() -> Resultat<()> {
  let mut app = App::new();
  app.add_plugins(Ruminative);
  // SystemStorage::add_system(&mut app.world, hot::hot_lib::move_right);
  // SystemStorage::add_system(&mut app.world, hot::hot_lib::move_left);
  SystemStorage::my_add_system(&mut app.world, hot::hot_lib::mov);
  SystemStorage::my_add_system(&mut app.world, hot::hot_lib::print);
  // let mut system_storage = app.world.resource_mut::<SystemStorage>();
  // system_storage.add_system(&mut app.world, hot::hot_lib::move_right);
  // let system_id = app.world.register_system(hot::hot_lib::move_right);
  // system_storage.s.push(system_id);
  // ctx.app.add_systems(Update, lib::gui);
  app.run();
  Ok(())
}
