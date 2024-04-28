#pragma once

#include "utils/ptr.h"

#define METHOD_FROM_PATTERN(field, pattern_name)                               \
  methods.field = patterns.pattern_name.transmute<decltype(methods.field)>()

#define METHOD_FROM_PATTERN_2(field)                                           \
  methods.field = patterns.field.transmute<decltype(methods.field)>()

namespace core
{
  struct Patterns {
    Patterns();

    utils::Ptr<void> d3d9_present, d3d9_reset;

    utils::Ptr<void> key_values_constructor;
    utils::Ptr<void> key_values_set_string;
    utils::Ptr<void> key_values_set_int;

    utils::Ptr<void> global_entity_list;
    utils::Ptr<void> add_entity_listener;

    utils::Ptr<void> is_local_player;
  };
}
