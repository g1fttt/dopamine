#pragma once

#include <Windows.h>

#include "ptr.h"

#include <optional>

namespace utils {
  template <typename T, typename... Args> struct VMTHook {
    constexpr VMTHook(const VMTHook &&) = delete;
    constexpr VMTHook(const VMTHook &) = delete;
    constexpr VMTHook() = default;

    template <size_t N>
    void init_and_hook(void *base,
                       const std::add_pointer_t<T STDCALL(Args...)> &hook) {
      index = N;
      this->base = base;

      vtable = *this->base.template cast<void **>();
      const auto ptr_to_target = get_ptr_to_target();

      original = *ptr_to_target;

      DWORD old = 0;
      if (VirtualProtect(ptr_to_target, sizeof(ptr_to_target),
                         PAGE_EXECUTE_READWRITE, &old)) {
        *ptr_to_target = LPVOID(hook);
        VirtualProtect(ptr_to_target, sizeof(ptr_to_target), old, nullptr);
      }
    }

    void reset() {
      const auto ptr_to_target = get_ptr_to_target();

      DWORD old = 0;
      if (VirtualProtect(ptr_to_target, sizeof(ptr_to_target),
                         PAGE_EXECUTE_READWRITE, &old)) {
        *ptr_to_target = original;
        VirtualProtect(ptr_to_target, sizeof(ptr_to_target), old, nullptr);
      }
    }

    constexpr T call_original(Args... args) const {
      return get_original()(base, args...);
    }
  private:
    constexpr auto get_original() const {
      return reinterpret_cast<T(THISCALL *)(Ptr<void>, Args...)>(original);
    }

    constexpr void **get_ptr_to_target() const {
      return vtable.add(index.value()).get();
    }

    Ptr<void> base;
    Ptr<void *> vtable;
    void *original = nullptr;
    std::optional<size_t> index = std::nullopt;
  };
}
