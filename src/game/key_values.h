#pragma once

#include <utils/pad.h>

#include <cstdint>

namespace game {
  struct KeyValues {
    KeyValues(const char *shader);

    void set_string(const char *key, const char *value);
    void set_int(const char *key, int32_t value);
  private:
    PAD(40); // For correct heap allocation and further struct initialization
  };
}
