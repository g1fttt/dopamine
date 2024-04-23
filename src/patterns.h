#pragma once

#include "utils/ptr.h"

#define METHOD_FROM_PATTERN(field, pattern_name)                               \
  methods.field = patterns.pattern_name.transmute<decltype(methods.field)>()

namespace core
{
  struct Patterns {
    Patterns();

    utils::Ptr<void> d3d9_present, d3d9_reset;

    utils::Ptr<void> key_values_constructor;
    utils::Ptr<void> key_values_set_string;
    utils::Ptr<void> key_values_set_int;
  };
}
