#include "app.h"

#include "hooks/hooks.h"
#include "interfaces/input_system.h"
#include "utils/utils.h"

#include <imgui.h>
#include <imgui_impl_dx9.h>
#include <imgui_impl_win32.h>

void App::reset() {
  SetWindowLongPtrW(window, GWLP_WNDPROC, LONG_PTR(original_wnd_proc));

  interfaces.input_system->enable_input(true);

  vmts.client_mode.reset();
  vmts.surface.reset();

  **d3d9_present_raw.cast<decltype(hooks::present) **>() =
      d3d9_present_original;
  **d3d9_reset_raw.cast<decltype(hooks::reset) **>() = d3d9_reset_original;

  ImGui_ImplWin32_Shutdown();
  ImGui_ImplDX9_Shutdown();
}

template <typename T>
static Ptr<T> interface_base(std::wstring_view module_name,
                             std::string_view interface_name) {
  const auto module = GetModuleHandleW(module_name.data());

  using CreateInterface = void *(*)(const char *, int32_t *);
  const auto create_interface = reinterpret_cast<CreateInterface>(
      GetProcAddress(module, "CreateInterface"));

  return reinterpret_cast<T *>(
      create_interface(interface_name.data(), nullptr));
}

void App::find_interfaces() {
  interfaces.client =
      interface_base<interfaces::Client>(L"client.dll", "VClient017");
  interfaces.entity_list = interface_base<interfaces::EntityList>(
      L"client.dll", "VClientEntityList003");
  interfaces.engine =
      interface_base<interfaces::Engine>(L"engine.dll", "VEngineClient013");
  interfaces.cvar =
      interface_base<interfaces::CVar>(L"vstdlib.dll", "VEngineCvar004");
  interfaces.input_system = interface_base<interfaces::InputSystem>(
      L"inputsystem.dll", "InputSystemVersion001");
  interfaces.surface = interface_base<interfaces::Surface>(
      L"vguimatsurface.dll", "VGUI_Surface030");

  const Ptr<void *> client_vmt = *interfaces.client.cast<void **>();
  client_mode = **Ptr<void>{*client_vmt.add(10)}.byte_add(5).cast<void **>();
}

void App::find_patterns() {
  const auto d3d9_present_call_op = utils::find_pattern(
      L"GameOverlayRenderer.dll", u8"\xA1\xCC\xCC\xCC\xCC\x51\xFF\x75\x14");
  d3d9_present_raw = d3d9_present_call_op.byte_add(1);
  d3d9_present_original =
      **d3d9_present_raw.cast<decltype(d3d9_present_original) *>();

  const auto d3d9_reset_call_op = utils::find_pattern(
      L"GameOverlayRenderer.dll",
      u8"\xA1\xCC\xCC\xCC\xCC\x57\x53\xC7\x45\xFC\x00\x00\x00\x00");
  d3d9_reset_raw = d3d9_reset_call_op.byte_add(1);
  d3d9_reset_original =
      **d3d9_reset_raw.cast<decltype(d3d9_reset_original) *>();
}

void App::init_vmts() {
  vmts.client_mode.init(client_mode);
  vmts.surface.init(interfaces.surface.get());
}

void App::setup_hooks() {
  **d3d9_present_raw.cast<decltype(hooks::present) **>() = hooks::present;
  **d3d9_reset_raw.cast<decltype(hooks::reset) **>() = hooks::reset;

  vmts.client_mode.hook(LPVOID(hooks::create_move), 21);
  vmts.surface.hook(LPVOID(hooks::lock_cursor), 62);
}

void App::init_or_nothing() {
  if (static bool inited = false; !inited) {
    window = FindWindowA("Valve001", nullptr);

    find_interfaces();
    find_patterns();

    init_vmts();
    setup_hooks();

    // Hook WndProc at the end of App initialization to prevent multiple
    // initialization
    original_wnd_proc = WNDPROC(
        SetWindowLongPtrW(window, GWLP_WNDPROC, LONG_PTR(hooks::wnd_proc)));

    inited = true;
  }
}
