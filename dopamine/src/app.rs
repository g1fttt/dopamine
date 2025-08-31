use crate::config::Config;
use crate::hooks::Hooks;
use crate::ui::{BlurEffect, Context as ImGuiContext, Menu};

use crate::features::FeatureContext;
use crate::features::chams::Chams;
use crate::features::glow::Glow;

use dopamine_sdk::Entity;
use dopamine_sdk::utils::Interfaces;

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
  pub blur_effect: Option<BlurEffect>,

  pub local_player: Option<&'s Entity>,
  pub player_resource: Option<&'s Entity>,

  pub background_imgui_context: OnceCell<ImGuiContext>,
  pub foreground_imgui_context: OnceCell<ImGuiContext>,

  pub glow: Glow<'s>,
  pub chams: Chams<'s>,
}

impl App<'_> {
  #[inline(always)]
  pub fn on_process_attach(module: HMODULE) -> WindowsResult<()> {
    unsafe { Self::get_mut_or_init(Some(module)).setup() }
  }

  fn setup(&mut self) -> WindowsResult<()> {
    unsafe {
      DisableThreadLibraryCalls(self.module)?;

      let _ =
        self.hooks.hook_all().inspect_err(|err| log::error!("Failed to setup hooks: {err:?}"));

      Beep(750, 200)
    }
  }

  pub fn on_process_detach() {
    let _ = App::get_mut()
      .config
      .save_to(Config::PATH)
      .inspect_err(|err| log::error!("Failed to write config: {err}"));
  }

  pub fn unload(&mut self) -> WindowsResult<()> {
    unsafe extern "system" fn free_library(app: *mut c_void) -> u32 {
      let app = unsafe { &mut *app.cast::<App>() };

      app.blur_effect.take();

      app.background_imgui_context.take();
      app.foreground_imgui_context.take();

      Interfaces::get().input_system.enable_input(true);

      unsafe {
        let _ = Beep(1500, 200);

        FreeLibraryAndExitThread(app.module, 0);
      }
    }

    unsafe {
      ShowCursor(true);

      let _ =
        self.hooks.unhook_all().inspect_err(|err| log::error!("Failed to remove hooks: {err:?}"));

      let handle = CreateThread(
        None,
        0,
        Some(free_library),
        Some(self as *mut App as *mut c_void),
        THREAD_CREATION_FLAGS::default(),
        None,
      )?;

      if !handle.is_invalid() {
        CloseHandle(handle)?;
      }
    }
    Ok(())
  }
}

impl App<'_> {
  #[inline(always)]
  pub fn capture_context<'a, T>(&self, config: &'a T) -> FeatureContext<'a, 'static, T> {
    FeatureContext::new(self, config)
  }
}

impl App<'_> {
  #[inline(always)]
  pub fn with_mut<T, F>(mut f: F) -> T
  where
    F: FnMut(&mut Self) -> T,
  {
    f(Self::get_mut())
  }

  #[inline(always)]
  fn get_mut() -> &'static mut Self {
    unsafe { Self::get_mut_or_init(None) }
  }

  unsafe fn get_mut_or_init(module: Option<HMODULE>) -> &'static mut Self {
    static mut APP: OnceCell<App> = OnceCell::new();

    unsafe {
      APP.get_mut_or_init(|| App {
        module: module.unwrap(),

        config: Config::create_and_load_from(Config::PATH),
        hooks: Hooks::create(),
        menu: Menu::new(),
        blur_effect: None,

        local_player: None,
        player_resource: None,

        background_imgui_context: OnceCell::new(),
        foreground_imgui_context: OnceCell::new(),

        glow: Glow::new(),
        chams: Chams::new(),
      })
    }
  }
}
