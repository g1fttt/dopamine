use windows::Win32::Graphics::Direct3D::Fxc::D3DCompile;
use windows::Win32::Graphics::Direct3D9::*;

use windows::core::Result as WindowsResult;
use windows_numerics::Matrix4x4;

use dopamine_sdk::{cstr, pcstr};
use imgui::{DrawCmd, DrawList, ImVec2, TextureRef};

use std::ffi::c_void;
use std::ptr;

const BLUR_DOWNSAMPLE: f32 = 4.0;

pub struct BlurEffect {
  device: IDirect3DDevice9,
  rt_backup: Option<IDirect3DSurface9>,
  blur_texture1: Option<IDirect3DTexture9>,
  blur_texture2: Option<IDirect3DTexture9>,
  blur_shader_x: IDirect3DPixelShader9,
  blur_shader_y: IDirect3DPixelShader9,
}

impl BlurEffect {
  pub fn new(device: &IDirect3DDevice9) -> Self {
    fn compile_pixel_shader(
      device: &IDirect3DDevice9,
      source: &str,
    ) -> WindowsResult<IDirect3DPixelShader9> {
      unsafe {
        let mut compiled = None;
        let mut errors = None;

        D3DCompile(
          cstr!(source).cast::<c_void>(),
          source.len(),
          None,
          None,
          None,
          pcstr!("main"),
          pcstr!("ps_2_0"), // ps_3_0 is not supported
          0,
          0,
          &mut compiled,
          Some(&mut errors),
        )?;

        if let Some(errors) = errors {
          let error_message = std::str::from_raw_parts(
            errors.GetBufferPointer().cast::<u8>(),
            errors.GetBufferSize(),
          );
          log::error!("Error occured during pixel shader compilation: {error_message}");
        }

        let compiled = compiled.unwrap();
        device.CreatePixelShader(compiled.GetBufferPointer().cast::<u32>())
      }
    }

    let pixel_shader = compile_pixel_shader(device, include_str!("../shaders/blur_x.hlsl"));
    let blur_shader_x = match pixel_shader {
      Ok(shader) => shader,
      Err(err) => panic!("Failed to compile `blur_shader_x`: {err}"),
    };

    let pixel_shader = compile_pixel_shader(device, include_str!("../shaders/blur_y.hlsl"));
    let blur_shader_y = match pixel_shader {
      Ok(shader) => shader,
      Err(err) => panic!("Failed to compile `blur_shader_y`: {err}"),
    };

    // Increments internal reference counter
    let device = device.clone();

    Self {
      device,
      rt_backup: None,
      blur_texture1: None,
      blur_texture2: None,
      blur_shader_x,
      blur_shader_y,
    }
  }

