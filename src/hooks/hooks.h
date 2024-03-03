#pragma once

#include <Windows.h>

#include <d3d9types.h>

#include <cstdint>

struct IDirect3DDevice9;

namespace hooks {
  LRESULT WINAPI wnd_proc(HWND window, UINT message, WPARAM wparam,
                          LPARAM lparam);

  void STDCALL frame_stage_notify(int32_t stage);

  HRESULT STDCALL reset(IDirect3DDevice9 *device,
                        D3DPRESENT_PARAMETERS *params);
  HRESULT STDCALL present(IDirect3DDevice9 *device, const RECT *src,
                          const RECT *dest, HWND window_override,
                          const RGNDATA *dirty_region);

  void STDCALL lock_cursor();
}
