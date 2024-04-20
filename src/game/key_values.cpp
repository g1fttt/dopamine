#include "key_values.h"

#include <patterns.h>

namespace game {
  KeyValues::KeyValues(const char *shader) {
    core::patterns->key_values_constructor(this, shader);
  }

  void KeyValues::set_string(const char *key, const char *value) {
    core::patterns->key_values_set_string(this, key, value);
  }

  void KeyValues::set_int(const char *key, int32_t value) {
    core::patterns->key_values_set_int(this, key, value);
  }
}
