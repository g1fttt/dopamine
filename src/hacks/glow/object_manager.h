#pragma once

#include <utils/color.h>

#include <vector>

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

    game::Entity *entity = nullptr;
    utils::Color color;
  };

  struct ObjectManager {
    ObjectManager(const core::Interfaces &interfaces,
                  core::MaterialCreator &material_creator);

    inline void register_object(const Object &obj) {
      objects.push_back(obj);
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
    void blur_glow_effects(const game::ViewSetup *view,
                           game::RenderContext *render_ctx) const;
    void apply_entity_glow_effects(const core::Interfaces &interfaces,
                                   const game::ViewSetup *view,
                                   game::RenderContext *render_ctx) const;

    game::Texture *rt_quarter_size_1 = nullptr;
    game::Texture *rt_glow_buf_1, *rt_glow_buf_2 = nullptr;

    game::Material *glow_material, *halo_add_to_screen_material = nullptr;
    game::Material *glow_blur_x_material, *glow_blur_y_material = nullptr;

    std::vector<Object> objects;
  };
}
