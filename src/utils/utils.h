#pragma once

#include "ptr.h"

#include <filesystem>
#include <string_view>

namespace fs = std::filesystem;

namespace utils {
  // Returns ALWAYS valid pointer to someplace in module
  // If provided pattern is not found in module, then MessageBox will show up
  // and std::terminate will be called
  Ptr<void> find_pattern(fs::path module_name, std::u8string_view pattern);
}
