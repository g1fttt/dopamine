use imgui::{CmdList, DrawCmd, ImVec2, TextureRef};

use windows::Foundation::Numerics::Matrix4x4;
use windows::Win32::Graphics::Direct3D9::*;
use windows::core::Result as WindowsResult;

use std::ptr;

const BLUR_DOWNSAMPLE: f32 = 4.0;

pub struct BlurEffect {
  rt_backup: Option<IDirect3DSurface9>,
  blur_texture1: Option<IDirect3DTexture9>,
  blur_texture2: Option<IDirect3DTexture9>,
  blur_shader_x: ShaderProgram,
  blur_shader_y: ShaderProgram,
}

impl BlurEffect {
  pub fn new() -> Self {
    let blur_shader_x = ShaderProgram::new(BLUR_X);
    let blur_shader_y = ShaderProgram::new(BLUR_Y);

    Self { rt_backup: None, blur_texture1: None, blur_texture2: None, blur_shader_x, blur_shader_y }
  }

  pub fn render(
    &mut self,
    device: &IDirect3DDevice9,
    draw_list: &mut imgui::DrawList,
    alpha: f32,
  ) -> WindowsResult<()> {
    self.new_frame(device)?;

    let self_ptr = self as *mut BlurEffect;

    let add_callback_ex = |dl: &mut imgui::DrawList, cb| {
      dl.add_callback_ex(cb).user_data(&(self_ptr, device)).build(dl)
    };

    let Some((blur1, blur2)) = self.blur_texture1.as_ref().zip(self.blur_texture2.as_ref()) else {
      return Ok(());
    };

    let rect_min = ImVec2 { x: -1.0, y: -1.0 };
    let rect_max = ImVec2 { x: 1.0, y: 1.0 };

    add_callback_ex(draw_list, Self::begin_aux);
    {
      for _ in 0..8 {
        add_callback_ex(draw_list, Self::first_pass_aux);
        draw_list.add_image(TextureRef::new(blur1), rect_min, rect_max);

        add_callback_ex(draw_list, Self::second_pass_aux);
        draw_list.add_image(TextureRef::new(blur2), rect_min, rect_max);
      }
    }
    add_callback_ex(draw_list, Self::end_aux);

    draw_list.add_callback(imgui::reset_render_state());

    let display_size = imgui::io().display_size;

    draw_list
      .add_image_ex(
        TextureRef::new(blur1),
        ImVec2 { x: 0.0, y: 0.0 },
        ImVec2 { x: display_size.x * 1.0, y: display_size.y * 1.0 },
      )
      .color(imgui::im_col32(1.0, 1.0, 1.0, alpha))
      .build(draw_list);

    Ok(())
  }

  #[inline]
  pub fn clear_textures(&mut self) {
    self.blur_texture1.take();
    self.blur_texture2.take();
  }

  extern "C" fn begin_aux(_: &CmdList, cmd: &DrawCmd) {
    let (effect, device): &mut (&mut BlurEffect, &IDirect3DDevice9) =
      cmd.user_callback_data().unwrap();
    let _ = unsafe { effect.begin(device) };
  }

  unsafe fn begin(&mut self, device: &IDirect3DDevice9) -> WindowsResult<()> {
    unsafe {
      let Some(blur_texture1) = self.blur_texture1.as_ref() else {
        return Ok(());
      };

      self.rt_backup.replace(device.GetRenderTarget(0)?);

      let backbuf = device.GetBackBuffer(0, 0, D3DBACKBUFFER_TYPE_MONO)?;

      device.StretchRect(
        &backbuf,
        ptr::null(),
        &blur_texture1.GetSurfaceLevel(0)?,
        ptr::null(),
        D3DTEXF_LINEAR,
      )?;

      device.SetSamplerState(0, D3DSAMP_SRGBTEXTURE, 1)?;

      device.SetSamplerState(0, D3DSAMP_ADDRESSU, D3DTADDRESS_CLAMP.0 as u32)?;
      device.SetSamplerState(0, D3DSAMP_ADDRESSV, D3DTADDRESS_CLAMP.0 as u32)?;

      device.SetRenderState(D3DRS_SCISSORTESTENABLE, 0)?;
    }

    let display_size = imgui::io().display_size;

    let offset_x = -1.0 / (display_size.x / BLUR_DOWNSAMPLE);
    let offset_y = 1.0 / (display_size.y / BLUR_DOWNSAMPLE);

    #[rustfmt::skip]
    let projection = array_to_matrix4x4([
      1.0, 0.0, 0.0, 0.0,
      0.0, 1.0, 0.0, 0.0,
      0.0, 0.0, 1.0, 0.0,
      offset_x, offset_y, 0.0, 1.0,
    ]);

    unsafe { device.SetTransform(D3DTS_PROJECTION, &projection) }
  }

  extern "C" fn first_pass_aux(_: &CmdList, cmd: &DrawCmd) {
    let (effect, device): &mut (&mut BlurEffect, &IDirect3DDevice9) =
      cmd.user_callback_data().unwrap();
    let _ = unsafe { effect.first_pass(device) };
  }

  unsafe fn first_pass(&mut self, device: &IDirect3DDevice9) -> WindowsResult<()> {
    let Some(blur_texture2) = self.blur_texture2.as_ref() else {
      return Ok(());
    };

    let display_width = imgui::io().display_size.x;

    self.blur_shader_x.use_it(device, 1.0 / (display_width / BLUR_DOWNSAMPLE))?;
    unsafe { device.SetRenderTarget(0, &blur_texture2.GetSurfaceLevel(0)?) }
  }

  extern "C" fn second_pass_aux(_: &imgui::CmdList, cmd: &imgui::DrawCmd) {
    let (effect, device): &mut (&mut BlurEffect, &IDirect3DDevice9) =
      cmd.user_callback_data().unwrap();
    let _ = unsafe { effect.second_pass(device) };
  }

  unsafe fn second_pass(&mut self, device: &IDirect3DDevice9) -> WindowsResult<()> {
    let Some(blur_texture1) = self.blur_texture1.as_ref() else {
      return Ok(());
    };

    let display_height = imgui::io().display_size.y;

    self.blur_shader_y.use_it(device, 1.0 / (display_height / BLUR_DOWNSAMPLE))?;
    unsafe { device.SetRenderTarget(0, &blur_texture1.GetSurfaceLevel(0)?) }
  }

  extern "C" fn end_aux(_: &CmdList, cmd: &DrawCmd) {
    let (effect, device): &mut (&mut BlurEffect, &IDirect3DDevice9) =
      cmd.user_callback_data().unwrap();
    let _ = unsafe { effect.end(device) };
  }

  unsafe fn end(&mut self, device: &IDirect3DDevice9) -> WindowsResult<()> {
    unsafe {
      device.SetRenderTarget(0, self.rt_backup.take().as_ref().unwrap())?;
      device.SetPixelShader(None)?;
      device.SetRenderState(D3DRS_SCISSORTESTENABLE, 1)
    }
  }

  fn new_frame(&mut self, device: &IDirect3DDevice9) -> WindowsResult<()> {
    let display_size = imgui::io().display_size;

    let create_texture =
      || create_texture(device, display_size.x / BLUR_DOWNSAMPLE, display_size.y / BLUR_DOWNSAMPLE);

    if self.blur_texture1.is_none() {
      self.blur_texture1 = Some(create_texture()?);
    }

    if self.blur_texture2.is_none() {
      self.blur_texture2 = Some(create_texture()?);
    }

    self.blur_shader_x.init(device)?;
    self.blur_shader_y.init(device)
  }
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
