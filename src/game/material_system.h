#pragma once

#include <utils/vmethod.h>

#include <cstdint>

namespace game
{
  struct Texture;
  struct Material;
  struct RenderContext;
  struct KeyValues;

  namespace texture_groups
  {
    constexpr auto RENDER_TARGETS = "RenderTargets";
  }

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
    VMETHOD(Texture *, create_named_render_target_texture, 85,
            (const char *rt_name, int32_t width, int32_t height),
            (this, rt_name, width, height, 1 /* RT_SIZE_DEFAULT */,
             0 /* IMAGE_FORMAT_RGBA8888 */, 1 /* MATERIAL_RT_DEPTH_SEPARATE */,
             0x200C /* TEXTUREFLAGS_CLAMPS (1 << 2) | TEXTUREFLAGS_CLAMPT (1 <<
                       3) | TEXTUREFLAGS_EIGHTBITALPHA (1 << 13) */
             ,
             1))
    VMETHOD(RenderContext *, render_context, 98, (), (this))
  };
}
