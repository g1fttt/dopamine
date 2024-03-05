#include "app.h"

#include "hooks/hooks.h"
#include "interfaces/input_system.h"
#include "utils/utils.h"

#include "ui/menu.h"
#include "ui/post_processing.h"

#include <imgui.h>
#include <imgui_impl_dx9.h>
#include <imgui_impl_win32.h>

static void find_interfaces(App &app) {
  app.interfaces.cvar = reinterpret_cast<interfaces::CVar *>(
      utils::interface_base("vstdlib.dll", "VEngineCvar004"));
  app.interfaces.input_system = reinterpret_cast<interfaces::InputSystem *>(
      utils::interface_base("inputsystem.dll", "InputSystemVersion001"));
  app.interfaces.surface = reinterpret_cast<interfaces::Surface *>(
      utils::interface_base("vguimatsurface.dll", "VGUI_Surface030"));
}

static void find_patterns(App &app) {
  const auto d3d9 = utils::find_pattern(
      "shaderapidx9.dll",
      u8"\xA1\xCC\xCC\xCC\xCC\x50\x8B\x08\xFF\x51\xCC\x8B\xF8");
  if (d3d9.has_value()) {
    app.interfaces.d3d9 =
        **reinterpret_cast<IDirect3DDevice9 ***>(d3d9.value() + 1);
  } else {
    // TODO: Log error to file
  }
}

static ImGuiContext *create_imgui_context(App &app) {
  auto *ctx = ImGui::CreateContext();
  ImGui::SetCurrentContext(ctx);

  ImGui_ImplDX9_Init(app.interfaces.d3d9);
  ImGui_ImplWin32_Init(app.window);

  ImGui::StyleColorsDark();

  auto &style = ImGui::GetStyle();
  style.ScrollbarSize = 9.0f;

  auto &io = ImGui::GetIO();
  io.IniFilename = nullptr;
  io.LogFilename = nullptr;
  io.ConfigFlags |= ImGuiConfigFlags_NoMouseCursorChange;
  io.Fonts->AddFontDefault();

  return ctx;
}

static void init_imgui(App &app) {
  auto *menu_ctx = create_imgui_context(app);
  ui::Menu::get().set_context(menu_ctx);

  auto *blur_ctx = create_imgui_context(app);
  ui::BlurEffect::get().set_context(blur_ctx);
}

static void init_vmts(App &app) {
  app.vmts.d3d9.init(app.interfaces.d3d9);
  app.vmts.surface.init(app.interfaces.surface);
}

static void setup_hooks(App &app) {
  app.vmts.d3d9.hook(LPVOID(hooks::reset), 16);
  app.vmts.d3d9.hook(LPVOID(hooks::present), 17);

  app.vmts.surface.hook(LPVOID(hooks::lock_cursor), 62);
}

App &App::get() {
  static App APP{};

  if (static bool inited = false; !inited) {
    APP.window = FindWindowA("Valve001", nullptr);

    find_interfaces(APP);
    find_patterns(APP);

    init_imgui(APP);
    init_vmts(APP);

    setup_hooks(APP);

    APP.original_wnd_proc = WNDPROC(
        SetWindowLongPtrW(APP.window, GWLP_WNDPROC, LONG_PTR(hooks::wnd_proc)));

    inited = true;
  }
  return APP;
}

void App::with(const std::function<void(App &)> &cb) {
  cb(App::get());
}

void App::reset() {
  SetWindowLongPtrW(window, GWLP_WNDPROC, LONG_PTR(original_wnd_proc));

  interfaces.input_system->enable_input(true);

  vmts.d3d9.reset();
  vmts.surface.reset();

  ImGui_ImplWin32_Shutdown();
  ImGui_ImplDX9_Shutdown();
}
