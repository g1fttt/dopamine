#pragma once

#include <patterns.h>
#include <utils/vmethod.h>

#include <cstdint>

namespace game
{
  struct Entity;

  struct EntityListener {
    virtual void on_entity_created(Entity *) {}
    virtual void on_entity_deleted(Entity *) {}
  };

  struct GlobalEntityList;

  struct EntityList {
    static void init_methods(const core::Patterns &patterns) {
      METHOD_FROM_PATTERN_2(add_entity_listener);
      global_entity_list =
          patterns.global_entity_list.cast<GlobalEntityList>().get();
    }

    inline void add_entity_listener(EntityListener *listener) {
      methods.add_entity_listener(global_entity_list, listener);
    }

    VMETHOD(Entity *, get_entity_by_index, 3, (int32_t index), (this, index))
    VMETHOD(Entity *, get_entity_from_handle, 4, (int32_t handle),
            (this, handle))
  private:
    struct Methods {
      void(THISCALL *add_entity_listener)(GlobalEntityList *, EntityListener *);
      void(THISCALL *remove_entity_listener)(GlobalEntityList *,
                                             EntityListener *);
    };

    inline static Methods methods{};
    inline static GlobalEntityList *global_entity_list = nullptr;
  };
}
