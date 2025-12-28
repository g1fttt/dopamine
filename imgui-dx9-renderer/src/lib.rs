use windows::core::Interface;
use windows::core::Result as WindowsResult;

use windows::Win32::Foundation::RECT;
use windows::Win32::Graphics::Direct3D9::*;

use imgui::*;
use windows_numerics::Matrix4x4;

use std::ffi::c_void;
use std::{mem, ptr, slice};

const D3DFVF_CUSTOMVERTEX: u32 = D3DFVF_XYZ | D3DFVF_DIFFUSE | D3DFVF_TEX1;

const VERTEX_BUF_ADD_CAPACITY: usize = 5000;
const INDEX_BUF_ADD_CAPACITY: usize = 10000;

#[repr(C)]
struct CustomVertex {
  pos: [f32; 3],
  col: u32,
  uv: [f32; 2],
}

pub struct Renderer {
  device: IDirect3DDevice9,
  vertex_buffer: (IDirect3DVertexBuffer9, usize),
  index_buffer: (IDirect3DIndexBuffer9, usize),
}

impl Renderer {
  pub fn new(device: &IDirect3DDevice9) -> WindowsResult<Self> {
    let io = imgui::io_mut();
    io.backend_flags |= BackendFlags::RENDERER_HAS_VTF_OFFSET;
    io.backend_flags |= BackendFlags::RENDERER_HAS_TEXTURES;

    const MAX_TEXTURE_SIZE: i32 = 4096;

    let platform_io = imgui::platform_io_mut();
    platform_io.texture_max_width = MAX_TEXTURE_SIZE;
    platform_io.texture_max_height = MAX_TEXTURE_SIZE;

    // Increments internal reference counter
    let device = device.clone();

    unsafe {
      Ok(Renderer {
        vertex_buffer: Self::create_vertex_buffer(&device, 0)?,
        index_buffer: Self::create_index_buffer(&device, 0)?,
        device,
      })
    }
  }

  pub fn render(&mut self, draw_data: &imgui::DrawData) -> WindowsResult<()> {
    if draw_data.display_size.x <= 0.0 || draw_data.display_size.y <= 0.0 {
      return Ok(());
    }

    let has_textures_to_update = draw_data.textures_size() > 0;

    if has_textures_to_update {
      draw_data
        .textures()
        .filter(|tx| tx.status != TextureStatus::Ok)
        .try_for_each(|tx| unsafe { self.update_texture(tx) })?;
    }

    unsafe {
      let vb = &mut self.vertex_buffer;
      if vb.1 /* vtx_count */ < draw_data.total_vtx_count as usize {
        *vb = Self::create_vertex_buffer(&self.device, draw_data.total_vtx_count as usize)?;
      }

      let ib = &mut self.index_buffer;
      if ib.1 /* idx_count */ < draw_data.total_idx_count as usize {
        *ib = Self::create_index_buffer(&self.device, draw_data.total_idx_count as usize)?;
      }

      if let Ok(state_block) = self.device.CreateStateBlock(D3DSBT_ALL) {
        state_block.Capture()?;

        self.write_buffers(draw_data)?;
        self.setup_render_state(draw_data)?;
        self.render_impl(draw_data)?;

        state_block.Apply()?;
      }
    }
    Ok(())
  }

  // NOTE: Make it more "Rust convenient"?
  unsafe fn copy_texture_region(
    src: *const u32,
    src_pitch: i32,
    dst: *mut u32,
    dst_pitch: i32,
    w: i32,
    h: i32,
  ) -> WindowsResult<()> {
    for y in 0..h {
      unsafe {
        let src_p = src.byte_add((src_pitch * y) as usize).cast::<u8>();
        let dst_p = dst.byte_add((dst_pitch * y) as usize).cast::<u8>();

        ptr::copy(src_p, dst_p, (w * 4) as usize);
      }
    }
    Ok(())
  }

