#pragma once

class Config;

namespace internal {
  class UserCommand;
}

namespace hacks {
  void bunnyhop(const Config &cfg, internal::UserCommand *cmd);
}
