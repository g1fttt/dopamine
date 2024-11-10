use super::FeatureContext;
use crate::config::EnumMapConfig;

use enum_map::{Enum, EnumMap};
use serde::{Deserialize, Serialize};
use strum::VariantNames;

use dopamine_sdk::engine::ModelRenderInfo;
use dopamine_sdk::material_system::{Material, MaterialFlag};
use dopamine_sdk::utils::Interfaces;
use dopamine_sdk::{Color, Entity, KeyValues};

pub struct Chams<'a> {
  materials: EnumMap<ChamsMaterialKind, &'a Material>,
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
    ctx: FeatureContext<'_, '_, ChamsConfig>,
    draw_model_execute: &impl Fn(),
    info: &ModelRenderInfo,
  ) {
    self.applied = false;

    let Some(entity) = ctx.interfaces.entity_list.get_entity_by_index(info.entity_index) else {
      return;
    };

    if entity.animated().is_viewmodel() {
      self.apply_chams(
        draw_model_execute,
        ctx.interfaces,
        &ctx.config[ChamsConfigKind::Viewmodel].layers,
      );
    } else if entity.is_player() && !entity.networkable().is_dormant() {
      self.apply_player_chams(ctx, draw_model_execute, entity);
    }
  }

  fn apply_player_chams(
    &mut self,
    ctx: FeatureContext<'_, '_, ChamsConfig>,
    draw_model_execute: &impl Fn(),
    player_entity: &Entity,
  ) {
    let config_kind = if let Some(lp) = ctx.local_player
      && lp.team() != player_entity.team()
    {
      ChamsConfigKind::Enemies
    } else {
      ChamsConfigKind::Allies
    };
    self.apply_chams(draw_model_execute, ctx.interfaces, &ctx.config[config_kind].layers);
  }

  fn apply_chams(
    &mut self,
    draw_model_execute: &impl Fn(),
    interfaces: &Interfaces,
    layers: &[ChamsLayerConfig],
  ) {
    for layer in layers {
      if !layer.enabled || !layer.ignore_z {
        continue;
      }

      self.apply_material(draw_model_execute, interfaces, layer);

      interfaces.model_render.reset_material();
    }

    for layer in layers {
      if !layer.enabled || layer.ignore_z {
        continue;
      }

      if layer.cover && !self.applied {
        draw_model_execute();
      }

      self.apply_material(draw_model_execute, interfaces, layer);
      self.applied = true;
    }
  }

  fn apply_material(
    &self,
    draw_model_execute: &impl Fn(),
    interfaces: &Interfaces,
    config: &ChamsLayerConfig,
  ) {
    let material = self.materials[config.material_kind];
    material.set_flag(MaterialFlag::IgnoreZ, config.ignore_z);
    material.set_flag(MaterialFlag::Wireframe, config.wireframe);

    let orig_color = interfaces.render_view.color_with_blend();

    let color = &config.material_color;
    interfaces.render_view.set_color(color);
    interfaces.render_view.set_blend(color.a);

    interfaces.model_render.override_material(material);
    draw_model_execute();

    interfaces.render_view.set_color_with_blend(&orig_color);
  }
}

#[derive(Clone, Copy, Enum, VariantNames, Serialize, Deserialize)]
pub enum ChamsConfigKind {
  Enemies,
  Allies,
  Viewmodel,
}

#[derive(Default, Clone, Copy, Enum, VariantNames, Serialize, Deserialize)]
#[repr(usize)] // Guarantee for `mem::transmute` in `ui::menu`
pub enum ChamsMaterialKind {
  #[default]
  Regular,
  Flat,
}

#[derive(Default, Serialize, Deserialize)]
#[serde(default)]
pub struct ChamsLayerConfig {
  pub enabled: bool,
  pub ignore_z: bool,
  pub wireframe: bool,
  pub cover: bool,
  pub material_kind: ChamsMaterialKind,
  pub material_color: Color,
}

#[derive(Default, Serialize, Deserialize)]
#[serde(default)]
pub struct ChamsLayersConfig {
  #[serde(skip)]
  pub current_layer_index: usize,
  pub layers: [ChamsLayerConfig; 4],
}

pub type ChamsConfig = EnumMapConfig<ChamsConfigKind, ChamsLayersConfig>;
