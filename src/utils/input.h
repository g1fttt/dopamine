#pragma once

#include <Windows.h>

#include <functional>

namespace utils {
  using KeyCode = WPARAM;

  struct Input {
    constexpr Input(const Input &) = delete;

    static Input &get() {
      static Input self{};
      return self;
    }

    static void with(UINT message, WPARAM wparam, LPARAM lparam,
                     const std::function<void(const Input &)> &cb);

    void update_state(UINT message, WPARAM wparam, LPARAM lparam);
    void reset_state();

    bool key_is_up(KeyCode code) const;
  private:
    constexpr Input() = default;

    struct Key {
      KeyCode code;
      LPARAM lparam;
    };

    Key last_up;
  };
}
