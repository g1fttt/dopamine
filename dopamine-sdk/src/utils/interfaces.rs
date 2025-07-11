use crate::game::client::{Client, ClientMode, EntityList};
use crate::game::engine::{Engine, ModelRender};
use crate::game::input_system::InputSystem;
use crate::game::material_system::MaterialSystem;
use crate::game::render_view::RenderView;
use crate::game::server::Server;
use crate::game::studio_render::StudioRender;
use crate::game::surface::Surface;

use crate::utils::rip_offset_value;
use crate::{cstr, pcstr};

use windows::Win32::System::LibraryLoader::{GetModuleHandleA, GetProcAddress};
use windows::core::{Error as WindowsError, Result as WindowsResult};

use std::ffi::{c_char, c_void};
use std::sync::LazyLock;

pub struct Interfaces<'a> {
  pub client: &'a Client,
  pub server: &'a Server,
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
    static INTERFACES: LazyLock<Interfaces> = LazyLock::new(Interfaces::find);
    &INTERFACES
  }

  fn find() -> Self {
    let client = find_interface("client.dll", "VClient017");

    Self {
      client,
      server: find_interface("server.dll", "PlayerInfoManager002"),
      client_mode: unsafe { client_mode_from_client(client) },
      entity_list: find_interface("client.dll", "VClientEntityList003"),
      engine: find_interface("engine.dll", "VEngineClient013"),
      render_view: find_interface("engine.dll", "VEngineRenderView014"),
      material_system: find_interface("MaterialSystem.dll", "VMaterialSystem080"),
      model_render: find_interface("engine.dll", "VEngineModel016"),
      surface: find_interface("vguimatsurface.dll", "VGUI_Surface030"),
      input_system: find_interface("inputsystem.dll", "InputSystemVersion001"),
      studio_render: find_interface("StudioRender.dll", "VStudioRender025"),
    }
  }
}

fn find_interface<'a, T>(module_name: &str, interface_name: &str) -> &'a T {
  unsafe {
    let module = GetModuleHandleA(pcstr!(module_name))
      .inspect_err(|err| log::error!("Failed to get handle for {module_name}: {err}"))
      .unwrap();

    let create_interface = GetProcAddress(module, pcstr!("CreateInterface"));
    let create_interface: extern "C" fn(*const c_char, *mut i32) -> *mut T =
      std::mem::transmute(create_interface);

    let interface = create_interface(cstr!(interface_name), std::ptr::null_mut()).as_ref();

    match interface {
      Some(int) => int,
      None => {
        log::error!("Failed to find {interface_name} in {module_name}");
        panic!();
      }
    }
  }
}

unsafe fn client_mode_from_client(client: &Client) -> &ClientMode {
  unsafe {
    let client_vtable = *(client as *const Client as *const *const *const c_void);
    let client_mode = rip_offset_value((*client_vtable.add(10)).cast_mut());
    &*client_mode.cast::<ClientMode>()
  }
}
