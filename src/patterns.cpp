#include "patterns.h"

#include "utils/ptr.h"

#include <Windows.h>

#include <Psapi.h>

#include <filesystem>
#include <format>

namespace fs = std::filesystem;

#define SHOW_MESSAGE_BOX_AND_EXIT(msg, ...)                                    \
  MessageBoxA(nullptr, std::format(msg, __VA_ARGS__).data(), nullptr, MB_OK);  \
  std::terminate()

// Returns ALWAYS valid pointer to someplace in module
// If provided pattern is not found in module, then MessageBox will show up
// and std::terminate will be called
static utils::Ptr<void> find_pattern(fs::path module_name,
                                     std::u8string_view pattern) {
  const auto module = GetModuleHandleW(module_name.c_str());

  static size_t pattern_id = 0;
  pattern_id += 1;

  if (!module) {
    SHOW_MESSAGE_BOX_AND_EXIT(
        "Failed to find pattern (#{}): handle to `{}` is nullptr", pattern_id,
        module_name.string());
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
  SHOW_MESSAGE_BOX_AND_EXIT(
      "Failed to find pattern (#{}): invalid or outdated pattern", pattern_id);
}

#undef SHOW_MESSAGE_BOX_AND_EXIT

namespace core
{
  Patterns::Patterns() {
    // clang-format off
    d3d9_present = find_pattern(L"GameOverlayRenderer.dll", u8"\xA1\xCC\xCC\xCC\xCC\x51\xFF\x75\x14")
                      .byte_add(1);
    d3d9_reset = find_pattern(L"GameOverlayRenderer.dll", u8"\xA1\xCC\xCC\xCC\xCC\x57\x53\xC7\x45\xFC\x00\x00\x00\x00")
                      .byte_add(1);

    key_values_constructor = find_pattern(L"StudioRender.dll", u8"\x55\x8B\xEC\x56\x8B\xF1\x6A");
    key_values_set_string = find_pattern(L"client.dll", u8"\x55\x8B\xEC\x57\x6A\x01\xFF\x75\x08\xE8\xCC\xCC\xCC\xCC\x8B\xF8\x85\xFF\x74\x60");
    key_values_set_int = find_pattern(L"StudioRender.dll", u8"\x55\x8B\xEC\x6A\x01");
    // clang-format on
  }
}
