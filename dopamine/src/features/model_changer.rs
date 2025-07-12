use crate::features::FeatureContext;

use educe::Educe;
use serde::{Deserialize, Serialize};
use strum::VariantNames;

use dopamine_sdk::data_cache::ModelHandle;
use dopamine_sdk::engine::Model;
use dopamine_sdk::utils::Interfaces;
use dopamine_sdk::{Entity, PlayerClass, WeaponId, rstr_path};

use std::collections::HashMap;
use std::path::{Path, PathBuf};

pub struct ModelChanger {
  gloves: Option<&'static Entity>,
  sleeves: Option<&'static Entity>,

  models: HashMap<WeaponId, ModelHandle>,
}

impl ModelChanger {
  pub fn new() -> Self {
    Self { gloves: None, sleeves: None, models: HashMap::new() }
  }

  pub fn destroy_entities(&mut self) {
    if let Some(gloves) = self.gloves.take() {
      gloves.networkable().release();
    }

    if let Some(sleeves) = self.sleeves.take() {
      sleeves.networkable().release();
    }
  }

  #[inline(always)]
  pub fn clear_sound_overrides(&self) {
    Interfaces::get().sound_emitter_system.clear_sound_overrides();
  }
}

// TODO: Own search path in the engine

impl ModelChanger {
  pub fn on_get_model_call(
    &self,
    ctx: FeatureContext<'_, '_, ModelChangerConfig>,
    model: &Model,
  ) -> Option<&'static Model> {
    if !ctx.config.enabled {
      return None;
    }

    let path = ctx.interfaces.model_info.get_model_name(Some(model))?;

    let Some((weapon_name, true /* is_weapon */)) = weapon_name_from_path(path) else {
      return None;
    };

    let weapon_id = WeaponId::from_weapon_name(weapon_name)?;
    let replacement = replacement_path_by_weapon_id(weapon_id)?;

    ctx.interfaces.model_info.find_or_load_model(replacement)
  }

  pub fn on_studio_call<'a, T, F>(
    &mut self,
    ctx: FeatureContext<'_, '_, ModelChangerConfig>,
    handle: ModelHandle,
    original: F,
  ) -> Option<&'a mut T>
  where
    F: Fn(ModelHandle) -> Option<&'a mut T>,
  {
    if !ctx.config.enabled || !ctx.interfaces.engine.is_in_game() {
      return None;
    }

    let path = ctx.interfaces.mdl_cache.get_model_name(handle);

    let Some((weapon_name, true /* is_weapon */)) = weapon_name_from_path(path) else {
      return None;
    };

    let script_path = PathBuf::from(SCRIPTS_PATH).join(weapon_name).with_extension("txt");
    ctx.interfaces.sound_emitter_system.add_sound_overrides(script_path);

    let weapon_id = WeaponId::from_weapon_name(weapon_name)?;
    let custom_handle = self.models.entry(weapon_id).or_insert(ModelHandle::invalid());

    if !custom_handle.is_invalid() {
      let result = original(*custom_handle);

      if result.is_some() {
        return result;
      }
    }

    let replacement = replacement_path_by_weapon_id(weapon_id)?;
    *custom_handle = ctx.interfaces.mdl_cache.find_mdl(replacement);

    original(*custom_handle)
  }

  pub fn on_fsn_call(&mut self, ctx: FeatureContext<'_, '_, ModelChangerConfig>) {
    if !ctx.config.enabled {
      return;
    }

    let Some(local_player) = ctx.local_player else {
      return;
    };

    let view_entity = local_player.view_entity();

    let Some(viewmodel) = view_entity.and_then(Entity::viewmodel) else {
      return;
    };

    let Some(player_class) = view_entity.map(Entity::player_class) else {
      return;
    };

    fn create_viewmodel() -> &'static Entity {
      let entity = Entity::create_by_name("viewmodel").unwrap();

      entity.spawn();
      entity
    }

    fn update_viewmodel(viewmodel: &Entity, parent: &Entity) {
      viewmodel.set_abs_origin(parent.abs_origin());
      viewmodel.follow_entity(parent, true);
    }

    let gloves = *self.gloves.get_or_insert_with(create_viewmodel);
    update_viewmodel(gloves, viewmodel);

    let sleeves = *self.sleeves.get_or_insert_with(create_viewmodel);
    update_viewmodel(sleeves, viewmodel);

    let (gloves_path, sleeves_path) = wearables_paths(ctx.config, player_class);

    precache_or_set_model(gloves, gloves_path, ctx.interfaces);

    if let Some(sleeves_path) = sleeves_path {
      precache_or_set_model(sleeves, sleeves_path, ctx.interfaces);
    }
  }

  pub fn should_remove_sleeves(
    &self,
    ctx: FeatureContext<'_, '_, ModelChangerConfig>,
    model: Option<&Model>,
  ) -> bool {
    // TODO: Check if config is enabled and if so, remove ALL wearables, not only sleeves

    let Some(local_player) = ctx.local_player else {
      return false;
    };

    let are_sleeves =
      ctx.interfaces.model_info.get_model_name(model).is_some_and(|s| s.contains("sleeve"));

    // No need to search for wearables below since current model isn't a pair of sleeves
    if !are_sleeves {
      return false;
    }

    let view_entity = local_player.view_entity();

    let player_class = match view_entity.map(Entity::player_class) {
      Some(class) => class,
      None => return false,
    };

    let (_, sleeves_path) = wearables_paths(ctx.config, player_class);

    // TODO: Allow to equip sleeves even for factions those don't have them by default
    ctx.config.remove_sleeves || sleeves_path.is_none()
  }
}

