#pragma once

namespace utils
{
  template <typename T, size_t index, typename... Args>
  T call_vmethod(void *base, Args... args) {
    return (*reinterpret_cast<T(THISCALL ***)(void *, Args...)>(base))[index](
        base, args...);
  }
}

#define VMETHOD(RetType, name, index, args_def, args)                          \
  inline RetType name args_def {                                               \
    return utils::call_vmethod<RetType, index> args;                           \
  }
