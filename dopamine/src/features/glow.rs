use crate::config::EnumMapConfig;
use crate::entities;

use educe::Educe;
use enum_map::Enum;
use serde::{Deserialize, Serialize};
use strum::VariantNames;

use dopamine_sdk::interfaces::{material_system, model_render, render_view, server};
use dopamine_sdk::material_system::*;
use dopamine_sdk::render_view::ViewSetup;
use dopamine_sdk::{Color, Entity};

use std::sync::atomic::{AtomicBool, Ordering};

pub struct Glow<'a> {
  rt_quarter_size_1: &'a Texture,
  rt_glow_buf_1: &'a Texture,
  rt_glow_buf_2: &'a Texture,
  glow_material: &'a Material,
  halo_material: &'a Material,
  glow_blur_x_material: &'a Material,
  glow_blur_y_material: &'a Material,
  spotted_time: [f32; 65],
  is_in_drawing_process: AtomicBool,
}

impl Glow<'_> {
  pub fn new() -> Self {
    let materials = material_system();

    let rt_quarter_size_1 = materials.find_texture("_rt_SmallFB1", "RenderTargets").unwrap();
    rt_quarter_size_1.inc_ref_counter();

    let rt_full_frame = materials.find_texture("_rt_FullFrameFB", "RenderTargets").unwrap();
    rt_full_frame.inc_ref_counter();

    let rt_glow_buf_1 =
      materials.create_named_rt("_rt_GlowBuf1", rt_full_frame.dimensions()).unwrap();

    let rt_glow_buf_2 =
      materials.create_named_rt("_rt_GlowBuf2", rt_full_frame.dimensions()).unwrap();

    let glow_material = materials.create_material_with_kv("_GlowMaterial", "UnlitGeneric", |kv| {
      kv.set("$BaseTexture", "white");
      kv.set("$IgnoreZ", "1");
      kv.set("$Model", "1");
      kv.set("$LinearWrite", "1");
    });

    let halo_material =
      materials.create_material_with_kv("_HaloMaterial", "screenspace_general", |kv| {
        kv.set("$PixShader", "haloaddoutline_ps20");
        kv.set("$Alpha_Blend_Color_Overlay", "1");
        kv.set("$BaseTexture", "_rt_GlowBuf1");
        kv.set("$C0_X", "1");
        kv.set("$IgnoreZ", "1");
        kv.set("$LinearRead_BaseTexture", "1");
        kv.set("$LinearWrite", "1");
      });

    let glow_blur_x_material =
      materials.create_material_with_kv("_GlowBlurX", "BlurFilterX", |kv| {
        kv.set("$BaseTexture", "_rt_GlowBuf1");
        kv.set("$IgnoreZ", "1");
        kv.set("$Translucent", "1");
        kv.set("$AlphaTest", "1");
      });

    let glow_blur_y_material =
      materials.create_material_with_kv("_GlowBlurY", "BlurFilterY", |kv| {
        kv.set("$BaseTexture", "_rt_GlowBuf2");
        kv.set("$IgnoreZ", "1");
        kv.set("$Translucent", "1");
        kv.set("$AlphaTest", "1");
      });

    Self {
      rt_quarter_size_1,
      rt_glow_buf_1,
      rt_glow_buf_2,
      glow_material,
      halo_material,
      glow_blur_x_material,
      glow_blur_y_material,
      spotted_time: [Default::default(); 65],
      is_in_drawing_process: AtomicBool::new(false),
    }
  }

  pub fn is_in_drawing_process(&self) -> bool {
    self.is_in_drawing_process.load(Ordering::Acquire)
  }

  pub fn draw(&mut self, player_resource: Option<&Entity>, config: &GlowConfig, view: &ViewSetup) {
    let should_glow = config.as_array().iter().any(|cfg| cfg.enabled);

    if should_glow {
      let render_ctx = material_system().render_ctx();

      self.is_in_drawing_process.swap(true, Ordering::Acquire);
      {
        self.apply_glow_effects(player_resource, config, view, render_ctx);
      }
      self.is_in_drawing_process.store(false, Ordering::Release);
    }
  }

  pub fn dec_ref_counters(&self) {
    self.rt_quarter_size_1.dec_ref_counter();
    self.rt_glow_buf_1.dec_ref_counter();
    self.rt_glow_buf_2.dec_ref_counter();
    self.glow_material.dec_ref_counter();
    self.halo_material.dec_ref_counter();
    self.glow_blur_x_material.dec_ref_counter();
    self.glow_blur_y_material.dec_ref_counter();
  }

  fn draw_glowing_models(
    &mut self,
    player_resource: Option<&Entity>,
    config: &GlowConfig,
    view: &ViewSetup,
    render_ctx: &RenderContext,
  ) {
    render_ctx.push_rt_and_set_viewport(self.rt_glow_buf_1, view.dimensions());

    let orig_color = render_view().color_with_blend();

    render_ctx.clear_color_3u8(Color::black());
    ClearBuffersBuilder::default().clear_color(true).clear_depth(false).build_and_clear(render_ctx);

    model_render().override_material(self.glow_material);

    StencilState { test_mask: 0xFF, ..Default::default() }.set(render_ctx);

    for entity in entities::iter() {
      if !should_draw_model(entity) {
        continue;
      }

      let config = match determine_config(config, entity) {
        Some(cfg) if cfg.enabled => cfg,
        Some(_) | None => continue,
      };

      let real_time = server().global_vars().real_time;
      let player_resource = player_resource.unwrap();

      let color = match self.calc_fade_out_alpha(real_time, entity, player_resource, config) {
        Some(a) => &config.color.with_alpha(a),
        None => &config.color,
      };

      if color.a == 0.0 {
        continue;
      }

      render_view().set_color_with_blend(color);

      draw_model(entity);
    }

    render_view().set_color_with_blend(&orig_color);
    model_render().reset_material();

    StencilState::default().set(render_ctx);

    render_ctx.pop_rt_and_viewport();
  }

  fn calc_fade_out_alpha(
    &mut self,
    real_time: f32,
    entity: &Entity,
    player_resource: &Entity,
    config: &GlowItemConfig,
  ) -> Option<f32> {
    if !config.fade_out_when_spotted {
      return None;
    }

    let entity_index = match entity.networkable().index() {
      -1 => return None,
      n => n as usize,
    };

    if entity.is_player() && player_resource.is_spotted(entity_index) {
      self.spotted_time[entity_index] = real_time;

      None
    } else {
      let spotted_time = self.spotted_time[entity_index];

      let time_since_spotted = real_time - spotted_time;
      let fade_progress = time_since_spotted / config.fade_out_rate;

      let alpha = (config.color.a - fade_progress).clamp(0.0, 1.0);

      Some(alpha)
    }
  }

  fn blur_glow_effects(&self, view: &ViewSetup, render_ctx: &RenderContext) {
    render_ctx.push_rt_and_set_viewport(self.rt_glow_buf_2, view.dimensions());

    let (view_width, view_height) = view.dimensions();

    let blur_screen_space_rect = ScreenSpaceRectBuilder::default()
      .material(self.glow_blur_x_material)
      .pos((0, 0))
      .dimensions(view.dimensions())
      .texture_x0_y0((0.0, 0.0))
      .texture_x1_y1((view_width as f32, view_height as f32))
      .texture_dimensions(view.dimensions());
    blur_screen_space_rect.clone().build_and_draw(render_ctx);

    render_ctx.set_render_target(self.rt_glow_buf_1);
    blur_screen_space_rect.material(self.glow_blur_y_material).build_and_draw(render_ctx);

    render_ctx.pop_rt_and_viewport();
  }

  fn apply_glow_effects(
    &mut self,
    player_resource: Option<&Entity>,
    config: &GlowConfig,
    view: &ViewSetup,
    render_ctx: &RenderContext,
  ) {
    OverrideDepthBuilder::default().enable(true).build_and_override(render_ctx);

    let orig_blend = render_view().blend();
    render_view().set_blend(0.0);

    StencilState {
      enable: true,
      z_fail_op: StencilOp::Replace,
      pass_op: StencilOp::Replace,
      ref_value: 1,
      ..Default::default()
    }
    .set(render_ctx);

    let mut drew_anything = false;

    for entity in entities::iter() {
      if !should_draw_model(entity) {
        continue;
      }

      let config = determine_config(config, entity);
      if config.is_none_or(|cfg| !cfg.enabled) {
        continue;
      }

      draw_model(entity);

      drew_anything = true;
    }

    OverrideDepthBuilder::default().enable(false).build_and_override(render_ctx);

    StencilState::default().set(render_ctx);

    render_view().set_blend(orig_blend);

    // https://github.com/ValveSoftware/source-sdk-2013/blob/0d8dceea4310fde5706b3ce1c70609d72a38efdf/sp/src/game/client/glow_outline_effect.cpp#L256-L260
    if !drew_anything {
      return;
    }

    self.draw_glowing_models(player_resource, config, view, render_ctx);
    self.blur_glow_effects(view, render_ctx);

    StencilState {
      enable: true,
      cmp_fn: StencilCmpFn::Equal,
      test_mask: 0xFF,
      write_mask: 0x0,
      ..Default::default()
    }
    .set(render_ctx);

    const GLOW_DOWNSAMPLE: f32 = 4.0;

    let (view_width, view_height) = view.dimensions();

    ScreenSpaceRectBuilder::default()
      .material(self.halo_material)
      .pos((0, 0))
      .dimensions(view.dimensions())
      .texture_x0_y0((0.0, -0.5))
      .texture_x1_y1((
        view_width as f32 / GLOW_DOWNSAMPLE - 1.0,
        view_height as f32 / GLOW_DOWNSAMPLE - 1.0,
      ))
      .texture_dimensions(self.rt_quarter_size_1.dimensions())
      .build_and_draw(render_ctx);

    StencilState::default().set(render_ctx);
  }
}

