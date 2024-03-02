#pragma once

#include <Windows.h>

namespace utils {
  using KeyCode = WPARAM;

  namespace {
    struct Key {
      KeyCode code;
      LPARAM lparam;
    };
  }

  class Input {
  public:
    void update_state(UINT message, WPARAM wparam, LPARAM lparam);
    void reset_state();

    bool key_is_up(KeyCode code) const;
  private:
    Key last_up;
  };
}
