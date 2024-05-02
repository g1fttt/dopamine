#pragma once

#include "game/key_values.h"
#include "game/material_system.h"

#include "interfaces.h"

#include <memory>

namespace game
{
  struct Material;
}

namespace core
{
  struct IntermedidateMaterial {
    IntermedidateMaterial(game::KeyValues *kv,
                          const core::Interfaces &interfaces)
        : kv{kv}
        , interfaces{interfaces} {}

    [[nodiscard]] inline IntermedidateMaterial &string(const char *key,
                                                       const char *value) {
      kv->set_string(key, value);
      return *this;
    }

    [[nodiscard]] inline IntermedidateMaterial &integer(const char *key,
                                                        int32_t value) {
      kv->set_integer(key, value);
      return *this;
    }

    [[nodiscard]] inline IntermedidateMaterial &boolean(const char *key,
                                                        bool value) {
      return integer(key, value);
    }

    inline game::Material *bind(const char *material_name) {
      return interfaces.material_system->create_material(material_name, kv);
    }
  private:
    game::KeyValues *kv = nullptr;
    const core::Interfaces &interfaces;
  };

  struct MaterialCreator {
    [[nodiscard]] IntermedidateMaterial
    create(const char *shader, const core::Interfaces &interfaces) {
      return {std::construct_at(kvs.allocate(1), shader), interfaces};
    }
  private:
    std::allocator<game::KeyValues> kvs;
  };
}
