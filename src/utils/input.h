#pragma once

#include <Windows.h>

#include <functional>
#include <optional>

namespace utils {
  using KeyCode = WPARAM;

  struct Input {
    void with(UINT message, WPARAM wparam, LPARAM lparam,
              const std::function<void(const Input &)> &cb);

    void update_state(UINT message, WPARAM wparam, LPARAM lparam);
    void reset_state();

    bool key_is_up(KeyCode code) const;
  private:
    struct Key {
      KeyCode code;
      LPARAM lparam;
    };

    Key last_up;
  };

  constinit inline std::optional<Input> input{};
}
