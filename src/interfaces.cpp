#include "interfaces.h"

#include <Windows.h>

#include <string_view>

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

namespace core {
  Interfaces::Interfaces() {
    client = interface_base<game::Client>(L"client.dll", "VClient017");
    entity_list =
        interface_base<game::EntityList>(L"client.dll", "VClientEntityList003");
    engine = interface_base<game::Engine>(L"engine.dll", "VEngineClient013");
    cvar = interface_base<game::CVar>(L"vstdlib.dll", "VEngineCvar004");
    input_system = interface_base<game::InputSystem>(L"inputsystem.dll",
                                                     "InputSystemVersion001");
    surface =
        interface_base<game::Surface>(L"vguimatsurface.dll", "VGUI_Surface030");
    render_view =
        interface_base<game::RenderView>(L"engine.dll", "VEngineRenderView014");
    material_system = interface_base<game::MaterialSystem>(
        L"MaterialSystem.dll", "VMaterialSystem080");
    model_render =
        interface_base<game::ModelRender>(L"engine.dll", "VEngineModel016");

    const utils::Ptr<void *> client_vmt = *client.cast<void **>();
    client_mode =
        **utils::Ptr<void>{*client_vmt.add(10)}.byte_add(5).cast<void **>();
  }
}
