use crate::config::Config;

use crate::features::chams::{ChamsConfig, ChamsConfigKind, ChamsMaterialKind};
use crate::features::glow::{GlowConfig, GlowConfigKind};
use crate::features::misc::MiscConfig;
use crate::features::visuals::VisualsConfig;

use dopamine_sdk::interfaces::input_system;
use enum_map::Enum;
use strum::VariantNames;
use windows::Win32::UI::WindowsAndMessaging::ShowCursor;

use imgui::{ColorEditFlags, StyleVarKind, WindowFlags};

use std::mem;

#[derive(Default)]
struct ShouldDrawWindow {
  misc: bool,
  visuals: bool,
  glow: bool,
  chams: bool,
  config: bool,
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

  #[inline(always)]
  pub fn is_open(&self) -> bool {
    self.open
  }

  pub fn transparency(&self) -> f32 {
    let t = if self.open { self.toggle_animation_end } else { 1.0 - self.toggle_animation_end };
    t.clamp(0.0, 1.0)
  }

  pub fn update_animation(&mut self) {
    self.toggle_animation_end += imgui::io().delta_time / 0.35 /* animation speed */;
  }

  pub fn handle_toggle(&mut self) {
    self.open = !self.open;

    if !self.open {
      input_system().reset_input_state();
    }
    input_system().enable_input(!self.open);

    if self.toggle_animation_end > 0.0 && self.toggle_animation_end < 1.0 {
      self.toggle_animation_end = 1.0 - self.toggle_animation_end;
    } else {
      self.toggle_animation_end = 0.0;
    }
  }

  pub fn update_mouse_cursor(&self) {
    imgui::io_mut().mouse_draw_cursor = self.open;
    unsafe { ShowCursor(!self.open) };
  }

  pub fn render(&mut self, config: &mut Config) {
    let style = imgui::push_style_var(StyleVarKind::Alpha(self.transparency()));
    {
      self.draw_menu_bar();
      self.draw_misc_window(&mut config.misc);
      self.draw_visuals_window(&mut config.visuals);
      self.draw_glow_window(&mut config.glow);
      self.draw_chams_window(&mut config.chams);
      self.draw_config_window(config);
    }
    style.pop();
  }

  fn draw_menu_bar(&mut self) {
    if let Some(bar) = imgui::main_menu_bar() {
      if imgui::menu_item("Misc") {
        self.should_draw_window.misc = true;
      } else if imgui::menu_item("Visuals") {
        self.should_draw_window.visuals = true;
      } else if imgui::menu_item("Glow") {
        self.should_draw_window.glow = true;
      } else if imgui::menu_item("Chams") {
        self.should_draw_window.chams = true;
      } else if imgui::menu_item("Config") {
        self.should_draw_window.config = true;
      }
      bar.end();
    }
  }

  fn draw_misc_window(&mut self, config: &mut MiscConfig) {
    if !self.should_draw_window.misc {
      return;
    }

    imgui::window_ex("Misc")
      .open(&mut self.should_draw_window.misc)
      .flags(Self::window_flags())
      .build(|| {
        imgui::checkbox("Bunnyhop", &mut config.bunnyhop.enabled);
        imgui::same_line();
        imgui::slider_int("Chance", &mut config.bunnyhop.chance, 10, 100);
      });
  }

  fn draw_visuals_window(&mut self, config: &mut VisualsConfig) {
    if !self.should_draw_window.visuals {
      return;
    }

    imgui::window_ex("Visuals")
      .open(&mut self.should_draw_window.visuals)
      .flags(Self::window_flags())
      .build(|| {
        imgui::checkbox("Better crosshair", &mut config.better_crosshair.enabled);
        imgui::same_line();
        imgui::checkbox("Force sniper rifles", &mut config.better_crosshair.force_sniper_rifles);
        imgui::same_line();
        imgui::color_edit4_ex("##Color", config.better_crosshair.color.as_mut_array())
          .flags(Self::color_edit_flags())
          .build();
        imgui::slider_float("Size", &mut config.better_crosshair.size, 1.0, 20.0);
        imgui::slider_float("Thickness", &mut config.better_crosshair.thickness, 1.0, 10.0);
        imgui::slider_float("Gap", &mut config.better_crosshair.gap, 0.0, 20.0);

        imgui::separator();

        imgui::checkbox("Add FOV", &mut config.add_fov.enabled);
        imgui::same_line();
        imgui::slider_float("Amount", &mut config.add_fov.amount, -50.0, 50.0);

        imgui::checkbox("Viewmodel origin", &mut config.viewmodel_origin.enabled);
        imgui::slider_float("X", &mut config.viewmodel_origin.value.x, -10.0, 10.0);
        imgui::slider_float("Y", &mut config.viewmodel_origin.value.y, -10.0, 10.0);
        imgui::slider_float("Z", &mut config.viewmodel_origin.value.z, -10.0, 10.0);
      });
  }

