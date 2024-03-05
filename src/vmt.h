#pragma once

#include <cstdint>
#include <memory>

namespace core {
  class VMT final {
  public:
    void init(void *base);
    void reset();

    void hook(void *hook, size_t index);

    template <typename T, size_t index, typename... Args>
    constexpr auto get_original() const {
      return reinterpret_cast<T(THISCALL *)(void *, Args...)>(
          original_vmt[index]);
    }

    template <typename T, size_t index, typename... Args>
    constexpr auto call_original(Args... args) const {
      return get_original<T, index, Args...>()(base, args...);
    }
  private:
    void *base;
    uintptr_t *vmt;
    std::unique_ptr<uintptr_t[]> original_vmt;
    size_t vmt_size;
  };
}
