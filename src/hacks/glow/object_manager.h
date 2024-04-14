#pragma once

#include <utils/color.h>
#include <utils/singleton.h>

#include <forward_list>

namespace game {
  struct Entity;
  struct ViewSetup;
  struct RenderContext;
  struct Material;
  struct Texture;
  struct MaterialSystem;
}

struct App;

namespace glow {
  struct Object {
    bool should_draw() const;
    void draw_model() const;

    bool enabled = false;
    game::Entity *entity = nullptr;
    utils::Color color;
  };

  struct ObjectManager : utils::Singleton<ObjectManager> {
    static Singleton<ObjectManager>::InitFunc
    init_func(game::MaterialSystem *mat_system) {
      return [=](ObjectManager &obj_manager) {
        obj_manager.init_or_nothing(mat_system);
      };
    }

    void register_entity(game::Entity *entity);
    // TODO: Unregister player's entity when the so leaves the server
    void unregister_object_by_entity(game::Entity *entity);
    void update_object_by_entity(game::Entity *entity,
                                 const utils::Color &color);

    void force_disable() {
      for (auto &obj : objects) {
        obj.enabled = false;
      }
    }

    constexpr void clear_objects() {
      objects.clear();
    }

    void draw_glow_effects(const game::ViewSetup *view, const App &app) const;
    bool has_glow_effect(game::Entity *entity) const;
  private:
    void draw_glow_models(const game::ViewSetup *view,
                          game::RenderContext *render_ctx,
                          const App &app) const;
    void apply_entity_glow_effects(const game::ViewSetup *view,
                                   game::RenderContext *render_ctx,
                                   const App &app) const;
    void init_or_nothing(game::MaterialSystem *mat_system);

    game::Texture *rt_full_frame, *rt_quarter_size_1 = nullptr;
    game::Material *glow_material, *halo_add_to_screen_material = nullptr;
    std::forward_list<Object> objects;
  };
}
