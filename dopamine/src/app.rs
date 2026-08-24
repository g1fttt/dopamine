use crate::config::Config;
use crate::hooks::Hooks;
use crate::ui::{BlurEffect, Context as ImGuiContext, Menu};

use crate::features::chams::Chams;
use crate::features::glow::Glow;

use presenceforge::DiscordIpcClient;
use windows::core::Result as WindowsResult;

use dopamine_sdk::Entity;
use dopamine_sdk::interfaces::{input_system, material_system};

use windows::Win32::Foundation::{CloseHandle, HMODULE};
use windows::Win32::System::Diagnostics::Debug::Beep;
use windows::Win32::System::LibraryLoader::{DisableThreadLibraryCalls, FreeLibraryAndExitThread};
use windows::Win32::System::Threading::{CreateThread, THREAD_CREATION_FLAGS};
use windows::Win32::UI::WindowsAndMessaging::ShowCursor;

use std::cell::OnceCell;
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
  pub imgui_context: OnceLock<ImGuiContext>,

  pub discord_ipc_client: OnceCell<DiscordIpcClient>,
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
        imgui_context: OnceLock::new(),

        discord_ipc_client: OnceCell::new(),
      })
    };
    app.setup()
  }

  pub fn on_process_detach() {
    if let Err(err) = App::get_mut().config.save_to(Config::PATH) {
      log::error!("Failed to write config: {err}");
    }

    unsafe { APP.take() };
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
    unsafe extern "system" fn free_library(module: *mut c_void) -> u32 {
      unsafe {
        ShowCursor(true);

        FreeLibraryAndExitThread(HMODULE(module), 0)
      }
    }

    if let Err(err) = unsafe { self.hooks.unhook_all() } {
      log::error!("Failed to remove hooks: {err:?}");
    }

    self.blur_effect.take();
    self.imgui_context.take();

    self.glow.dec_ref_counters();
    self.chams.dec_ref_counters();

    material_system().uncache_unused_materials(true);
    input_system().enable_input(true);

    unsafe {
      Beep(1500, 200)?;

      let handle = CreateThread(
        None,
        0,
        Some(free_library),
        Some(self.module.0),
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
