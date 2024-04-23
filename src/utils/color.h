#pragma once

#include <config.h>

#include <cstdint>

namespace utils
{
  struct Color {
    // clang-format off
    DERIVE_SERDE(Color,
      FIELD(r, "r")
      FIELD(g, "g")
      FIELD(b, "b")
      FIELD(a, "a"))
    // clang-format on

    constexpr Color()
        : Color(255.0f, 255.0f, 255.0f) {}
    constexpr Color(float r, float g, float b, float a = 255.0f)
        : r(div_col_chan(r))
        , g(div_col_chan(g))
        , b(div_col_chan(b))
        , a(div_col_chan(a)) {}

    inline float *float_array() {
      return reinterpret_cast<float *>(this);
    }

    inline const float *float_array() const {
      return reinterpret_cast<const float *>(this);
    }

    int32_t im_u32() const;

    float r, g, b, a;
  private:
    constexpr static float div_col_chan(float x) {
      return x * (1.0f / 255.0f);
    }
  };
}
