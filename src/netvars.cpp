#include "game/client_class.h"
#include "game/recv.h"

#include "game/client.h"

#include "app.h"

#include <algorithm>
#include <cctype>

namespace core
{
  Netvars::Netvars(const core::Interfaces &interfaces) {
    for (auto *client_class = interfaces.client->get_all_classes();
         client_class; client_class = client_class->next)
    {
      walk_table(client_class->network_name, client_class->recv_table);
    }
    std::ranges::sort(hashed, {}, &HashOffset::first);
  }

  std::optional<uintptr_t> Netvars::find_by_hash(uintptr_t hash) {
    const auto it =
        std::ranges::lower_bound(hashed, hash, {}, &HashOffset::first);

    if (it != hashed.end() && it->first == hash) {
      return it->second;
    }
    return std::nullopt;
  }

  void Netvars::walk_table(std::string_view network_name,
                           const game::RecvTable *recv_table,
                           uintptr_t offset) {
    for (size_t i = 0; i < recv_table->prop_amount; i += 1) {
      const auto *prop = recv_table->props + i;

      if (std::isdigit(prop->var_name[0])) {
        continue;
      }

      if (utils::fnv::hash(prop->var_name) == utils::fnv::hash("baseclass")) {
        continue;
      }

      if (prop->recv_type == game::SendPropType::NumSendPropTypes &&
          prop->data_table && prop->data_table->name[0] == 'D')
      {
        walk_table(network_name, prop->data_table, prop->offset + offset);
      }

      const auto hash = utils::fnv::hash(
          {network_name.data() + std::string("->") + prop->var_name});
      hashed.emplace_back(hash, prop->offset + offset);
    }
  }

  std::optional<uintptr_t> find_netvar_by_hash(uintptr_t hash) {
    return app->netvars->find_by_hash(hash);
  }
}
