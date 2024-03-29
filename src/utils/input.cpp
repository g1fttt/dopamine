#include "input.h"

namespace utils {
  void Input::with(UINT message, WPARAM wparam, LPARAM lparam,
                   const std::function<void(const Input &)> &cb) {
    auto &input = Input::get();
    input.update_state(message, wparam, lparam);
    { cb(input); }
    input.reset_state();
  }

  void Input::update_state(UINT message, WPARAM wparam, LPARAM lparam) {
    switch (message) {
    case WM_KEYUP:
      last_up = {wparam, lparam};
      break;
    }
  }

  void Input::reset_state() {
    last_up = {};
  }

  bool Input::key_is_up(KeyCode code) const {
    return last_up.code == code;
  }
}
