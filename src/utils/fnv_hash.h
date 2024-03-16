#pragma once

#include <cstdint>
#include <string_view>

namespace utils::fnv {
  constexpr uintptr_t hash(std::string_view s) {
    constexpr auto PRIME = 0x01000193;
    constexpr auto OFFSET_BASIS = 0x811C9DC5;

    auto x = OFFSET_BASIS;

    for (size_t i = 0; s[i]; i += 1) {
      x = (x ^ s[i]) * PRIME;
    }
    return x;
  }
}
