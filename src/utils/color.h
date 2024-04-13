#pragma once

#include <config.h>

#include <cstdint>

namespace utils {
  struct Color {
    // clang-format off
    DERIVE_SERDE(Color,
      FIELD(r, "r")
      FIELD(g, "g")
      FIELD(b, "b")
      FIELD(a, "a"))
    // clang-format on

    constexpr Color() : Color(255.0f, 255.0f, 255.0f) {}
    constexpr Color(float r, float g, float b, float a = 255.0f)
        : r(div_col_chan(r)), g(div_col_chan(g)), b(div_col_chan(b)),
          a(div_col_chan(a)) {}

    // FIXME: Give me name without bindings to amount of elements
    inline float *float_array() {
      return reinterpret_cast<float *>(this);
    }

    inline const float *float_array() const {
      return reinterpret_cast<const float *>(this);
    }

    constexpr int32_t im_u32() const {
      return (int32_t(mul_col_chan(r)) << 24) |
             (int32_t(mul_col_chan(g)) << 16) |
             (int32_t(mul_col_chan(b)) << 8) | int32_t(mul_col_chan(a));
    }

    static Color WHITE;

    float r, g, b, a;
  private:
    constexpr static float div_col_chan(float x) {
      return x * (1.0f / 255.0f);
    }

    constexpr static float mul_col_chan(float x) {
      return x * 255.0f;
    }
  };
}
