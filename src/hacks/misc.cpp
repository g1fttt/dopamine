#include "misc.h"

#include <game/entity.h>
#include <game/user_command.h>

#include <app.h>

#include <random>

namespace hacks {
  void Misc::bunnyhop(game::UserCommand *cmd) const {
    const auto &cfg = config.bunnyhop;
    if (!cfg.enabled || !core::app->local_player) {
      return;
    }

    std::random_device rd{};
    std::minstd_rand gen{rd()};
    std::bernoulli_distribution distr{config.bunnyhop.chance / 100.0f};

    if (const auto should_bunnyhop = distr(gen);
        !core::app->local_player->is_on_ground() || !should_bunnyhop) {
      cmd->buttons &= ~game::UserCommand::InJump;
    }
  }
}
