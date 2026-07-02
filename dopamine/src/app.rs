use crate::config::Config;
use crate::hooks::Hooks;
use crate::ui::{BlurEffect, Context as ImGuiContext, Menu};

use crate::features::chams::Chams;
use crate::features::glow::Glow;

use bumpalo::Bump;

use windows::core::Result as WindowsResult;

use dopamine_sdk::Entity;
use dopamine_sdk::interfaces::{input_system, material_system};

use windows::Win32::Foundation::{CloseHandle, HMODULE};
use windows::Win32::System::Diagnostics::Debug::Beep;
use windows::Win32::System::LibraryLoader::{DisableThreadLibraryCalls, FreeLibraryAndExitThread};
use windows::Win32::System::Threading::{CreateThread, THREAD_CREATION_FLAGS};
use windows::Win32::UI::WindowsAndMessaging::ShowCursor;

use std::ffi::c_void;
use std::sync::OnceLock;

static mut APP: OnceLock<App> = OnceLock::new();

pub struct App<'s: 'static> {
  module: HMODULE,

  pub config: Config,
  pub hooks: Hooks,
  pub menu: Menu,
  pub glow: Glow<'s>,
  pub chams: Chams<'s>,

  pub player_resource: Option<&'s Entity>,

  pub blur_effect: OnceLock<BlurEffect>,
  pub background_imgui_context: OnceLock<ImGuiContext>,
  pub foreground_imgui_context: OnceLock<ImGuiContext>,

  #[allow(dead_code, reason = "App has to hold this in order to make a graceful cleanup on drop")]
  pub bump: Bump,
}

impl App<'_> {
  pub fn on_process_attach(module: HMODULE) -> WindowsResult<()> {
    let app = unsafe {
      APP.get_mut_or_init(|| {
        let bump = Bump::new();

        App {
          module,

          config: Config::create_and_load_from(Config::PATH),
          hooks: Hooks::create(),
          menu: Menu::new(),
          glow: Glow::new(&bump),
          chams: Chams::new(&bump),

          player_resource: None,

          blur_effect: OnceLock::new(),
          background_imgui_context: OnceLock::new(),
          foreground_imgui_context: OnceLock::new(),

          bump,
        }
      })
    };
    app.setup()
  }

  pub fn on_process_detach() {
    if let Err(err) = App::get_mut().config.save_to(Config::PATH) {
      log::error!("Failed to write config: {err}");
    }
  }

  fn setup(&mut self) -> WindowsResult<()> {
    unsafe {
      DisableThreadLibraryCalls(self.module)?;

      if let Err(err) = self.hooks.hook_all() {
        log::error!("Failed to setup hooks: {err:?}");
      }

      Beep(750, 200)
    }
  }

  pub fn unload(&mut self) -> WindowsResult<()> {
    unsafe extern "system" fn free_library(app: *mut c_void) -> u32 {
      let app = unsafe { &mut *app.cast::<App>() };

      app.blur_effect.take();

      app.background_imgui_context.take();
      app.foreground_imgui_context.take();

      app.bump.reset();

      input_system().enable_input(true);

      unsafe {
        let _ = Beep(1500, 200);

        FreeLibraryAndExitThread(app.module, 0);
      }
    }

    unsafe {
      ShowCursor(true);

      if let Err(err) = self.hooks.unhook_all() {
        log::error!("Failed to remove hooks: {err:?}");
      }

      self.glow.dec_ref_counters();
      self.chams.dec_ref_counters();

      material_system().uncache_unused_materials(true);

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

impl<'s: 'static> App<'s> {
  pub fn with_mut<T, F>(mut f: F) -> T
  where
    F: FnMut(&mut Self) -> T,
  {
    f(Self::get_mut())
  }

  fn get_mut() -> &'s mut Self {
    unsafe { APP.get_mut().unwrap() }
  }
}

unsafe impl Send for App<'_> {}
unsafe impl Sync for App<'_> {}
