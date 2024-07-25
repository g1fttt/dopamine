use crate::game::engine::ModelRenderInfo;
use crate::game::material_system::{Material, MaterialFlag};
use crate::game::{Entity, KeyValues};

use crate::config::{ChamsConfigKind, ChamsGroupConfig, ChamsKind, ChamsLayerConfig};
use crate::interfaces::Interfaces;

use enum_map::EnumMap;

use super::FeatureContext;

pub struct Chams<'a> {
  materials: EnumMap<ChamsKind, &'a Material>,
  applied: bool,
}

impl Chams<'_> {
  pub fn new() -> Self {
    let material_system = Interfaces::get().material_system;

    let kv = KeyValues::new_leaked("VertexLitGeneric");
    let regular_material = material_system.create_material("_RegularMaterial", kv).unwrap();

    let kv = KeyValues::new_leaked("UnlitGeneric");
    let flat_material = material_system.create_material("_FlatMaterial", kv).unwrap();

    let materials = EnumMap::from_array([regular_material, flat_material]);

    Self { materials, applied: false }
  }

  #[inline]
  pub fn applied(&self) -> bool {
    self.applied
  }

  pub fn draw(
    &mut self,
    ctx: FeatureContext<'_, '_, ChamsGroupConfig>,
    original_dme: &impl Fn(),
    info: &ModelRenderInfo,
  ) {
    if ctx.local_player.is_none() {
      return;
    }

    self.applied = false;

    let model_name = ctx.interfaces.model_info.model_name(info.model);
    let entity = ctx.interfaces.entity_list.get_entity_by_index(info.entity_index);

    if model_name.starts_with("models/weapons/v_") {
      self.apply_chams(
        original_dme,
        ctx.interfaces,
        &ctx.config[ChamsConfigKind::Viewmodel].layers,
      );
    } else if let Some(entity) = entity
      && entity.is_player()
      && !entity.networkable().is_dormant()
    {
      self.apply_player_chams(ctx, original_dme, entity);
    }
  }

  fn apply_player_chams(
    &mut self,
    ctx: FeatureContext<'_, '_, ChamsGroupConfig>,
    original_dme: &impl Fn(),
    player_entity: &Entity,
  ) {
    let config_kind = if unsafe { ctx.local_player() }.team() != player_entity.team() {
      ChamsConfigKind::Enemies
    } else {
      ChamsConfigKind::Allies
    };
    self.apply_chams(original_dme, ctx.interfaces, &ctx.config[config_kind].layers);
  }

  fn apply_chams(
    &mut self,
    original_dme: &impl Fn(),
    interfaces: &Interfaces,
    layers: &[ChamsLayerConfig],
  ) {
    for layer in layers {
      if !layer.enabled || !layer.ignore_z {
        continue;
      }

      self.apply_material(original_dme, interfaces, layer);

      interfaces.model_render.reset_material();
    }

    for layer in layers {
      if !layer.enabled || layer.ignore_z {
        continue;
      }

      if layer.cover && !self.applied {
        original_dme();
      }

      self.apply_material(original_dme, interfaces, layer);
      self.applied = true;
    }
  }

  fn apply_material(
    &self,
    original_dme: &impl Fn(),
    interfaces: &Interfaces,
    config: &ChamsLayerConfig,
  ) {
    let material = self.materials[config.material_kind];
    material.set_flag(MaterialFlag::IgnoreZ, config.ignore_z);
    material.set_flag(MaterialFlag::Wireframe, config.wireframe);

    let color = &config.material_color;
    interfaces.render_view.set_color(color);
    interfaces.render_view.set_blend(color.a);

    interfaces.model_render.override_material(material);
    original_dme();
  }
}
