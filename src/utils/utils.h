#pragma once

#include <expected>
#include <string_view>

namespace utils {
  void *interface_base(std::string_view module_name,
                       std::string_view interface_name);

  // Returns ALWAYS valid pointer to someplace in module
  // If provided pattern is not found in module, then MessageBox will show up
  // and std::unreachable will be called
  std::byte *find_pattern(std::string_view module_name,
                          std::u8string_view pattern);
}
