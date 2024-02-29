#include "post_processing.h"

#include <d3d9.h>

#include <imgui.h>

namespace post_processing {
  void ShaderProgram::use(float uniform, int32_t location) {
    device->SetPixelShader(pixel_shader);

    const float params[4] = {uniform};
    device->SetPixelShaderConstantF(location, params, 1);
  }

  void ShaderProgram::init(const char *pixel_shader_src) {
    if (inited) {
      return;
    }
    device->CreatePixelShader(PDWORD(pixel_shader_src), &pixel_shader);
    inited = true;
  }

  void ShaderProgram::set_device(IDirect3DDevice9 *device) {
    this->device = device;
  }
}

static IDirect3DTexture9 *create_texture(IDirect3DDevice9 *device,
                                         uint32_t width, uint32_t height) {
  IDirect3DTexture9 *texture;
  device->CreateTexture(width, height, 1, D3DUSAGE_RENDERTARGET,
                        D3DFMT_X8R8G8B8, D3DPOOL_DEFAULT, &texture, nullptr);
  return texture;
}

static void copy_backbuf_to_texture(IDirect3DDevice9 *device,
                                    IDirect3DTexture9 *texture,
                                    D3DTEXTUREFILTERTYPE filter) {
  IDirect3DSurface9 *backbuf;
  if (device->GetBackBuffer(0, 0, D3DBACKBUFFER_TYPE_MONO, &backbuf) ==
      D3D_OK) {
    IDirect3DSurface9 *surface;
    if (texture->GetSurfaceLevel(0, &surface) == D3D_OK) {
      device->StretchRect(backbuf, nullptr, surface, nullptr, filter);
    }
  }
}

static void set_render_target(IDirect3DDevice9 *device,
                              IDirect3DTexture9 *rt_texture) {
  IDirect3DSurface9 *surface;
  if (rt_texture->GetSurfaceLevel(0, &surface) == D3D_OK) {
    device->SetRenderTarget(0, surface);
  }
}

constexpr uint32_t BLUR_DOWNSAMPLE = 4;

namespace post_processing {
  BlurEffect::~BlurEffect() {
    if (rt_backup) {
      rt_backup->Release();
    }

    if (blur_texture1) {
      blur_texture1->Release();
    }

    if (blur_texture2) {
      blur_texture2->Release();
    }
  }

  BlurEffect &BlurEffect::get() {
    static BlurEffect effect{};
    return effect;
  }

  void BlurEffect::new_frame() {
    const auto [width, height] = ImGui::GetIO().DisplaySize;

    if (backbuf_width != uint32_t(width) ||
        backbuf_height != uint32_t(height)) {
      clear_textures();

      backbuf_width = uint32_t(width);
      backbuf_height = uint32_t(height);
    }
  }

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
    create_textures();
    create_shaders();

    if (!blur_texture1 || !blur_texture2) {
      return;
    }

    draw_list->AddCallback(post_processing::begin, nullptr);

    for (size_t i = 0; i < 8; i += 1) {
      draw_list->AddCallback(post_processing::first_pass, nullptr);
      draw_list->AddImage(blur_texture1, {-1.0f, -1.0f}, {1.0f, 1.0f});
      draw_list->AddCallback(post_processing::second_pass, nullptr);
      draw_list->AddImage(blur_texture2, {-1.0f, -1.0f}, {1.0f, 1.0f});
    }
    draw_list->AddCallback(post_processing::end, nullptr);
    draw_list->AddCallback(ImDrawCallback_ResetRenderState, nullptr);

    draw_list->AddImage(blur_texture1, {0.0f, 0.0f},
                        {backbuf_width * 1.0f, backbuf_height * 1.0f},
                        {0.0f, 0.0f}, {1.0f, 1.0f},
                        IM_COL32(255, 255, 255, 255 * alpha));
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

  void BlurEffect::create_shaders() {
    blur_shader_x.init(
#include "resources/blur_x.hlsl"
    );
    blur_shader_y.init(
#include "resources/blur_y.hlsl"
    );
  }

  void BlurEffect::begin() {
    device->GetRenderTarget(0, &rt_backup);

    copy_backbuf_to_texture(device, blur_texture1, D3DTEXF_LINEAR);

    device->SetSamplerState(0, D3DSAMP_ADDRESSU, D3DTADDRESS_CLAMP);
    device->SetSamplerState(0, D3DSAMP_ADDRESSV, D3DTADDRESS_CLAMP);
    device->SetRenderState(D3DRS_SCISSORTESTENABLE, false);

    // clang-format off
    const D3DMATRIX projection{{{
      1.0f, 0.0f, 0.0f, 0.0f,
      0.0f, 1.0f, 0.0f, 0.0f,
      0.0f, 0.0f, 1.0f, 0.0f,
      -1.0f / uint32_t(backbuf_width / BLUR_DOWNSAMPLE),
        1.0f / uint32_t(backbuf_height / BLUR_DOWNSAMPLE), 0.0f, 1.0f
    }}};
    // clang-format on

    device->SetVertexShaderConstantF(0, &projection.m[0][0], 4);
  }

  void BlurEffect::first_pass() {
    blur_shader_x.use(1.0f / uint32_t(backbuf_width / BLUR_DOWNSAMPLE), 0);
    set_render_target(device, blur_texture2);
  }

  void BlurEffect::second_pass() {
    blur_shader_y.use(1.0f / uint32_t(backbuf_height / BLUR_DOWNSAMPLE), 0);
    set_render_target(device, blur_texture1);
  }

  void BlurEffect::end() {
    device->SetRenderTarget(0, rt_backup);
    rt_backup->Release();

    device->SetPixelShader(nullptr);
    device->SetRenderState(D3DRS_SCISSORTESTENABLE, true);
  }

  void BlurEffect::set_device(IDirect3DDevice9 *device) {
    this->device = device;
    blur_shader_x.set_device(device);
    blur_shader_y.set_device(device);
  }
}
