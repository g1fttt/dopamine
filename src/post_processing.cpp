#include "post_processing.h"

#include <d3d9.h>

#include "resources/blur_x.h"
#include "resources/blur_y.h"

#include <imgui.h>

namespace post_processing {
  void ShaderProgram::use(float uniform) {
    device->SetPixelShader(pixel_shader);

    const float params[4] = {uniform};
    device->SetPixelShaderConstantF(0, params, 1);
  }

  void ShaderProgram::init(const BYTE *pixel_shader_src) {
    if (!inited) {
      device->CreatePixelShader(PDWORD(BLUR_X), &pixel_shader);
      inited = true;
    }
  }
}

static void set_render_target_texture(IDirect3DDevice9 *device,
                                      IDirect3DTexture9 *texture,
                                      D3DTEXTUREFILTERTYPE filter) {
  ComPtr<IDirect3DSurface9> backbuf{};
  if (device->GetBackBuffer(0, 0, D3DBACKBUFFER_TYPE_MONO,
                            backbuf.GetAddressOf()) == D3D_OK) {
    ComPtr<IDirect3DSurface9> surface{};
    if (texture->GetSurfaceLevel(0, surface.GetAddressOf()) == D3D_OK) {
      device->StretchRect(backbuf.Get(), nullptr, surface.Get(), nullptr,
                          filter);
      device->SetRenderTarget(0, surface.Get());
    }
  }
}

namespace post_processing {
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

  BlurEffect &BlurEffect::get() {
    static BlurEffect self{};
    return self;
  }

  void BlurEffect::draw(ImDrawList *draw_list, float alpha) {
    draw_list->AddCallback(post_processing::begin, nullptr);
    {
      for (int i = 0; i < 8; i += 1) {
        draw_list->AddCallback(post_processing::first_pass, nullptr);
        draw_list->AddImage(blur_texture.Get(), {0.0f, 0.0f},
                            {backbuf_width * 1.0f, backbuf_height * 1.0f});
        draw_list->AddCallback(post_processing::second_pass, nullptr);
        draw_list->AddImage(blur_texture.Get(), {0.0f, 0.0f},
                            {backbuf_width * 1.0f, backbuf_height * 1.0f});
      }
    }
    draw_list->AddCallback(post_processing::end, nullptr);

    draw_list->AddImage(blur_texture.Get(), {0.0f, 0.0f},
                        {backbuf_width * 1.0f, backbuf_height * 1.0f},
                        {0.0f, 0.0f}, {1.0f, 1.0f},
                        IM_COL32(255, 255, 255, 255 * alpha));
  }

  void BlurEffect::begin() {
    create_shaders();

    new_frame();

    device->GetRenderTarget(0, &rt_backup);

    set_render_target_texture(device.Get(), blur_texture.Get(), D3DTEXF_NONE);

    device->SetSamplerState(0, D3DSAMP_ADDRESSU, D3DTADDRESS_CLAMP);
    device->SetSamplerState(0, D3DSAMP_ADDRESSV, D3DTADDRESS_CLAMP);
  }

  void BlurEffect::first_pass() {
    blur_shader_x.use(1.0f / backbuf_width);
  }

  void BlurEffect::second_pass() {
    blur_shader_y.use(1.0f / backbuf_height);
  }

  void BlurEffect::end() {
    device->SetRenderTarget(0, rt_backup.Get());
    rt_backup->Release();

    device->SetPixelShader(nullptr);
  }

  void BlurEffect::set_device(IDirect3DDevice9 *device) {
    this->device = device;
    blur_shader_x.set_device(device);
    blur_shader_y.set_device(device);
  }

  void BlurEffect::clear_textures() {
    if (blur_texture) {
      blur_texture->Release();
      blur_texture = nullptr;
    }
  }

  void BlurEffect::new_frame() {
    const auto [width, height] = ImGui::GetIO().DisplaySize;

    if (backbuf_width != width || backbuf_height != height) {
      clear_textures();

      backbuf_width = width;
      backbuf_height = height;

      create_textures();
    }
  }

  void BlurEffect::create_shaders() {
    blur_shader_x.init(BLUR_X);
    blur_shader_y.init(BLUR_Y);
  }

  void BlurEffect::create_textures() {
    if (!blur_texture) {
      device->CreateTexture(backbuf_width, backbuf_height, 1,
                            D3DUSAGE_RENDERTARGET, D3DFMT_A8R8G8B8,
                            D3DPOOL_DEFAULT, &blur_texture, nullptr);
    }
  }
}
