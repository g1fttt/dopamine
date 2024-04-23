#include "config.h"

#include <Windows.h>

#include "hacks/glow/hack.h"

#include "hacks/misc.h"
#include "hacks/visuals.h"

#include <filesystem>

namespace fs = std::filesystem;

constexpr auto PATH = "config.toml";
constexpr auto DIR = "dopamine";

static void create_hidden_dir() {
  if (fs::create_directory(DIR)) {
    SetFileAttributes(DIR, FILE_ATTRIBUTE_HIDDEN);
  }
}

static fs::path full_config_path() {
  return std::format("{}/{}", DIR, PATH);
}

struct Config {
  // clang-format off
  DERIVE_SERDE(Config,
    FIELD(misc, "misc")
    FIELD(visuals, "visuals")
    FIELD(glow, "glow"))
  // clang-format on

  hacks::Misc::Config misc;
  hacks::Visuals::Config visuals;
  glow::Hack::Config glow;
};

namespace core::config
{
  void save() {
    const auto value = serde::serialize<toml::value>(Config{
        .misc = hacks::misc.config,
        .visuals = hacks::visuals.config,
        .glow = glow::hack.config,
    });
    std::ofstream file_desc{full_config_path()};
    file_desc << value << std::endl;
  }

  void load() {
    if (!fs::exists(full_config_path())) {
      return;
    }

    auto value = toml::parse(full_config_path());

    const auto config = serde::deserialize<Config>(value);
    {
      hacks::misc.config = config.misc;
      hacks::visuals.config = config.visuals;
      glow::hack.config = config.glow;
    }
  }

  void init_or_nothing() {
    if (static bool inited = false; !inited) {
      create_hidden_dir();
      load();

      inited = true;
    }
  }
}
