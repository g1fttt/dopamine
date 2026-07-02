use crate::config::EnumMapConfig;

use bumpalo::Bump;
use enum_map::{Enum, EnumMap};
use serde::{Deserialize, Serialize};
use strum::{EnumCount, VariantNames};

use dopamine_sdk::interfaces::{material_system, model_render, render_view};
use dopamine_sdk::material_system::{Material, MaterialFlag};
use dopamine_sdk::{Color, Entity};

pub struct Chams<'a> {
  materials: EnumMap<ChamsMaterialKind, &'a Material>,
  applied: bool,
}

impl Chams<'_> {
  pub fn new(bump: &Bump) -> Self {
    let regular_material =
      material_system().create_material_dummy("_RegularMaterial", "VertexLitGeneric", bump);

    let flat_material =
      material_system().create_material_dummy("_FlatMaterial", "UnlitGeneric", bump);

    let materials = EnumMap::from_array([regular_material, flat_material]);

    Self { materials, applied: false }
  }

  pub fn dec_ref_counters(&self) {
    for material in self.materials.values() {
      material.dec_ref_counter();
    }
  }

  pub fn applied(&self) -> bool {
    self.applied
  }

  pub fn draw(
    &mut self,
    config: &ChamsConfig,
    draw_model_execute: &impl Fn(),
    entity: Option<&Entity>,
  ) {
    self.applied = false;

    let Some(entity) = entity else {
      return;
    };

    if entity.is_viewmodel() {
      self.apply_chams(draw_model_execute, &config[ChamsConfigKind::Viewmodel].layers);
    } else if entity.is_player() && !entity.networkable().is_dormant() {
      self.apply_player_chams(config, draw_model_execute, entity);
    }
  }

  fn apply_player_chams(
    &mut self,
    config: &ChamsConfig,
    draw_model_execute: &impl Fn(),
    player_entity: &Entity,
  ) {
    let config_kind = if let Some(lp) = Entity::local_player()
      && lp.team() != player_entity.team()
    {
      ChamsConfigKind::Enemies
    } else {
      ChamsConfigKind::Allies
    };
    self.apply_chams(draw_model_execute, &config[config_kind].layers);
  }

  fn apply_chams(&mut self, draw_model_execute: &impl Fn(), layers: &[ChamsLayerConfig]) {
    for layer in layers {
      if !layer.enabled || !layer.ignore_z {
        continue;
      }

      self.apply_material(draw_model_execute, layer);

      model_render().reset_material();
    }

    for layer in layers {
      if !layer.enabled || layer.ignore_z {
        continue;
      }

      if layer.cover && !self.applied {
        draw_model_execute();
      }

      self.apply_material(draw_model_execute, layer);
      self.applied = true;
    }
  }

  fn apply_material(&self, draw_model_execute: &impl Fn(), config: &ChamsLayerConfig) {
    let material = self.materials[config.material_kind()];
    material.set_flag(MaterialFlag::IgnoreZ, config.ignore_z);
    material.set_flag(MaterialFlag::Wireframe, config.wireframe);

    let orig_color = render_view().color_with_blend();

    let color = &config.material_color;
    render_view().set_color(color);
    render_view().set_blend(color.a);

    model_render().override_material(material);
    draw_model_execute();

    render_view().set_color_with_blend(&orig_color);
  }
}

#[derive(Clone, Copy, Enum, VariantNames, Serialize, Deserialize)]
pub enum ChamsConfigKind {
  Enemies,
  Allies,
  Viewmodel,
}

#[derive(Default, Clone, Copy, Enum, EnumCount, VariantNames, Serialize, Deserialize)]
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
  pub material_index: usize,
  pub material_color: Color,
}

impl ChamsLayerConfig {
  pub fn material_kind(&self) -> ChamsMaterialKind {
    ChamsMaterialKind::from_usize(self.material_index)
  }
}

#[derive(Default, Serialize, Deserialize)]
#[serde(default)]
pub struct ChamsLayersConfig {
  #[serde(skip)]
  pub current_layer_index: usize,
  pub layers: [ChamsLayerConfig; 4],
}

pub type ChamsConfig = EnumMapConfig<ChamsConfigKind, ChamsLayersConfig>;
