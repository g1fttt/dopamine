pub use easy_imgui_sys::ImVec2;

use bitflags::bitflags;
use derive_builder::Builder;
use easy_imgui_sys::*;

use std::ffi::{CString, c_char, c_int, c_void};
use std::{mem, ptr};

#[doc(alias = "ImGuiContext")]
#[repr(transparent)]
pub struct Context(*mut ImGuiContext);

impl Context {
  #[doc(alias = "CreateContext")]
  #[inline(always)]
  pub fn new() -> Self {
    Self(unsafe { ImGui_CreateContext(ptr::null_mut()) })
  }

  #[doc(alias = "SetCurrentContext")]
  #[inline(always)]
  pub fn set_current(&self) {
    unsafe { ImGui_SetCurrentContext(self.0) };
  }

  #[doc(alias = "NewFrame")]
  #[inline(always)]
  pub fn new_frame(&self) -> Frame {
    Frame::new()
  }

  #[doc(alias = "Render")]
  #[inline(always)]
  pub fn render(&self) {
    unsafe { ImGui_Render() };
  }
}

impl Default for Context {
  #[inline(always)]
  fn default() -> Self {
    Self::new()
  }
}

impl Drop for Context {
  #[inline(always)]
  fn drop(&mut self) {
    unsafe { ImGui_DestroyContext(self.0) };
  }
}

pub struct Frame;

impl Frame {
  #[inline(always)]
  fn new() -> Self {
    unsafe { ImGui_NewFrame() };

    Self
  }

  #[doc(alias = "EndFrame")]
  #[inline(always)]
  pub fn end(self) {}
}

impl Drop for Frame {
  #[doc(alias = "EndFrame")]
  #[inline(always)]
  fn drop(&mut self) {
    unsafe { ImGui_EndFrame() };
  }
}

#[doc(alias = "ShowDemoWindow")]
#[inline(always)]
pub fn show_demo_window() {
  unsafe { ImGui_ShowDemoWindow(ptr::null_mut()) };
}

#[doc(alias = "GetDrawData")]
#[inline(always)]
pub fn draw_data() -> &'static DrawData {
  unsafe { &*ImGui_GetDrawData().cast::<DrawData>() }
}

#[doc(alias = "ImDrawData")]
#[repr(C)]
pub struct DrawData {
  pub valid: bool,
  pub cmd_lists_count: i32,
  pub total_idx_count: i32,
  pub total_vtx_count: i32,
  cmd_lists: ImVector<*mut DrawList>,
  pub display_pos: ImVec2,
  pub display_size: ImVec2,
  pub framebuffer_scale: ImVec2,
  pub owner_viewport: Option<&'static Viewport>,
  textures: *mut ImVector<*mut TextureData>,
}

impl DrawData {
  pub fn cmd_lists(&self) -> impl Iterator<Item = &mut DrawList> {
    unsafe { (*self.cmd_lists).iter().map(|cl| &mut **cl) }
  }

  pub fn textures(&self) -> impl Iterator<Item = &mut TextureData> {
    unsafe { (*self.textures).iter().map(|tx| &mut **tx) }
  }

  pub fn textures_size(&self) -> usize {
    unsafe { (*self.textures).Size as usize }
  }
}

// TODO: All fields
#[doc(alias = "ImTextureData")]
#[repr(C)]
pub struct TextureData {
  unique_id: i32,
  pub status: TextureStatus,
  backend_user_data: *mut c_void,
  pub tex_id: TextureID,
  pub format: TextureFormat,
  pub width: i32,
  pub height: i32,
  pub bytes_per_pixel: i32,
  // TODO: Iterator
  pub pixels: *mut u8,
  used_rect: TextureRect,
  pub update_rect: TextureRect,
  updates: ImVector<TextureRect>,
}

impl TextureData {
  #[inline(always)]
  pub fn updates(&self) -> impl Iterator<Item = &TextureRect> {
    self.updates.iter()
  }
}

#[doc(alias = "ImTextureStatus")]
#[derive(PartialEq)]
#[repr(C)]
pub enum TextureStatus {
  Ok,
  Destroyed,
  WantCreate,
  WantUpdates,
  WantDestroy,
}

// TODO: All fields
#[doc(alias = "ImGuiViewport")]
#[repr(C)]
pub struct Viewport;

#[doc(alias = "StyleColorsDark")]
#[inline(always)]
pub fn style_colors_dark() {
  unsafe { ImGui_StyleColorsDark(ptr::null_mut()) };
}

#[doc(alias = "Text")]
#[inline]
pub fn text<S: AsRef<str>>(fmt: S) {
  let s = CString::new(fmt.as_ref()).unwrap();

  unsafe { ImGui_Text(s.as_ptr()) };
}

#[doc(alias = "Separator")]
#[inline(always)]
pub fn separator() {
  unsafe { ImGui_Separator() };
}

#[doc(alias = "Checkbox")]
#[inline]
pub fn checkbox(label: &str, v: &mut bool) -> bool {
  let label = CString::new(label).unwrap();

  unsafe { ImGui_Checkbox(label.as_ptr(), v) }
}

#[doc(alias = "Begin")]
#[inline(always)]
pub fn window(name: &str, f: impl FnMut()) {
  window_ex(name).build(f);
}

