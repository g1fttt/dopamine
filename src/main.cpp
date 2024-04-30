#include "app.h"

BOOL WINAPI DllMain(HMODULE module, DWORD reason, LPVOID reserved) {
  switch (reason) {
  case DLL_PROCESS_ATTACH:
    app = std::make_unique<core::App>(module);
    break;
  case DLL_PROCESS_DETACH:
    std::destroy_at(app.release());
    break;
  default:
    break;
  };
  return TRUE;
}
