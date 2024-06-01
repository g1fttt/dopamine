use crate::game::material_system::{Material, MaterialFlag, StencilOp};
use crate::game::{Entity, KeyValues};

use super::shared::{RenderableObject, StencilState};
use crate::config::{ChamsConfigKind, ChamsGroupConfig, ChamsKind, ChamsLayer};
use crate::interfaces::Interfaces;
use crate::utils::Color;

use enum_map::EnumMap;

use std::ptr;

pub struct Chams<'a> {
  materials: EnumMap<ChamsKind, &'a Material>,
  renderable_objects: Vec<RenderableObject<'a>>,
}

impl Chams<'_> {
  pub fn new() -> Self {
    let material_system = Interfaces::get().material_system;

    let kv = KeyValues::new_leaked("VertexLitGeneric");
    let regular_material = material_system
      .create_material("_RegularMaterial", kv)
      .unwrap();

    let kv = KeyValues::new_leaked("UnlitGeneric");
    let flat_material = material_system
      .create_material("_FlatMaterial", kv)
      .unwrap();

    let materials = EnumMap::from_array([regular_material, flat_material]);

    Self {
      materials,
      renderable_objects: Vec::new(),
    }
  }
}

impl<'a> Chams<'a> {
  pub fn draw(
    &self,
    objects: &mut [RenderableObject<'a>],
    interfaces: &Interfaces,
    ChamsGroupConfig(config): &ChamsGroupConfig,
    local_player: Option<&Entity>,
  ) {
    let render_ctx = interfaces.material_system.render_ctx();

    StencilState {
      enable: true,
      z_fail_op: StencilOp::Replace,
      pass_op: StencilOp::Replace,
      ref_value: 1,
      ..Default::default()
    }
    .set(render_ctx);

    for object in objects.iter_mut() {
      if !object.should_draw_model() {
        continue;
      }

      let is_enemy = local_player.is_some_and(|lp| lp.team() != object.entity.team());

      if is_enemy {
        self.draw_chams(object, &config[ChamsConfigKind::Enemies], interfaces);
      } else {
        self.draw_chams(object, &config[ChamsConfigKind::Allies], interfaces);
      }
      draw_player_attachments(object, interfaces);
    }
    StencilState::default().set(render_ctx);
  }

  fn draw_chams(
    &self,
    object: &mut RenderableObject,
    layers: &[ChamsLayer],
    interfaces: &Interfaces,
  ) {
    for layer in layers {
      if !layer.enabled || !layer.ignore_z {
        continue;
      }
      self.apply_material_and_draw(object, layer, interfaces);
    }

    for layer in layers {
      if !layer.enabled || layer.ignore_z {
        continue;
      }
      self.apply_material_and_draw(object, layer, interfaces);
    }
  }

  fn apply_material_and_draw(
    &self,
    object: &mut RenderableObject,
    layer: &ChamsLayer,
    interfaces: &Interfaces,
  ) {
    let material = self.materials[layer.material_kind];
    material.set_flag(MaterialFlag::IgnoreZ, layer.ignore_z);

    let color = &layer.material_color;
    interfaces.render_view.set_color(color);
    interfaces.render_view.set_blend(color.a);

    interfaces.model_render.override_material(material);

    object.draw_model();
    object.model_was_drawn = true;
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

fn draw_player_attachments(object: &RenderableObject, interfaces: &Interfaces) {
  interfaces.render_view.set_color(&Color::white());
  interfaces.render_view.set_blend(1.0);

  interfaces.model_render.reset_material();

  object.draw_attachments();
}
