use super::ImGuiContext;

use imgui::{DrawListMut, Io, TextureId, Textures};
use imgui_dx9_renderer::Renderer;

use windows::Foundation::Numerics::Matrix4x4;
use windows::Win32::Graphics::Direct3D9::*;
use windows::core::Result as WindowsResult;

use std::ptr;

const BLUR_DOWNSAMPLE: f32 = 4.0;

pub struct BlurEffect {
  rt_backup: Option<IDirect3DSurface9>,
  blur_texture1: Texture,
  blur_texture2: Texture,
  blur_shader_x: ShaderProgram,
  blur_shader_y: ShaderProgram,
  backbuf_size: (f32, f32),
}

impl BlurEffect {
  pub fn new() -> Self {
    let blur_shader_x = ShaderProgram::new(BLUR_X);
    let blur_shader_y = ShaderProgram::new(BLUR_Y);

    Self {
      rt_backup: None,
      blur_texture1: Texture::invalid(),
      blur_texture2: Texture::invalid(),
      blur_shader_x,
      blur_shader_y,
      backbuf_size: (-1.0, -1.0),
    }
  }

  pub fn render(
    &mut self,
    device: &IDirect3DDevice9,
    renderer: &mut Renderer,
    io: &Io,
    draw_list: DrawListMut,
    alpha: f32,
  ) -> WindowsResult<()> {
    self.new_frame(device, renderer, io)?;

    add_imgui_callback(&draw_list, BlurEffect::begin, self, device);
    {
      for _ in 0..8 {
        add_imgui_callback(&draw_list, BlurEffect::first_pass, self, device);
        draw_list.add_image(self.blur_texture1.id.unwrap(), [-1.0, -1.0], [1.0, 1.0]).build();
        add_imgui_callback(&draw_list, BlurEffect::second_pass, self, device);
        draw_list.add_image(self.blur_texture2.id.unwrap(), [-1.0, -1.0], [1.0, 1.0]).build();
      }
    }
    add_imgui_callback(&draw_list, BlurEffect::end, self, device);

    add_imgui_callback(&draw_list, BlurEffect::reset_render_state, self, device);

    let (bb_width, bb_height) = self.backbuf_size;

    draw_list
      .add_image(self.blur_texture1.id.unwrap(), [0.0, 0.0], [bb_width * 1.0, bb_height * 1.0])
      .col([1.0, 1.0, 1.0, 1.0 * alpha])
      .build();

    Ok(())
  }

  pub unsafe fn clear_textures(&mut self) {
    self.blur_texture1.uninit();
    self.blur_texture2.uninit();
  }

  unsafe fn begin(&mut self, device: &IDirect3DDevice9) -> WindowsResult<()> {
    unsafe {
      self.rt_backup.replace(device.GetRenderTarget(0)?);

      self.blur_texture1.copy_from_backbuf(device)?;

      device.SetSamplerState(0, D3DSAMP_SRGBTEXTURE, 1)?;

      device.SetSamplerState(0, D3DSAMP_ADDRESSU, D3DTADDRESS_CLAMP.0 as u32)?;
      device.SetSamplerState(0, D3DSAMP_ADDRESSV, D3DTADDRESS_CLAMP.0 as u32)?;

      device.SetRenderState(D3DRS_SCISSORTESTENABLE, 0)?;
    }

    let (bb_width, bb_height) = self.backbuf_size;

    let offset_x = -1.0 / (bb_width / BLUR_DOWNSAMPLE);
    let offset_y = 1.0 / (bb_height / BLUR_DOWNSAMPLE);

    #[rustfmt::skip]
    let projection = array_to_matrix4x4([
      1.0, 0.0, 0.0, 0.0,
      0.0, 1.0, 0.0, 0.0,
      0.0, 0.0, 1.0, 0.0,
      offset_x, offset_y, 0.0, 1.0,
    ]);

    unsafe { device.SetTransform(D3DTS_PROJECTION, &projection) }
  }

  unsafe fn first_pass(&mut self, device: &IDirect3DDevice9) -> WindowsResult<()> {
    let (bb_width, _) = self.backbuf_size;

    self.blur_shader_x.use_it(device, 1.0 / (bb_width / BLUR_DOWNSAMPLE))?;
    unsafe { self.blur_texture2.set_as_rt(device) }
  }

  unsafe fn second_pass(&mut self, device: &IDirect3DDevice9) -> WindowsResult<()> {
    let (_, bb_height) = self.backbuf_size;

    self.blur_shader_y.use_it(device, 1.0 / (bb_height / BLUR_DOWNSAMPLE))?;
    unsafe { self.blur_texture1.set_as_rt(device) }
  }

