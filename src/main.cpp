#include "app.h"

#include <thread>

BOOL WINAPI DllMain(HMODULE module, DWORD reason, LPVOID reserved) {
  if (reason == DLL_PROCESS_ATTACH) {
    auto t = std::thread([=]() {
      // Inclose app instance and implicitly initialize it on first call (now)
      App::get_or_init(App::init_func(module)).and_then<void>([](App &app) {
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
