#include "menu.h"

#include <Windows.h>

#include "app.h"

#include "interfaces/input_system.h"

#include <imgui.h>

void menu::render() {
  App::with([](App &app) {
    if (!app.should_render_menu) {
      return;
    }
    ImGui::ShowDemoWindow();
  });
}

void menu::handle_toggle() {
  App::with([](App &app) {
    if (GetAsyncKeyState(VK_INSERT) & 1) {
      app.should_render_menu = !app.should_render_menu;

      if (!app.should_render_menu) {
        app.input_system->reset_input_state();
      }
    }
  });
}
