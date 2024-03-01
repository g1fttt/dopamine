#pragma once

#include <Windows.h>

#include <wrl/client.h>

#include <cstdint>

using namespace Microsoft::WRL;

struct ImDrawList;

struct IDirect3DDevice9;
struct IDirect3DTexture9;
struct IDirect3DPixelShader9;
struct IDirect3DSurface9;

namespace post_processing {
  namespace {
    class ShaderProgram {
    public:
      ~ShaderProgram() = default;

      void use(float uniform);
      void init(const BYTE *pixel_shader_src);

      constexpr void set_device(IDirect3DDevice9 *device) {
        this->device = device;
      }
    private:
      IDirect3DDevice9 *device;
      ComPtr<IDirect3DPixelShader9> pixel_shader;
      bool inited = false;
    };
  }
}

namespace post_processing {
  class BlurEffect {
  public:
    ~BlurEffect() = default;

    static BlurEffect &get();

    void draw(ImDrawList *draw_list, float alpha);

    void begin();
    void first_pass();
    void second_pass();
    void end();

    void set_device(IDirect3DDevice9 *device);

    void clear_textures();
  private:
    void new_frame();

    void create_shaders();
    void create_textures();
  private:
    IDirect3DDevice9 *device;
    // Yes, I LOVE smart pointers
    ComPtr<IDirect3DSurface9> rt_backup;
    ComPtr<IDirect3DTexture9> blur_texture;
    ShaderProgram blur_shader_x, blur_shader_y;
    uint32_t backbuf_width, backbuf_height;
  };
}
