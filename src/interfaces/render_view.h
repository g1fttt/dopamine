#pragma once

#include <utils/color.h>
#include <utils/vmethod.h>

namespace interfaces {
  struct RenderView {
    constexpr utils::Color get_color_modulation() {
      utils::Color color{};
      {
        get_color_modulation(color.float_array());
        color.a = get_blend();
      }
      return color;
    }

    VMETHOD(void, set_blend, 4, (float blend), (this, blend))
    VMETHOD(float, get_blend, 5, (), (this))
    VMETHOD(void, set_color_modulation, 6, (const float *color), (this, color))
    VMETHOD(void, get_color_modulation, 7, (float *color), (this, color))
  };
}
