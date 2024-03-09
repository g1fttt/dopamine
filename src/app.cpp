#include "app.h"

#include "hooks/hooks.h"
#include "interfaces/input_system.h"
#include "utils/utils.h"

#include <imgui.h>
#include <imgui_impl_dx9.h>
#include <imgui_impl_win32.h>

void App::find_interfaces() {
  interfaces.cvar = reinterpret_cast<interfaces::CVar *>(
      utils::interface_base("vstdlib.dll", "VEngineCvar004"));
  interfaces.input_system = reinterpret_cast<interfaces::InputSystem *>(
      utils::interface_base("inputsystem.dll", "InputSystemVersion001"));
  interfaces.surface = reinterpret_cast<interfaces::Surface *>(
      utils::interface_base("vguimatsurface.dll", "VGUI_Surface030"));
}

void App::find_patterns() {
  /*
    0:  FF 15 ? ? ? ?           call   DWORD PTR ds:0x????????
              |
              +> GameOverlayRenderer.dll IDirect3DDevice9::Present hook
    6:  8B F0                   mov    esi, eax
    8:  85 FF                   test   edi, edi
  */
  const auto d3d9_present_addr = utils::find_pattern(
      "GameOverlayRenderer.dll", u8"\xFF\x15\xCC\xCC\xCC\xCC\x8B\xF0\x85\xFF");
  if (d3d9_present_addr.has_value()) {
    const auto d3d9_present = uintptr_t(d3d9_present_addr.value() + 2);
    d3d9_present_original =
        **reinterpret_cast<decltype(d3d9_present_original) **>(d3d9_present);
    d3d9_present_raw = d3d9_present;
  }

  /*
    0:  C7 45 FC ? ? ? ?        mov    DWORD PTR [ebp-0x4], 0x????????
    7:  FF 15 ? ? ? ?           call   DWORD PTR ds:0x????????
              |
              +> GameOverlayRenderer.dll IDirect3DDevice9::Reset hook
    d:  8B D8                   mov    ebx, eax
  */
  // I believe that exactly this signature was made and +9 offset used because
  // direct call-op signature is too long
  const auto d3d9_reset_addr = utils::find_pattern(
      "GameOverlayRenderer.dll",
      u8"\xC7\x45\xFC\xCC\xCC\xCC\xCC\xFF\x15\xCC\xCC\xCC\xCC\x8B\xD8");
  if (d3d9_reset_addr.has_value()) {
    const auto d3d9_reset = uintptr_t(d3d9_reset_addr.value() + 9);
    d3d9_reset_original =
        **reinterpret_cast<decltype(d3d9_reset_original) **>(d3d9_reset);
    d3d9_reset_raw = d3d9_reset;
  }
}

void App::init_vmts() {
  vmts.surface.init(interfaces.surface);
}

void App::setup_hooks() {
  // FIXME: These hooks won't work (and game will crash ofc) if
  // GameOverlayRender.dll is not inited.
  **reinterpret_cast<decltype(hooks::present) ***>(d3d9_present_raw) =
      hooks::present;
  **reinterpret_cast<decltype(hooks::reset) ***>(d3d9_reset_raw) = hooks::reset;

  vmts.surface.hook(LPVOID(hooks::lock_cursor), 62);
}

App &App::get() {
  static App app{};

  if (static bool inited = false; !inited) {
    app.window = FindWindowA("Valve001", nullptr);
    app.original_wnd_proc = WNDPROC(
        SetWindowLongPtrW(app.window, GWLP_WNDPROC, LONG_PTR(hooks::wnd_proc)));

    app.find_interfaces();
    app.find_patterns();

    app.init_vmts();
    app.setup_hooks();

    inited = true;
  }
  return app;
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
