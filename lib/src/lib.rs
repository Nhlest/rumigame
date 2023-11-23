use std::mem::ManuallyDrop;
use ruminative::bevy::NonSendMut;
use ruminative::imgui::{Context, igSetCurrentContext, Image, ImGuiContext, StyleVar, TextureId, Ui};

#[no_mangle]
pub fn gui(
  // mut game_viewport: ResMut<GameViewport>,
  // mut commands: Commands,
  mut imgui: NonSendMut<Context>,
  // mut ctx: NonSendMut<*mut ImGuiContext>,
  // mut ui: NonSendMut<*mut Ui>,
  // surface: Res<ASingleton<Surface>>,
) {
  imgui.set_current_context();
  let ui = imgui.current_frame();
  // unsafe { igSetCurrentContext(ctx.cast()) } ;

  // let window = surface.object().unwrap().downcast_ref::<Window>().unwrap();
  // let scale_factor = window.scale_factor();
  // return;

  // return;
  // let ui : &mut Ui = unsafe { &mut*ui.cast() };
  {
    ui.dockspace_over_main_viewport();
  }
  ui.show_demo_window(&mut true);


  let t = ui.push_style_var(StyleVar::WindowPadding([0.0, 0.0]));
  ui.window("viewport")
    .bg_alpha(0.0)
    .resizable(true)
    .build(|| {
      let mut min = ui.window_content_region_min();
      let mut max = ui.window_content_region_max();

      let x = max[0]-min[0];
      let y = max[1]-min[1];
      Image::new(TextureId::new(1), [x, y])
        .uv0([0.0, 0.0])
        .uv1([1.0, 1.0])
        .build(ui);
    });

  // unsafe { igSetCurrentContext(ptr::null_mut()) };
  // unsafe {
  //   let dock_id = imgui_sys::igGetID_Str(CString::new("Dockspace").unwrap().as_ptr());
  //   let _dock = imgui_sys::igDockBuilderGetNode(dock_id);
  // }
  // imgui::Pan
  // ui.window("Bottom panel").dock
}
