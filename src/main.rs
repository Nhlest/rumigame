use std::ffi::CString;
use ruminative::bevy::{App, Update};
use ruminative::engine::{ASingleton, GameViewport, Resultat};
use ruminative::Ruminative;

pub mod hot;


fn main() -> Resultat<()> {
  let mut app = App::new();
  app.add_plugins(Ruminative);
  app.add_systems(Update, hot::hot_lib::gui);
  // ctx.app.add_systems(Update, lib::gui);
  app.run();
  Ok(())
}