#[doc(alias = "Begin")]
#[inline(always)]
pub fn window_ex<'a>(name: &'a str) -> WindowBuilder<'a> {
  WindowBuilder::create_empty().name(name)
}

#[derive(Builder)]
#[builder(pattern = "owned", build_fn(private, name = "_build"))]
pub struct Window<'a> {
  name: &'a str,
  #[builder(setter(strip_option), default)]
  open: Option<&'a mut bool>,
  #[builder(setter(strip_option), default)]
  flags: Option<WindowFlags>,
}

impl<'a> WindowBuilder<'a> {
  pub fn build(self, mut f: impl FnMut()) {
    let window = self._build().unwrap();

    let name = CString::new(window.name).unwrap();
    let open = window.open.map(|b| b as *mut bool).unwrap_or(ptr::null_mut());
    let flags = window.flags.unwrap_or_else(WindowFlags::empty);

    unsafe {
      ImGui_Begin(name.as_ptr(), open, flags.bits());
      f();
      ImGui_End();
    }
  }
}

bitflags! {
  #[doc(alias = "ImGuiWindowFlags")]
  #[repr(transparent)]
  pub struct WindowFlags: ImGuiWindowFlags {
    const NO_RESIZE = ImGuiWindowFlags_::ImGuiWindowFlags_NoResize.0;
    const NO_SCROLLBAR = ImGuiWindowFlags_::ImGuiWindowFlags_NoScrollbar.0;
    const ALWAYS_AUTO_RESIZE = ImGuiWindowFlags_::ImGuiWindowFlags_AlwaysAutoResize.0;
  }
}

#[doc(alias = "SliderFloat")]
#[inline(always)]
pub fn slider_float(label: &str, v: &mut f32, min: f32, max: f32) -> bool {
  slider_float_ex(label, v, min, max).build()
}

#[doc(alias = "SliderFloat")]
#[inline(always)]
pub fn slider_float_ex<'a>(
  label: &'a str,
  v: &'a mut f32,
  min: f32,
  max: f32,
) -> SliderFloatBuilder<'a> {
  SliderFloatBuilder::create_empty().label(label).value(v).min(min).max(max)
}

#[derive(Builder)]
#[builder(pattern = "owned", build_fn(private, name = "_build"))]
pub struct SliderFloat<'a> {
  label: &'a str,
  value: &'a mut f32,
  min: f32,
  max: f32,
  #[builder(setter(strip_option), default)]
  format: Option<&'a str>,
  #[builder(setter(strip_option), default)]
  flags: Option<SliderFlags>,
}

impl<'a> SliderFloatBuilder<'a> {
  pub fn build(self) -> bool {
    let slider_float = self._build().unwrap();

    let label = CString::new(slider_float.label).unwrap();
    let format = CString::new(slider_float.format.unwrap_or("%.3f")).unwrap();
    let flags = slider_float.flags.unwrap_or_else(SliderFlags::empty);

    unsafe {
      ImGui_SliderFloat(
        label.as_ptr(),
        slider_float.value,
        slider_float.min,
        slider_float.max,
        format.as_ptr(),
        flags.bits(),
      )
    }
  }
}

#[doc(alias = "SliderInt")]
#[inline(always)]
pub fn slider_int(label: &str, v: &mut i32, min: i32, max: i32) -> bool {
  slider_int_ex(label, v, min, max).build()
}

#[doc(alias = "SliderInt")]
#[inline(always)]
pub fn slider_int_ex<'a>(
  label: &'a str,
  v: &'a mut i32,
  min: i32,
  max: i32,
) -> SliderIntBuilder<'a> {
  SliderIntBuilder::create_empty().label(label).value(v).min(min).max(max)
}

#[derive(Builder)]
#[builder(pattern = "owned", build_fn(private, name = "_build"))]
pub struct SliderInt<'a> {
  label: &'a str,
  value: &'a mut i32,
  min: i32,
  max: i32,
  #[builder(setter(strip_option), default)]
  format: Option<&'a str>,
  #[builder(setter(strip_option), default)]
  flags: Option<SliderFlags>,
}

impl<'a> SliderIntBuilder<'a> {
  pub fn build(self) -> bool {
    let slider_int = self._build().unwrap();

    let label = CString::new(slider_int.label).unwrap();
    let format = CString::new(slider_int.format.unwrap_or("%d")).unwrap();
    let flags = slider_int.flags.unwrap_or_else(SliderFlags::empty);

    unsafe {
      ImGui_SliderInt(
        label.as_ptr(),
        slider_int.value,
        slider_int.min,
        slider_int.max,
        format.as_ptr(),
        flags.bits(),
      )
    }
  }
}

bitflags! {
  #[doc(alias = "ImGuiSliderFlags")]
  #[repr(transparent)]
  pub struct SliderFlags: ImGuiSliderFlags {}
}

#[doc(alias = "SameLine")]
#[inline(always)]
pub fn same_line() {
  same_line_ex().build();
}

#[doc(alias = "SameLine")]
#[inline(always)]
pub fn same_line_ex() -> SameLineBuilder {
  SameLineBuilder::create_empty()
}

#[derive(Builder)]
#[builder(pattern = "owned", build_fn(private, name = "_build"))]
pub struct SameLine {
  #[builder(setter(strip_option), default)]
  offset_from_start_x: Option<f32>,
  #[builder(setter(strip_option), default)]
  spacing: Option<f32>,
}

