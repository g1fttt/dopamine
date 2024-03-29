#include "hacks.h"

#include <internal/entity.h>
#include <internal/user_command.h>

#include <app.h>

#include <random>

void hacks::bunnyhop(const Config &cfg, internal::UserCommand *cmd) {
  if (!cfg.misc.bunnyhop.enabled) {
    return;
  }

  std::random_device rd{};
  std::minstd_rand gen{rd()};
  std::bernoulli_distribution distr{cfg.misc.bunnyhop.chance / 100.0f};

  if (const auto should_bunnyhop = distr(gen);
      !App::get().local_player->is_on_ground() || !should_bunnyhop) {
    cmd->buttons &= ~internal::UserCommand::InJump;
  }
}