fn should_draw_model(entity: &Entity) -> bool {
  entity.renderable().should_draw() && !entity.networkable().is_dormant()
}

fn draw_model(entity: &Entity) {
  entity.renderable().draw_model();

  for attach in entity.attachments().map(|a| a.renderable()) {
    if !attach.should_draw() {
      continue;
    }
    attach.draw_model();
  }
}

fn determine_config<'config>(
  config: &'config GlowConfig,
  entity: &Entity,
) -> Option<&'config GlowItemConfig> {
  let config_kind = if entity.is_player() {
    if let Some(lp) = Entity::local_player()
      && lp.team() != entity.team()
    {
      GlowConfigKind::Enemies
    } else {
      GlowConfigKind::Allies
    }
  } else if entity.is_weapon() && entity.owner_handle().is_invalid() {
    GlowConfigKind::Weapons
  } else {
    return None;
  };
  Some(&config[config_kind])
}

#[derive(Educe)]
#[educe(Default)]
struct StencilState {
  enable: bool,
  fail_op: StencilOp,
  z_fail_op: StencilOp,
  pass_op: StencilOp,
  cmp_fn: StencilCmpFn,
  ref_value: i32,
  #[educe(Default = u32::MAX)]
  test_mask: u32,
  #[educe(Default = u32::MAX)]
  write_mask: u32,
}

impl StencilState {
  fn set(&self, render_ctx: &RenderContext) {
    render_ctx.set_stencil_enable(self.enable);
    render_ctx.set_stencil_fail_op(self.fail_op);
    render_ctx.set_stencil_z_fail_op(self.z_fail_op);
    render_ctx.set_stencil_pass_op(self.pass_op);
    render_ctx.set_stencil_cmp_fn(self.cmp_fn);
    render_ctx.set_stencil_ref_value(self.ref_value);
    render_ctx.set_stencil_test_mask(self.test_mask);
    render_ctx.set_stencil_write_mask(self.write_mask);
  }
}

#[derive(Clone, Copy, Enum, VariantNames, Serialize, Deserialize)]
pub enum GlowConfigKind {
  Enemies,
  Allies,
  Weapons,
}

#[derive(Educe, Serialize, Deserialize)]
#[serde(default)]
#[educe(Default)]
pub struct GlowItemConfig {
  pub enabled: bool,
  pub color: Color,
  pub fade_out_when_spotted: bool,
  #[educe(Default = 3.0)]
  pub fade_out_rate: f32,
}

pub type GlowConfig = EnumMapConfig<GlowConfigKind, GlowItemConfig>;
