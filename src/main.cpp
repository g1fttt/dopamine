#include <Windows.h>

#include <chrono>
#include <functional>
#include <thread>

#include "internal/convar.h"

#include "interfaces/cvar.h"

#include "hooks/hooks.h"
#include "utils/utils.h"

#include "app.h"
#include "vmt.h"

BOOL WINAPI DllMain(HINSTANCE inst_dll, DWORD reason, LPVOID reserved) {
  if (reason == DLL_PROCESS_ATTACH) {
    auto t = std::thread([]() {
      App::with([](App &app) {
        app.client_vmt.hook(LPVOID(hooks::frame_stage_notify), 36);

        // TODO: Make it work (should_unhook)
        while (!app.should_unhook) {
          using namespace std::chrono_literals;

          std::this_thread::sleep_for(50ms);
        }

        // TODO: Maybe RAII destructor?
        app.client_vmt.reset();
      });
    });

    t.detach();
  }
  return TRUE;
}
