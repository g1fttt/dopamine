#pragma once

#include <cstdint>

struct IDirect3DPixelShader9;
struct IDirect3DSurface9;
struct IDirect3DTexture9;
struct IDirect3DDevice9;

struct ImDrawList;

namespace post_processing {
  namespace {
    class ShaderProgram {
    public:
      void use(float uniform, int32_t location);
      void init(const char *pixel_shader_src);

      void set_device(IDirect3DDevice9 *device);
    private:
      IDirect3DPixelShader9 *pixel_shader;
      IDirect3DDevice9 *device;
      bool inited = false;
    };
  }

  class BlurEffect {
  public:
    ~BlurEffect();

    static BlurEffect &get();

    void new_frame();
    void draw(ImDrawList *draw_list, float alpha);

    void clear_textures();

    void create_textures();
    void create_shaders();

    void begin();
    void first_pass();
    void second_pass();
    void end();

    void set_device(IDirect3DDevice9 *device);
  private:
    IDirect3DDevice9 *device;
    IDirect3DSurface9 *rt_backup;
    IDirect3DTexture9 *blur_texture1, *blur_texture2;
    ShaderProgram blur_shader_x, blur_shader_y;
    uint32_t backbuf_width, backbuf_height;
  };
}
