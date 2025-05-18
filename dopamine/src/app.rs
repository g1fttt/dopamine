use crate::config::Config;
use crate::hooks::Hooks;
use crate::ui::{ImGuiContext, Menu};

use crate::features::chams::Chams;
use crate::features::glow::Glow;
use crate::features::FeatureContext;

use dopamine_sdk::utils::Interfaces;
use dopamine_sdk::Entity;

use windows::Win32::Foundation::{CloseHandle, HMODULE};
use windows::Win32::System::Diagnostics::Debug::Beep;
use windows::Win32::System::LibraryLoader::{DisableThreadLibraryCalls, FreeLibraryAndExitThread};
use windows::Win32::System::Threading::{CreateThread, THREAD_CREATION_FLAGS};
use windows::Win32::UI::WindowsAndMessaging::ShowCursor;

use windows::core::Result as WindowsResult;

use std::cell::OnceCell;
use std::ffi::c_void;

pub struct App<'s: 'static> {
  module: HMODULE,

  pub config: Config,
  pub hooks: Hooks,
  pub menu: Menu,

  pub local_player: Option<&'s Entity>,
  pub player_resource: Option<&'s Entity>,

  pub glow: Glow<'s>,
  pub chams: Chams<'s>,
}

impl App<'_> {
  pub fn on_process_attach(module: HMODULE) -> WindowsResult<()> {
    unsafe { Self::get_mut_or_init(Some(module)).setup() }
  }

  unsafe fn setup(&mut self) -> WindowsResult<()> {
    DisableThreadLibraryCalls(self.module)?;

    let _ =
      self.hooks.hook_all().inspect_err(|err| log::error!("Failed to setup hooks: {:?}", err));

    Beep(750, 200)
  }

  pub fn on_process_detach() {
    Self::with(|app| {
      let _ = app
        .config
        .save_to(Config::PATH)
        .inspect_err(|err| log::error!("Failed to write config: {}", err));
    });
  }

  pub unsafe fn unload(&mut self) -> WindowsResult<()> {
    unsafe extern "system" fn free_library(app: *mut c_void) -> u32 {
      let _ = Beep(1500, 200);

      ImGuiContext::destroy();

      Interfaces::get().input_system.enable_input(true);

      let app = app.cast::<App>().as_ref_unchecked();
      FreeLibraryAndExitThread(app.module, 0);
    }

    ShowCursor(true);

    let _ =
      self.hooks.unhook_all().inspect_err(|err| log::error!("Failed to remove hooks: {:?}", err));

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

impl App<'_> {
  #[inline]
  pub fn capture_context<'a, T>(&self, config: &'a T) -> FeatureContext<'a, 'static, T> {
    FeatureContext::new(self, config)
  }
}

impl App<'_> {
  #[inline]
  pub fn with_mut<T, F>(mut f: F) -> T
  where
    F: FnMut(&mut Self) -> T,
  {
    f(Self::get_mut())
  }

  #[inline]
  pub fn with<T, F>(mut f: F) -> T
  where
    F: FnMut(&Self) -> T,
  {
    f(Self::get())
  }

  #[inline]
  fn get_mut() -> &'static mut Self {
    unsafe { Self::get_mut_or_init(None) }
  }

  #[inline]
  fn get() -> &'static Self {
    Self::get_mut()
  }

  unsafe fn get_mut_or_init(module: Option<HMODULE>) -> &'static mut Self {
    static mut APP: OnceCell<App> = OnceCell::new();

    APP.get_mut_or_init(|| App {
      module: module.unwrap(),

      config: Config::create_and_load_from(Config::PATH),
      hooks: Hooks::create(),
      menu: Menu::new(),

      local_player: None,
      player_resource: None,

      glow: Glow::new(),
      chams: Chams::new(),
    })
  }
}
