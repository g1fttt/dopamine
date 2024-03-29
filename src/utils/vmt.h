#pragma once

#include <memory>

namespace utils {
  class VMT {
  public:
    void reset();
    void init(void *base);
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
