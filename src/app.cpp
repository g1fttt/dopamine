#include "app.h"

#include "hooks/hooks.h"
#include "interfaces/input_system.h"
#include "utils/utils.h"

#include <imgui.h>
#include <imgui_impl_dx9.h>
#include <imgui_impl_win32.h>

void App::find_interfaces() {
  const auto client = utils::interface_base("client.dll", "VClient017");
  client_mode = **reinterpret_cast<void ***>(
      (*reinterpret_cast<uintptr_t **>(client))[10] + 5);

  interfaces.entity_list = reinterpret_cast<interfaces::EntityList *>(
      utils::interface_base("client.dll", "VClientEntityList003"));
  interfaces.engine = reinterpret_cast<interfaces::Engine *>(
      utils::interface_base("engine.dll", "VEngineClient013"));
  interfaces.cvar = reinterpret_cast<interfaces::CVar *>(
      utils::interface_base("vstdlib.dll", "VEngineCvar004"));
  interfaces.input_system = reinterpret_cast<interfaces::InputSystem *>(
      utils::interface_base("inputsystem.dll", "InputSystemVersion001"));
  interfaces.surface = reinterpret_cast<interfaces::Surface *>(
      utils::interface_base("vguimatsurface.dll", "VGUI_Surface030"));
}

void App::find_patterns() {
  const auto d3d9_present_addr = utils::find_pattern(
      "GameOverlayRenderer.dll", u8"\xA1\xCC\xCC\xCC\xCC\x51\xFF\x75\x14");
  const auto d3d9_present = d3d9_present_addr + 1;
  d3d9_present_original =
      **reinterpret_cast<decltype(d3d9_present_original) **>(d3d9_present);
  d3d9_present_raw = d3d9_present;

  const auto d3d9_reset_addr = utils::find_pattern(
      "GameOverlayRenderer.dll",
      u8"\xA1\xCC\xCC\xCC\xCC\x57\x53\xC7\x45\xFC\x00\x00\x00\x00");
  const auto d3d9_reset = d3d9_reset_addr + 1;
  d3d9_reset_original =
      **reinterpret_cast<decltype(d3d9_reset_original) **>(d3d9_reset);
  d3d9_reset_raw = d3d9_reset;
}

void App::init_vmts() {
  vmts.client_mode.init(client_mode);
  vmts.surface.init(interfaces.surface);
}

void App::setup_hooks() {
  **reinterpret_cast<decltype(hooks::present) ***>(d3d9_present_raw) =
      hooks::present;
  **reinterpret_cast<decltype(hooks::reset) ***>(d3d9_reset_raw) = hooks::reset;

  vmts.client_mode.hook(LPVOID(hooks::create_move), 21);
  vmts.surface.hook(LPVOID(hooks::lock_cursor), 62);
}

App &App::get() {
  static App self{};

  if (static bool inited = false; !inited) {
    self.window = FindWindowA("Valve001", nullptr);

    self.find_interfaces();
    self.find_patterns();

    self.init_vmts();
    self.setup_hooks();

    // Hook WndProc at the end of App initialization to prevent multiple
    // initialization
    self.original_wnd_proc = WNDPROC(SetWindowLongPtrW(
        self.window, GWLP_WNDPROC, LONG_PTR(hooks::wnd_proc)));

    inited = true;
  }
  return self;
}

void App::with(const std::function<void(App &)> &cb) {
  cb(App::get());
}

void App::reset() {
  SetWindowLongPtrW(window, GWLP_WNDPROC, LONG_PTR(original_wnd_proc));

  interfaces.input_system->enable_input(true);

  vmts.surface.reset();

  **reinterpret_cast<void ***>(d3d9_present_raw) =
      LPVOID(d3d9_present_original);
  **reinterpret_cast<void ***>(d3d9_reset_raw) = LPVOID(d3d9_reset_original);

  ImGui_ImplWin32_Shutdown();
  ImGui_ImplDX9_Shutdown();
}
