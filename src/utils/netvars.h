#pragma once

#include <utils/fnv_hash.h>
#include <utils/singleton.h>

namespace game {
  struct RecvTable;
}

struct App;

namespace utils {
  struct Netvars : Singleton<Netvars> {
    static Singleton<Netvars>::InitFunc init_func(const App &app) {
      return [&](Netvars &netvars) {
        netvars.init_or_nothing(app);
      };
    }

    std::optional<uintptr_t> find_by_hash(uintptr_t hash);
  private:
    using HashOffset = std::pair<uintptr_t, uintptr_t>;

    void init_or_nothing(const App &app);
    void walk_table(std::string_view network_name,
                    const game::RecvTable *recv_table, uintptr_t offset = 0);

    std::vector<HashOffset> hashed;
  };
}

#define NETVAR_OFFSET(Type, func_name, class_name, var_name, offset)           \
  Type func_name() {                                                           \
    const auto hash = utils::fnv::hash(class_name "->" var_name);              \
    return *reinterpret_cast<Type *>(                                          \
        this + utils::Netvars::get().find_by_hash(hash).value() + offset);     \
  }

#define NETVAR(Type, func_name, class_name, var_name)                          \
  NETVAR_OFFSET(Type, func_name, class_name, var_name, 0)