impl SameLineBuilder {
  pub fn build(self) {
    let same_line = self._build().unwrap();

    let offset_from_start_x = same_line.offset_from_start_x.unwrap_or(0.0);
    let spacing = same_line.spacing.unwrap_or(-1.0);

    unsafe { ImGui_SameLine(offset_from_start_x, spacing) };
  }
}

#[doc(alias = "Button")]
#[inline(always)]
pub fn button(label: &str) -> bool {
  button_ex(label).build()
}

#[doc(alias = "Button")]
#[inline(always)]
pub fn button_ex<'a>(label: &'a str) -> ButtonBuilder<'a> {
  ButtonBuilder::create_empty().label(label)
}

#[derive(Builder)]
#[builder(pattern = "owned", build_fn(private, name = "_build"))]
pub struct Button<'a> {
  label: &'a str,
  #[builder(setter(strip_option), default)]
  size: Option<ImVec2>,
}

impl<'a> ButtonBuilder<'a> {
  pub fn build(self) -> bool {
    let button = self._build().unwrap();

    let label = CString::new(button.label).unwrap();
    let size = button.size.unwrap_or(ImVec2 { x: 0.0, y: 0.0 });

    unsafe { ImGui_Button(label.as_ptr(), &size) }
  }
}

#[doc(alias = "Combo")]
#[inline(always)]
pub fn combo(label: &str, current_item: &mut usize, items: &[&str]) -> bool {
  combo_ex(label, current_item, items).build()
}

#[doc(alias = "Combo")]
#[inline(always)]
pub fn combo_ex<'a>(
  label: &'a str,
  current_item: &'a mut usize,
  items: &'a [&'a str],
) -> ComboBuilder<'a> {
  ComboBuilder::create_empty().label(label).current_item(current_item).items(items)
}

#[derive(Builder)]
#[builder(pattern = "owned", build_fn(private, name = "_build"))]
pub struct Combo<'a> {
  label: &'a str,
  current_item: &'a mut usize,
  items: &'a [&'a str],
  #[builder(setter(strip_option), default)]
  popup_max_height_in_items: Option<i32>,
}

impl<'a> ComboBuilder<'a> {
  pub fn build(self) -> bool {
    let combo = self._build().unwrap();

    let label = CString::new(combo.label).unwrap();
    let current_item = combo.current_item as *mut usize;
    let items: Vec<*const c_char> =
      combo.items.iter().map(|&s| CString::new(s).unwrap().into_raw() as *const c_char).collect();
    let popup_max_height_in_items = combo.popup_max_height_in_items.unwrap_or(-1);

    let result = unsafe {
      ImGui_Combo(
        label.as_ptr(),
        current_item as *mut i32,
        items.as_ptr(),
        items.len() as i32,
        popup_max_height_in_items,
      )
    };

    for item in items {
      unsafe { mem::drop(CString::from_raw(item.cast_mut())) };
    }

    result
  }
}

#[doc(alias = "ColorEdit4")]
#[inline(always)]
pub fn color_edit4(label: &str, color: &mut [f32; 4]) -> bool {
  color_edit4_ex(label, color).build()
}

#[doc(alias = "ColorEdit4")]
#[inline(always)]
pub fn color_edit4_ex<'a>(label: &'a str, color: &'a mut [f32; 4]) -> ColorEdit4Builder<'a> {
  ColorEdit4Builder::create_empty().label(label).color(color)
}

#[derive(Builder)]
#[builder(pattern = "owned", build_fn(private, name = "_build"))]
pub struct ColorEdit4<'a> {
  label: &'a str,
  color: &'a mut [f32; 4],
  #[builder(setter(strip_option), default)]
  flags: Option<ColorEditFlags>,
}

impl<'a> ColorEdit4Builder<'a> {
  pub fn build(self) -> bool {
    let color_edit4 = self._build().unwrap();

    let label = CString::new(color_edit4.label).unwrap();
    let flags = color_edit4.flags.unwrap_or_else(ColorEditFlags::empty);

    unsafe { ImGui_ColorEdit4(label.as_ptr(), color_edit4.color.as_mut_ptr(), flags.bits()) }
  }
}

bitflags! {
  #[doc(alias = "ImGuiColorEditFlags")]
  #[repr(transparent)]
  pub struct ColorEditFlags: ImGuiColorEditFlags {
    const ALPHA_BAR = ImGuiColorEditFlags_::ImGuiColorEditFlags_AlphaBar.0;
    const NO_INPUTS = ImGuiColorEditFlags_::ImGuiColorEditFlags_NoInputs.0;
  }
}

#[doc(alias = "PushStyleVar")]
pub fn push_style_var(kind: StyleVarKind) -> StyleVar {
  let (variant, value) = match kind {
    StyleVarKind::Alpha(a) => (ImGuiStyleVar_::ImGuiStyleVar_Alpha, a),
  };

  unsafe { ImGui_PushStyleVar(variant.0, value) };

  StyleVar
}

pub struct StyleVar;

impl StyleVar {
  #[doc(alias = "PopStyleVar")]
  #[inline(always)]
  pub fn pop(self) {}
}

impl Drop for StyleVar {
  #[inline(always)]
  fn drop(&mut self) {
    unsafe { ImGui_PopStyleVar(1) };
  }
}