/// Returns `false` if model was just precached, and `true`
/// if model index was successfuly set to desired one.
fn precache_or_set_model(
  entity: &Entity,
  model_path: impl AsRef<Path>,
  interfaces: &Interfaces,
) -> bool {
  match interfaces.model_info.get_model_index(&model_path) {
    -1 => {
      // At this point our model isn't presented in game memory,
      // therefore we have to precache it in order to use it
      // right upon next `ModelChanger::run` method call.
      precache_model(model_path, interfaces);

      false
    }
    model_index => {
      entity.set_model_index(model_index);

      true
    }
  }
}

fn precache_model(model_path: impl AsRef<Path>, interfaces: &Interfaces) {
  let precache_table = interfaces.network_string_table_container.find_table("modelprecache");

  if let Some(table) = precache_table {
    interfaces.model_info.find_or_load_model(&model_path);

    table.add_string(false, rstr_path!(model_path.as_ref()));
  }
}

fn weapon_name_from_path(path: &str) -> Option<(&str, bool)> {
  // v_rif_ak47.mdl
  let file_name = path.split('/').next_back()?;
  // v_rif_ak47 mdl
  let mut file_info = file_name.split('.');

  // Any other file can be passed into this function,
  // therefore we have to check the file extension
  if let Some("mdl") = file_info.next_back() {
    let model_name = file_info.next()?;

    if model_name.contains("w_") {
      return None;
    }

    let weapon_type = model_name.split('_').nth(1)?;
    let is_weapon = matches!(
      weapon_type,
      "c4" | "eq" | "knife" | "mach" | "pist" | "rif" | "shot" | "smg" | "snip"
    );

    let mut model_name_split = model_name.split('_');

    // v_rif_ak47 => ak47
    // v_knife_t => knife
    let model_name = match model_name.contains("knife") {
      true => model_name_split.nth(1)?,
      false => model_name_split.next_back()?,
    };

    return Some((model_name, is_weapon));
  }

  None
}

fn replacement_path_by_weapon_id(id: WeaponId) -> Option<PathBuf> {
  let model_path = match id {
    WeaponId::AK47 => "rif_ak47.mdl",
    WeaponId::Knife => "knife_css.mdl",
    _ => return None,
  };

  Some(PathBuf::from(V_MODELS_PATH).join(model_path))
}

fn wearables_paths(config: &ModelChangerConfig, class: PlayerClass) -> (PathBuf, Option<PathBuf>) {
  fn from_player_class<'s: 'static>(class: PlayerClass) -> (&'s str, Option<&'s str>) {
    match class {
      PlayerClass::PhoenixConnection => ("glove_fullfinger/glove_fullfinger.mdl", None),
      PlayerClass::LeetKrew => ("glove_fingerless/glove_fingerless.mdl", None),
      PlayerClass::ArcticAvengers => {
        ("glove_fullfinger/glove_fullfinger.mdl", Some("professional/sleeve_professional.mdl"))
      }
      PlayerClass::GuerillaWarfare => {
        ("glove_fullfinger/glove_fullfinger.mdl", Some("balkan/sleeve_balkan.mdl"))
      }
      PlayerClass::SealTeam6 => {
        ("glove_hardknuckle/glove_hardknuckle.mdl", Some("st6/sleeve_st6.mdl"))
      }
      PlayerClass::GSG9 => {
        ("glove_hardknuckle/glove_hardknuckle_blue.mdl", Some("gsg9/sleeve_gsg9.mdl"))
      }
      PlayerClass::Sas => {
        ("glove_hardknuckle/glove_hardknuckle_black.mdl", Some("sas/sleeve_sas.mdl"))
      }
      PlayerClass::Gign => {
        ("glove_hardknuckle/glove_hardknuckle_black.mdl", Some("gign/sleeve_gign.mdl"))
      }
      // NOTE: PlayerClass::None is not handled here because
      //       it will probably break established architecture
      _ => unreachable!(),
    }
  }

  let (gloves, sleeves) = from_player_class(class);

  let gloves_path = match config.glove_kind {
    GloveConfigKind::Default => gloves,
    GloveConfigKind::Motorcycle => "glove_motorcycle/glove_motorcycle.mdl",
  };

  let sleeves_path = sleeves.map(|s| match config.sleeve_kind {
    SleeveConfigKind::Default => s,
    SleeveConfigKind::SwatMedic => "swat/sleeve_swat_medic.mdl",
  });

  let arms = PathBuf::from(V_MODELS_PATH).join("arms");

  (arms.join(gloves_path), sleeves_path.map(|sp| arms.join(sp)))
}

const V_MODELS_PATH: &str = "models/weapons/v_models";
const SCRIPTS_PATH: &str = "scripts/dopamine/weapons";

#[derive(Educe, Serialize, Deserialize)]
#[serde(default)]
#[educe(Default)]
pub struct ModelChangerConfig {
  pub enabled: bool,
  pub remove_sleeves: bool,
  pub glove_kind: GloveConfigKind,
  pub sleeve_kind: SleeveConfigKind,
}

#[derive(Default, VariantNames, Serialize, Deserialize)]
#[repr(usize)]
pub enum GloveConfigKind {
  #[default]
  Default,
  Motorcycle,
}

#[derive(Default, VariantNames, Serialize, Deserialize)]
#[repr(usize)]
pub enum SleeveConfigKind {
  #[default]
  Default,
  SwatMedic,
}
