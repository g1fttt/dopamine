use dopamine_sdk::game::Entity;
use dopamine_sdk::Interfaces;

/// Returns iterator over all players (except _local player_)
pub fn players_iter<'a>() -> impl Iterator<Item = &'a Entity> {
  let interfaces = Interfaces::get();

  (1..=interfaces.engine.max_clients())
    .filter_map(|i| interfaces.entity_list.get_entity_by_index(i))
    .filter(|ent| !ent.is_local_player())
}