#[doc(alias = "ImGuiStyleVar")]
pub enum StyleVarKind {
  Alpha(f32),
}

#[doc(alias = "BeginMainMenuBar")]
#[inline(always)]
pub fn main_menu_bar() -> Option<MainMenuBar> {
  unsafe { ImGui_BeginMainMenuBar() }.then(|| MainMenuBar)
}

pub struct MainMenuBar;

impl MainMenuBar {
  #[doc(alias = "EndMainMenuBar")]
  #[inline(always)]
  pub fn end(self) {}
}

impl Drop for MainMenuBar {
  #[doc(alias = "EndMainMenuBar")]
  #[inline(always)]
  fn drop(&mut self) {
    unsafe { ImGui_EndMainMenuBar() };
  }
}

#[doc(alias = "MenuItem")]
#[inline(always)]
pub fn menu_item(label: &str) -> bool {
  menu_item_ex(label).build()
}

#[doc(alias = "MenuItem")]
#[inline(always)]
pub fn menu_item_ex<'a>(label: &'a str) -> MenuItemBuilder<'a> {
  MenuItemBuilder::create_empty().label(label)
}

#[derive(Builder)]
#[builder(pattern = "owned", build_fn(private, name = "_build"))]
pub struct MenuItem<'a> {
  label: &'a str,
  #[builder(setter(strip_option), default)]
  shortcut: Option<&'a str>,
  #[builder(setter(strip_option), default)]
  selected: Option<bool>,
  #[builder(setter(strip_option), default)]
  enabled: Option<bool>,
}

impl<'a> MenuItemBuilder<'a> {
  pub fn build(self) -> bool {
    let menu_item = self._build().unwrap();

    let label = CString::new(menu_item.label).unwrap();
    let shortcut =
      menu_item.shortcut.map(|s| CString::new(s).unwrap().into_raw()).unwrap_or(ptr::null_mut());
    let selected = menu_item.selected.unwrap_or(false);
    let enabled = menu_item.enabled.unwrap_or(true);

    let result =
      unsafe { ImGui_MenuItem(label.as_ptr(), shortcut.cast_const(), selected, enabled) };

    if !shortcut.is_null() {
      unsafe { mem::drop(CString::from_raw(shortcut)) };
    }

    result
  }
}

#[doc(alias = "GetIO")]
#[inline(always)]
pub fn io() -> &'static Io {
  io_mut()
}

#[doc(alias = "GetIO")]
#[inline(always)]
pub fn io_mut() -> &'static mut Io {
  unsafe { &mut *ImGui_GetIO().cast::<Io>() }
}

#[doc(alias = "ImGuiIO")]
#[repr(C)]
pub struct Io {
  pub config_flags: ConfigFlags,
  pub backend_flags: BackendFlags,
  pub display_size: ImVec2,
  display_framebuffer_scale: ImVec2,
  pub delta_time: f32,
  ini_saving_rate: f32,
  ini_filename: *const c_char,
  log_filename: *const c_char,
  user_data: *mut c_void,
  fonts: *mut ImFontAtlas,
  font_default: *mut ImFont,
  font_allow_user_scaling: bool,
  config_nav_swap_gamepad_buttons: bool,
  config_nav_move_set_mouse_pos: bool,
  config_nav_capture_keyboard: bool,
  config_nav_escape_clear_focus_item: bool,
  config_nav_escape_clear_focus_window: bool,
  config_nav_cursor_visible_auto: bool,
  config_nav_cursor_visible_always: bool,
  config_docking_no_split: bool,
  config_docking_no_docking_over: bool,
  config_docking_with_shift: bool,
  config_docking_always_tab_bar: bool,
  config_docking_transparent_payload: bool,
  config_viewports_no_auto_merge: bool,
  config_viewports_no_task_bar_icon: bool,
  config_viewports_no_decoration: bool,
  config_viewports_no_default_parent: bool,
  config_viewports_platform_focus_sets_imgui_focus: bool,
  config_dpi_scale_fonts: bool,
  config_dpi_scale_viewports: bool,
  pub mouse_draw_cursor: bool,
  config_mac_os_x_behaviors: bool,
  config_input_trickle_event_queue: bool,
  config_input_text_cursor_blink: bool,
  config_input_text_enter_keep_active: bool,
  config_drag_click_to_input_text: bool,
  config_windows_resize_from_edges: bool,
  config_windows_move_from_title_bar_only: bool,
  config_windows_copy_contents_with_ctrl_c: bool,
  config_scrollbar_scroll_by_page: bool,
  config_memory_compact_timer: f32,
  mouse_double_click_time: f32,
  mouse_double_click_max_dist: f32,
  mouse_drag_threshold: f32,
  pub key_repeat_delay: f32,
  pub key_repeat_rate: f32,
  config_error_recovery: bool,
  config_error_recovery_enable_assert: bool,
  config_error_recovery_enable_debug_log: bool,
  config_error_recovery_enable_tooltip: bool,
  config_debug_is_debugger_present: bool,
  config_debug_highlight_id_conflicts: bool,
  config_debug_highlight_id_conflicts_show_item_picker: bool,
  config_debug_begin_return_value_once: bool,
  config_debug_begin_return_value_loop: bool,
  config_debug_ignore_focus_loss: bool,
  config_debug_ini_settings: bool,
  backend_platform_name: *const c_char,
  backend_renderer_name: *const c_char,
  backend_platform_user_data: *mut c_void,
  backend_renderer_user_data: *mut c_void,
  backend_language_user_data: *mut c_void,
  want_capture_mouse: bool,
  want_capture_keyboard: bool,
  want_text_input: bool,
  pub want_set_mouse_pos: bool,
  want_save_ini_settings: bool,
  nav_active: bool,
  nav_visible: bool,
  framerate: f32,
  metrics_render_vertices: c_int,
  metrics_render_indices: c_int,
  metrics_render_windows: c_int,
  metrics_active_windows: c_int,
  mouse_delta: ImVec2,
  ctx: *mut ImGuiContext,
  pub mouse_pos: ImVec2,
  mouse_down: [bool; 5],
  mouse_wheel: f32,
  mouse_wheel_h: f32,
  mouse_source: ImGuiMouseSource,
  mouse_hovered_viewport: ImGuiID,
  key_ctrl: bool,
  key_shift: bool,
  key_alt: bool,
  key_super: bool,
  key_mods: ImGuiKeyChord,
  keys_data: [ImGuiKeyData; 155],
  want_capture_mouse_unless_popup_close: bool,
  mouse_pos_prev: ImVec2,
  mouse_clicked_pos: [ImVec2; 5],
  mouse_clicked_time: [f64; 5],
  mouse_clicked: [bool; 5],
  mouse_double_clicked: [bool; 5],
  mouse_clicked_count: [ImU16; 5],
  mouse_clicked_last_count: [ImU16; 5],
  mouse_released: [bool; 5],
  mouse_released_time: [f64; 5],
  mouse_down_owned: [bool; 5],
  mouse_down_owned_unless_popup_close: [bool; 5],
  mouse_wheel_request_axis_swap: bool,
  mouse_ctrl_left_as_right_click: bool,
  mouse_down_duration: [f32; 5],
  mouse_down_duration_prev: [f32; 5],
  mouse_drag_max_distance_abs: [ImVec2; 5],
  mouse_drag_max_distance_sqr: [f32; 5],
  pen_pressure: f32,
  app_focus_lost: bool,
  app_accepting_events: bool,
  input_queue_surrogate: ImWchar16,
  input_queue_characters: ImVector<ImWchar>,
}

