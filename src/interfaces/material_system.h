#pragma once

#include <utils/vmethod.h>

namespace game {
  struct Texture;
  struct Material;
  struct RenderContext;
}

namespace interfaces {
  struct MaterialSystem {
    VMETHOD(game::Material *, find_material, 71,
            (const char *material_name, const char *group_name),
            (this, material_name, group_name, true, nullptr))
    VMETHOD(game::Texture *, find_texture, 79,
            (const char *texture_name, const char *group_name),
            (this, texture_name, group_name, true, 0))
    VMETHOD(game::RenderContext *, render_context, 98, (), (this))
  };
}