  unsafe fn update_texture(&mut self, texture: &mut TextureData) -> WindowsResult<()> {
    match texture.status {
      TextureStatus::WantCreate => {
        assert_eq!(texture.tex_id, 0 /* ImTextureID_Invalid */);
        assert_eq!(texture.format, TextureFormat::RGBA32);

        unsafe {
          let mut dx_texture = None;

          self.device.CreateTexture(
            texture.width as u32,
            texture.height as u32,
            1,
            D3DUSAGE_DYNAMIC as u32,
            D3DFMT_A8R8G8B8,
            D3DPOOL_DEFAULT,
            &mut dx_texture,
            ptr::null_mut(),
          )?;

          let dx_texture = dx_texture.unwrap();

          let mut locked_rect = D3DLOCKED_RECT::default();

          if dx_texture.LockRect(0, &mut locked_rect, ptr::null(), 0).is_ok() {
            Self::copy_texture_region(
              texture.pixels.cast::<u32>(),
              texture.width * 4,
              locked_rect.pBits.cast::<u32>(),
              locked_rect.Pitch,
              texture.width,
              texture.height,
            )?;
            dx_texture.UnlockRect(0)?;
          }

          texture.tex_id = mem::transmute::<IDirect3DTexture9, TextureID>(dx_texture);
          texture.status = TextureStatus::Ok;
        }
      }
      TextureStatus::WantUpdates => {
        let texture_ptr = texture.tex_id as *mut c_void;
        let backend_texture =
          unsafe { IDirect3DTexture9::from_raw_borrowed(&texture_ptr).unwrap() };

        let update_rect = RECT {
          left: texture.update_rect.x as i32,
          top: texture.update_rect.y as i32,
          right: (texture.update_rect.x + texture.update_rect.w) as i32,
          bottom: (texture.update_rect.y + texture.update_rect.h) as i32,
        };

        let mut locked_rect = D3DLOCKED_RECT::default();

        unsafe {
          backend_texture.LockRect(0, &mut locked_rect, &update_rect, 0)?;

          for r in texture.updates() {
            let src = texture
              .pixels
              .add(((r.x as i32 + r.y as i32 * texture.width) * texture.bytes_per_pixel) as usize)
              .cast::<u32>();
            let dst = locked_rect.pBits.cast::<u32>().add(
              ((r.x as i32 - update_rect.left)
                + (r.y as i32 - update_rect.top) * (locked_rect.Pitch / 4)) as usize,
            );

            Self::copy_texture_region(
              src,
              texture.width * 4,
              dst,
              locked_rect.Pitch,
              r.w as i32,
              r.h as i32,
            )?;
          }

          backend_texture.UnlockRect(0)?;
        }

        texture.status = TextureStatus::Ok;
      }
      TextureStatus::WantDestroy => {
        if texture.tex_id == 0 {
          return Ok(());
        }

        let backend_texture = unsafe { IDirect3DTexture9::from_raw(texture.tex_id as *mut c_void) };
        mem::drop(backend_texture);

        texture.tex_id = 0; // ImTextureID_Invalid
        texture.status = TextureStatus::Destroyed;
      }
      _ => (),
    }
    Ok(())
  }

  unsafe fn render_impl(&mut self, draw_data: &DrawData) -> WindowsResult<()> {
    let mut vertex_offset = 0;
    let mut index_offset = 0;

    for cmd_list in draw_data.cmd_lists() {
      for cmd in cmd_list.cmd_buffer() {
        match cmd.user_callback {
          Some(cb) => unsafe {
            if mem::transmute_copy::<Option<DrawCallback>, i32>(&cmd.user_callback) == -8 {
              self.setup_render_state(draw_data)?;
            } else {
              cb(cmd_list, cmd);
            }
          },
          None => {
            let clip_off = draw_data.display_pos;

            let clip_min =
              ImVec2 { x: cmd.clip_rect.x - clip_off.x, y: cmd.clip_rect.y - clip_off.y };
            let clip_max =
              ImVec2 { x: cmd.clip_rect.z - clip_off.x, y: cmd.clip_rect.w - clip_off.y };

            if clip_max.x <= clip_min.x || clip_max.y <= clip_min.y {
              continue;
            }

            unsafe {
              let r = RECT {
                left: clip_min.x as i32,
                top: clip_min.y as i32,
                right: clip_max.x as i32,
                bottom: clip_max.y as i32,
              };
              self.device.SetScissorRect(&r)?;

              let texture_ptr = cmd.texture_ref.id() as *mut c_void;
              let texture = IDirect3DBaseTexture9::from_raw_borrowed(&texture_ptr).unwrap();
              self.device.SetTexture(0, texture)?;
              self.device.DrawIndexedPrimitive(
                D3DPT_TRIANGLELIST,
                (cmd.vtx_offset + vertex_offset) as i32,
                0,
                cmd_list.vtx_buffer_size() as u32,
                cmd.idx_offset + index_offset,
                cmd.elem_count / 3,
              )?;
            }
          }
        }
      }

      index_offset += cmd_list.idx_buffer_size() as u32;
      vertex_offset += cmd_list.vtx_buffer_size() as u32;
    }
    Ok(())
  }

