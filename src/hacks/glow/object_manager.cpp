#include "object_manager.h"

#include <game/entity.h>
#include <game/entity_list.h>
#include <game/key_values.h>
#include <game/material.h>
#include <game/material_system.h>
#include <game/material_var.h>
#include <game/model_render.h>
#include <game/render_context.h>
#include <game/render_view.h>
#include <game/texture.h>
#include <game/view.h>

#include <app.h>

struct StencilState {
  constexpr static void create_and_set(const StencilState &self,
                                       game::RenderContext *render_ctx) {
    self.set(render_ctx);
  }

  constexpr static void default_and_set(game::RenderContext *render_ctx) {
    create_and_set({}, render_ctx);
  }

  bool enable = false;
  game::StencilOp fail_op, z_fail_op, pass_op = game::StencilOp::Keep;
  game::StencilCmpFunc cmp_func = game::StencilCmpFunc::Always;
  int32_t ref_value = 0;
  uint32_t test_mask, write_mask = 0xFFFFFFFF;
private:
  constexpr void set(game::RenderContext *render_ctx) const {
    render_ctx->set_stencil_enable(enable);
    render_ctx->set_stencil_fail_operation(fail_op);
    render_ctx->set_stencil_z_fail_operation(z_fail_op);
    render_ctx->set_stencil_pass_operation(pass_op);
    render_ctx->set_stencil_cmp_func(cmp_func);
    render_ctx->set_stencil_ref_value(ref_value);
    render_ctx->set_stencil_test_mask(test_mask);
    render_ctx->set_stencil_write_mask(write_mask);
  }
};

namespace glow {
  bool Object::should_draw() const {
    return enabled && entity && entity->renderable()->should_draw() &&
           !entity->networkable()->is_dormant() &&
           !app->should_anti_screenshot();
  }

  void Object::draw_model() const {
    entity->renderable()->draw_model();

    for (auto attachment = entity->move_child(); attachment;
         attachment = attachment->move_peer()) {
      if (const auto renderable = attachment->renderable();
          renderable->should_draw()) {
        renderable->draw_model();
      }
    }
  }
}

namespace glow {
  void ObjectManager::unregister_object_by_entity(game::Entity *entity) {
    objects.remove_if([=, this](const Object &obj) {
      return entity == obj.entity;
    });
  }

  void ObjectManager::update_object_by_entity(game::Entity *entity,
                                              const utils::Color &color) {
    auto it = std::find_if(objects.begin(), objects.end(),
                           [=, this](const Object &obj) {
                             return entity == obj.entity;
                           });
    if (it != objects.end()) {
      it->enabled = true;
      it->color = color;
    }
  }

  void ObjectManager::draw_glow_effects(const game::ViewSetup *view) const {
    const auto render_ctx = app->interfaces.material_system->render_context();
    {
      render_ctx->begin_pix_event("apply_entity_glow_effects");
      { apply_entity_glow_effects(view, render_ctx); }
      render_ctx->end_pix_event();
    }
  }

  bool ObjectManager::has_glow_effect(game::Entity *entity) const {
    return std::any_of(objects.begin(), objects.end(), [=](const Object &obj) {
      return entity == obj.entity;
    });
  }

  ObjectManager::ObjectManager(game::MaterialSystem *mat_system) {
    rt_full_frame =
        mat_system->find_texture("_rt_FullFrameFB", "RenderTargets");
    rt_full_frame->inc_ref_counter();

    rt_quarter_size_1 =
        mat_system->find_texture("_rt_SmallFB1", "RenderTargets");
    rt_quarter_size_1->inc_ref_counter();

    // TODO: Featureful "Material Creator" with std::initializer_list support

    // FIXME: Cleanup on hack unload
    auto glow_kv = new game::KeyValues{"UnlitGeneric"};
    {
      glow_kv->set_string("$BaseTexture", "white");
      glow_kv->set_int("$IgnoreZ", 1);
      glow_kv->set_int("$Model", 1);
      glow_kv->set_int("$LinearWrite", 1);
    }
    glow_material = mat_system->create_material("glow_color", glow_kv);

    auto halo_kv = new game::KeyValues{"screenspace_general"};
    {
      halo_kv->set_string("$PixShader", "haloaddoutline_ps20");
      halo_kv->set_int("$Alpha_Blend_Color_Overlay", 1);
      halo_kv->set_string("$BaseTexture", "_rt_FullFrameFB");
      halo_kv->set_int("$IgnoreZ", 1);
      halo_kv->set_int("$LinearRead_BaseTexture", 1);
      halo_kv->set_int("$LinearWrite", 1);
    }
    halo_add_to_screen_material =
        mat_system->create_material("halo_add_to_screen", halo_kv);
  }