impl Io {
  #[doc(alias = "Fonts")]
  #[inline(always)]
  pub fn font_atlas(&mut self) -> FontAtlas {
    FontAtlas(self.fonts)
  }

  // NOTE: Leaks memory.
  //       Clean it up during `Context::drop` call?
  pub fn set_ini_filename(&mut self, new_filename: Option<&str>) {
    let fname = match new_filename {
      Some(fname) => CString::new(fname).unwrap().into_raw(),
      None => ptr::null_mut(),
    };
    self.ini_filename = fname;
  }

  // NOTE: Leaks memory.
  //       Clean it up during `Context::drop` call?
  pub fn set_log_filename(&mut self, new_filename: Option<&str>) {
    let fname = match new_filename {
      Some(fname) => CString::new(fname).unwrap().into_raw(),
      None => ptr::null_mut(),
    };
    self.log_filename = fname;
  }

  #[doc(alias = "AddMousePosEvent")]
  pub fn add_mouse_pos_event(&mut self, x: f32, y: f32) {
    unsafe { self.ffi_mut().AddMousePosEvent(x, y) };
  }

  #[doc(alias = "AddMouseButtonEvent")]
  pub fn add_mouse_button_event(&mut self, button: i32, down: bool) {
    unsafe { self.ffi_mut().AddMouseButtonEvent(button, down) };
  }

  #[doc(alias = "AddMouseWheelEvent")]
  pub fn add_mouse_wheel_event(&mut self, wheel_x: f32, wheel_y: f32) {
    unsafe { self.ffi_mut().AddMouseWheelEvent(wheel_x, wheel_y) };
  }

  #[doc(alias = "AddFocusEvent")]
  pub fn add_focus_event(&mut self, focused: bool) {
    unsafe { self.ffi_mut().AddFocusEvent(focused) };
  }

  #[doc(alias = "AddInputCharacter")]
  pub fn add_input_char(&mut self, c: char) {
    unsafe { self.ffi_mut().AddInputCharacter(c as u32) };
  }

  #[doc(alias = "AddInputCharacterUTF16")]
  pub fn add_input_char_utf16(&mut self, c: u16) {
    unsafe { self.ffi_mut().AddInputCharacterUTF16(c) };
  }

  #[doc(alias = "AddKeyEvent")]
  pub fn add_key_event(&mut self, key: Key, down: bool) {
    unsafe {
      let key_raw = mem::transmute::<Key, ImGuiKey>(key);

      self.ffi_mut().AddKeyEvent(key_raw, down);
    }
  }

  #[inline(always)]
  fn ffi_mut(&mut self) -> &'static mut ImGuiIO {
    unsafe { &mut *(self as *mut Io as *mut ImGuiIO) }
  }
}

bitflags! {
  #[doc(alias = "ImGuiConfigFlags")]
  #[repr(transparent)]
  pub struct ConfigFlags: ImGuiConfigFlags {
    const NO_MOUSE_CURSOR_CHANGE = ImGuiConfigFlags_::ImGuiConfigFlags_NoMouseCursorChange.0;
  }
}

