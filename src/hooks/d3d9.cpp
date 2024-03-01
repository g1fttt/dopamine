#include "hooks.h"

#include <d3d9.h>

#include <wrl/client.h>

#include <app.h>
#include <menu.h>
#include <post_processing.h>

#include <interfaces/input_system.h>

#include <imgui.h>
#include <imgui_impl_dx9.h>
#include <imgui_impl_win32.h>

using namespace Microsoft::WRL;

HRESULT STDCALL hooks::reset(void *device, D3DPRESENT_PARAMETERS *params) {
  post_processing::BlurEffect::get().clear_textures();

  ImGui_ImplDX9_InvalidateDeviceObjects();

  const auto result =
      App::get().d3d9_vmt.call_original<HRESULT, 16>(device, params);

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
  App::with_mut([&](App &app) {
    app.menu.update_animation();

    ComPtr<IDirect3DStateBlock9> state_block{};
    if (device->CreateStateBlock(D3DSBT_ALL, state_block.GetAddressOf()) !=
        D3D_OK) {
      return;
    }

    state_block->Capture();

    // Fix menu (and blur) not rendering without `net_graph` or `cl_showfps`
    device->SetRenderState(D3DRS_COLORWRITEENABLE, 0xFFFFFFFF);

    if (!app.menu.is_fully_closed()) {
      ImGui::SetCurrentContext(app.blur_ctx);

      create_imgui_frame();
      {
        auto &blur_effect = post_processing::BlurEffect::get();
        blur_effect.set_device(device);
        blur_effect.draw(ImGui::GetBackgroundDrawList(),
                         app.menu.get_transparency());
      }
      draw_imgui_frame(device);
    }

    ImGui::SetCurrentContext(app.menu_ctx);

    create_imgui_frame();
    {
      // Fix broken ImGui menu colors with Source engine gamma correction
      device->SetRenderState(D3DRS_SRGBWRITEENABLE, false);

      app.menu.render();
    }
    draw_imgui_frame(device);

    state_block->Apply();
  });
  return App::get().d3d9_vmt.call_original<HRESULT, 17>(
      device, src, dest, window_override, dirty_region);
}
