#pragma once

#include <utils/vmethod.h>

#include <cstdint>

namespace game
{
  struct Texture;
  struct Material;

  enum StencilOp {
    Keep = 1,
    Replace = 3,
  };

  enum StencilCmpFunc {
    Equal = 3,
    Always = 8,
  };

  struct RenderContext {
    VMETHOD(void, set_render_target, 6, (const Texture *texture),
            (this, texture))
    VMETHOD(void, clear_buffers, 12,
            (bool clear_color, bool clear_depth, bool clear_stencil = false),
            (this, clear_color, clear_depth, clear_stencil))
    VMETHOD(void, set_viewport, 38,
            (int32_t x, int32_t y, int32_t width, int32_t height),
            (this, x, y, width, height))
    VMETHOD(void, clear_color_3ub, 72,
            (std::uint8_t r, std::uint8_t g, std::uint8_t b), (this, r, g, b))
    VMETHOD(void, override_depth_enable, 74, (bool enable, bool depth_enable),
            (this, enable, depth_enable))
    VMETHOD(void, draw_screen_space_rect, 103,
            (const Material *material, int32_t x, int32_t y, int32_t width,
             int32_t height, float texture_x0, float texture_y0,
             float texture_x1, float texture_y1, int32_t texture_width,
             int32_t texture_height),
            (this, material, x, y, width, height, texture_x0, texture_y0,
             texture_x1, texture_y1, texture_width, texture_height, nullptr, 1,
             1))
    VMETHOD(void, push_render_target_and_viewport, 107, (Texture * rt),
            (this, rt))
    VMETHOD(void, pop_render_target_and_viewport, 109, (), (this))
    VMETHOD(void, set_stencil_enable, 117, (bool enable), (this, enable))
    VMETHOD(void, set_stencil_fail_operation, 118, (StencilOp op), (this, op))
    VMETHOD(void, set_stencil_z_fail_operation, 119, (StencilOp op), (this, op))
    VMETHOD(void, set_stencil_pass_operation, 120, (StencilOp op), (this, op))
    VMETHOD(void, set_stencil_cmp_func, 121, (StencilCmpFunc cmp_func),
            (this, cmp_func))
    VMETHOD(void, set_stencil_ref_value, 122, (int32_t ref), (this, ref))
    VMETHOD(void, set_stencil_test_mask, 123, (uint32_t mask), (this, mask))
    VMETHOD(void, set_stencil_write_mask, 124, (uint32_t mask), (this, mask))
    VMETHOD(void, begin_pix_event, 140,
            (const char *name, uint32_t color = 0xFFF5940F),
            (this, color, name))
    VMETHOD(void, end_pix_event, 141, (), (this))
  };
};
