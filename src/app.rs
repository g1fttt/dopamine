use crate::game::Entity;
use crate::hooks::Hooks;
use crate::interfaces::Interfaces;
use crate::pcstr;

use windows::Win32::Foundation::{CloseHandle, HMODULE};
use windows::Win32::System::Diagnostics::Debug::Beep;
use windows::Win32::System::LibraryLoader::{DisableThreadLibraryCalls, FreeLibraryAndExitThread};
use windows::Win32::System::Threading::{CreateThread, THREAD_CREATION_FLAGS};
use windows::Win32::UI::WindowsAndMessaging::FindWindowA;

use std::ffi::c_void;
use std::sync::{Mutex, MutexGuard, OnceLock};

pub struct App {
    module: HMODULE,
    pub hooks: Hooks,
    pub interfaces: Interfaces,
    pub local_player: Option<&'static Entity>,
}

impl App {
    pub fn init_and_setup(module: HMODULE) -> windows::core::Result<()> {
        unsafe {
            Self::get_or_init(Some(module))
                .lock()
                .expect("Failed to lock `App` mutex")
                .setup()
        }
    }

    unsafe fn setup(&mut self) -> windows::core::Result<()> {
        DisableThreadLibraryCalls(self.module)?;

        self.hooks.hook_all()?;

        Beep(750, 200)
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
    /// Attempts to acquire `App` lock guard, then `f` closure is called.
    ///
    /// If the attempt was successful, `Some(T)` is returned. Otherwise, `None` is returned.
    pub fn with<T, F>(mut f: F) -> Option<T>
    where
        F: FnMut(&mut MutexGuard<Self>) -> T,
    {
        let guard = &mut Self::get().try_lock().ok()?;
        Some(f(guard))
    }

    #[inline(always)]
    pub fn get() -> &'static Mutex<Self> {
        unsafe { Self::get_or_init(None) }
    }

    unsafe fn get_or_init(module: Option<HMODULE>) -> &'static Mutex<Self> {
        static APP: OnceLock<Mutex<App>> = OnceLock::new();
        APP.get_or_init(|| {
            let interfaces = Interfaces::find().expect("Failed to find interfaces");
            let window = FindWindowA(pcstr!("Valve001"), pcstr!());

            Mutex::new(App {
                module: module.unwrap(),
                hooks: Hooks::create(&interfaces, window),
                interfaces,
                local_player: None,
            })
        })
    }
}

unsafe impl Sync for App {}
unsafe impl Send for App {}
