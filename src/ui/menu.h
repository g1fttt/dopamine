#pragma once

#include "shared.h"

#include <algorithm>
#include <string_view>

class Config;

namespace ui {
  class Menu : public ImGuiContextual {
  public:
    constexpr Menu(const Menu &&) = delete;
    constexpr Menu(const Menu &) = delete;

    static Menu &get() {
      static Menu self{};
      return self;
    }

    void draw();
    void handle_toggle();

    void update_animation();

    constexpr bool is_open() const {
      return open;
    }

    constexpr bool is_fully_closed() const {
      return !open && toggle_animation_end > 1.0f;
    }

    constexpr float get_transparency() const {
      return std::clamp(open ? toggle_animation_end
                             : 1.0f - toggle_animation_end,
                        0.0f, 1.0f);
    }
  private:
    struct ShouldDrawWindow {
      bool misc = false;
    };
  private:
    constexpr Menu() = default;

    constexpr float animation_len() const {
      return 0.35f;
    }

    void draw_menu_bar();
    void draw_menu_bar_item(std::string_view window_name,
                            bool &should_draw_window);
    void draw_misc_window(Config &cfg);
  private:
    bool open = false;
    float toggle_animation_end = 1.0f;
    ShouldDrawWindow should_draw_window;
  };
}
