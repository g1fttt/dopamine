use crate::app::App;
use crate::config::Config;

use crate::features::chams::{ChamsConfig, ChamsConfigKind, ChamsMaterialKind};
use crate::features::glow::{GlowConfig, GlowConfigKind};
use crate::features::misc::MiscConfig;
use crate::features::visuals::VisualsConfig;

use dopamine_sdk::input_system::InputSystem;
use enum_map::Enum;
use imgui::{ColorEditFlags, Io, StyleVar, Ui, WindowFlags};
use strum::VariantNames;
use windows::Win32::UI::WindowsAndMessaging::ShowCursor;

#[derive(Default)]
struct ShouldDrawWindow {
  misc: bool,
  visuals: bool,
  glow: bool,
  chams: bool,
}

pub struct Menu {
  open: bool,
  toggle_animation_end: f32,
  should_draw_window: ShouldDrawWindow,
}

impl Menu {
  pub fn new() -> Self {
    Self { open: false, toggle_animation_end: 1.0, should_draw_window: ShouldDrawWindow::default() }
  }

  pub fn is_fully_closed(&self) -> bool {
    !self.open && self.toggle_animation_end > 1.0
  }

  #[inline]
  pub fn is_open(&self) -> bool {
    self.open
  }

  pub fn transparency(&self) -> f32 {
    let t = if self.open { self.toggle_animation_end } else { 1.0 - self.toggle_animation_end };
    t.clamp(0.0, 1.0)
  }

  pub fn update_animation(&mut self, io: &Io) {
    self.toggle_animation_end += io.delta_time / 0.35 /* animation speed */;
  }

  pub fn handle_toggle(&mut self, input_system: &InputSystem) {
    self.open = !self.open;

    if !self.open {
      input_system.reset_input_state();
    }
    input_system.enable_input(!self.open);

    if self.toggle_animation_end > 0.0 && self.toggle_animation_end < 1.0 {
      self.toggle_animation_end = 1.0 - self.toggle_animation_end;
    } else {
      self.toggle_animation_end = 0.0;
    }
  }

  pub fn update_mouse_cursor(&self, io: &mut Io) {
    io.mouse_draw_cursor = self.open;
    unsafe { ShowCursor(!self.open) };
  }

  pub fn render(&mut self, ui: &Ui, config: &mut Config) {
    let style = ui.push_style_var(StyleVar::Alpha(self.transparency()));
    {
      self.draw_menu_bar(ui);
      self.draw_misc_window(ui, &mut config.misc);
      self.draw_visuals_window(ui, &mut config.visuals);
      self.draw_glow_window(ui, &mut config.glow);
      self.draw_chams_window(ui, &mut config.chams);
    }
    style.pop();
  }

  fn draw_menu_bar(&mut self, ui: &Ui) {
    if let Some(bar) = ui.begin_main_menu_bar() {
      if ui.menu_item("Misc") {
        self.should_draw_window.misc = true;
      } else if ui.menu_item("Visuals") {
        self.should_draw_window.visuals = true;
      } else if ui.menu_item("Glow") {
        self.should_draw_window.glow = true;
      } else if ui.menu_item("Chams") {
        self.should_draw_window.chams = true;
      }
      bar.end();
    }
  }

  fn draw_misc_window(&mut self, ui: &Ui, config: &mut MiscConfig) {
    if !self.should_draw_window.misc {
      return;
    }

    ui.window("Misc").opened(&mut self.should_draw_window.misc).flags(Self::window_flags()).build(
      || {
        ui.checkbox("Bunnyhop", &mut config.bunnyhop.enabled);
        ui.same_line();
        ui.slider("Chance", 10, 100, &mut config.bunnyhop.chance);
      },
    );
  }

