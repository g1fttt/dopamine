#pragma once

#include <imgui.h>

namespace ui {
  struct ImGuiContext {
    constexpr void destroy() const {
      ImGui::DestroyContext(ctx);
    }

    constexpr void set(::ImGuiContext *ctx) {
      this->ctx = ctx;
    }

    constexpr void push() const {
      ImGui::SetCurrentContext(ctx);
    }
  private:
    ::ImGuiContext *ctx = nullptr;
  };
}
