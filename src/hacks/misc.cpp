#include "misc.h"

#include <internal/entity.h>
#include <internal/user_command.h>

#include <app.h>

#include <random>

namespace hacks {
  void Misc::bunnyhop(internal::UserCommand *cmd) const {
    if (!config.bunnyhop.enabled) {
      return;
    }

    std::random_device rd{};
    std::minstd_rand gen{rd()};
    std::bernoulli_distribution distr{config.bunnyhop.chance / 100.0f};

    App::with<void>([&](const App &app) {
      if (!app.local_player) {
        return;
      }

      if (const auto should_bunnyhop = distr(gen);
          !app.local_player->is_on_ground() || !should_bunnyhop) {
        cmd->buttons &= ~internal::UserCommand::InJump;
      }
    });
  }
}
