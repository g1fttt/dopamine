#pragma once

#include <algorithm>

namespace core {
  class Menu {
  public:
    static Menu &get();

    void render() const;
    void update_animation();
    void handle_toggle();

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
    constexpr float animation_len() const {
      return 0.35f;
    }
  private:
    bool open = false;
    float toggle_animation_end = 0.0f;
  };
}
