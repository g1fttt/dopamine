use crate::game::{Client, ClientMode, Engine, EntityList};
use crate::macros::{p_to_cs, s_to_cs};

use winapi::um::libloaderapi::{GetModuleHandleA, GetProcAddress};

use std::ffi::{c_char, c_void};
use std::path::Path;
use std::{mem, ptr};

pub struct Interfaces {
    pub client: &'static Client,
    pub client_mode: &'static ClientMode,
    pub entity_list: &'static EntityList,
    pub engine: &'static Engine,
}

impl Interfaces {
    pub unsafe fn find() -> Self {
        let client = find_interface("client.dll", "VClient017");
        Self {
            client,
            client_mode: client_mode_from_client(client),
            entity_list: find_interface("client.dll", "VClientEntityList003"),
            engine: find_interface("engine.dll", "VEngineClient013"),
        }
    }
}

unsafe fn find_interface<T, P>(module_path: P, interface_name: &str) -> &T
where
    P: AsRef<Path>,
{
    let module = GetModuleHandleA(p_to_cs!(module_path));

    let create_interface = GetProcAddress(module, s_to_cs!("CreateInterface"));

    type CreateInterfaceFn<T> = extern "C" fn(*const c_char, *mut i32) -> *mut T;
    let create_interface = mem::transmute::<_, CreateInterfaceFn<T>>(create_interface);

    let interface = create_interface(s_to_cs!(interface_name), ptr::null_mut());
    interface.as_ref().unwrap()
}

unsafe fn client_mode_from_client(client: &Client) -> &ClientMode {
    let client_vtable = *(client as *const Client).cast::<*const *const c_void>();
    (**(*client_vtable.add(10))
        .byte_add(5)
        .cast::<*const *const ClientMode>())
    .as_ref()
    .unwrap()
}
