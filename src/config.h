#pragma once

#include <serdepp/adaptor/toml11.hpp>
#include <serdepp/serde.hpp>

#define FIELD(field, key)                                                      \
  (&Self::field, key, default_<decltype(Self::field)>((Self{}).field))

namespace core::config
{
  template <typename T>
  concept Fundamental = std::is_fundamental_v<T>;

  template <typename T, typename Context = serde::serde_context<T>>
  concept Serde = requires(Context &ctx, T &value) {
    {
      T::serde(ctx, value)
    };
  };

  template <typename T>
    requires(Fundamental<T> || Serde<T>)
  struct Feature {
    // clang-format off
    DERIVE_SERDE(Feature,
      FIELD(enabled, "enabled")
      FIELD(value, "value"))
    // clang-format on

    bool enabled = false;
    T value;
  };

  void save();
  void load();
  void init_or_nothing();
}