  fn draw_glow_window(&mut self, config: &mut GlowConfig) {
    if !self.should_draw_window.glow {
      return;
    }

    imgui::window_ex("Glow")
      .open(&mut self.should_draw_window.glow)
      .flags(Self::window_flags())
      .build(|| {
        imgui::combo("##ConfigKind", &mut config.current_config_index, GlowConfigKind::VARIANTS);

        imgui::separator();

        let current_config_index = config.current_config_index;
        let cfg = &mut config.as_mut_slice()[current_config_index];

        imgui::checkbox("Enabled", &mut cfg.enabled);
        imgui::same_line();
        imgui::color_edit4_ex("##Color", cfg.color.as_mut_array())
          .flags(Self::color_edit_flags())
          .build();

        let config_kind = GlowConfigKind::from_usize(current_config_index);
        if matches!(config_kind, GlowConfigKind::Enemies) {
          imgui::checkbox("Fade out when spotted", &mut cfg.fade_out_when_spotted);
          imgui::same_line();
          imgui::slider_float("Rate", &mut cfg.fade_out_rate, 1.0, 8.0);
        }
      });
  }

  fn draw_chams_window(&mut self, config: &mut ChamsConfig) {
    if !self.should_draw_window.chams {
      return;
    }

    imgui::window_ex("Chams")
      .open(&mut self.should_draw_window.chams)
      .flags(Self::window_flags())
      .build(|| {
        imgui::combo("##ConfigKind", &mut config.current_config_index, ChamsConfigKind::VARIANTS);

        let current_config_index = config.current_config_index;
        let cfg = &mut config.as_mut_slice()[current_config_index];

        imgui::same_line();
        imgui::text("Layer");
        imgui::same_line();

        if imgui::button("<") && cfg.current_layer_index > 0 {
          cfg.current_layer_index -= 1;
        }

        imgui::same_line();
        imgui::text(format!("{}", cfg.current_layer_index + 1));
        imgui::same_line();

        if imgui::button(">") && cfg.current_layer_index < cfg.layers.len() - 1 {
          cfg.current_layer_index += 1;
        }

        imgui::separator();

        let current_layer = &mut cfg.layers[cfg.current_layer_index];
        imgui::checkbox("Enabled", &mut current_layer.enabled);
        imgui::same_line();
        imgui::color_edit4_ex("##Color", current_layer.material_color.as_mut_array())
          .flags(Self::color_edit_flags())
          .build();
        imgui::checkbox("Cover", &mut current_layer.cover);
        imgui::same_line();
        imgui::checkbox("Ignore Z", &mut current_layer.ignore_z);
        imgui::same_line();
        imgui::checkbox("Wireframe", &mut current_layer.wireframe);

        let current_material_index = unsafe {
          mem::transmute::<&mut ChamsMaterialKind, &mut usize>(&mut current_layer.material_kind)
        };
        imgui::combo("Material", current_material_index, ChamsMaterialKind::VARIANTS);
      });
  }

  fn draw_config_window(&mut self, config: &mut Config) {
    if !self.should_draw_window.config {
      return;
    }

    imgui::window_ex("Config")
      .open(&mut self.should_draw_window.config)
      .flags(Self::window_flags())
      .build(|| {
        imgui::checkbox("Background blur", &mut config.blur_enabled);
      });
  }

  #[inline(always)]
  fn window_flags() -> WindowFlags {
    WindowFlags::NO_RESIZE | WindowFlags::NO_SCROLLBAR | WindowFlags::ALWAYS_AUTO_RESIZE
  }

  #[inline(always)]
  fn color_edit_flags() -> ColorEditFlags {
    ColorEditFlags::ALPHA_BAR | ColorEditFlags::NO_INPUTS
  }
}
