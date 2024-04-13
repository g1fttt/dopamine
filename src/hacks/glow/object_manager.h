#pragma once

#include <utils/color.h>

#include <forward_list>

namespace game {
  struct Entity;
  struct ViewSetup;
  struct RenderContext;
  struct Material;
  struct Texture;
}

namespace interfaces {
  struct MaterialSystem;
}

PRIVATE_USE(interfaces::MaterialSystem)

struct App;

namespace glow {
  struct Object {
    bool should_draw() const;
    void draw_model() const;

    game::Entity *entity = nullptr;
    const utils::Color &color;
  };

  struct ObjectManager {
    constexpr ObjectManager(const ObjectManager &) = delete;

    static ObjectManager &get_or_init(MaterialSystem *mat_system) {
      static ObjectManager self{};
      if (mat_system) {
        self.init_or_nothing(mat_system);
      }
      return self;
    }

    static ObjectManager &get() {
      return get_or_init(nullptr);
    }

    void register_object(const Object &object);
    void unregister_object_by_entity(game::Entity *entity);

    void draw_glow_effects(const game::ViewSetup *view, const App &app) const;
    bool has_glow_effect(game::Entity *entity) const;
  private:
    ObjectManager() = default;

    void draw_glow_models(const game::ViewSetup *view,
                          game::RenderContext *render_ctx,
                          const App &app) const;
    void apply_entity_glow_effects(const game::ViewSetup *view,
                                   game::RenderContext *render_ctx,
                                   const App &app) const;
    void init_or_nothing(MaterialSystem *mat_system);

    game::Texture *rt_full_frame, *rt_quarter_size_1 = nullptr;
    game::Material *glow_material, *halo_add_to_screen_material = nullptr;
    std::forward_list<Object> objects;
  };
}
