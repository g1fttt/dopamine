#pragma once

#include <imgui.h>

namespace ui {
  struct ImGuiContext {
    inline void destroy() const {
      ImGui::DestroyContext(ctx);
    }

    inline void set(::ImGuiContext *ctx) {
      this->ctx = ctx;
    }

    inline void push() const {
      ImGui::SetCurrentContext(ctx);
    }
  private:
    ::ImGuiContext *ctx = nullptr;
  };
}
