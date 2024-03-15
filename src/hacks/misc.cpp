#include "hacks.h"

#include <internal/entity.h>
#include <internal/user_command.h>

#include <app.h>

void hacks::bunnyhop(internal::UserCommand *cmd) {
  if (!App::get().local_player->is_on_ground()) {
    cmd->buttons &= ~internal::UserCommand::InJump;
  }
}
