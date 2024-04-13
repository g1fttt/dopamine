#pragma once

#include <utils/vmt.h>

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

  utils::VMTHook<bool, float, game::UserCommand *> create_move;
  utils::VMTHook<void, game::ViewSetup *> override_view;
  utils::VMTHook<bool, const game::ViewSetup *> do_post_screen_space_effects;
  utils::VMTHook<void> level_init_post_entity;
  utils::VMTHook<void> level_shutdown;
  utils::VMTHook<float> get_screen_aspect_ratio;
  utils::VMTHook<bool> is_cursor_visible;
  utils::VMTHook<void> lock_cursor;

  std::add_pointer_t<D3D9_Present> d3d9_present_original = nullptr;
  std::add_pointer_t<D3D9_Reset> d3d9_reset_original = nullptr;

  WNDPROC wnd_proc_original = nullptr;
};
