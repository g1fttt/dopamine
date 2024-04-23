#pragma once

#include "utils/ptr.h"

struct IDirect3DDevice9;
struct _D3DPRESENT_PARAMETERS;

namespace game {
  struct KeyValues;
}

namespace core {
  struct Patterns {
    Patterns();

    utils::Ptr<void> d3d9_present, d3d9_reset;

    utils::Ptr<void> key_values_constructor = nullptr;
    utils::Ptr<void> key_values_set_string = nullptr;
    utils::Ptr<void> key_values_set_int = nullptr;
  };
}
