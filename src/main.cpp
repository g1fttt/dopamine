#include "app.h"

#include <thread>

BOOL WINAPI DllMain(HMODULE module, DWORD reason, LPVOID reserved) {
  if (reason == DLL_PROCESS_ATTACH) {
    auto t = std::thread([=]() {
      app.emplace(module);
      {
        while (!app->must_unload) {
          using namespace std::chrono_literals;

          std::this_thread::sleep_for(50ms);
        }
      }
      app.reset();
    });
    t.detach();
  }
  return TRUE;
}
