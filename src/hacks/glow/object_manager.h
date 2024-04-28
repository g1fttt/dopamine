#pragma once

#include <utils/color.h>

#include <forward_list>

namespace game
{
  struct Entity;
  struct ViewSetup;
  struct RenderContext;
  struct Material;
  struct Texture;
}

namespace core
{
  struct Interfaces;
  struct MaterialCreator;
}

namespace glow
{
  struct Object {
    bool should_draw() const;
    void draw_model() const;

    bool enabled = false;
    game::Entity *entity = nullptr;
    utils::Color color;
  };

  struct ObjectManager {
    ObjectManager(const core::Interfaces &interfaces,
                  core::MaterialCreator &material_creator);

    inline void register_entity(game::Entity *entity) {
      objects.push_front({.entity = entity});
    }

    void unregister_object_by_entity(game::Entity *entity);
    void update_object_by_entity(game::Entity *entity,
                                 const utils::Color &color);

    inline void force_disable() {
      for (auto &obj: objects) {
        obj.enabled = false;
      }
    }

    inline void clear_objects() {
      objects.clear();
    }

    void draw_glow_effects(const core::Interfaces &interfaces,
                           const game::ViewSetup *view) const;
    bool has_glow_effect(game::Entity *entity) const;
  private:
    void draw_glow_models(const core::Interfaces &interfaces,
                          const game::ViewSetup *view,
                          game::RenderContext *render_ctx) const;
    void apply_entity_glow_effects(const core::Interfaces &interfaces,
                                   const game::ViewSetup *view,
                                   game::RenderContext *render_ctx) const;

    game::Texture *rt_full_frame, *rt_quarter_size_1 = nullptr;
    game::Material *glow_material, *halo_add_to_screen_material = nullptr;
    std::forward_list<Object> objects;
  };
}
