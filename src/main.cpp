#include "app.h"

#include <thread>

BOOL WINAPI DllMain(HMODULE module, DWORD reason, LPVOID reserved) {
  if (reason == DLL_PROCESS_ATTACH) {
    auto t = std::thread([=]() {
      app = std::make_unique<core::App>(module);
      {
        while (!app->should_unload) {
          using namespace std::chrono_literals;

          std::this_thread::sleep_for(50ms);
        }
      }
      std::destroy_at(app.release());
    });
    t.detach();
  }
  return TRUE;
}
