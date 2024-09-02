use crate::game::client::{Client, ClientMode, EntityList};
use crate::game::engine::{Engine, ModelRender};
use crate::game::input_system::InputSystem;
use crate::game::material_system::MaterialSystem;
use crate::game::render_view::RenderView;
use crate::game::studio_render::StudioRender;
use crate::game::surface::Surface;

use dopamine_misc::{cstr, pcstr};

use windows::core::{Error as WindowsError, Result as WindowsResult};
use windows::Win32::System::LibraryLoader::{GetModuleHandleA, GetProcAddress};

use std::ffi::{c_char, c_void};
use std::sync::LazyLock;

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
  pub studio_render: &'a StudioRender<'a>,
}

impl Interfaces<'_> {
  pub fn get() -> &'static Self {
    static INTERFACES: LazyLock<Interfaces> = LazyLock::new(|| {
      Interfaces::find()
        .inspect_err(|err| log::error!("Failed to find interfaces: {}", err))
        .unwrap()
    });
    &INTERFACES
  }

  fn find() -> WindowsResult<Self> {
    let client = find_interface("client.dll", "VClient017")?;

    Ok(Self {
      client,
      client_mode: unsafe { client_mode_from_client(client) }.ok_or(WindowsError::empty())?,
      entity_list: find_interface("client.dll", "VClientEntityList003")?,
      engine: find_interface("engine.dll", "VEngineClient013")?,
      render_view: find_interface("engine.dll", "VEngineRenderView014")?,
      material_system: find_interface("MaterialSystem.dll", "VMaterialSystem080")?,
      model_render: find_interface("engine.dll", "VEngineModel016")?,
      surface: find_interface("vguimatsurface.dll", "VGUI_Surface030")?,
      input_system: find_interface("inputsystem.dll", "InputSystemVersion001")?,
      studio_render: find_interface("StudioRender.dll", "VStudioRender025")?,
    })
  }
}

fn find_interface<'a, T>(module_name: &str, interface_name: &str) -> WindowsResult<&'a T> {
  unsafe {
    let module = GetModuleHandleA(pcstr!(module_name))?;

    let create_interface = GetProcAddress(module, pcstr!("CreateInterface"));
    let create_interface: extern "C" fn(*const c_char, *mut i32) -> *mut T =
      std::mem::transmute(create_interface);

    create_interface(cstr!(interface_name), std::ptr::null_mut())
      .as_ref()
      .ok_or(WindowsError::empty())
  }
}

unsafe fn client_mode_from_client(client: &Client) -> Option<&ClientMode> {
  let client_vtable = *(client as *const Client as *const *const *const c_void);
  (**(*client_vtable.add(10)).byte_add(5).cast::<*const *const ClientMode>()).as_ref()
}
