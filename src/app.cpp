#include "app.h"

#include "hooks/hooks.h"
#include "utils/utils.h"

#include <imgui.h>
#include <imgui_impl_dx9.h>
#include <imgui_impl_win32.h>

// TODO: ISurface LockCursor & UnlockCursor

static void find_interfaces(App &app) {
  app.client = utils::interface_base("client.dll", "VClient017");
  app.cvar = reinterpret_cast<interfaces::CVar *>(
      utils::interface_base("vstdlib.dll", "VEngineCvar004"));
  app.input_system = reinterpret_cast<interfaces::InputSystem *>(
      utils::interface_base("inputsystem.dll", "InputSystemVersion001"));
}

static void find_patterns(App &app) {
  const auto d3d9 = utils::find_pattern(
      "shaderapidx9.dll",
      u8"\xA1\xCC\xCC\xCC\xCC\x50\x8B\x08\xFF\x51\xCC\x8B\xF8");
  if (d3d9.has_value()) {
    app.d3d9 = **reinterpret_cast<IDirect3DDevice9 ***>(d3d9.value() + 1);
  } else {
    // TODO: Log error to file
  }
}

static void init_imgui_context(App &app, ImGuiContext **ctx) {
  *ctx = ImGui::CreateContext();
  ImGui::SetCurrentContext(*ctx);

  ImGui_ImplDX9_Init(app.d3d9);
  ImGui_ImplWin32_Init(app.window);
}

static void init_imgui_style() {
  ImGui::StyleColorsDark();

  auto &style = ImGui::GetStyle();
  style.ScrollbarSize = 9.0f;

  auto &io = ImGui::GetIO();
  io.IniFilename = nullptr;
  io.LogFilename = nullptr;
  io.ConfigFlags |= ImGuiConfigFlags_NoMouseCursorChange;
  io.Fonts->AddFontDefault();
}

static void init_imgui(App &app) {
  init_imgui_context(app, &app.menu_ctx);
  init_imgui_style();

  init_imgui_context(app, &app.blur_ctx);
  init_imgui_style();
}

static void init_vmts(App &app) {
  app.client_vmt.init(app.client);
  app.d3d9_vmt.init(app.d3d9);
}

static void setup_hooks(App &app) {
  app.client_vmt.hook(LPVOID(hooks::frame_stage_notify), 36);

  app.d3d9_vmt.hook(LPVOID(hooks::reset), 16);
  app.d3d9_vmt.hook(LPVOID(hooks::present), 17);
}

App &App::get() {
  static App APP{};

  if (static bool inited = false; !inited) {
    APP.window = FindWindowA(nullptr, "Counter-Strike: Source Offensive");

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

static void destroy_imgui_context(ImGuiContext *ctx) {
  ImGui_ImplWin32_Shutdown();
  ImGui_ImplDX9_Shutdown();

  ImGui::DestroyContext(ctx);
}

void App::reset() {
  client_vmt.reset();
  d3d9_vmt.reset();

  destroy_imgui_context(blur_ctx);
  destroy_imgui_context(menu_ctx);

  SetWindowLongPtrW(window, GWLP_WNDPROC, LONG_PTR(original_wnd_proc));
}