  pub fn draw(&mut self, draw_list: &mut DrawList, alpha: f32) -> WindowsResult<()> {
    self.new_frame()?;

    let Some((blur1, blur2)) = self.blur_texture1.as_ref().zip(self.blur_texture2.as_ref()) else {
      return Ok(());
    };

    let rect_min = ImVec2 { x: -1.0, y: -1.0 };
    let rect_max = ImVec2 { x: 1.0, y: 1.0 };

    draw_list.add_callback_ex(Self::begin_aux).user_data(self).build(draw_list);
    {
      for _ in 0..8 {
        draw_list.add_callback_ex(Self::first_pass_aux).user_data(self).build(draw_list);
        draw_list.add_image(TextureRef::new(blur1), rect_min, rect_max);

        draw_list.add_callback_ex(Self::second_pass_aux).user_data(self).build(draw_list);
        draw_list.add_image(TextureRef::new(blur2), rect_min, rect_max);
      }
    }
    draw_list.add_callback_ex(Self::end_aux).user_data(self).build(draw_list);

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

  extern "C" fn begin_aux(_: &DrawList, cmd: &DrawCmd) {
    let effect = cmd.user_callback_data::<BlurEffect>().unwrap();
    let _ = unsafe { effect.begin() };
  }

  unsafe fn begin(&mut self) -> WindowsResult<()> {
    unsafe {
      let Some(blur_texture1) = self.blur_texture1.as_ref() else {
        return Ok(());
      };

      self.rt_backup.replace(self.device.GetRenderTarget(0)?);

      let backbuf = self.device.GetBackBuffer(0, 0, D3DBACKBUFFER_TYPE_MONO)?;

      self.device.StretchRect(
        &backbuf,
        ptr::null(),
        &blur_texture1.GetSurfaceLevel(0)?,
        ptr::null(),
        D3DTEXF_LINEAR,
      )?;

      self.device.SetSamplerState(0, D3DSAMP_SRGBTEXTURE, 1)?;
      self.device.SetSamplerState(0, D3DSAMP_ADDRESSU, D3DTADDRESS_CLAMP.0 as u32)?;
      self.device.SetSamplerState(0, D3DSAMP_ADDRESSV, D3DTADDRESS_CLAMP.0 as u32)?;

      self.device.SetRenderState(D3DRS_SCISSORTESTENABLE, 0)?;
      self.device.SetRenderState(D3DRS_SRGBWRITEENABLE, 1)?;
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

    unsafe { self.device.SetTransform(D3DTS_PROJECTION, &projection) }
  }

  extern "C" fn first_pass_aux(_: &DrawList, cmd: &DrawCmd) {
    let effect = cmd.user_callback_data::<BlurEffect>().unwrap();
    let _ = unsafe { effect.first_pass() };
  }

  unsafe fn first_pass(&mut self) -> WindowsResult<()> {
    let Some(blur_texture2) = self.blur_texture2.as_ref() else {
      return Ok(());
    };

    let display_width = imgui::io().display_size.x;
    let uniform = 1.0 / (display_width / BLUR_DOWNSAMPLE);

    unsafe {
      self.device.SetPixelShader(&self.blur_shader_x)?;
      self.device.SetPixelShaderConstantF(0, [uniform, 0.0, 0.0, 0.0].as_ptr(), 1)?;
      self.device.SetRenderTarget(0, &blur_texture2.GetSurfaceLevel(0)?)
    }
  }

  extern "C" fn second_pass_aux(_: &DrawList, cmd: &DrawCmd) {
    let effect = cmd.user_callback_data::<BlurEffect>().unwrap();
    let _ = unsafe { effect.second_pass() };
  }

  unsafe fn second_pass(&mut self) -> WindowsResult<()> {
    let Some(blur_texture1) = self.blur_texture1.as_ref() else {
      return Ok(());
    };

    let display_height = imgui::io().display_size.y;
    let uniform = 1.0 / (display_height / BLUR_DOWNSAMPLE);

    unsafe {
      self.device.SetPixelShader(&self.blur_shader_y)?;
      self.device.SetPixelShaderConstantF(0, [uniform, 0.0, 0.0, 0.0].as_ptr(), 1)?;
      self.device.SetRenderTarget(0, &blur_texture1.GetSurfaceLevel(0)?)
    }
  }

  extern "C" fn end_aux(_: &DrawList, cmd: &DrawCmd) {
    let effect = cmd.user_callback_data::<BlurEffect>().unwrap();
    let _ = unsafe { effect.end() };
  }

  unsafe fn end(&mut self) -> WindowsResult<()> {
    unsafe {
      self.device.SetRenderTarget(0, self.rt_backup.take().as_ref().unwrap())?;
      self.device.SetPixelShader(None)?;
      self.device.SetSamplerState(0, D3DSAMP_SRGBTEXTURE, 0)?;

      self.device.SetRenderState(D3DRS_SCISSORTESTENABLE, 1)?;
      self.device.SetRenderState(D3DRS_SRGBWRITEENABLE, 0)
    }
  }

  fn new_frame(&mut self) -> WindowsResult<()> {
    let display_size = imgui::io().display_size;

    let create_texture = || {
      create_texture(
        &self.device,
        display_size.x / BLUR_DOWNSAMPLE,
        display_size.y / BLUR_DOWNSAMPLE,
      )
    };

    if self.blur_texture1.is_none() {
      self.blur_texture1 = Some(create_texture()?);
    }

    if self.blur_texture2.is_none() {
      self.blur_texture2 = Some(create_texture()?);
    }
    Ok(())
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
      D3DFMT_A8R8G8B8,
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
