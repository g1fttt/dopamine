#include "hooks.h"

#include <d3d9.h>

#include <ui/menu.h>
#include <ui/post_processing.h>

#include <interfaces/engine.h>
#include <interfaces/surface.h>

#include <hacks/visuals.h>

#include <app.h>

#include <imgui.h>
#include <imgui_impl_dx9.h>
#include <imgui_impl_win32.h>

using ui::BlurEffect;

namespace hooks {
  HRESULT WINAPI reset(IDirect3DDevice9 *device,
                       _D3DPRESENT_PARAMETERS *params) {
    BlurEffect::get().clear_textures();

    ImGui_ImplDX9_InvalidateDeviceObjects();

    const auto result = App::get().hooks->d3d9_reset_original(device, params);

    ImGui_ImplDX9_CreateDeviceObjects();

    return result;
  }
}

static ImGuiContext *create_imgui_context(IDirect3DDevice9 *device) {
  auto *ctx = ImGui::CreateContext();
  ImGui::SetCurrentContext(ctx);

  ImGui_ImplDX9_Init(device);
  ImGui_ImplWin32_Init(App::get().window);

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

static bool init_imgui(IDirect3DDevice9 *device) {
  App::get().and_then<void>([=](App &app) {
    app.fore_imgui_ctx.set(create_imgui_context(device));
    app.back_imgui_ctx.set(create_imgui_context(device));
  });
  return true;
}

static void draw_frame(IDirect3DDevice9 *device, const ui::ImGuiContext &ctx,
                       const std::function<void()> &cb) {
  ctx.push();

  ImGui_ImplDX9_NewFrame();
  ImGui_ImplWin32_NewFrame();

  ImGui::NewFrame();
  { cb(); }
  ImGui::EndFrame();

  ImGui::Render();

  if (device->BeginScene() == D3D_OK) {
    ImGui_ImplDX9_RenderDrawData(ImGui::GetDrawData());
    device->EndScene();
  }
}

namespace hooks {
  HRESULT WINAPI present(IDirect3DDevice9 *device, const RECT *src,
                         const RECT *dest, HWND window_override,
                         const RGNDATA *dirty_region) {
    static auto _ = init_imgui(device);

    auto &menu = ui::Menu::get();
    menu.update_animation();

    ComPtr<IDirect3DStateBlock9> state_block{};
    if (device->CreateStateBlock(D3DSBT_ALL, state_block.GetAddressOf()) !=
        D3D_OK) {
      goto end;
    }

    state_block->Capture();

    // Fix menu (and blur) not rendering without `net_graph` or `cl_showfps`
    device->SetRenderState(D3DRS_COLORWRITEENABLE, 0xFFFFFFFF);

    App::get().and_then<void>([&](const App &app) {
      draw_frame(device, app.back_imgui_ctx, [&] {
        auto *draw_list = ImGui::GetBackgroundDrawList();

        if (auto &blur_effect = BlurEffect::get(); !menu.is_fully_closed()) {
          blur_effect.set_device(device);
          blur_effect.draw(draw_list, menu.get_transparency());
        }

        if (const auto &visuals = hacks::Visuals::get();
            app.should_draw_visuals()) {
          visuals.draw_sniper_crosshair(draw_list);
        }
      });

      draw_frame(device, app.fore_imgui_ctx, [&] {
        // Fix broken ImGui menu colors with Source engine gamma correction
        device->SetRenderState(D3DRS_SRGBWRITEENABLE, false);

        menu.draw();
      });
    });
    state_block->Apply();
  end:
    return App::get().and_then<HRESULT>([=](App &app) {
      if (app.should_unhook) {
        ShowCursor(true);
        app.must_unhook = true;
      }
      return app.hooks->d3d9_present_original(device, src, dest,
                                              window_override, dirty_region);
    });
  }
}
