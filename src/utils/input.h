#pragma once

#include <Windows.h>

#include <functional>

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
    static Input &get();

    static void with(UINT message, WPARAM wparam, LPARAM lparam,
                     const std::function<void()> &cb);

    void update_state(UINT message, WPARAM wparam, LPARAM lparam);
    void reset_state();

    bool key_is_up(KeyCode code) const;
  private:
    Key last_up;
  };
}
