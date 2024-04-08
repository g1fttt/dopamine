#pragma once

#include <Windows.h>

#include <cstdint>

struct IDirect3DDevice9;
struct _D3DPRESENT_PARAMETERS;

namespace game {
  struct UserCommand;

  struct ViewSetup;
}

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
