#pragma once

#include <Windows.h>

struct IDirect3DDevice9;
struct _D3DPRESENT_PARAMETERS;

namespace hooks {
  LRESULT WINAPI wnd_proc(HWND window, UINT message, WPARAM wparam,
                          LPARAM lparam);

  HRESULT STDCALL reset(IDirect3DDevice9 *device,
                        _D3DPRESENT_PARAMETERS *params);
  HRESULT STDCALL present(IDirect3DDevice9 *device, const RECT *src,
                          const RECT *dest, HWND window_override,
                          const RGNDATA *dirty_region);

  void STDCALL lock_cursor();
}
