#include "hooks.h"

#include <d3d9.h>

#include <wrl/client.h>

#include <app.h>

#include <interfaces/input_system.h>

#include <ui/menu.h>
#include <ui/post_processing.h>

#include <imgui.h>
#include <imgui_impl_dx9.h>
#include <imgui_impl_win32.h>

using namespace Microsoft::WRL;

HRESULT STDCALL hooks::reset(IDirect3DDevice9 *device,
                             D3DPRESENT_PARAMETERS *params) {
  ui::BlurEffect::get().clear_textures();

  ImGui_ImplDX9_InvalidateDeviceObjects();

  const auto result =
      App::get().vmts.d3d9.call_original<HRESULT, 16>(device, params);

  ImGui_ImplDX9_CreateDeviceObjects();

  return result;
}

static void create_imgui_frame() {
  ImGui_ImplDX9_NewFrame();
  ImGui_ImplWin32_NewFrame();

  ImGui::NewFrame();
}

static void draw_imgui_frame(IDirect3DDevice9 *device) {
  ImGui::EndFrame();

  ImGui::Render();

  if (device->BeginScene() == D3D_OK) {
    ImGui_ImplDX9_RenderDrawData(ImGui::GetDrawData());
    device->EndScene();
  }
}

HRESULT STDCALL hooks::present(IDirect3DDevice9 *device, const RECT *src,
                               const RECT *dest, HWND window_override,
                               const RGNDATA *dirty_region) {
  auto &menu = ui::Menu::get();
  menu.update_animation();

  auto &app = App::get();

  ComPtr<IDirect3DStateBlock9> state_block{};
  if (device->CreateStateBlock(D3DSBT_ALL, state_block.GetAddressOf()) !=
      D3D_OK) {
    goto end;
  }

  state_block->Capture();

  // Fix menu (and blur) not rendering without `net_graph` or `cl_showfps`
  device->SetRenderState(D3DRS_COLORWRITEENABLE, 0xFFFFFFFF);

  if (!menu.is_fully_closed()) {
    auto &blur_effect = ui::BlurEffect::get();
    blur_effect.make_context_current();

    create_imgui_frame();
    {
      blur_effect.set_device(device);
      blur_effect.draw(ImGui::GetBackgroundDrawList(), menu.get_transparency());
    }
    draw_imgui_frame(device);
  }

  menu.make_context_current();

  create_imgui_frame();
  {
    // Fix broken ImGui menu colors with Source engine gamma correction
    device->SetRenderState(D3DRS_SRGBWRITEENABLE, false);

    menu.render();
  }
  draw_imgui_frame(device);

  state_block->Apply();

  if (app.should_unhook) {
    ShowCursor(true);
    app.must_unhook = true;
  }
end:
  return app.vmts.d3d9.call_original<HRESULT, 17>(
      device, src, dest, window_override, dirty_region);
}
