use crate::config::{Color, GlowConfig};
use crate::game::material_system::{
    ClearBuffersBuilder, Material, MaterialSystem, OverrideDepthBuilder, RenderContext,
    ScreenSpaceRectBuilder, StencilCmpFn, StencilOp, Texture,
};
use crate::game::render_view::ViewSetup;
use crate::game::Entity;

use crate::features::shared::MaterialCreator;
use crate::interfaces::Interfaces;

pub struct GlowObjectManager<'a> {
    rt_quarter_size_1: &'a Texture,
    rt_glow_buf_1: &'a Texture,
    rt_glow_buf_2: &'a Texture,
    glow_material: Option<&'a Material>,
    halo_material: Option<&'a Material>,
    glow_blur_x_material: Option<&'a Material>,
    glow_blur_y_material: Option<&'a Material>,
    objects: Vec<Object<'a>>,
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

        Self {
            rt_quarter_size_1,
            rt_glow_buf_1,
            rt_glow_buf_2,
            glow_material: None,
            halo_material: None,
            glow_blur_x_material: None,
            glow_blur_y_material: None,
            objects: Vec::new(),
        }
    }

    pub fn setup_materials(
        &mut self,
        material_system: &'a MaterialSystem,
        material_creator: &mut MaterialCreator,
    ) {
        self.glow_material = material_creator
            .shader("UnlitGeneric")
            .string("$BaseTexture", "white")
            .string("$IgnoreZ", "1")
            .string("$Model", "1")
            .string("$LinearWrite", "1")
            .bind("_GlowMaterial", material_system);

        self.halo_material = material_creator
            .shader("screenspace_general")
            .string("$PixShader", "haloaddoutline_ps20")
            .string("$Alpha_Blend_Color_Overlay", "1")
            .string("$BaseTexture", "_rt_GlowBuf1")
            .string("$C0_X", "1")
            .string("$IgnoreZ", "1")
            .string("$LinearRead_BaseTexture", "1")
            .string("$LinearWrite", "1")
            .bind("_HaloMaterial", material_system);

        self.glow_blur_x_material = material_creator
            .shader("BlurFilterX")
            .string("$BaseTexture", "_rt_GlowBuf1")
            .string("$IgnoreZ", "1")
            .string("$Translucent", "1")
            .string("$AlphaTest", "1")
            .bind("_GlowBlurX", material_system);

        self.glow_blur_y_material = material_creator
            .shader("BlurFilterY")
            .string("$BaseTexture", "_rt_GlowBuf2")
            .string("$BloomAmount", "1")
            .string("$IgnoreZ", "1")
            .string("$Translucent", "1")
            .string("$AlphaTest", "1")
            .bind("_GlowBlurY", material_system);
    }

    pub fn register_object(&mut self, obj: Object<'a>) {
        self.objects.push(obj)
    }

    pub fn clear_objects(&mut self) {
        self.objects.clear();
    }

    pub fn has_glow_effect(&self, entity: &Entity) -> bool {
        self.objects
            .iter()
            .any(move |obj| obj.entity.is_some_and(|ent| std::ptr::addr_eq(ent, entity)))
    }
}

