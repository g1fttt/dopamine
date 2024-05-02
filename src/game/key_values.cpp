#include "key_values.h"

#include <app.h>

namespace game
{
  KeyValues::KeyValues(const char *shader) {
    app->patterns->key_values_constructor(this, shader);
  }

  void KeyValues::set_string(const char *key, const char *value) {
    app->patterns->key_values_set_string(this, key, value);
  }

  void KeyValues::set_integer(const char *key, int32_t value) {
    app->patterns->key_values_set_integer(this, key, value);
  }
}
