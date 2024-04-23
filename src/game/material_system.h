#pragma once

#include <utils/vmethod.h>

namespace game
{
  struct Texture;
  struct Material;
  struct RenderContext;
  struct KeyValues;

  struct MaterialSystem {
    VMETHOD(Material *, create_material, 70,
            (const char *material_name, KeyValues *key_values),
            (this, material_name, key_values))
    VMETHOD(Material *, find_material, 71,
            (const char *material_name, const char *group_name),
            (this, material_name, group_name, true, nullptr))
    VMETHOD(Texture *, find_texture, 79,
            (const char *texture_name, const char *group_name),
            (this, texture_name, group_name, true, 0))
    VMETHOD(RenderContext *, render_context, 98, (), (this))
  };
}
