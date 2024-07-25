use crate::game::client::{Client, ClientMode, EntityList};
use crate::game::engine::{Engine, ModelInfo, ModelRender};
use crate::game::input_system::InputSystem;
use crate::game::material_system::MaterialSystem;
use crate::game::render_view::RenderView;
use crate::game::studio_render::StudioRender;
use crate::game::surface::Surface;

use crate::{cstr, ok_or_empty_err, pcstr};

use windows::Win32::System::LibraryLoader::{GetModuleHandleA, GetProcAddress};

use std::ffi::{c_char, c_void};
use std::sync::LazyLock;
use std::{mem, ptr};

pub struct Interfaces<'a> {
  pub client: &'a Client,
  pub client_mode: &'a ClientMode,
  pub entity_list: &'a EntityList,
  pub engine: &'a Engine,
  pub render_view: &'a RenderView,
  pub material_system: &'a MaterialSystem,
  pub model_render: &'a ModelRender,
  pub surface: &'a Surface,
  pub input_system: &'a InputSystem,
  pub model_info: &'a ModelInfo,
  pub studio_render: &'a StudioRender<'a>,
}

impl Interfaces<'_> {
  pub fn get() -> &'static Self {
    static INTERFACES: LazyLock<Interfaces> =
      LazyLock::new(|| Interfaces::find().expect("Failed to find interfaces"));
    &INTERFACES
  }

  fn find() -> windows::core::Result<Self> {
    let client = find_interface("client.dll", "VClient017")?;

    Ok(Self {
      client,
      client_mode: ok_or_empty_err!(unsafe { client_mode_from_client(client) })?,
      entity_list: find_interface("client.dll", "VClientEntityList003")?,
      engine: find_interface("engine.dll", "VEngineClient013")?,
      render_view: find_interface("engine.dll", "VEngineRenderView014")?,
      material_system: find_interface("MaterialSystem.dll", "VMaterialSystem080")?,
      model_render: find_interface("engine.dll", "VEngineModel016")?,
      surface: find_interface("vguimatsurface.dll", "VGUI_Surface030")?,
      input_system: find_interface("inputsystem.dll", "InputSystemVersion001")?,
      model_info: find_interface("engine.dll", "VModelInfoClient006")?,
      studio_render: find_interface("StudioRender.dll", "VStudioRender025")?,
    })
  }
}

fn find_interface<'a, T>(module_name: &str, interface_name: &str) -> windows::core::Result<&'a T> {
  unsafe {
    let module = GetModuleHandleA(pcstr!(module_name))?;

    let create_interface = GetProcAddress(module, pcstr!("CreateInterface"));
    let create_interface: extern "C" fn(*const c_char, *mut i32) -> *mut T =
      mem::transmute(create_interface);

    ok_or_empty_err!(create_interface(cstr!(interface_name), ptr::null_mut()).as_ref())
  }
}

unsafe fn client_mode_from_client(client: &Client) -> Option<&ClientMode> {
  let client_vtable = *(client as *const Client as *const *const *const c_void);
  (**(*client_vtable.add(10)).byte_add(5).cast::<*const *const ClientMode>()).as_ref()
}