  void ObjectManager::draw_glow_models(const game::ViewSetup *view,
                                       game::RenderContext *render_ctx) const {
    render_ctx->push_render_target_and_viewport(rt_full_frame);
    render_ctx->set_viewport(0, 0, view->width, view->height);

    const auto orig_color = app->interfaces.render_view->get_color_modulation();

    render_ctx->clear_color_3ub(0, 0, 0);
    render_ctx->clear_buffers(true, false);

    app->interfaces.model_render->forced_material_override(glow_material);

    StencilState::create_and_set({.test_mask = 0xFF}, render_ctx);

    for (const auto &obj : objects) {
      if (!obj.should_draw()) {
        continue;
      }

      app->interfaces.render_view->set_blend(obj.color.a);
      app->interfaces.render_view->set_color_modulation(
          obj.color.float_array());

      obj.draw_model();
    }

    app->interfaces.model_render->forced_material_override(nullptr);
    app->interfaces.render_view->set_color_modulation(orig_color.float_array());
    app->interfaces.render_view->set_blend(orig_color.a);

    StencilState::default_and_set(render_ctx);

    render_ctx->pop_render_target_and_viewport();
  }

  void ObjectManager::apply_entity_glow_effects(
      const game::ViewSetup *view, game::RenderContext *render_ctx) const {
    const auto glow_material = app->interfaces.material_system->find_material(
        "dev/glow_color", "Other Textures");
    app->interfaces.model_render->forced_material_override(glow_material);

    render_ctx->override_depth_enable(true, false);

    StencilState::default_and_set(render_ctx);

    const auto saved_blend = app->interfaces.render_view->get_blend();
    app->interfaces.render_view->set_blend(0.0f);

    bool drew_anything = false;

    for (const auto &obj : objects) {
      if (!obj.should_draw()) {
        continue;
      }

      StencilState::create_and_set({.enable = true,
                                    .z_fail_op = game::StencilOp::Replace,
                                    .pass_op = game::StencilOp::Replace,
                                    .ref_value = 1},
                                   render_ctx);

      obj.draw_model();

      drew_anything = true;
    }

    render_ctx->override_depth_enable(false, false);

    StencilState::default_and_set(render_ctx);

    app->interfaces.render_view->set_blend(saved_blend);
    app->interfaces.model_render->forced_material_override(nullptr);

    // https://github.com/ValveSoftware/source-sdk-2013/blob/0d8dceea4310fde5706b3ce1c70609d72a38efdf/sp/src/game/client/glow_outline_effect.cpp#L256-L260
    if (!drew_anything) {
      return;
    }

    render_ctx->begin_pix_event("draw_glow_models");
    { draw_glow_models(view, render_ctx); }
    render_ctx->end_pix_event();

    const auto dim_var = halo_add_to_screen_material->find_var("$C0_X");
    dim_var->set_value(1.0f);

    StencilState::create_and_set({.enable = true,
                                  .cmp_func = game::StencilCmpFunc::Equal,
                                  .test_mask = 0xFF,
                                  .write_mask = 0x0},
                                 render_ctx);

    constexpr auto GLOW_DOWNSAMPLE = 4.0f;

    render_ctx->draw_screen_space_rect(
        halo_add_to_screen_material, 0, 0, view->width, view->height, 0.0f,
        -0.5f, view->width / GLOW_DOWNSAMPLE - 1.0f,
        view->height / GLOW_DOWNSAMPLE - 1.0f,
        rt_quarter_size_1->actual_width(), rt_quarter_size_1->actual_height());

    StencilState::default_and_set(render_ctx);
  }
}
