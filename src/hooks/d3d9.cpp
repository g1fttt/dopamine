#include "hooks.h"

#include <d3d9.h>

#include <app.h>
#include <menu.h>

#include <interfaces/input_system.h>

#include <imgui.h>
#include <imgui_impl_dx9.h>
#include <imgui_impl_win32.h>

HRESULT STDCALL hooks::reset(void *device, D3DPRESENT_PARAMETERS *params) {
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

    menu::render();

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