bitflags! {
  #[doc(alias = "ImGuiBackendFlags")]
  #[repr(transparent)]
  pub struct BackendFlags: ImGuiBackendFlags {
    const RENDERER_HAS_VTF_OFFSET = ImGuiBackendFlags_::ImGuiBackendFlags_RendererHasVtxOffset.0;
    const RENDERER_HAS_TEXTURES = ImGuiBackendFlags_::ImGuiBackendFlags_RendererHasTextures.0;
    const HAS_MOUSE_CURSORS = ImGuiBackendFlags_::ImGuiBackendFlags_HasMouseCursors.0;
    const HAS_SET_MOUSE_POS = ImGuiBackendFlags_::ImGuiBackendFlags_HasSetMousePos.0;
  }
}

#[doc(alias = "ImFontAtlas")]
#[repr(transparent)]
pub struct FontAtlas(*mut ImFontAtlas);

impl FontAtlas {
  #[doc(alias = "AddFontDefault")]
  #[inline(always)]
  pub fn add_font_default(&mut self) {
    unsafe { (*self.0).AddFontDefault(ptr::null()) };
  }
}

#[doc(alias = "GetPlatformIO")]
#[inline(always)]
pub fn platform_io() -> &'static PlatformIo {
  platform_io_mut()
}

#[doc(alias = "GetPlatformIO")]
#[inline(always)]
pub fn platform_io_mut() -> &'static mut PlatformIo {
  unsafe { &mut *ImGui_GetPlatformIO().cast::<PlatformIo>() }
}

#[doc(alias = "ImGuiPlatformIO")]
#[repr(C)]
pub struct PlatformIo {
  get_clipboard_text_fn: GetClipboardTextFn,
  set_clipboard_text_fn: SetClipBoardTextFn,
  clipboard_user_data: *mut c_void,

  open_in_shell_fn: OpenInShellFn,
  open_in_shell_user_data: *mut c_void,

  set_ime_data_fn: SetImeDataFn,
  ime_user_data: *mut c_void,

  locale_decimal_point: ImWchar,

  pub texture_max_width: i32,
  pub texture_max_height: i32,

  render_state: *mut c_void,

  textures: ImVector<*mut ImTextureData>,
}

type GetClipboardTextFn = unsafe extern "C" fn(*mut ImGuiContext) -> *const c_char;
type SetClipBoardTextFn = unsafe extern "C" fn(*mut ImGuiContext, text: *const c_char);

type OpenInShellFn = unsafe extern "C" fn(*mut ImGuiContext, path: *const c_char) -> bool;

type SetImeDataFn =
  unsafe extern "C" fn(*mut ImGuiContext, *mut ImGuiViewport, *mut ImGuiPlatformImeData);

#[doc(alias = "GetBackgroundDrawList")]
#[inline(always)]
pub fn background_draw_list() -> &'static mut DrawList {
  unsafe { &mut *ImGui_GetBackgroundDrawList(ImGui_GetMainViewport()).cast::<DrawList>() }
}

#[doc(alias = "GetForegroundDrawList")]
#[inline(always)]
pub fn foreground_draw_list() -> &'static mut DrawList {
  unsafe { &mut *ImGui_GetForegroundDrawList(ImGui_GetMainViewport()).cast::<DrawList>() }
}

// TODO: All fields
#[doc(alias = "ImDrawList")]
#[repr(C)]
pub struct DrawList {
  cmd_buffer: ImVector<DrawCmd>,
  idx_buffer: ImVector<DrawIndex>,
  vtx_buffer: ImVector<DrawVertex>,
}

impl DrawList {
  #[doc(alias = "AddRectFilled")]
  #[inline(always)]
  pub fn add_rect_filled(&mut self, min: ImVec2, max: ImVec2, color: u32) {
    self.add_rect_filled_ex(min, max, color).build(self);
  }

  #[doc(alias = "AddRectFilled")]
  #[inline(always)]
  pub fn add_rect_filled_ex(
    &mut self,
    min: ImVec2,
    max: ImVec2,
    color: u32,
  ) -> AddRectFilledBuilder {
    AddRectFilledBuilder::create_empty().min(min).max(max).color(color)
  }

  #[doc(alias = "AddImage")]
  #[inline(always)]
  pub fn add_image_ex(
    &mut self,
    texture_ref: TextureRef,
    min: ImVec2,
    max: ImVec2,
  ) -> AddImageBuilder {
    AddImageBuilder::create_empty().texture_ref(texture_ref).min(min).max(max)
  }

  #[doc(alias = "AddImage")]
  #[inline(always)]
  pub fn add_image(&mut self, texture_ref: TextureRef, min: ImVec2, max: ImVec2) {
    self.add_image_ex(texture_ref, min, max).build(self);
  }

  #[doc(alias = "AddCallback")]
  #[inline(always)]
  pub fn add_callback(&mut self, callback: DrawCallback) {
    self.add_callback_ex::<()>(callback).build(self);
  }

  #[doc(alias = "AddCallback")]
  #[inline(always)]
  pub fn add_callback_ex<'a, T>(&mut self, callback: DrawCallback) -> AddCallbackBuilder<'a, T> {
    AddCallbackBuilder::create_empty().callback(callback)
  }

  #[inline(always)]
  pub fn cmd_buffer(&self) -> impl Iterator<Item = &DrawCmd> {
    self.cmd_buffer.iter()
  }