impl GlowObjectManager<'_> {
    pub fn draw_glow_effects(&self, interfaces: &Interfaces, view: &ViewSetup) {
        let render_ctx = interfaces.material_system.render_ctx();

        render_ctx.with_pix_event("ApplyEntityGlowEffects", || {
            self.apply_entity_glow_effects(interfaces, view, render_ctx);
        });
    }

    fn draw_glow_models(
        &self,
        interfaces: &Interfaces,
        view: &ViewSetup,
        render_ctx: &RenderContext,
    ) {
        render_ctx.push_rt_and_set_viewport(self.rt_glow_buf_1, view.dimensions());

        let orig_color = interfaces.render_view.color_modulation();
        let orig_alpha = interfaces.render_view.blend();

        render_ctx.clear_color_3ub((0, 0, 0));
        ClearBuffersBuilder::default()
            .clear_color(true)
            .clear_depth(false)
            .build_and_clear(render_ctx);

        interfaces
            .model_render
            .forced_material_override(self.glow_material);

        StencilState {
            test_mask: 0xFF,
            ..Default::default()
        }
        .set(render_ctx);

        for obj in &self.objects {
            if !obj.should_draw() {
                continue;
            }

            interfaces
                .render_view
                .set_color_modulation(obj.color.color_modulation());
            interfaces.render_view.set_blend(obj.color.alpha());

            obj.draw_model();
        }

        interfaces.model_render.forced_material_override(None);
        interfaces.render_view.set_color_modulation(orig_color);
        interfaces.render_view.set_blend(orig_alpha);

        StencilState::default().set(render_ctx);

        render_ctx.pop_rt_and_viewport();
    }

    fn blur_glow_effects(&self, view: &ViewSetup, render_ctx: &RenderContext) {
        render_ctx.push_rt_and_set_viewport(self.rt_glow_buf_2, view.dimensions());
        {
            let (view_width, view_height) = view.dimensions();

            let blur_screen_space_rect = ScreenSpaceRectBuilder::default()
                .material(self.glow_blur_x_material.unwrap())
                .pos((0, 0))
                .dimensions(view.dimensions())
                .texture_x0_y0((0.0, 0.0))
                .texture_x1_y1((view_width as f32 / 1.0, view_height as f32 / 1.0))
                .texture_dimensions(view.dimensions());
            blur_screen_space_rect.clone().build_and_draw(render_ctx);

            render_ctx.set_render_target(self.rt_glow_buf_1);
            blur_screen_space_rect
                .material(self.glow_blur_y_material.unwrap())
                .build_and_draw(render_ctx);
        }
        render_ctx.pop_rt_and_viewport();
    }

    fn apply_entity_glow_effects(
        &self,
        interfaces: &Interfaces,
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

        let mut drew_anything = false;

        for obj in &self.objects {
            if !obj.should_draw() {
                continue;
            }

            StencilState {
                enable: true,
                z_fail_op: StencilOp::Replace,
                pass_op: StencilOp::Replace,
                ref_value: 1,
                ..Default::default()
            }
            .set(render_ctx);

            obj.draw_model();

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

        render_ctx.with_pix_event("DrawGlowModels", || {
            self.draw_glow_models(interfaces, view, render_ctx);
        });

        render_ctx.with_pix_event("BlurGlowEffects", || {
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
            .material(self.halo_material.unwrap())
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

pub struct Object<'a> {
    entity: Option<&'a Entity>,
    color: Color,
}

impl<'a> Object<'a> {
    pub fn new(entity: &'a Entity, config: &GlowConfig) -> Self {
        Self {
            entity: Some(entity),
            color: config.color,
        }
    }

    fn should_draw(&self) -> bool {
        self.entity
            .is_some_and(|ent| ent.should_draw() && !ent.is_dormant())
    }

    fn draw_model(&self) {
        let ent = self.entity.unwrap();
        ent.draw_model();

        let mut attachment = ent.move_child();
        while let Some(att) = attachment {
            if att.should_draw() {
                att.draw_model();
            }
            attachment = ent.move_peer();
        }
    }
}

impl<'a> From<(&'a Entity, &GlowConfig)> for Object<'a> {
    fn from(val: (&'a Entity, &GlowConfig)) -> Self {
        Object::new(val.0, val.1)
    }
}

struct StencilState {
    enable: bool,
    fail_op: StencilOp,
    z_fail_op: StencilOp,
    pass_op: StencilOp,
    cmp_fn: StencilCmpFn,
    ref_value: i32,
    test_mask: u32,
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

impl Default for StencilState {
    fn default() -> Self {
        Self {
            enable: false,
            fail_op: StencilOp::default(),
            z_fail_op: StencilOp::default(),
            pass_op: StencilOp::default(),
            cmp_fn: StencilCmpFn::default(),
            ref_value: 0,
            test_mask: 0xFFFFFFFF,
            write_mask: 0xFFFFFFFF,
        }
    }
}
