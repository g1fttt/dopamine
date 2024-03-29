#include "utils.h"

#include <Windows.h>

#include <Psapi.h>

#include <utils/logger.h>

#define LOG_FATAL_AND_EXIT(msg, ...)                                           \
  Logger::get().log<Level::Fatal>(msg, __VA_ARGS__);                           \
  std::exit(1)

Ptr<void> utils::find_pattern(std::wstring_view module_name,
                              std::u8string_view pattern) {
  const auto module = GetModuleHandleW(module_name.data());

  static size_t pattern_id = 0;
  pattern_id += 1;

  if (!module) {
    LOG_FATAL_AND_EXIT(
        "Failed to find pattern (#{}): handle to `{}` is nullptr", pattern_id,
        WSV_TO_S(module_name));
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
  LOG_FATAL_AND_EXIT(
      "Failed to find pattern (#{}): invalid or outdated pattern", pattern_id);
}

#undef LOG_FATAL_AND_EXIT
