#pragma once

#include <expected>
#include <string>
#include <string_view>

namespace utils {
  void *interface_base(std::string_view module_name,
                       std::string_view interface_name);
  std::expected<std::byte *, std::string>
  find_pattern(std::string_view module_name, std::u8string_view pattern);
}
