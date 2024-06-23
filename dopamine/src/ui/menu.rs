use crate::config::*;
use crate::game::input_system::InputSystem;

use imgui::{ColorEditFlags, Io, Ui, WindowFlags};
use strum::VariantNames;
use windows::Win32::UI::WindowsAndMessaging::ShowCursor;

use std::mem;

#[derive(Default)]
struct ShouldDrawWindow {
  misc: bool,
  visuals: bool,
  glow: bool,
  chams: bool,
}

pub struct Menu {
  open: bool,
  should_draw_window: ShouldDrawWindow,
}

impl Menu {
  pub fn new() -> Self {
    Self {
      open: bool::default(),
      should_draw_window: ShouldDrawWindow::default(),
    }
  }

  #[inline]
  pub fn is_open(&self) -> bool {
    self.open
  }

  pub fn render(&mut self, ui: &mut Ui, config: &mut Config) {
    self.render_menu_bar(ui);
    self.render_misc_window(ui, &mut config.misc);
    self.render_visuals_window(ui, &mut config.visuals);
    self.render_glow_window(ui, &mut config.glow);
    self.render_chams_window(ui, &mut config.chams);
  }

  pub fn handle_toggle(&mut self, input_system: &InputSystem) {
    self.open = !self.open;

    if !self.open {
      input_system.reset_input_state();
    }
    input_system.enable_input(!self.open);
  }

  pub fn update_mouse_cursor(&self, io: &mut Io) {
    io.mouse_draw_cursor = self.open;
    unsafe { ShowCursor(!self.open) };
  }

  fn render_menu_bar(&mut self, ui: &mut Ui) {
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

  fn render_misc_window(&mut self, ui: &mut Ui, config: &mut MiscGroupConfig) {
    if !self.should_draw_window.misc {
      return;
    }

    ui.window("Misc")
      .opened(&mut self.should_draw_window.misc)
      .flags(Self::window_flags())
      .build(|| {
        ui.checkbox("Bunnyhop", &mut config.bunnyhop.enabled);
        ui.same_line();
        ui.slider("Chance", 10, 100, &mut config.bunnyhop.chance);
      });
  }

  fn render_visuals_window(&mut self, ui: &mut Ui, config: &mut VisualsGroupConfig) {
    if !self.should_draw_window.visuals {
      return;
    }

    ui.window("Visuals")
      .opened(&mut self.should_draw_window.visuals)
      .flags(Self::window_flags())
      .build(|| {
        ui.checkbox("No-scope crosshair", &mut config.no_scope_crosshair.enabled);
        ui.same_line();
        ui.color_edit4_config("##Color", config.no_scope_crosshair.color.as_mut_array())
          .flags(Self::color_edit_flags())
          .build();
        ui.slider("Size", 1.0, 20.0, &mut config.no_scope_crosshair.size);
        ui.slider(
          "Thickness",
          1.0,
          3.0,
          &mut config.no_scope_crosshair.thickness,
        );
      });
  }

  fn render_glow_window(&mut self, ui: &mut Ui, config: &mut GlowGroupConfig) {
    if !self.should_draw_window.glow {
      return;
    }

    ui.window("Glow")
      .opened(&mut self.should_draw_window.glow)
      .flags(Self::window_flags())
      .build(|| {
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
      });
  }

  fn render_chams_window(&mut self, ui: &mut Ui, config: &mut ChamsGroupConfig) {
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
        ui.checkbox("Ignore Z", &mut current_layer.ignore_z);
        ui.same_line();
        ui.color_edit4_config("##Color", current_layer.material_color.as_mut_array())
          .flags(Self::color_edit_flags())
          .build();
        ui.combo_simple_string(
          "Material",
          unsafe { mem::transmute(&mut current_layer.material_kind) },
          ChamsKind::VARIANTS,
        );
      });
  }

  fn window_flags() -> WindowFlags {
    WindowFlags::NO_RESIZE | WindowFlags::NO_SCROLLBAR | WindowFlags::ALWAYS_AUTO_RESIZE
  }

  fn color_edit_flags() -> ColorEditFlags {
    ColorEditFlags::ALPHA_BAR | ColorEditFlags::NO_INPUTS
  }
}
