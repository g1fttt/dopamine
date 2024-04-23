#pragma once

#include <patterns.h>
#include <utils/pad.h>

#include <cstdint>

// TODO: Move to separate file
#define METHOD_FROM_PATTERN(field, pattern_name)                               \
  methods.field = patterns.pattern_name.transmute<decltype(methods.field)>()

namespace game {
  struct KeyValues {
    static void init_methods(const core::Patterns &patterns) {
      METHOD_FROM_PATTERN(constructor, key_values_constructor);
      METHOD_FROM_PATTERN(set_string, key_values_set_string);
      METHOD_FROM_PATTERN(set_int, key_values_set_int);
    }

    inline KeyValues(const char *shader) {
      methods.constructor(this, shader);
    }

    inline void set_string(const char *key, const char *value) {
      methods.set_string(this, key, value);
    }

    inline void set_int(const char *key, int32_t value) {
      methods.set_int(this, key, value);
    }
  private:
    struct Methods {
      KeyValues *(THISCALL *constructor)(KeyValues *, const char *);
      void(THISCALL *set_string)(KeyValues *, const char *, const char *);
      void(THISCALL *set_int)(KeyValues *, const char *, int32_t);
    };

    inline static Methods methods{};

    PAD(40); // For correct heap & stack allocation and further struct
             // initialization
  };
}
