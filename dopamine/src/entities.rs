use dopamine_sdk::game::Entity;
use dopamine_sdk::Interfaces;

pub type EntityIterator<'a> = Box<dyn Iterator<Item = &'a Entity>>;

/// Returns iterator over all players (except _local player_)
pub fn players_iter<'a>() -> EntityIterator<'a> {
  let interfaces = Interfaces::get();

  Box::new(
    (1..=interfaces.engine.max_clients())
      .filter_map(|i| interfaces.entity_list.get_entity_by_index(i))
      .filter(|ent| !ent.is_local_player()),
  )
}
