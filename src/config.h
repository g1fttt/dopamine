#pragma once

#include <Windows.h>

#include <serdepp/adaptor/toml11.hpp>
#include <serdepp/serde.hpp>

#define FIELD(field, key, value) (&field, key, default_<decltype(field)>(value))

namespace {
  template <typename T>
  concept Fundamental = std::is_fundamental_v<T>;

  template <typename T, typename Context = serde::serde_context<T>>
  concept Serde = requires(Context &ctx, T &value) {
    { T::serde(ctx, value) };
  };

  template <typename T>
    requires(Fundamental<T> || Serde<T>)
  struct Feature {
    // clang-format off
    DERIVE_SERDE(Feature,
      FIELD(Self::enabled, "enabled", false)
      FIELD(Self::value, "value", {}))
    // clang-format on

    bool enabled;
    T value;
  };

  struct Bunnyhop {
    // clang-format off
    DERIVE_SERDE(Bunnyhop,
      FIELD(Self::enabled, "enabled", false)
      FIELD(Self::chance, "chance", 100.0f))
    // clang-format on

    bool enabled;
    float chance;
  };

  struct Misc {
    // clang-format off
    DERIVE_SERDE(Misc,
      FIELD(Self::bunnyhop, "bunnyhop", {})
      FIELD(Self::aspect_ratio, "aspect_ratio", {.value = 1.0f})
      FIELD(Self::anti_screenshot, "anti_screenshot", false))
    // clang-format on

    Bunnyhop bunnyhop;
    Feature<float> aspect_ratio;
    bool anti_screenshot;
  };

  class ConfigAux {
  public:
    DERIVE_SERDE(ConfigAux, FIELD(Self::misc, "misc", {}))
  public:
    Misc misc;
  protected:
    void create_hidden_dir() const {
      if (std::filesystem::create_directory(DIR)) {
        SetFileAttributes(DIR, FILE_ATTRIBUTE_HIDDEN);
      }
    }

    void save() const {
      const auto value = serde::serialize<toml::value>(*this);
      std::ofstream file_desc{full_config_path()};
      file_desc << value << std::endl;
    }

    void load() {
      if (std::filesystem::exists(full_config_path())) {
        auto value = toml::parse(full_config_path());
        *this = serde::deserialize<ConfigAux>(value);
      }
    }
  private:
    static std::filesystem::path full_config_path() {
      return std::format("{}/{}", DIR, PATH);
    }
  private:
    constexpr static auto PATH = "config.toml";
    constexpr static auto DIR = "dopamine";
  };
}

class Config : public ConfigAux {
public:
  constexpr Config(Config &&) = delete;
  constexpr Config(Config &) = delete;
private:
  friend class App;

  constexpr Config() = default;

  void init_or_nothing() {
    if (static bool inited = false; !inited) {
      create_hidden_dir();
      load();

      inited = true;
    }
  }
};
