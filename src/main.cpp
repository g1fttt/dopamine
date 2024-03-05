#include <Windows.h>

#include <thread>

#include "app.h"

BOOL WINAPI DllMain(HINSTANCE inst_dll, DWORD reason, LPVOID reserved) {
  if (reason == DLL_PROCESS_ATTACH) {
    auto t = std::thread([]() {
      App::with([](App &app) {
        while (!app.must_unhook) {
          using namespace std::chrono_literals;

          std::this_thread::sleep_for(50ms);
        }
        app.reset();
      });
    });
    t.detach();
  }
  return TRUE;
}
