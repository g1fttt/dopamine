use dopamine_sdk::utils::Interfaces;
use dopamine_sdk::Entity;

/// Returns iterator over all players (except _local player_)
#[expect(dead_code)] // Reserved for Aimbot and Esp
pub fn players_iter<'a>() -> impl Iterator<Item = &'a Entity> {
  generic_iter(Interfaces::get().engine.max_clients())
}

/// Returns iterator over all entities (except _local player_)
pub fn iter<'a>() -> impl Iterator<Item = &'a Entity> {
  generic_iter(Interfaces::get().entity_list.highest_entity_index())
}

fn generic_iter<'a>(max_entities: i32) -> impl Iterator<Item = &'a Entity> {
  (1..=max_entities)
    .filter_map(|i| Interfaces::get().entity_list.get_entity_by_index(i))
    .filter(|ent| !ent.is_local_player())
}