  pub unsafe fn setup_render_state(&mut self, draw_data: &imgui::DrawData) -> WindowsResult<()> {
    let fb_width = draw_data.display_size.x * draw_data.framebuffer_scale.x;
    let fb_height = draw_data.display_size.y * draw_data.framebuffer_scale.y;

    let vp = D3DVIEWPORT9 {
      X: 0,
      Y: 0,
      Width: fb_width as u32,
      Height: fb_height as u32,
      MinZ: 0.0,
      MaxZ: 1.0,
    };

    let device = &self.device;

    unsafe {
      device.SetViewport(&vp)?;

      device.SetPixelShader(None)?;
      device.SetVertexShader(None)?;
      device.SetRenderState(D3DRS_FILLMODE, D3DFILL_SOLID.0 as u32)?;
      device.SetRenderState(D3DRS_SHADEMODE, D3DSHADE_GOURAUD.0 as u32)?;
      device.SetRenderState(D3DRS_ZWRITEENABLE, 0)?;
      device.SetRenderState(D3DRS_ALPHATESTENABLE, 0)?;
      device.SetRenderState(D3DRS_CULLMODE, D3DCULL_NONE.0 as u32)?;
      device.SetRenderState(D3DRS_ZENABLE, 0)?;
      device.SetRenderState(D3DRS_ALPHABLENDENABLE, 1)?;
      device.SetRenderState(D3DRS_BLENDOP, D3DBLENDOP_ADD.0 as u32)?;
      device.SetRenderState(D3DRS_SRCBLEND, D3DBLEND_SRCALPHA.0 as u32)?;
      device.SetRenderState(D3DRS_DESTBLEND, D3DBLEND_INVSRCALPHA.0 as u32)?;
      device.SetRenderState(D3DRS_SEPARATEALPHABLENDENABLE, 1)?;
      device.SetRenderState(D3DRS_SRCBLENDALPHA, D3DBLEND_ONE.0 as u32)?;
      device.SetRenderState(D3DRS_DESTBLENDALPHA, D3DBLEND_INVSRCALPHA.0 as u32)?;
      device.SetRenderState(D3DRS_SCISSORTESTENABLE, 1)?;
      device.SetRenderState(D3DRS_FOGENABLE, 0)?;
      device.SetRenderState(D3DRS_RANGEFOGENABLE, 0)?;
      device.SetRenderState(D3DRS_SPECULARENABLE, 0)?;
      device.SetRenderState(D3DRS_STENCILENABLE, 0)?;
      device.SetRenderState(D3DRS_CLIPPING, 1)?;
      device.SetRenderState(D3DRS_LIGHTING, 0)?;
      device.SetTextureStageState(0, D3DTSS_COLOROP, D3DTOP_MODULATE.0 as u32)?;
      device.SetTextureStageState(0, D3DTSS_COLORARG1, D3DTA_TEXTURE)?;
      device.SetTextureStageState(0, D3DTSS_COLORARG2, D3DTA_DIFFUSE)?;
      device.SetTextureStageState(0, D3DTSS_ALPHAOP, D3DTOP_MODULATE.0 as u32)?;
      device.SetTextureStageState(0, D3DTSS_ALPHAARG1, D3DTA_TEXTURE)?;
      device.SetTextureStageState(0, D3DTSS_ALPHAARG2, D3DTA_DIFFUSE)?;
      device.SetTextureStageState(1, D3DTSS_COLOROP, D3DTOP_DISABLE.0 as u32)?;
      device.SetTextureStageState(1, D3DTSS_ALPHAOP, D3DTOP_DISABLE.0 as u32)?;
      device.SetSamplerState(0, D3DSAMP_MINFILTER, D3DTEXF_LINEAR.0 as u32)?;
      device.SetSamplerState(0, D3DSAMP_MAGFILTER, D3DTEXF_LINEAR.0 as u32)?;
    }

    let l = draw_data.display_pos.x + 0.5;
    let r = draw_data.display_pos.x + draw_data.display_size.x + 0.5;
    let t = draw_data.display_pos.y + 0.5;
    let b = draw_data.display_pos.y + draw_data.display_size.y + 0.5;

    #[rustfmt::skip]
    const MAT_IDENTITY: Matrix4x4 = Matrix4x4 {
      M11: 1.0, M12: 0.0, M13: 0.0, M14: 0.0,
      M21: 0.0, M22: 1.0, M23: 0.0, M24: 0.0,
      M31: 0.0, M32: 0.0, M33: 1.0, M34: 0.0,
      M41: 0.0, M42: 0.0, M43: 0.0, M44: 1.0,
    };

    #[rustfmt::skip]
    let mat_projection = Matrix4x4 {
      M11: 2.0 / (r - l),     M12: 0.0,               M13: 0.0, M14: 0.0,
      M21: 0.0,               M22: 2.0 / (t - b),     M23: 0.0, M24: 0.0,
      M31: 0.0,               M32: 0.0,               M33: 0.5, M34: 0.0,
      M41: (l + r) / (l - r), M42: (t + b) / (b - t), M43: 0.5, M44: 1.0,
    };

    unsafe {
      device.SetTransform(D3DTRANSFORMSTATETYPE(0), &MAT_IDENTITY)?;
      device.SetTransform(D3DTS_VIEW, &MAT_IDENTITY)?;
      device.SetTransform(D3DTS_PROJECTION, &mat_projection)?;
    }
    Ok(())
  }

