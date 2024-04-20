#pragma once

#include <algorithm>

namespace core {
  struct Input;
}

namespace ui {
  struct Menu {
    void draw();
    void handle_toggle(const core::Input &input);

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
      bool visuals = false;
      bool glow = false;
    };

    constexpr float animation_len() const {
      return 0.35f;
    }

    void draw_menu_bar();
    void draw_menu_bar_item(const char *window_name, bool &should_draw_window);
    void draw_misc_window();
    void draw_visuals_window();
    void draw_glow_window();

    bool open = false;
    float toggle_animation_end = 1.0f;
    ShouldDrawWindow should_draw_window;
  };

  constinit inline Menu menu{};
}
