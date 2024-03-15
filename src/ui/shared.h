#pragma once

#include <imgui.h>

namespace ui {
  class ImGuiContextual {
  public:
    constexpr virtual ~ImGuiContextual() {
      ImGui::DestroyContext(imgui_ctx);
    }

    constexpr void set_context(ImGuiContext *ctx) {
      imgui_ctx = ctx;
    }

    constexpr void make_current() const {
      ImGui::SetCurrentContext(imgui_ctx);
    }
  private:
    ImGuiContext *imgui_ctx;
  };
}
