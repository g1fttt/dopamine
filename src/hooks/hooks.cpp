#include "hooks.h"

#include <utils/patterns.h>

#include <app.h>

namespace hooks {
  LRESULT WINAPI wnd_proc(HWND window, UINT message, WPARAM wparam,
                          LPARAM lparam);

  HRESULT WINAPI reset(IDirect3DDevice9 *device,
                       _D3DPRESENT_PARAMETERS *params);
  HRESULT WINAPI present(IDirect3DDevice9 *device, const RECT *src,
                         const RECT *dest, HWND window_override,
                         const RGNDATA *dirty_region);

  bool STDCALL create_move(float input_sample_frame_time,
                           game::UserCommand *cmd);
  void STDCALL override_view(game::ViewSetup *view);
  bool STDCALL do_post_screen_space_effects(const game::ViewSetup *view);

  void STDCALL level_init_post_entity();
  void STDCALL level_shutdown();

  float STDCALL get_screen_aspect_ratio();

  void STDCALL lock_cursor();
  bool STDCALL is_cursor_visible();
}

void Hooks::setup(App *app) {
  const auto client_mode = app->interfaces.client_mode;
  override_view.init_and_hook<16>(client_mode, hooks::override_view);
  create_move.init_and_hook<21>(client_mode, hooks::create_move);
  do_post_screen_space_effects.init_and_hook<39>(
      client_mode, hooks::do_post_screen_space_effects);

  const auto client = app->interfaces.client.get();
  level_init_post_entity.init_and_hook<6>(client,
                                          hooks::level_init_post_entity);
  level_shutdown.init_and_hook<7>(client, hooks::level_shutdown);

  const auto engine = app->interfaces.engine;
  get_screen_aspect_ratio.init_and_hook<95>(engine,
                                            hooks::get_screen_aspect_ratio);

  const auto surface = app->interfaces.surface;
  is_cursor_visible.init_and_hook<53>(surface, hooks::is_cursor_visible);
  lock_cursor.init_and_hook<62>(surface, hooks::lock_cursor);

  wnd_proc_original = WNDPROC(
      SetWindowLongPtrW(app->window, GWLP_WNDPROC, LONG_PTR(hooks::wnd_proc)));

  d3d9_present_original =
      **utils::patterns->d3d9_present.cast<decltype(d3d9_present_original) *>();
  d3d9_reset_original =
      **utils::patterns->d3d9_reset.cast<decltype(d3d9_reset_original)>();

  **utils::patterns->d3d9_present.cast<decltype(hooks::present) **>() =
      hooks::present;
  **utils::patterns->d3d9_reset.cast<decltype(hooks::reset) **>() =
      hooks::reset;
}

void Hooks::remove(App *app) {
  **utils::patterns->d3d9_present.cast<decltype(hooks::present) **>() =
      d3d9_present_original;
  **utils::patterns->d3d9_reset.cast<decltype(hooks::reset) **>() =
      d3d9_reset_original;

  SetWindowLongPtrW(app->window, GWLP_WNDPROC, LONG_PTR(wnd_proc_original));

  create_move.unhook();
  override_view.unhook();
  do_post_screen_space_effects.unhook();

  level_init_post_entity.unhook();
  level_shutdown.unhook();

  get_screen_aspect_ratio.unhook();

  is_cursor_visible.unhook();
  lock_cursor.unhook();
}
