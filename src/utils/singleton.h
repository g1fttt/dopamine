#pragma once

#include <functional>
#include <optional>

namespace utils {
  template <typename T> struct Singleton {
    using InitFunc = std::function<void(T &)>;

    constexpr Singleton(const Singleton &) = delete;
    constexpr Singleton &operator=(const Singleton &) = delete;

    static T &get() {
      return get_or_init(std::nullopt);
    }

    static T &get_or_init(const std::optional<InitFunc> &f) {
      static T self{};
      if (static bool inited = false; !inited && f.has_value()) {
        f.value()(self);
        inited = true;
      }
      return self;
    }
  protected:
    constexpr Singleton() = default;
  };
}
