#include "netvars.h"

#include <internal/client_class.h>
#include <internal/recv.h>

#include <interfaces/client.h>

#include <utils/fnv_hash.h>
#include <utils/logger.h>

#include <app.h>

#include <algorithm>
#include <cctype>
#include <string_view>

namespace utils {
  std::optional<uintptr_t> Netvars::find_by_hash(uintptr_t hash) {
    const auto it =
        std::ranges::lower_bound(hashed, hash, {}, &HashOffset::first);

    if (it != hashed.end() && it->first == hash) {
      return it->second;
    }
    return std::nullopt;
  }

  void Netvars::init_or_nothing() {
    if (static bool inited = false; !inited) {
      for (auto *client_class = App::get().interfaces.client->get_all_classes();
           client_class; client_class = client_class->next) {
        walk_table(client_class->network_name, client_class->recv_table);
      }
      std::ranges::sort(hashed, {}, &HashOffset::first);

      inited = true;
    }
  }

  void Netvars::walk_table(std::string_view network_name,
                           const internal::RecvTable *recv_table,
                           uintptr_t offset) {
    for (size_t i = 0; i < recv_table->prop_amount; i += 1) {
      const auto *prop = recv_table->props + i;

      if (std::isdigit(prop->var_name[0])) {
        continue;
      }

      if (fnv::hash(prop->var_name) == fnv::hash("baseclass")) {
        continue;
      }

      if (prop->recv_type == internal::SendPropType::NumSendPropTypes &&
          prop->data_table && prop->data_table->name[0] == 'D') {
        walk_table(network_name, prop->data_table, prop->offset + offset);
      }

      const auto hash =
          fnv::hash({network_name.data() + std::string("->") + prop->var_name});
      hashed.emplace_back(hash, prop->offset + offset);
    }
  }
}