  #[inline(always)]
  pub fn cmd_buffer_size(&self) -> usize {
    self.cmd_buffer.Size as usize
  }

  #[inline(always)]
  pub fn idx_buffer(&self) -> impl Iterator<Item = &DrawIndex> {
    self.idx_buffer.iter()
  }

  #[inline(always)]
  pub fn idx_buffer_raw(&self) -> *mut DrawIndex {
    self.idx_buffer.Data
  }

  #[inline(always)]
  pub fn idx_buffer_size(&self) -> usize {
    self.idx_buffer.Size as usize
  }

  #[inline(always)]
  pub fn vtx_buffer(&self) -> impl Iterator<Item = &DrawVertex> {
    self.vtx_buffer.iter()
  }

  #[inline(always)]
  pub fn vtx_buffer_size(&self) -> usize {
    self.vtx_buffer.Size as usize
  }
}

#[derive(Builder)]
#[builder(pattern = "owned", build_fn(private, name = "_build"))]
pub struct AddRectFilled {
  min: ImVec2,
  max: ImVec2,
  color: u32,
  #[builder(setter(strip_option), default)]
  rounding: Option<f32>,
  #[builder(setter(strip_option), default)]
  flags: Option<DrawFlags>,
}

impl AddRectFilledBuilder {
  pub fn build(self, draw_list: &mut DrawList) {
    let add_rect_filled = self._build().unwrap();

    let rounding = add_rect_filled.rounding.unwrap_or(0.0);
    let flags = add_rect_filled.flags.unwrap_or_else(DrawFlags::empty);

    unsafe {
      ImDrawList_AddRectFilled(
        draw_list as *mut DrawList as *mut ImDrawList,
        &add_rect_filled.min,
        &add_rect_filled.max,
        add_rect_filled.color,
        rounding,
        flags.bits(),
      )
    };
  }
}

bitflags! {
  #[doc(alias = "ImDrawFlags")]
  #[repr(transparent)]
  pub struct DrawFlags: ImDrawFlags {}
}

#[derive(Builder)]
#[builder(pattern = "owned", build_fn(private, name = "_build"))]
pub struct AddImage {
  texture_ref: TextureRef,
  min: ImVec2,
  max: ImVec2,
  #[builder(setter(strip_option), default)]
  uv_min: Option<ImVec2>,
  #[builder(setter(strip_option), default)]
  uv_max: Option<ImVec2>,
  #[builder(setter(strip_option), default)]
  color: Option<u32>,
}

impl AddImageBuilder {
  pub fn build(self, draw_list: &mut DrawList) {
    let add_image = self._build().unwrap();

    let tex_ref = unsafe { mem::transmute::<TextureRef, ImTextureRef>(add_image.texture_ref) };
    let uv_min = add_image.uv_min.unwrap_or(ImVec2 { x: 0.0, y: 0.0 });
    let uv_max = add_image.uv_max.unwrap_or(ImVec2 { x: 1.0, y: 1.0 });
    let color = add_image.color.unwrap_or(im_col32(1.0, 1.0, 1.0, 1.0));

    unsafe {
      ImDrawList_AddImage(
        draw_list as *mut DrawList as *mut ImDrawList,
        tex_ref,
        &add_image.min,
        &add_image.max,
        &uv_min,
        &uv_max,
        color,
      )
    };
  }
}

#[derive(Builder)]
#[builder(pattern = "owned", build_fn(private, name = "_build"))]
pub struct AddCallback<'a, T> {
  callback: DrawCallback,
  #[builder(setter(strip_option), default)]
  user_data: Option<&'a T>,
}

impl<'a, T> AddCallbackBuilder<'a, T> {
  pub fn build(self, draw_list: &mut DrawList) {
    let add_callback = self._build().unwrap();

    unsafe {
      let callback = mem::transmute::<DrawCallback, ImDrawCallback>(add_callback.callback);

      let user_data = mem::transmute::<Option<&T>, *mut c_void>(add_callback.user_data);
      let user_data_size = 0;

      ImDrawList_AddCallback(
        draw_list as *mut DrawList as *mut ImDrawList,
        callback,
        user_data,
        user_data_size,
      );
    }
  }
}

#[doc(alias = "ImDrawCallback")]
pub type DrawCallback = extern "C" fn(parent_list: &DrawList, &DrawCmd);

#[doc(alias = "ImDrawCallback_ResetRenderState")]
#[inline(always)]
pub const fn reset_render_state() -> DrawCallback {
  unsafe { mem::transmute(-8i64) }
}

#[doc(alias = "ImDrawCmd")]
#[repr(C)]
pub struct DrawCmd {
  pub clip_rect: ImVec4,
  pub texture_ref: TextureRef,
  pub vtx_offset: u32,
  pub idx_offset: u32,
  pub elem_count: u32,
  pub user_callback: Option<DrawCallback>,
  user_callback_data: *mut c_void,
  user_callback_data_size: i32,
  user_callback_data_offset: i32,
}

impl DrawCmd {
  pub fn user_callback_data<T>(&self) -> Option<&'static mut T> {
    unsafe { self.user_callback_data.cast::<T>().as_mut() }
  }
}

#[doc(alias = "ImTextureRef")]
#[repr(C)]
pub struct TextureRef {
  data: Option<&'static mut TextureData>,
  id: TextureID,
}

