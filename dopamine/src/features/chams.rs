use crate::game::material_system::{Material, MaterialFlag, StencilOp};
use crate::game::{Entity, KeyValues};

use super::shared::{RenderableObject, StencilState};
use crate::config::{ChamsConfig, ChamsGroupConfig, ChamsKind};
use crate::interfaces::Interfaces;
use crate::App;

use std::ptr;

pub struct Chams<'a> {
    flat_material: &'a Material,
    regular_material: &'a Material,

    renderable_objects: Vec<RenderableObject<'a>>,
}

impl<'a> Chams<'a> {
    pub fn new() -> Self {
        let material_system = Interfaces::get().material_system;

        let kv = KeyValues::new_leaked("UnlitGeneric");
        {
            kv.set("$BaseTexture", "white");
        }
        let flat_material = material_system
            .create_material("_FlatMaterial", kv)
            .unwrap();

        let kv = KeyValues::new_leaked("VertexLitGeneric");
        {
            kv.set("$BaseTexture", "white");
        }
        let regular_material = material_system
            .create_material("_RegularMaterial", kv)
            .unwrap();

        Self {
            flat_material,
            regular_material,

            renderable_objects: Vec::new(),
        }
    }

    pub fn draw(&self, objects: &mut [RenderableObject<'a>], interfaces: &Interfaces, app: &App) {
        let render_ctx = interfaces.material_system.render_ctx();

        StencilState {
            enable: true,
            z_fail_op: StencilOp::Replace,
            pass_op: StencilOp::Replace,
            ref_value: 1,
            ..Default::default()
        }
        .set(render_ctx);

        for obj in objects.iter_mut() {
            if !obj.should_draw_model() {
                continue;
            }

            let (config_occluded, config_visible) =
                determine_chams_config(obj, app.local_player, &app.config.chams);

            if config_occluded.enabled {
                self.apply_chams(obj, config_occluded, interfaces, true);
            }

            if config_visible.enabled {
                self.apply_chams(obj, config_visible, interfaces, false);
            }

            obj.model_was_drawn = config_occluded.enabled || config_visible.enabled;
        }
        StencilState::default().set(render_ctx);
    }

    fn apply_chams(
        &self,
        object: &RenderableObject,
        config: &ChamsConfig,
        interfaces: &Interfaces,
        ignore_z: bool,
    ) {
        let material = match config.kind {
            ChamsKind::Regular => self.regular_material,
            ChamsKind::Flat => self.flat_material,
        };
        material.set_flag(MaterialFlag::IgnoreZ, ignore_z);

        interfaces.render_view.set_color(&config.color);
        interfaces.render_view.set_blend(config.color.a);
        interfaces
            .model_render
            .forced_material_override(Some(material));

        object.draw_model();
        object.draw_attachments();

        interfaces.model_render.forced_material_override(None);
    }

    pub fn cache_renderable_objects(&mut self, objects: &[RenderableObject<'a>]) {
        self.renderable_objects = objects.to_vec();
    }

    pub fn should_process_dme(&self, current_entity: &Entity) -> bool {
        let current_renderable = self
            .renderable_objects
            .iter()
            .find(move |&obj| ptr::addr_eq(obj.entity, current_entity));
        match current_renderable {
            Some(obj) => !obj.model_was_drawn,
            None => true,
        }
    }
}

fn determine_chams_config<'a>(
    obj: &RenderableObject,
    local_player: Option<&Entity>,
    config: &'a ChamsGroupConfig,
) -> (&'a ChamsConfig, &'a ChamsConfig) {
    if let Some(lp) = local_player
        && lp.team() != obj.entity.team()
    {
        (&config.enemies_occluded, &config.enemies_visible)
    } else {
        (&config.allies_occluded, &config.allies_visible)
    }
}
