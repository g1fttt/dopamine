use crate::config::Config;
use crate::game::Entity;
use crate::hooks::Hooks;
use crate::interfaces::Interfaces;
use crate::netvar_manager::NetvarManager;

use windows::Win32::Foundation::{CloseHandle, HMODULE};
use windows::Win32::System::Diagnostics::Debug::Beep;
use windows::Win32::System::LibraryLoader::{DisableThreadLibraryCalls, FreeLibraryAndExitThread};
use windows::Win32::System::Threading::{CreateThread, THREAD_CREATION_FLAGS};

use std::cell::OnceCell;
use std::ffi::c_void;

pub struct App {
    module: HMODULE,
    pub config: Config,
    pub hooks: Hooks,
    pub netvar_manager: NetvarManager<'static>,
    pub interfaces: Interfaces<'static>,
    pub local_player: Option<&'static Entity>,
}

impl App {
    pub fn init_and_setup(module: HMODULE) -> windows::core::Result<()> {
        unsafe { Self::get_mut_or_init(Some(module)).setup() }
    }

    unsafe fn setup(&mut self) -> windows::core::Result<()> {
        DisableThreadLibraryCalls(self.module)?;

        self.hooks.hook_all()?;

        Beep(750, 200)
    }

    pub fn make_final_config_save() {
        Self::with(|app| {
            app.config
                .save_to(Config::PATH)
                .expect("Failed to write config");
        });
    }

    pub unsafe fn unload(&mut self) -> windows::core::Result<()> {
        self.hooks.unhook_all()?;

        let handle = CreateThread(
            None,
            0,
            Some(free_library),
            Some(self as *const App as _),
            THREAD_CREATION_FLAGS::default(),
            None,
        )?;

        if !handle.is_invalid() {
            CloseHandle(handle)?;
        }
        Ok(())
    }
}

unsafe extern "system" fn free_library(app: *mut c_void) -> u32 {
    Beep(1500, 200).expect("Failed to make beep sound upon unhooking");

    let app = app.cast::<App>().as_ref().unwrap();
    FreeLibraryAndExitThread(app.module, 0);
}

impl App {
    pub fn netvar_offset(class_name: &str, prop_name: &str) -> Option<usize> {
        Self::with(move |app| {
            app.netvar_manager
                .offsets
                .get(&(class_name, prop_name))
                .cloned()
        })
    }
}

impl App {
    pub fn with_mut<T, F>(mut f: F) -> T
    where
        F: FnMut(&mut Self) -> T,
    {
        f(Self::get_mut())
    }

    pub fn with<T, F>(mut f: F) -> T
    where
        F: FnMut(&Self) -> T,
    {
        f(Self::get_mut())
    }

    #[inline(always)]
    fn get_mut() -> &'static mut Self {
        unsafe { Self::get_mut_or_init(None) }
    }

    unsafe fn get_mut_or_init(module: Option<HMODULE>) -> &'static mut Self {
        static mut APP: OnceCell<App> = OnceCell::new();
        APP.get_mut_or_init(|| {
            let interfaces = Interfaces::find().expect("Failed to find interfaces");

            App {
                module: module.unwrap(),
                config: Config::create_and_load_from(Config::PATH),
                hooks: Hooks::create(&interfaces),
                netvar_manager: NetvarManager::precache(&interfaces),
                interfaces,
                local_player: None,
            }
        })
    }
}
