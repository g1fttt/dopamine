#pragma once

#include <algorithm>

class Menu {
public:
  void render();
  void update_animation();
  void handle_toggle();

  constexpr bool is_open() {
    return open;
  }

  constexpr bool is_fully_closed() {
    return !open && toggle_animation_end > 1.0f;
  }

  constexpr float get_transparency() {
    return std::clamp(open ? toggle_animation_end : 1.0f - toggle_animation_end,
                      0.0f, 1.0f);
  }
private:
  constexpr float animation_len() {
    return 0.35f;
  }
private:
  bool open = false;
  float toggle_animation_end = 0.0f;
};
