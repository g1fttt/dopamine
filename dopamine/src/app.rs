use crate::config::Config;
use crate::hooks::Hooks;
use crate::ui::{BlurEffect, Context as ImGuiContext, Menu};

use crate::features::chams::Chams;
use crate::features::glow::Glow;

use dopamine_sdk::interfaces::input_system;
use dopamine_sdk::Entity;

use windows::Win32::Foundation::{CloseHandle, HMODULE};
use windows::Win32::System::Diagnostics::Debug::Beep;
use windows::Win32::System::LibraryLoader::{DisableThreadLibraryCalls, FreeLibraryAndExitThread};
use windows::Win32::System::Threading::{CreateThread, THREAD_CREATION_FLAGS};
use windows::Win32::UI::WindowsAndMessaging::ShowCursor;

use windows::core::Result as WindowsResult;

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
}

impl App<'_> {
  pub fn on_process_attach(module: HMODULE) -> WindowsResult<()> {
    let app = unsafe {
      APP.get_mut_or_init(|| App {
        module,

        config: Config::create_and_load_from(Config::PATH),
        hooks: Hooks::create(),
        menu: Menu::new(),
        glow: Glow::new(),
        chams: Chams::new(),

        player_resource: None,

        blur_effect: OnceLock::new(),
        background_imgui_context: OnceLock::new(),
        foreground_imgui_context: OnceLock::new(),
      })
    };
    app.setup()
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

      input_system().enable_input(true);

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
