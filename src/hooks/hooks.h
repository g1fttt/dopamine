#pragma once

#include <utils/vmt.h>

PRIVATE_USE(utils::VMTHook)

namespace game {
  struct UserCommand;
  struct ViewSetup;
}

struct IDirect3DDevice9;
struct _D3DPRESENT_PARAMETERS;

struct App;

struct Hooks {
  using D3D9_Present = HRESULT WINAPI(IDirect3DDevice9 *, const RECT *,
                                      const RECT *, HWND, const RGNDATA *);
  using D3D9_Reset = HRESULT WINAPI(IDirect3DDevice9 *,
                                    _D3DPRESENT_PARAMETERS *);

  void setup(App &app);
  void remove(App &app);

  VMTHook<bool, float, game::UserCommand *> create_move;
  VMTHook<void, game::ViewSetup *> override_view;
  VMTHook<void> level_init_post_entity;
  VMTHook<void> level_shutdown;
  VMTHook<float> get_screen_aspect_ratio;
  VMTHook<bool> is_cursor_visible;
  VMTHook<void> lock_cursor;

  std::add_pointer_t<D3D9_Present> d3d9_present_original = nullptr;
  std::add_pointer_t<D3D9_Reset> d3d9_reset_original = nullptr;

  WNDPROC wnd_proc_original = nullptr;
};