  unsafe fn end(&mut self, device: &IDirect3DDevice9) -> WindowsResult<()> {
    unsafe {
      device.SetRenderTarget(0, self.rt_backup.take().as_ref().unwrap())?;
      device.SetPixelShader(None)?;
      device.SetRenderState(D3DRS_SCISSORTESTENABLE, 1)
    }
  }

  unsafe fn reset_render_state(&mut self, _device: &IDirect3DDevice9) -> WindowsResult<()> {
    if let Some((_, back_ctx)) = ImGuiContext::get_mut() {
      back_ctx.reset_render_state()?;
    }
    Ok(())
  }

  fn new_frame(
    &mut self,
    device: &IDirect3DDevice9,
    renderer: &mut Renderer,
    io: &Io,
  ) -> WindowsResult<()> {
    let display_width = io.display_size[0];
    let display_height = io.display_size[1];

    if self.backbuf_size != (display_width, display_height) {
      unsafe { self.clear_textures() };

      self.backbuf_size = (display_width, display_height);
    }

    let (bb_width, bb_height) = self.backbuf_size;

    let create_texture =
      || create_texture(device, bb_width / BLUR_DOWNSAMPLE, bb_height / BLUR_DOWNSAMPLE);

    let pool = renderer.textures_mut();

    self.blur_texture1.init_and_update_in_pool(create_texture()?, pool);
    self.blur_texture2.init_and_update_in_pool(create_texture()?, pool);

    self.blur_shader_x.init(device)?;
    self.blur_shader_y.init(device)
  }
}

type ImGuiCallback = unsafe fn(&mut BlurEffect, &IDirect3DDevice9) -> WindowsResult<()>;

fn add_imgui_callback(
  draw_list: &DrawListMut,
  callback: ImGuiCallback,
  blur_effect: *mut BlurEffect,
  device: *const IDirect3DDevice9,
) {
  draw_list
    .add_callback(move || unsafe {
      callback(blur_effect.as_mut_unchecked(), device.as_ref_unchecked()).unwrap()
    })
    .build();
}

fn create_texture(
  device: &IDirect3DDevice9,
  width: f32,
  height: f32,
) -> WindowsResult<IDirect3DTexture9> {
  let mut texture = None;
  unsafe {
    device.CreateTexture(
      width as u32,
      height as u32,
      1,
      D3DUSAGE_RENDERTARGET as u32,
      D3DFMT_X8R8G8B8,
      D3DPOOL_DEFAULT,
      &mut texture,
      ptr::null_mut(),
    )?;
  }
  Ok(texture.unwrap())
}

fn array_to_matrix4x4(data: [f32; 16]) -> Matrix4x4 {
  Matrix4x4 {
    M11: data[0],
    M12: data[1],
    M13: data[2],
    M14: data[3],
    M21: data[4],
    M22: data[5],
    M23: data[6],
    M24: data[7],
    M31: data[8],
    M32: data[9],
    M33: data[10],
    M34: data[11],
    M41: data[12],
    M42: data[13],
    M43: data[14],
    M44: data[15],
  }
}

struct Texture {
  raw: Option<IDirect3DTexture9>,
  id: Option<TextureId>,
}

impl Texture {
  fn invalid() -> Self {
    Self { raw: None, id: None }
  }

  fn surface(&self) -> Option<IDirect3DSurface9> {
    self.raw.as_ref().map(|r| unsafe { r.GetSurfaceLevel(0) }).unwrap().ok()
  }

  fn init_and_update_in_pool(
    &mut self,
    raw: IDirect3DTexture9,
    pool: &mut Textures<IDirect3DTexture9>,
  ) {
    if self.raw.is_some() {
      return;
    }

    self.raw.replace(raw.clone());

    match self.id {
      Some(id) => {
        pool.replace(id, raw);
      }
      None => self.id = Some(pool.insert(raw)),
    }
  }

  fn uninit(&mut self) {
    self.raw.take();
  }

  unsafe fn set_as_rt(&self, device: &IDirect3DDevice9) -> WindowsResult<()> {
    unsafe { device.SetRenderTarget(0, &self.surface().unwrap()) }
  }

  unsafe fn copy_from_backbuf(&mut self, device: &IDirect3DDevice9) -> WindowsResult<()> {
    unsafe {
      let backbuf = device.GetBackBuffer(0, 0, D3DBACKBUFFER_TYPE_MONO)?;
      let surface = self.surface().unwrap();

      device.StretchRect(&backbuf, ptr::null(), &surface, ptr::null(), D3DTEXF_LINEAR)
    }
  }
}

struct ShaderProgram {
  pixel_shader_src: &'static [u8],
  pixel_shader: Option<IDirect3DPixelShader9>,
}

