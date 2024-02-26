#pragma once

namespace utils {
  namespace __vmethod {
    template <typename T, size_t index, typename... Args>
    T call(void *base, Args... args) {
      return (*reinterpret_cast<T(THISCALL ***)(void *, Args...)>(base))[index](
          base, args...);
    }
  }

#define VMETHOD(RetType, name, index, args_def, args)                          \
  RetType name args_def {                                                      \
    return utils::__vmethod::call<RetType, index> args;                        \
  }
}
