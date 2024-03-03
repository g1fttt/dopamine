#include "hooks.h"

#include <app.h>

void STDCALL hooks::frame_stage_notify(int32_t stage) {
  return App::get().vmts.client.call_original<void, 35>(stage);
}