impl TextureRef {
  pub fn new<T>(texture: &T) -> Self {
    Self {
      data: None,
      id: unsafe { mem::transmute_copy::<T, TextureID>(texture) }, /* legacy */
    }
  }

  pub fn id(&self) -> TextureID {
    self.data.as_ref().map(|d| d.tex_id).unwrap_or(self.id)
  }
}

#[doc(alias = "IM_COl32")]
#[inline]
pub fn im_col32(r: f32, g: f32, b: f32, a: f32) -> u32 {
  let col = ImVec4 { x: r, y: g, z: b, w: a };
  unsafe { ImGui_ColorConvertFloat4ToU32(&col) }
}

#[doc(alias = "ImTextureID")]
pub type TextureID = ImTextureID;

#[doc(alias = "ImTextureFormat")]
#[derive(Debug, PartialEq)]
#[repr(C)]
pub enum TextureFormat {
  RGBA32,
  Alpha8,
}

#[doc(alias = "ImTextureRect")]
pub type TextureRect = ImTextureRect;

#[doc(alias = "ImDrawVert")]
pub type DrawVertex = ImDrawVert;

#[doc(alias = "ImDrawIdx")]
pub type DrawIndex = ImDrawIdx;

#[doc(alias = "ImGuiMouseCursor")]
#[derive(PartialEq, Clone, Copy)]
#[repr(C)]
pub enum MouseCursor {
  Arrow,
  TextInput,
  ResizeAll,
  ResizeNS,
  ResizeEW,
  ResizeNESW,
  ResizeNWSE,
  Hand,
  Wait,
  Progress,
  NotAllowed,
}

#[doc(alias = "GetMouseCursor")]
pub fn mouse_cursor() -> Option<MouseCursor> {
  let cursor = unsafe { ImGui_GetMouseCursor() };

  if cursor == -1 { None } else { Some(unsafe { mem::transmute::<i32, MouseCursor>(cursor) }) }
}

#[doc(alias = "ImGuiKey")]
#[derive(Clone, Copy, PartialEq)]
#[repr(C)]
pub enum Key {
  None = 0,
  Tab = 512,
  LeftArrow = 513,
  RightArrow = 514,
  UpArrow = 515,
  DownArrow = 516,
  PageUp = 517,
  PageDown = 518,
  Home = 519,
  End = 520,
  Insert = 521,
  Delete = 522,
  Backspace = 523,
  Space = 524,
  Enter = 525,
  Escape = 526,
  LeftCtrl = 527,
  LeftShift = 528,
  LeftAlt = 529,
  LeftSuper = 530,
  RightCtrl = 531,
  RightShift = 532,
  RightAlt = 533,
  RightSuper = 534,
  Menu = 535,
  Num0 = 536,
  Num1 = 537,
  Num2 = 538,
  Num3 = 539,
  Num4 = 540,
  Num5 = 541,
  Num6 = 542,
  Num7 = 543,
  Num8 = 544,
  Num9 = 545,
  A = 546,
  B = 547,
  C = 548,
  D = 549,
  E = 550,
  F = 551,
  G = 552,
  H = 553,
  I = 554,
  J = 555,
  K = 556,
  L = 557,
  M = 558,
  N = 559,
  O = 560,
  P = 561,
  Q = 562,
  R = 563,
  S = 564,
  T = 565,
  U = 566,
  V = 567,
  W = 568,
  X = 569,
  Y = 570,
  Z = 571,
  F1 = 572,
  F2 = 573,
  F3 = 574,
  F4 = 575,
  F5 = 576,
  F6 = 577,
  F7 = 578,
  F8 = 579,
  F9 = 580,
  F10 = 581,
  F11 = 582,
  F12 = 583,
  Apostrophe = 596,
  Comma = 597,
  Minus = 598,
  Period = 599,
  Slash = 600,
  Semicolon = 601,
  Equal = 602,
  LeftBracket = 603,
  Backslash = 604,
  RightBracket = 605,
  GraveAccent = 606,
  CapsLock = 607,
  ScrollLock = 608,
  NumLock = 609,
  PrintScreen = 610,
  Pause = 611,
  Keypad0 = 612,
  Keypad1 = 613,
  Keypad2 = 614,
  Keypad3 = 615,
  Keypad4 = 616,
  Keypad5 = 617,
  Keypad6 = 618,
  Keypad7 = 619,
  Keypad8 = 620,
  Keypad9 = 621,
  KeypadDecimal = 622,
  KeypadDivide = 623,
  KeypadMultiply = 624,
  KeypadSubtract = 625,
  KeypadAdd = 626,
  KeypadEnter = 627,
  KeypadEqual = 628,
  AppBack = 629,
  AppForward = 630,
  Oem102 = 631,
  MouseLeft = 656,
  MouseRight = 657,
  MouseMiddle = 658,
  MouseX1 = 659,
  MouseX2 = 660,
  MouseWheelX = 661,
  MouseWheelY = 662,
  ModCtrl = 4_096,
  ModShift = 8_192,
  ModAlt = 16_384,
  ModSuper = 32_768,
}

#[doc(alias = "IsKeyDown")]
pub fn is_key_down(key: Key) -> bool {
  unsafe {
    let key_raw = mem::transmute::<Key, ImGuiKey>(key);

    ImGui_IsKeyDown(key_raw)
  }
}
