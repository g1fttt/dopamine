#pragma once

#include "utils/ptr.h"

namespace game {
  struct Client;
  struct EntityList;
  struct Engine;
  struct CVar;
  struct InputSystem;
  struct Surface;
  struct RenderView;
  struct MaterialSystem;
  struct ModelRender;
}

namespace core {
  struct Interfaces {
    Interfaces();

    utils::Ptr<game::Client> client;
    game::EntityList *entity_list = nullptr;
    game::Engine *engine = nullptr;
    game::CVar *cvar = nullptr;
    game::InputSystem *input_system = nullptr;
    game::Surface *surface = nullptr;
    game::RenderView *render_view;
    game::MaterialSystem *material_system;
    game::ModelRender *model_render;
    void *client_mode = nullptr;
  };
}
