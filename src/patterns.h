#pragma once

#include "utils/ptr.h"

#include <cstdint>

namespace game
{
  struct GlobalEntityList;
  struct PlayerEntity;
  struct EntityListener;
  struct KeyValues;
}

namespace core
{
  struct Patterns {
    Patterns();

    utils::Ptr<void> d3d9_present, d3d9_reset;

    game::KeyValues *(THISCALL *key_values_constructor)(game::KeyValues *,
                                                        const char *);
    void(THISCALL *key_values_set_string)(game::KeyValues *, const char *,
                                          const char *);
    void(THISCALL *key_values_set_integer)(game::KeyValues *, const char *,
                                           int32_t);

    bool(THISCALL *is_local_player)(game::PlayerEntity *);
  };
}
