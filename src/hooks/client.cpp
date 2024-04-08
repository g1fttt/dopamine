#include "hooks.h"

#include <interfaces/engine.h>
#include <interfaces/entity_list.h>

#include <app.h>

void STDCALL hooks::frame_stage_notify(int32_t stage) {
  App::with<void>([=](App &app) {
    app.local_player = app.interfaces.entity_list->get_entity_by_index(
        app.interfaces.engine->get_local_player_index());
    return app.vmts.client.call_original<void, 35>(stage);
  });
}
