#include "hooks.h"

#include <app.h>

namespace hooks {
  LRESULT WINAPI wnd_proc(HWND window, UINT message, WPARAM wparam,
                          LPARAM lparam);

  HRESULT WINAPI reset(IDirect3DDevice9 *device,
                       _D3DPRESENT_PARAMETERS *params);
  HRESULT WINAPI present(IDirect3DDevice9 *device, const RECT *src,
                         const RECT *dest, HWND window_override,
                         const RGNDATA *dirty_region);

  float STDCALL get_screen_aspect_ratio();

  bool STDCALL create_move(float input_sample_frame_time,
                           game::UserCommand *cmd);

  bool STDCALL is_cursor_visible();
  void STDCALL lock_cursor();

  void STDCALL override_view(game::ViewSetup *view);

  void STDCALL frame_stage_notify(int32_t stage);
}

void Hooks::setup(App &app) {
  const auto client_mode = app.interfaces.client_mode;
  override_view.init_and_hook<16>(client_mode, hooks::override_view);
  create_move.init_and_hook<21>(client_mode, hooks::create_move);

  const auto client = app.interfaces.client.get();
  frame_stage_notify.init_and_hook<35>(client, hooks::frame_stage_notify);

  const auto engine = app.interfaces.engine.get();
  get_screen_aspect_ratio.init_and_hook<95>(engine,
                                            hooks::get_screen_aspect_ratio);

  const auto surface = app.interfaces.surface.get();
  is_cursor_visible.init_and_hook<53>(surface, hooks::is_cursor_visible);
  lock_cursor.init_and_hook<62>(surface, hooks::lock_cursor);

  wnd_proc_original = WNDPROC(
      SetWindowLongPtrW(app.window, GWLP_WNDPROC, LONG_PTR(hooks::wnd_proc)));

  d3d9_present_original =
      **app.d3d9_present_raw.cast<decltype(d3d9_present_original) *>();
  d3d9_reset_original =
      **app.d3d9_reset_raw.cast<decltype(d3d9_reset_original) *>();

  **app.d3d9_present_raw.cast<decltype(hooks::present) **>() = hooks::present;
  **app.d3d9_reset_raw.cast<decltype(hooks::reset) **>() = hooks::reset;
}

void Hooks::remove(App &app) {
  **app.d3d9_present_raw.cast<decltype(hooks::present) **>() =
      d3d9_present_original;
  **app.d3d9_reset_raw.cast<decltype(hooks::reset) **>() = d3d9_reset_original;

  SetWindowLongPtrW(app.window, GWLP_WNDPROC, LONG_PTR(wnd_proc_original));

  create_move.unhook();
  override_view.unhook();
  frame_stage_notify.unhook();
  get_screen_aspect_ratio.unhook();
  is_cursor_visible.unhook();
  lock_cursor.unhook();
}
