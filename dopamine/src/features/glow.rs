use super::FeatureContext;
use crate::config::{GlowConfig, GlowConfigKind, GlowGroupConfig};
use crate::entities;

use dopamine_utils::Color;
use educe::Educe;

use dopamine_sdk::game::material_system::*;
use dopamine_sdk::game::render_view::ViewSetup;
use dopamine_sdk::game::{Entity, KeyValues};
use dopamine_sdk::Interfaces;

pub struct Glow<'a> {
  rt_quarter_size_1: &'a Texture,
  rt_glow_buf_1: &'a Texture,
  rt_glow_buf_2: &'a Texture,
  glow_material: &'a Material,
  halo_material: &'a Material,
  glow_blur_x_material: &'a Material,
  glow_blur_y_material: &'a Material,
}

impl Glow<'_> {
  pub fn new() -> Self {
    let material_system = Interfaces::get().material_system;

    let rt_quarter_size_1 = material_system.find_texture("_rt_SmallFB1", "RenderTargets").unwrap();
    rt_quarter_size_1.inc_ref_counter();

    let rt_full_frame = material_system.find_texture("_rt_FullFrameFB", "RenderTargets").unwrap();

    let rt_glow_buf_1 =
      material_system.create_named_rt("_rt_GlowBuf1", rt_full_frame.dimensions()).unwrap();

    let rt_glow_buf_2 =
      material_system.create_named_rt("_rt_GlowBuf2", rt_full_frame.dimensions()).unwrap();

    let kv = KeyValues::new_leaked("UnlitGeneric");
    {
      kv.set("$BaseTexture", "white");
      kv.set("$IgnoreZ", "1");
      kv.set("$Model", "1");
      kv.set("$LinearWrite", "1");
    }
    let glow_material = material_system.create_material("_GlowMaterial", kv).unwrap();

    let kv = KeyValues::new_leaked("screenspace_general");
    {
      kv.set("$PixShader", "haloaddoutline_ps20");
      kv.set("$Alpha_Blend_Color_Overlay", "1");
      kv.set("$BaseTexture", "_rt_GlowBuf1");
      kv.set("$C0_X", "1");
      kv.set("$IgnoreZ", "1");
      kv.set("$LinearRead_BaseTexture", "1");
      kv.set("$LinearWrite", "1");
    }
    let halo_material = material_system.create_material("_HaloMaterial", kv).unwrap();

    let kv = KeyValues::new_leaked("BlurFilterX");
    {
      kv.set("$BaseTexture", "_rt_GlowBuf1");
      kv.set("$IgnoreZ", "1");
      kv.set("$Translucent", "1");
      kv.set("$AlphaTest", "1");
    }
    let glow_blur_x_material = material_system.create_material("_GlowBlurX", kv).unwrap();

    let kv = KeyValues::new_leaked("BlurFilterY");
    {
      kv.set("$BaseTexture", "_rt_GlowBuf2");
      kv.set("$IgnoreZ", "1");
      kv.set("$Translucent", "1");
      kv.set("$AlphaTest", "1");
    }
    let glow_blur_y_material = material_system.create_material("_GlowBlurY", kv).unwrap();

    Self {
      rt_quarter_size_1,
      rt_glow_buf_1,
      rt_glow_buf_2,
      glow_material,
      halo_material,
      glow_blur_x_material,
      glow_blur_y_material,
    }
  }

  pub fn draw(&self, ctx: FeatureContext<'_, '_, GlowGroupConfig>, view: &ViewSetup) {
    let should_glow = ctx.config.as_array().iter().any(|cfg| cfg.enabled);

    if should_glow {
      let render_ctx = ctx.interfaces.material_system.render_ctx();

      self.apply_glow_effects(ctx, view, render_ctx);
    }
  }

  fn draw_glowing_models(
    &self,
    ctx: FeatureContext<'_, '_, GlowGroupConfig>,
    view: &ViewSetup,
    render_ctx: &RenderContext,
  ) {
    render_ctx.push_rt_and_set_viewport(self.rt_glow_buf_1, view.dimensions());

    let orig_color = ctx.interfaces.render_view.color_with_blend();

    render_ctx.clear_color_3u8(Color::black());
    ClearBuffersBuilder::default().clear_color(true).clear_depth(false).build_and_clear(render_ctx);

    ctx.interfaces.model_render.override_material(self.glow_material);

    StencilState { test_mask: 0xFF, ..Default::default() }.set(render_ctx);

    for entity in entities::iter() {
      if !should_draw_model(entity) {
        continue;
      }

      let config = match determine_config(&ctx, entity) {
        Some(cfg) if cfg.enabled => cfg,
        Some(_) | None => continue,
      };

      ctx.interfaces.render_view.set_color_with_blend(&config.color);

      draw_model(entity);
    }

    ctx.interfaces.render_view.set_color_with_blend(&orig_color);
    ctx.interfaces.model_render.reset_material();

    StencilState::default().set(render_ctx);

    render_ctx.pop_rt_and_viewport();
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
    &self,
    ctx: FeatureContext<'_, '_, GlowGroupConfig>,
    view: &ViewSetup,
    render_ctx: &RenderContext,
  ) {
    OverrideDepthBuilder::default().enable(true).build_and_override(render_ctx);

    let orig_blend = ctx.interfaces.render_view.blend();
    ctx.interfaces.render_view.set_blend(0.0);

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

      let config = determine_config(&ctx, entity);
      if config.is_none_or(|cfg| !cfg.enabled) {
        continue;
      }

      draw_model(entity);

      drew_anything = true;
    }

    OverrideDepthBuilder::default().enable(false).build_and_override(render_ctx);

    StencilState::default().set(render_ctx);

    ctx.interfaces.render_view.set_blend(orig_blend);

    // https://github.com/ValveSoftware/source-sdk-2013/blob/0d8dceea4310fde5706b3ce1c70609d72a38efdf/sp/src/game/client/glow_outline_effect.cpp#L256-L260
    if !drew_anything {
      return;
    }

    self.draw_glowing_models(ctx, view, render_ctx);
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
  ctx: &FeatureContext<'config, '_, GlowGroupConfig>,
  entity: &Entity,
) -> Option<&'config GlowConfig> {
  let config_kind = if entity.is_player() {
    if let Some(lp) = ctx.local_player
      && lp.team() != entity.team()
    {
      GlowConfigKind::Enemies
    } else {
      GlowConfigKind::Allies
    }
  } else if entity.is_weapon() && entity.owner_handle() == u16::MAX {
    GlowConfigKind::Weapons
  } else {
    return None;
  };
  Some(&ctx.config[config_kind])
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
