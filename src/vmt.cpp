#include "vmt.h"

#include <Windows.h>

#include <algorithm>

size_t calc_vmt_size(uintptr_t *vmt) {
  MEMORY_BASIC_INFORMATION info;
  size_t len = 0;

  const auto page_flags = PAGE_EXECUTE | PAGE_EXECUTE_READ |
                          PAGE_EXECUTE_READWRITE | PAGE_EXECUTE_WRITECOPY;
  while (true) {
    const auto query = VirtualQuery(LPCVOID(vmt[len]), &info, sizeof(info));
    if (!query || !(info.Protect & page_flags)) {
      break;
    }
    len += 1;
  }
  return len;
}

namespace core {
  void VMT::init(void *base) {
    this->base = base;
    vmt = *reinterpret_cast<uintptr_t **>(base);
    vmt_size = calc_vmt_size(vmt);
    original_vmt = std::make_unique<uintptr_t[]>(vmt_size);
    std::copy(vmt, vmt + vmt_size, original_vmt.get());
  }

  void VMT::reset() {
    DWORD old = 0;

    if (VirtualProtect(vmt, vmt_size, PAGE_EXECUTE_READWRITE, &old)) {
      std::copy(original_vmt.get(), original_vmt.get() + vmt_size, vmt);
      VirtualProtect(vmt, vmt_size, old, nullptr);
    }
  }

  void VMT::hook(void *hook, size_t index) {
    uintptr_t *target = vmt + index;
    DWORD old = 0;

    if (VirtualProtect(target, sizeof(target), PAGE_EXECUTE_READWRITE, &old)) {
      *target = uintptr_t(hook);
      VirtualProtect(target, sizeof(target), old, nullptr);
    }
  }
}
