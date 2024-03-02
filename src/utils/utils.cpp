#include "utils.h"

#include <Windows.h>

#include <Psapi.h>

#include <cstdint>
#include <format>

namespace utils {
  void *interface_base(std::string_view module_name,
                       std::string_view interface_name) {
    const auto module = GetModuleHandleA(module_name.data());

    using CreateInterface = void *(*)(const char *, int32_t *);
    const auto create_interface = reinterpret_cast<CreateInterface>(
        GetProcAddress(module, "CreateInterface"));

    return create_interface(interface_name.data(), nullptr);
  }

  std::expected<std::byte *, std::string>
  find_pattern(std::string_view module_name, std::u8string_view pattern) {
    const auto module = GetModuleHandleA(module_name.data());

    static size_t pattern_id = 0;
    pattern_id += 1;

    if (!module) {
      return std::unexpected(
          std::format("Failed to find pattern (#{}): handle to `{}` is nullptr",
                      pattern_id, module_name));
    }

    MODULEINFO info{};
    GetModuleInformation(GetCurrentProcess(), module, &info, sizeof(info));

    const auto base = reinterpret_cast<std::byte *>(info.lpBaseOfDll);
    const auto size = info.SizeOfImage;

    for (size_t i = 0; i < size - pattern.length(); i += 1) {
      bool found = true;

      for (size_t j = 0; j < pattern.length(); j += 1) {
        // 0xCC is a breakpoint opcode (usually unused in regular assembly),
        // so we can use it as wildcard
        if (pattern[j] != char8_t(base[i + j]) && pattern[j] != u8'\xCC') {
          found = false;
          break;
        }
      }

      if (found) {
        return base + i;
      }
    }
    return std::unexpected(
        std::format("Failed to find pattern (#{}): invalid or outdated pattern",
                    pattern_id));
  }
}
