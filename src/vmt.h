#pragma once

#include <cstdint>
#include <memory>

class VMT final {
public:
  void init(void *base);
  void reset();

  void hook(void *hook, size_t index);

  template <typename T, size_t index, typename... Args>
  constexpr auto get_original() {
    return reinterpret_cast<T(__thiscall *)(void *, Args...)>(
        original_vmt[index]);
  }

  template <typename T, size_t index, typename... Args>
  constexpr auto call_original(Args... args) {
    return get_original<T, index, Args...>()(base, args...);
  }
private:
  void *base;
  uintptr_t *vmt;
  std::unique_ptr<uintptr_t[]> original_vmt;
  size_t vmt_size;
};
