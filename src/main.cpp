#include "app.h"
#include "config.h"

BOOL WINAPI DllMain(HMODULE module, DWORD reason, LPVOID reserved) {
  switch (reason) {
  case DLL_PROCESS_ATTACH:
    app = std::make_unique<core::App>(module);
    app->setup();
    break;
  case DLL_PROCESS_DETACH:
    std::destroy_at(app.release());
    core::config::save();
    break;
  default:
    break;
  };
  return TRUE;
}
