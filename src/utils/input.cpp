#include "input.h"

namespace utils {
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
