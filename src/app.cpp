#include "app.h"

#include "hacks/visuals.h"
#include "hooks/hooks.h"
#include "utils/netvars.h"
#include "utils/utils.h"

#include "game/engine.h"
#include "game/input_system.h"
#include "game/surface.h"

#include "config.h"

#include <imgui.h>
#include <imgui_impl_dx9.h>
#include <imgui_impl_win32.h>

App::~App() = default;

void App::reset() {
  interfaces.input_system->enable_input(true);

  hooks->remove(*this);

  ImGui_ImplWin32_Shutdown();
  ImGui_ImplDX9_Shutdown();

  FreeLibraryAndExitThread(module, 0);
}

bool App::should_anti_screenshot() const {
  const auto anti_screenshot = hacks::Visuals::get().config.anti_screenshot;
  return anti_screenshot && interfaces.engine->is_taking_screenshot();
}

bool App::should_draw_visuals() const {
  return interfaces.engine->is_in_game() &&
         !interfaces.surface->is_cursor_visible();
}

template <typename T>
static T *interface_base(std::wstring_view module_name,
                         std::string_view interface_name) {
  const auto module = GetModuleHandleW(module_name.data());

  using CreateInterface = void *(*)(const char *, int32_t *);
  const auto create_interface = reinterpret_cast<CreateInterface>(
      GetProcAddress(module, "CreateInterface"));

  return reinterpret_cast<T *>(
      create_interface(interface_name.data(), nullptr));
}

App::App() {
  hooks = std::make_unique<Hooks>();
}

void App::init_or_nothing(HMODULE module) {
  this->module = module;
  DisableThreadLibraryCalls(module);

  config::init_or_nothing();

  std::atexit([] {
    config::save();
  });

  window = FindWindowA("Valve001", nullptr);

  find_interfaces();
  find_patterns();

  hooks->setup(*this);

  utils::Netvars::get_or_init(utils::Netvars::init_func(*this));
}

void App::find_interfaces() {
  interfaces.client = interface_base<game::Client>(L"client.dll", "VClient017");
  interfaces.entity_list =
      interface_base<game::EntityList>(L"client.dll", "VClientEntityList003");
  interfaces.engine =
      interface_base<game::Engine>(L"engine.dll", "VEngineClient013");
  interfaces.cvar =
      interface_base<game::CVar>(L"vstdlib.dll", "VEngineCvar004");
  interfaces.input_system = interface_base<game::InputSystem>(
      L"inputsystem.dll", "InputSystemVersion001");
  interfaces.surface =
      interface_base<game::Surface>(L"vguimatsurface.dll", "VGUI_Surface030");
  interfaces.render_view =
      interface_base<game::RenderView>(L"engine.dll", "VEngineRenderView014");
  interfaces.material_system = interface_base<game::MaterialSystem>(
      L"MaterialSystem.dll", "VMaterialSystem080");
  interfaces.model_render =
      interface_base<game::ModelRender>(L"engine.dll", "VEngineModel016");

  const utils::Ptr<void *> client_vmt = *interfaces.client.cast<void **>();
  interfaces.client_mode =
      **utils::Ptr<void>{*client_vmt.add(10)}.byte_add(5).cast<void **>();
}

void App::find_patterns() {
  const auto d3d9_present_call_op = utils::find_pattern(
      L"GameOverlayRenderer.dll", u8"\xA1\xCC\xCC\xCC\xCC\x51\xFF\x75\x14");
  d3d9_present_raw = d3d9_present_call_op.byte_add(1);

  const auto d3d9_reset_call_op = utils::find_pattern(
      L"GameOverlayRenderer.dll",
      u8"\xA1\xCC\xCC\xCC\xCC\x57\x53\xC7\x45\xFC\x00\x00\x00\x00");
  d3d9_reset_raw = d3d9_reset_call_op.byte_add(1);
}
