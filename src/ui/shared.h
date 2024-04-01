#pragma once

#include <imgui.h>

namespace ui {
  struct ImGuiContextual {
    constexpr virtual ~ImGuiContextual() {
      ImGui::DestroyContext(imgui_ctx);
    }

    void set_context(ImGuiContext *ctx) {
      imgui_ctx = ctx;
    }

    void make_current() const {
      ImGui::SetCurrentContext(imgui_ctx);
    }
  private:
    ImGuiContext *imgui_ctx;
  };
}