impl ShaderProgram {
  fn new(pixel_shader_src: &'static [u8]) -> Self {
    Self { pixel_shader_src, pixel_shader: None }
  }

  fn init(&mut self, device: &IDirect3DDevice9) -> WindowsResult<()> {
    if self.pixel_shader.is_none() {
      let px_shader = unsafe { device.CreatePixelShader(self.pixel_shader_src.as_ptr().cast())? };

      self.pixel_shader.replace(px_shader);
    }
    Ok(())
  }

  fn use_it(&self, device: &IDirect3DDevice9, uniform: f32) -> WindowsResult<()> {
    unsafe {
      device.SetPixelShader(self.pixel_shader.as_ref().unwrap())?;

      let params = [uniform, uniform, uniform, uniform];
      device.SetPixelShaderConstantF(0, params.as_ptr(), 1)
    }
  }
}

const BLUR_X: &[u8] = &[
  0, 2, 255, 255, 254, 255, 44, 0, 67, 84, 65, 66, 28, 0, 0, 0, 131, 0, 0, 0, 0, 2, 255, 255, 2, 0,
  0, 0, 28, 0, 0, 0, 0, 1, 0, 0, 124, 0, 0, 0, 68, 0, 0, 0, 3, 0, 0, 0, 1, 0, 0, 0, 80, 0, 0, 0, 0,
  0, 0, 0, 96, 0, 0, 0, 2, 0, 0, 0, 1, 0, 0, 0, 108, 0, 0, 0, 0, 0, 0, 0, 116, 101, 120, 83, 97,
  109, 112, 108, 101, 114, 0, 171, 4, 0, 12, 0, 1, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 116, 101, 120,
  101, 108, 87, 105, 100, 116, 104, 0, 171, 0, 0, 3, 0, 1, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 112,
  115, 95, 50, 95, 48, 0, 77, 105, 99, 114, 111, 115, 111, 102, 116, 32, 40, 82, 41, 32, 72, 76,
  83, 76, 32, 83, 104, 97, 100, 101, 114, 32, 67, 111, 109, 112, 105, 108, 101, 114, 32, 49, 48,
  46, 49, 0, 171, 81, 0, 0, 5, 1, 0, 15, 160, 20, 59, 177, 63, 24, 231, 161, 62, 198, 121, 104, 62,
  236, 196, 78, 64, 81, 0, 0, 5, 2, 0, 15, 160, 220, 233, 143, 61, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
  0, 31, 0, 0, 2, 0, 0, 0, 128, 0, 0, 3, 176, 31, 0, 0, 2, 0, 0, 0, 144, 0, 8, 15, 160, 1, 0, 0, 2,
  0, 0, 9, 128, 1, 0, 228, 160, 4, 0, 0, 4, 1, 0, 1, 128, 0, 0, 0, 160, 0, 0, 0, 128, 0, 0, 0, 176,
  1, 0, 0, 2, 1, 0, 2, 128, 0, 0, 85, 176, 4, 0, 0, 4, 0, 0, 1, 128, 0, 0, 0, 160, 0, 0, 0, 129, 0,
  0, 0, 176, 1, 0, 0, 2, 0, 0, 2, 128, 0, 0, 85, 176, 4, 0, 0, 4, 2, 0, 1, 128, 0, 0, 0, 160, 0, 0,
  255, 129, 0, 0, 0, 176, 1, 0, 0, 2, 2, 0, 2, 128, 0, 0, 85, 176, 4, 0, 0, 4, 3, 0, 1, 128, 0, 0,
  0, 160, 0, 0, 255, 128, 0, 0, 0, 176, 1, 0, 0, 2, 3, 0, 2, 128, 0, 0, 85, 176, 66, 0, 0, 3, 1, 0,
  15, 128, 1, 0, 228, 128, 0, 8, 228, 160, 66, 0, 0, 3, 0, 0, 15, 128, 0, 0, 228, 128, 0, 8, 228,
  160, 66, 0, 0, 3, 4, 0, 15, 128, 0, 0, 228, 176, 0, 8, 228, 160, 66, 0, 0, 3, 2, 0, 15, 128, 2,
  0, 228, 128, 0, 8, 228, 160, 66, 0, 0, 3, 3, 0, 15, 128, 3, 0, 228, 128, 0, 8, 228, 160, 5, 0, 0,
  3, 0, 0, 7, 128, 0, 0, 228, 128, 1, 0, 85, 160, 4, 0, 0, 4, 0, 0, 7, 128, 4, 0, 228, 128, 1, 0,
  170, 160, 0, 0, 228, 128, 4, 0, 0, 4, 0, 0, 7, 128, 1, 0, 228, 128, 1, 0, 85, 160, 0, 0, 228,
  128, 4, 0, 0, 4, 0, 0, 7, 128, 2, 0, 228, 128, 2, 0, 0, 160, 0, 0, 228, 128, 4, 0, 0, 4, 4, 0, 7,
  128, 3, 0, 228, 128, 2, 0, 0, 160, 0, 0, 228, 128, 1, 0, 0, 2, 0, 8, 15, 128, 4, 0, 228, 128,
  255, 255, 0, 0,
];

