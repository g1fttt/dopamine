use crate::game::material_system::{
    ClearBuffersBuilder, Material, MaterialSystem, OverrideDepthBuilder, RenderContext,
    ScreenSpaceRectBuilder, StencilCmpFn, StencilOp, Texture,
};
use crate::game::render_view::ViewSetup;
use crate::game::{Entity, KeyValues};

use super::shared::RenderableObject;
use super::shared::StencilState;

use crate::app::App;
use crate::config::{GlowConfig, GlowGroupConfig};
use crate::interfaces::Interfaces;
use crate::utils::Color;

pub struct GlowObjectManager<'a> {
    rt_quarter_size_1: &'a Texture,
    rt_glow_buf_1: &'a Texture,
    rt_glow_buf_2: &'a Texture,
    glow_material: &'a Material,
    halo_material: &'a Material,
    glow_blur_x_material: &'a Material,
    glow_blur_y_material: &'a Material,
}

impl<'a> GlowObjectManager<'a> {
    pub fn new(material_system: &'a MaterialSystem) -> Self {
        let rt_quarter_size_1 = material_system
            .find_texture("_rt_SmallFB1", "RenderTargets")
            .unwrap();
        rt_quarter_size_1.inc_ref_counter();

        let rt_full_frame = material_system
            .find_texture("_rt_FullFrameFB", "RenderTargets")
            .unwrap();

        let rt_glow_buf_1 = material_system
            .create_named_rt("_rt_GlowBuf1", rt_full_frame.dimensions())
            .unwrap();

        let rt_glow_buf_2 = material_system
            .create_named_rt("_rt_GlowBuf2", rt_full_frame.dimensions())
            .unwrap();

        let kv = KeyValues::new_leaked("UnlitGeneric");
        {
            kv.set("$BaseTexture", "white");
            kv.set("$IgnoreZ", "1");
            kv.set("$Model", "1");
            kv.set("$LinearWrite", "1");
        }
        let glow_material = material_system
            .create_material("_GlowMaterial", kv)
            .unwrap();

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
        let halo_material = material_system
            .create_material("_HaloMaterial", kv)
            .unwrap();

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
            kv.set("$BloomAmount", "1");
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
}

impl GlowObjectManager<'_> {
    pub fn draw_glow_effects(&self, objects: &mut [RenderableObject], app: &App, view: &ViewSetup) {
        let render_ctx = app.interfaces.material_system.render_ctx();

        render_ctx.with_pix_event("ApplyEntityGlowEffects", move || {
            self.apply_entity_glow_effects(objects, &app.interfaces, app, view, render_ctx);
        });
    }

    fn draw_glow_models(
        &self,
        objects: &mut [RenderableObject],
        interfaces: &Interfaces,
        app: &App,
        view: &ViewSetup,
        render_ctx: &RenderContext,
    ) {
        render_ctx.push_rt_and_set_viewport(self.rt_glow_buf_1, view.dimensions());

        let orig_color = interfaces.render_view.color();
        let orig_alpha = interfaces.render_view.blend();

        render_ctx.clear_color_3u8(Color::black());
        ClearBuffersBuilder::default()
            .clear_color(true)
            .clear_depth(false)
            .build_and_clear(render_ctx);

        interfaces
            .model_render
            .forced_material_override(Some(self.glow_material));

        StencilState {
            test_mask: 0xFF,
            ..Default::default()
        }
        .set(render_ctx);

        for obj in objects {
            if !obj.should_draw_model() {
                continue;
            }

            let config = determine_glow_config(obj, app.local_player, &app.config.glow);
            if !config.enabled {
                continue;
            }

            interfaces.render_view.set_color(&config.color);
            interfaces.render_view.set_blend(config.color.a);

            obj.draw_model();
            obj.draw_attachments();
        }

        interfaces.model_render.forced_material_override(None);
        interfaces.render_view.set_color(&orig_color);
        interfaces.render_view.set_blend(orig_alpha);

        StencilState::default().set(render_ctx);

        render_ctx.pop_rt_and_viewport();
    }

    fn blur_glow_effects(&self, view: &ViewSetup, render_ctx: &RenderContext) {
        render_ctx.push_rt_and_set_viewport(self.rt_glow_buf_2, view.dimensions());
        {
            let (view_width, view_height) = view.dimensions();

            let blur_screen_space_rect = ScreenSpaceRectBuilder::default()
                .material(self.glow_blur_x_material)
                .pos((0, 0))
                .dimensions(view.dimensions())
                .texture_x0_y0((0.0, 0.0))
                .texture_x1_y1((view_width as f32 / 1.0, view_height as f32 / 1.0))
                .texture_dimensions(view.dimensions());
            blur_screen_space_rect.clone().build_and_draw(render_ctx);

            render_ctx.set_render_target(self.rt_glow_buf_1);
            blur_screen_space_rect
                .material(self.glow_blur_y_material)
                .build_and_draw(render_ctx);
        }
        render_ctx.pop_rt_and_viewport();
    }

    fn apply_entity_glow_effects(
        &self,
        objects: &mut [RenderableObject],
        interfaces: &Interfaces,
        app: &App,
        view: &ViewSetup,
        render_ctx: &RenderContext,
    ) {
        interfaces.model_render.forced_material_override(None);

        OverrideDepthBuilder::default()
            .enable(true)
            .build_and_override(render_ctx);

        StencilState::default().set(render_ctx);

        let saved_blend = interfaces.render_view.blend();
        interfaces.render_view.set_blend(0.0);

        StencilState {
            enable: true,
            z_fail_op: StencilOp::Replace,
            pass_op: StencilOp::Replace,
            ref_value: 1,
            ..Default::default()
        }
        .set(render_ctx);

        let mut drew_anything = false;

        for obj in objects.iter_mut() {
            if !obj.should_draw_model() {
                continue;
            }

            // Draw regular player model over glowing one
            // if we didn't drew it before (chams)
            if !obj.model_was_drawn {
                obj.draw_model();
                obj.draw_attachments();
                obj.model_was_drawn = true;
            }
            drew_anything = true;
        }

        OverrideDepthBuilder::default()
            .enable(false)
            .build_and_override(render_ctx);

        StencilState::default().set(render_ctx);

        interfaces.render_view.set_blend(saved_blend);
        interfaces.model_render.forced_material_override(None);

        // https://github.com/ValveSoftware/source-sdk-2013/blob/0d8dceea4310fde5706b3ce1c70609d72a38efdf/sp/src/game/client/glow_outline_effect.cpp#L256-L260
        if !drew_anything {
            return;
        }

        render_ctx.with_pix_event("DrawGlowModels", move || {
            self.draw_glow_models(objects, interfaces, app, view, render_ctx);
        });

        render_ctx.with_pix_event("BlurGlowEffects", move || {
            self.blur_glow_effects(view, render_ctx);
        });

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

fn determine_glow_config<'a>(
    obj: &RenderableObject,
    local_player: Option<&Entity>,
    config: &'a GlowGroupConfig,
) -> &'a GlowConfig {
    if let Some(lp) = local_player
        && lp.team() != obj.entity.team()
    {
        &config.enemies
    } else {
        &config.allies
    }
}
