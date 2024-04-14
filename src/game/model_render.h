#pragma once

#include <utils/vmethod.h>

namespace game {
  struct Material;

  enum OverrideType {
    Normal,
  };

  struct ModelRender {
    VMETHOD(void, forced_material_override, 1, (game::Material * new_material),
            (this, new_material, game::OverrideType::Normal))
  };
}
