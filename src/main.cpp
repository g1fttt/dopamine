#include "app.h"

#include <thread>

BOOL WINAPI DllMain(HMODULE module, DWORD reason, LPVOID reserved) {
  if (reason == DLL_PROCESS_ATTACH) {
    auto t = std::thread([=]() {
      core::app.emplace(module);
      {
        while (!core::app->must_unload) {
          using namespace std::chrono_literals;

          std::this_thread::sleep_for(50ms);
        }
      }
      core::app.reset();
    });
    t.detach();
  }
  return TRUE;
}
