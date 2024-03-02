#include "hooks.h"

#include <app.h>

void STDCALL hooks::frame_stage_notify(int32_t stage) {
  return App::get().client_vmt.call_original<void, 35>(stage);
}
