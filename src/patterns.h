#pragma once

#include "utils/ptr.h"

#include <optional>

struct IDirect3DDevice9;
struct _D3DPRESENT_PARAMETERS;

namespace game {
  struct KeyValues;
}

namespace core {
  struct Patterns {
    Patterns();

    utils::Ptr<void> d3d9_present, d3d9_reset;

    game::KeyValues *(THISCALL *key_values_constructor)(game::KeyValues *,
                                                        const char *) = nullptr;
    void(THISCALL *key_values_set_string)(game::KeyValues *, const char *,
                                          const char *) = nullptr;
    void(THISCALL *key_values_set_int)(game::KeyValues *, const char *,
                                       int32_t) = nullptr;
  };

  constinit inline std::optional<Patterns> patterns{};
}
