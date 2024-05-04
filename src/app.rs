use crate::game::Entity;
use crate::hooks::Hooks;
use crate::interfaces::Interfaces;
use crate::macros::s_to_cs;

use winapi::ctypes::c_void;
use winapi::shared::minwindef::HMODULE;
use winapi::um::handleapi::CloseHandle;
use winapi::um::libloaderapi::{DisableThreadLibraryCalls, FreeLibraryAndExitThread};
use winapi::um::processthreadsapi::CreateThread;
use winapi::um::utilapiset::Beep;
use winapi::um::winuser::FindWindowA;

use std::ptr;
use std::sync::{Mutex, MutexGuard, OnceLock};

pub struct App {
    module: HMODULE,
    pub hooks: Hooks,
    pub interfaces: Interfaces,
    pub local_player: Option<&'static Entity>,
}

impl App {
    pub fn create_and_setup(module: HMODULE) {
        unsafe { Self::get_or_init(Some(module)).lock().unwrap().setup() };
    }

    unsafe fn setup(&mut self) {
        DisableThreadLibraryCalls(self.module);
        self.hooks.hook_all();
        Beep(750, 200);
    }

    pub unsafe fn unload(&mut self) {
        self.hooks.unhook_all();

        let handle = CreateThread(
            ptr::null_mut(),
            0,
            Some(reset_state),
            self as *const App as _,
            0,
            ptr::null_mut(),
        );
        if !handle.is_null() {
            CloseHandle(handle);
        }
    }
}

unsafe extern "system" fn reset_state(app: *mut c_void) -> u32 {
    let app = app.cast::<App>().as_ref().unwrap();
    FreeLibraryAndExitThread(app.module, 0);
    0
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
        Self::get_or_init(None)
    }

    fn get_or_init(module: Option<HMODULE>) -> &'static Mutex<Self> {
        static APP: OnceLock<Mutex<App>> = OnceLock::new();
        APP.get_or_init(|| {
            let interfaces = unsafe { Interfaces::find() };
            let window = unsafe { FindWindowA(s_to_cs!("Valve001"), ptr::null_mut()) };

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
