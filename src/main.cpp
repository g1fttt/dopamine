#include <Windows.h>

#include <chrono>
#include <functional>
#include <thread>

#include "convar.h"
#include "cvar.h"
#include "vmt.h"

struct App {
  static App &get();
  static void with(const std::function<void(App &)> &cb);

  bool should_unhook = false;

  VMT client_vmt;

  void *client;
  CVar *cvar;
};

void *interface_base(const char *, const char *);

App &App::get() {
  static App APP;

  if (static bool inited = false; !inited) {
    APP.client = interface_base("client.dll", "VClient017");
    APP.cvar = reinterpret_cast<CVar *>(
        interface_base("vstdlib.dll", "VEngineCvar004"));

    APP.client_vmt.init(APP.client);

    inited = true;
  }
  return APP;
}

void App::with(const std::function<void(App &)> &cb) {
  cb(App::get());
}

void *interface_base(const char *module_name, const char *interface_name) {
  const auto module = GetModuleHandleA(module_name);

  using CreateInterface = void *(*)(const char *, int32_t *);
  const auto create_interface = reinterpret_cast<CreateInterface>(
      GetProcAddress(module, "CreateInterface"));

  return create_interface(interface_name, nullptr);
}

void __stdcall frame_stage_notify(int stage) {
  App::with([=](App &app) {
    app.cvar->find_var("mat_postprocess_enable")->set_value(0);
    app.client_vmt.call_original<void, 36>(stage);
  });
}

BOOL WINAPI DllMain(HINSTANCE inst_dll, DWORD reason, LPVOID reserved) {
  if (reason == DLL_PROCESS_ATTACH) {
    auto t = std::thread([]() {
      App::with([](App &app) {
        app.client_vmt.hook(LPVOID(frame_stage_notify), 36);

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
