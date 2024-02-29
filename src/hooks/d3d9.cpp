#include "hooks.h"

#include <d3d9.h>

#include <app.h>
#include <menu.h>
#include <post_processing.h>

#include <interfaces/input_system.h>

#include <imgui.h>
#include <imgui_impl_dx9.h>
#include <imgui_impl_win32.h>

HRESULT STDCALL hooks::reset(void *device, D3DPRESENT_PARAMETERS *params) {
  post_processing::BlurEffect::get().clear_textures();

  ImGui_ImplDX9_InvalidateDeviceObjects();

  const auto result =
      App::get().d3d9_vmt.call_original<HRESULT, 16>(device, params);

  ImGui_ImplDX9_CreateDeviceObjects();

  return result;
}

HRESULT STDCALL hooks::present(IDirect3DDevice9 *device, const RECT *src,
                               const RECT *dest, HWND window_override,
                               const RGNDATA *dirty_region) {
  DWORD srgb = 0;
  device->GetRenderState(D3DRS_SRGBWRITEENABLE, &srgb);

  // Source engine color correction
  device->SetRenderState(D3DRS_SRGBWRITEENABLE, false);
  {
    ImGui_ImplDX9_NewFrame();
    ImGui_ImplWin32_NewFrame();
    ImGui::NewFrame();

    auto &blur_effect = post_processing::BlurEffect::get();

    blur_effect.set_device(device);
    blur_effect.new_frame();

    App::with([&](App &app) {
      app.menu.render();

      if (!app.menu.is_fully_closed()) {
        blur_effect.draw(ImGui::GetBackgroundDrawList(),
                         app.menu.get_transparency());
      }
    });

    ImGui::EndFrame();
    ImGui::Render();

    if (device->BeginScene() == D3D_OK) {
      ImGui_ImplDX9_RenderDrawData(ImGui::GetDrawData());
      device->EndScene();
    }
  }
  device->SetRenderState(D3DRS_SRGBWRITEENABLE, srgb);

  return App::get().d3d9_vmt.call_original<HRESULT, 17>(
      device, src, dest, window_override, dirty_region);
}
