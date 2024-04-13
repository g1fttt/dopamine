#include "config.h"

#include <Windows.h>

#include "hacks/glow/hack.h"

#include "hacks/misc.h"
#include "hacks/visuals.h"

#include <filesystem>

namespace fs = std::filesystem;

PRIVATE_USE(hacks::Misc)
PRIVATE_USE(hacks::Visuals)

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

  Misc::Config misc;
  Visuals::Config visuals;
  glow::Hack::Config glow;
};

void config::save() {
  const auto value = serde::serialize<toml::value>(Config{
      .misc = Misc::get().config,
      .visuals = Visuals::get().config,
      .glow = glow::Hack::get().config,
  });
  std::ofstream file_desc{full_config_path()};
  file_desc << value << std::endl;
}

void config::load() {
  if (!fs::exists(full_config_path())) {
    return;
  }

  auto value = toml::parse(full_config_path());

  const auto config = serde::deserialize<Config>(value);
  {
    Misc::get().config = config.misc;
    Visuals::get().config = config.visuals;
    glow::Hack::get().config = config.glow;
  }
}

void config::init_or_nothing() {
  if (static bool inited = false; !inited) {
    create_hidden_dir();
    load();

    inited = true;
  }
}
