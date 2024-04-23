#pragma once

#include <Windows.h>

#include "ptr.h"

#include <type_traits>

namespace utils {
  template <typename T, typename... Args> struct VMTHook {
    constexpr VMTHook(const VMTHook &) = delete;
    constexpr VMTHook() = default;

    template <size_t N>
    void init_and_hook(void *base,
                       const std::add_pointer_t<T STDCALL(Args...)> &hook) {
      this->base = base;

      const Ptr<void *> vtable = *this->base.template cast<void **>();
      ptr_to_target = vtable.add(N).get();
      original = *ptr_to_target;

      DWORD old = 0;
      if (VirtualProtect(ptr_to_target, sizeof(ptr_to_target),
                         PAGE_EXECUTE_READWRITE, &old)) {
        *ptr_to_target = LPVOID(hook);
        VirtualProtect(ptr_to_target, sizeof(ptr_to_target), old, nullptr);
      }
    }

    void unhook() {
      DWORD old = 0;
      if (VirtualProtect(ptr_to_target, sizeof(ptr_to_target),
                         PAGE_EXECUTE_READWRITE, &old)) {
        *ptr_to_target = original;
        VirtualProtect(ptr_to_target, sizeof(ptr_to_target), old, nullptr);
      }
    }

    T call_original(Args... args) const {
      return OriginalMethodPtr(original)(base, args...);
    }
  private:
    using OriginalMethodPtr =
        std::add_pointer_t<T THISCALL(Ptr<void>, Args...)>;

    Ptr<void> base;
    void *original = nullptr;
    void **ptr_to_target = nullptr;
  };
}