  fn draw_visuals_window(&mut self, ui: &Ui, config: &mut VisualsConfig) {
    if !self.should_draw_window.visuals {
      return;
    }

    ui.window("Visuals")
      .opened(&mut self.should_draw_window.visuals)
      .flags(Self::window_flags())
      .build(|| {
        ui.checkbox("Better crosshair", &mut config.better_crosshair.enabled);
        ui.same_line();
        ui.checkbox("Force sniper rifles", &mut config.better_crosshair.force_sniper_rifles);
        ui.same_line();
        ui.color_edit4_config("##Color", config.better_crosshair.color.as_mut_array())
          .flags(Self::color_edit_flags())
          .build();
        ui.slider("Size", 1.0, 20.0, &mut config.better_crosshair.size);
        ui.slider("Thickness", 1.0, 10.0, &mut config.better_crosshair.thickness);
        ui.slider("Gap", 0.0, 20.0, &mut config.better_crosshair.gap);

        ui.separator();

        ui.checkbox("Add FOV", &mut config.add_fov.enabled);
        ui.same_line();
        ui.slider("Amount", -50.0, 50.0, &mut config.add_fov.amount);

        ui.separator();

        // TODO: Curve editor
        ui.checkbox("Viewmodel origin", &mut config.viewmodel_origin.enabled);
        ui.slider("X", -10.0, 10.0, &mut config.viewmodel_origin.origin.x);
        ui.slider("Y", -10.0, 10.0, &mut config.viewmodel_origin.origin.y);
        ui.slider("Z", -10.0, 10.0, &mut config.viewmodel_origin.origin.z);
      });
  }

  fn draw_glow_window(&mut self, ui: &Ui, config: &mut GlowConfig) {
    if !self.should_draw_window.glow {
      return;
    }

    ui.window("Glow").opened(&mut self.should_draw_window.glow).flags(Self::window_flags()).build(
      || {
        ui.combo_simple_string(
          "##ConfigKind",
          &mut config.current_config_index,
          GlowConfigKind::VARIANTS,
        );

        ui.separator();

        let current_config_index = config.current_config_index;
        let cfg = &mut config.as_mut_slice()[current_config_index];

        ui.checkbox("Enabled", &mut cfg.enabled);
        ui.same_line();
        ui.color_edit4_config("##Color", cfg.color.as_mut_array())
          .flags(Self::color_edit_flags())
          .build();

        let config_kind = GlowConfigKind::from_usize(current_config_index);
        if matches!(config_kind, GlowConfigKind::Enemies) {
          ui.checkbox("Fade out when spotted", &mut cfg.fade_out_when_spotted);
          ui.same_line();
          ui.slider("Rate", 1.0, 8.0, &mut cfg.fade_out_rate);
        }
      },
    );
  }

  fn draw_chams_window(&mut self, ui: &Ui, config: &mut ChamsConfig) {
    if !self.should_draw_window.chams {
      return;
    }

    ui.window("Chams")
      .opened(&mut self.should_draw_window.chams)
      .flags(Self::window_flags())
      .build(|| {
        ui.combo_simple_string(
          "##ConfigKind",
          &mut config.current_config_index,
          ChamsConfigKind::VARIANTS,
        );

        let current_config_index = config.current_config_index;
        let cfg = &mut config.as_mut_slice()[current_config_index];

        ui.same_line();
        ui.text("Layer");
        ui.same_line();

        if ui.button("<") {
          cfg.current_layer_index = cfg.current_layer_index.saturating_sub(1);
        }

        ui.same_line();
        ui.text(format!("{}", cfg.current_layer_index + 1));
        ui.same_line();

        if ui.button(">") && cfg.current_layer_index < cfg.layers.len() - 1 {
          cfg.current_layer_index += 1;
        }

        ui.separator();

        let current_layer = &mut cfg.layers[cfg.current_layer_index];
        ui.checkbox("Enabled", &mut current_layer.enabled);
        ui.same_line();
        ui.color_edit4_config("##Color", current_layer.material_color.as_mut_array())
          .flags(Self::color_edit_flags())
          .build();
        ui.checkbox("Cover", &mut current_layer.cover);
        ui.same_line();
        ui.checkbox("Ignore Z", &mut current_layer.ignore_z);
        ui.same_line();
        ui.checkbox("Wireframe", &mut current_layer.wireframe);
        ui.combo_simple_string(
          "Material",
          unsafe {
            std::mem::transmute::<&mut ChamsMaterialKind, &mut usize>(
              &mut current_layer.material_kind,
            )
          },
          ChamsMaterialKind::VARIANTS,
        );
      });
  }

  #[inline]
  fn window_flags() -> WindowFlags {
    WindowFlags::NO_RESIZE | WindowFlags::NO_SCROLLBAR | WindowFlags::ALWAYS_AUTO_RESIZE
  }

  #[inline]
  fn color_edit_flags() -> ColorEditFlags {
    ColorEditFlags::ALPHA_BAR | ColorEditFlags::NO_INPUTS
  }
}
