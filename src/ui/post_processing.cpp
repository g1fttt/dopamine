#include "post_processing.h"

#include <d3d9.h>

#include <resources/blur_x.h>
#include <resources/blur_y.h>

#include <imgui.h>

namespace ui {
  void ShaderProgram::use(float uniform) {
    device->SetPixelShader(pixel_shader.Get());

    const float params[4] = {uniform};
    device->SetPixelShaderConstantF(0, params, 1);
  }

  void ShaderProgram::init(const BYTE *pixel_shader_src) {
    if (!inited) {
      device->CreatePixelShader(PDWORD(pixel_shader_src),
                                pixel_shader.GetAddressOf());
      inited = true;
    }
  }
}

static void copy_backbuf_to_texture(IDirect3DDevice9 *device,
                                    IDirect3DTexture9 *texture,
                                    D3DTEXTUREFILTERTYPE filter) {
  ComPtr<IDirect3DSurface9> backbuf{};
  if (device->GetBackBuffer(0, 0, D3DBACKBUFFER_TYPE_MONO,
                            backbuf.GetAddressOf()) == D3D_OK) {
    ComPtr<IDirect3DSurface9> surface{};
    if (texture->GetSurfaceLevel(0, surface.GetAddressOf()) == D3D_OK) {
      device->StretchRect(backbuf.Get(), nullptr, surface.Get(), nullptr,
                          filter);
    }
  }
}

static void set_render_target(IDirect3DDevice9 *device,
                              IDirect3DTexture9 *texture) {
  ComPtr<IDirect3DSurface9> surface{};
  if (texture->GetSurfaceLevel(0, surface.GetAddressOf()) == D3D_OK) {
    device->SetRenderTarget(0, surface.Get());
  }
}

static ComPtr<IDirect3DTexture9>
create_texture(IDirect3DDevice9 *device, uint32_t width, uint32_t height) {
  ComPtr<IDirect3DTexture9> texture{};
  device->CreateTexture(width, height, 1, D3DUSAGE_RENDERTARGET,
                        D3DFMT_X8R8G8B8, D3DPOOL_DEFAULT,
                        texture.GetAddressOf(), nullptr);
  return texture;
}

namespace ui {
  constexpr auto BLUR_DOWNSAMPLE = 4.0f;

  static void begin(const ImDrawList *, const ImDrawCmd *) {
    BlurEffect::get().begin();
  }

  static void first_pass(const ImDrawList *, const ImDrawCmd *) {
    BlurEffect::get().first_pass();
  }

  static void second_pass(const ImDrawList *, const ImDrawCmd *) {
    BlurEffect::get().second_pass();
  }

  static void end(const ImDrawList *, const ImDrawCmd *) {
    BlurEffect::get().end();
  }

  void BlurEffect::draw(ImDrawList *draw_list, float alpha) {
    new_frame();

    create_textures();
    create_shaders();

    if (!blur_texture1 || !blur_texture2) {
      return;
    }

    draw_list->AddCallback(ui::begin, nullptr);
    {
      for (uint8_t i = 0; i < 8; i += 1) {
        draw_list->AddCallback(ui::first_pass, nullptr);
        draw_list->AddImage(blur_texture1.Get(), {-1.0f, -1.0f}, {1.0f, 1.0f});
        draw_list->AddCallback(ui::second_pass, nullptr);
        draw_list->AddImage(blur_texture2.Get(), {-1.0f, -1.0f}, {1.0f, 1.0f});
      }
    }
    draw_list->AddCallback(ui::end, nullptr);

    draw_list->AddCallback(ImDrawCallback_ResetRenderState, nullptr);

    draw_list->AddImage(blur_texture1.Get(), {0.0f, 0.0f},
                        {backbuf_width * 1.0f, backbuf_height * 1.0f},
                        {0.0f, 0.0f}, {1.0f, 1.0f},
                        IM_COL32(255, 255, 255, 255 * alpha));
  }

  void BlurEffect::begin() {
    device->GetRenderTarget(0, &rt_backup);

    copy_backbuf_to_texture(device, blur_texture1.Get(), D3DTEXF_LINEAR);

    // Fix blur became brightly white with D3DRS_COLORWRITEENABLE
    device->SetSamplerState(0, D3DSAMP_SRGBTEXTURE, true);

    device->SetSamplerState(0, D3DSAMP_ADDRESSU, D3DTADDRESS_CLAMP);
    device->SetSamplerState(0, D3DSAMP_ADDRESSV, D3DTADDRESS_CLAMP);

    device->SetRenderState(D3DRS_SCISSORTESTENABLE, false);

    const auto offset_x = -1.0f / (backbuf_width / BLUR_DOWNSAMPLE);
    const auto offset_y = 1.0f / (backbuf_height / BLUR_DOWNSAMPLE);

    // clang-format off
    const D3DMATRIX projection = {{
      1.0f, 0.0f, 0.0f, 0.0f,
      0.0f, 1.0f, 0.0f, 0.0f,
      0.0f, 0.0f, 1.0f, 0.0f,
      offset_x, offset_y, 0.0f, 1.0f
    }};
    // clang-format on

    device->SetTransform(D3DTS_PROJECTION, &projection);
  }

  void BlurEffect::first_pass() {
    blur_shader_x.use(1.0f / (backbuf_width / BLUR_DOWNSAMPLE));
    set_render_target(device, blur_texture2.Get());
  }

  void BlurEffect::second_pass() {
    blur_shader_y.use(1.0f / (backbuf_height / BLUR_DOWNSAMPLE));
    set_render_target(device, blur_texture1.Get());
  }

  void BlurEffect::end() {
    device->SetRenderTarget(0, rt_backup.Get());
    rt_backup->Release();

    device->SetPixelShader(nullptr);
    device->SetRenderState(D3DRS_SCISSORTESTENABLE, true);
  }

  void BlurEffect::clear_textures() {
    if (blur_texture1) {
      blur_texture1->Release();
      blur_texture1 = nullptr;
    }

    if (blur_texture2) {
      blur_texture2->Release();
      blur_texture2 = nullptr;
    }
  }

  void BlurEffect::new_frame() {
    const auto [width, height] = ImGui::GetIO().DisplaySize;

    if (backbuf_width != width || backbuf_height != height) {
      clear_textures();

      backbuf_width = width;
      backbuf_height = height;
    }
  }

  void BlurEffect::create_shaders() {
    blur_shader_x.init(BLUR_X);
    blur_shader_y.init(BLUR_Y);
  }

  void BlurEffect::create_textures() {
    if (!blur_texture1) {
      blur_texture1 = create_texture(device, backbuf_width / BLUR_DOWNSAMPLE,
                                     backbuf_height / BLUR_DOWNSAMPLE);
    }

    if (!blur_texture2) {
      blur_texture2 = create_texture(device, backbuf_width / BLUR_DOWNSAMPLE,
                                     backbuf_height / BLUR_DOWNSAMPLE);
    }
  }
}
