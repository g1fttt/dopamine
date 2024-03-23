#pragma once

#include <expected>
#include <string_view>

#include "ptr.h"

#define WSV_TO_S(wsv)                                                          \
  std::string {                                                                \
    wsv.begin(), wsv.end()                                                     \
  }

namespace utils {
  // Returns ALWAYS valid pointer to someplace in module
  // If provided pattern is not found in module, then MessageBox will show up
  // and std::unreachable will be called
  Ptr<void> find_pattern(std::wstring_view module_name,
                         std::u8string_view pattern);
}