const BLUR_Y: &[u8] = &[
  0, 2, 255, 255, 254, 255, 44, 0, 67, 84, 65, 66, 28, 0, 0, 0, 131, 0, 0, 0, 0, 2, 255, 255, 2, 0,
  0, 0, 28, 0, 0, 0, 0, 1, 0, 0, 124, 0, 0, 0, 68, 0, 0, 0, 3, 0, 0, 0, 1, 0, 0, 0, 80, 0, 0, 0, 0,
  0, 0, 0, 96, 0, 0, 0, 2, 0, 0, 0, 1, 0, 0, 0, 108, 0, 0, 0, 0, 0, 0, 0, 116, 101, 120, 83, 97,
  109, 112, 108, 101, 114, 0, 171, 4, 0, 12, 0, 1, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 116, 101, 120,
  101, 108, 72, 101, 105, 103, 104, 116, 0, 0, 0, 3, 0, 1, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 112,
  115, 95, 50, 95, 48, 0, 77, 105, 99, 114, 111, 115, 111, 102, 116, 32, 40, 82, 41, 32, 72, 76,
  83, 76, 32, 83, 104, 97, 100, 101, 114, 32, 67, 111, 109, 112, 105, 108, 101, 114, 32, 49, 48,
  46, 49, 0, 171, 81, 0, 0, 5, 1, 0, 15, 160, 20, 59, 177, 63, 24, 231, 161, 62, 198, 121, 104, 62,
  236, 196, 78, 64, 81, 0, 0, 5, 2, 0, 15, 160, 220, 233, 143, 61, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
  0, 31, 0, 0, 2, 0, 0, 0, 128, 0, 0, 3, 176, 31, 0, 0, 2, 0, 0, 0, 144, 0, 8, 15, 160, 1, 0, 0, 2,
  0, 0, 9, 128, 1, 0, 228, 160, 4, 0, 0, 4, 1, 0, 2, 128, 0, 0, 0, 160, 0, 0, 0, 128, 0, 0, 85,
  176, 1, 0, 0, 2, 1, 0, 1, 128, 0, 0, 0, 176, 4, 0, 0, 4, 0, 0, 2, 128, 0, 0, 0, 160, 0, 0, 0,
  129, 0, 0, 85, 176, 1, 0, 0, 2, 0, 0, 1, 128, 0, 0, 0, 176, 4, 0, 0, 4, 2, 0, 2, 128, 0, 0, 0,
  160, 0, 0, 255, 129, 0, 0, 85, 176, 1, 0, 0, 2, 2, 0, 1, 128, 0, 0, 0, 176, 4, 0, 0, 4, 3, 0, 2,
  128, 0, 0, 0, 160, 0, 0, 255, 128, 0, 0, 85, 176, 1, 0, 0, 2, 3, 0, 1, 128, 0, 0, 0, 176, 66, 0,
  0, 3, 1, 0, 15, 128, 1, 0, 228, 128, 0, 8, 228, 160, 66, 0, 0, 3, 0, 0, 15, 128, 0, 0, 228, 128,
  0, 8, 228, 160, 66, 0, 0, 3, 4, 0, 15, 128, 0, 0, 228, 176, 0, 8, 228, 160, 66, 0, 0, 3, 2, 0,
  15, 128, 2, 0, 228, 128, 0, 8, 228, 160, 66, 0, 0, 3, 3, 0, 15, 128, 3, 0, 228, 128, 0, 8, 228,
  160, 5, 0, 0, 3, 0, 0, 7, 128, 0, 0, 228, 128, 1, 0, 85, 160, 4, 0, 0, 4, 0, 0, 7, 128, 4, 0,
  228, 128, 1, 0, 170, 160, 0, 0, 228, 128, 4, 0, 0, 4, 0, 0, 7, 128, 1, 0, 228, 128, 1, 0, 85,
  160, 0, 0, 228, 128, 4, 0, 0, 4, 0, 0, 7, 128, 2, 0, 228, 128, 2, 0, 0, 160, 0, 0, 228, 128, 4,
  0, 0, 4, 4, 0, 7, 128, 3, 0, 228, 128, 2, 0, 0, 160, 0, 0, 228, 128, 1, 0, 0, 2, 0, 8, 15, 128,
  4, 0, 228, 128, 255, 255, 0, 0,
];