  unsafe fn lock_buffers(&mut self) -> WindowsResult<(&mut [CustomVertex], &mut [DrawIndex])> {
    let mut vtx_dst: *mut CustomVertex = ptr::null_mut();
    let mut idx_dst: *mut DrawIndex = ptr::null_mut();

    unsafe {
      let (ref vb, vtx_count) = self.vertex_buffer;
      vb.Lock(
        0,
        (vtx_count * size_of::<CustomVertex>()) as u32,
        &raw mut vtx_dst as *mut *mut c_void,
        D3DLOCK_DISCARD as u32,
      )?;

      let (ref ib, idx_count) = self.index_buffer;
      match ib.Lock(
        0,
        (idx_count * size_of::<DrawIndex>()) as u32,
        &raw mut idx_dst as *mut *mut c_void,
        D3DLOCK_DISCARD as u32,
      ) {
        Ok(_) => Ok((
          slice::from_raw_parts_mut(vtx_dst, vtx_count),
          slice::from_raw_parts_mut(idx_dst, idx_count),
        )),
        Err(e) => {
          vb.Unlock()?;
          Err(e)
        }
      }
    }
  }

  unsafe fn write_buffers(&mut self, draw_data: &imgui::DrawData) -> WindowsResult<()> {
    fn col_to_dx9_argb(col: u32) -> u32 {
      (col & 0xFF00FF00) | ((col & 0xFF0000) >> 16) | ((col & 0xFF) << 16)
    }

    let (mut vertex_buffer, mut index_buffer) = unsafe { self.lock_buffers()? };

    for cmd_list in draw_data.cmd_lists() {
      for (vtx_src, vtx_dst) in cmd_list.vtx_buffer().zip(vertex_buffer.iter_mut()) {
        vtx_dst.pos[0] = vtx_src.pos.x;
        vtx_dst.pos[1] = vtx_src.pos.y;
        vtx_dst.pos[2] = 0.0;
        vtx_dst.col = col_to_dx9_argb(vtx_src.col);
        vtx_dst.uv[0] = vtx_src.uv.x;
        vtx_dst.uv[1] = vtx_src.uv.y;
      }

      let idx_src =
        unsafe { slice::from_raw_parts(cmd_list.idx_buffer_raw(), cmd_list.idx_buffer_size()) };
      index_buffer[..idx_src.len()].copy_from_slice(idx_src);

      vertex_buffer = &mut vertex_buffer[cmd_list.vtx_buffer_size()..];
      index_buffer = &mut index_buffer[cmd_list.idx_buffer_size()..];
    }
    unsafe {
      let (ref vb, _vtx_count) = self.vertex_buffer;
      vb.Unlock()?;

      let (ref ib, _idx_count) = self.index_buffer;
      ib.Unlock()?;

      self.device.SetStreamSource(0, vb, 0, size_of::<CustomVertex>() as u32)?;
      self.device.SetIndices(ib)?;
      self.device.SetFVF(D3DFVF_CUSTOMVERTEX)?;
    }
    Ok(())
  }

  unsafe fn create_vertex_buffer(
    device: &IDirect3DDevice9,
    vtx_count: usize,
  ) -> WindowsResult<(IDirect3DVertexBuffer9, usize)> {
    let len = vtx_count + VERTEX_BUF_ADD_CAPACITY;
    let mut vertex_buffer = None;

    unsafe {
      device.CreateVertexBuffer(
        (len * size_of::<CustomVertex>()) as u32,
        (D3DUSAGE_DYNAMIC | D3DUSAGE_WRITEONLY) as u32,
        D3DFVF_CUSTOMVERTEX,
        D3DPOOL_DEFAULT,
        &mut vertex_buffer,
        ptr::null_mut(),
      )?;
    }
    Ok((vertex_buffer.unwrap(), len))
  }

  unsafe fn create_index_buffer(
    device: &IDirect3DDevice9,
    idx_count: usize,
  ) -> WindowsResult<(IDirect3DIndexBuffer9, usize)> {
    let len = idx_count + INDEX_BUF_ADD_CAPACITY;
    let mut index_buffer = None;

    unsafe {
      device.CreateIndexBuffer(
        (len * size_of::<DrawIndex>()) as u32,
        (D3DUSAGE_DYNAMIC | D3DUSAGE_WRITEONLY) as u32,
        if size_of::<DrawIndex>() == 2 { D3DFMT_INDEX16 } else { D3DFMT_INDEX32 },
        D3DPOOL_DEFAULT,
        &mut index_buffer,
        ptr::null_mut(),
      )?;
    }
    Ok((index_buffer.unwrap(), len))
  }
}
