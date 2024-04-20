#pragma once

// Include here so we don't have to include it everywhere along with "netvars.h"
// header
#include <utils/fnv_hash.h>

#include <optional>
#include <vector>

namespace game {
  struct RecvTable;
}

namespace core {
  struct Netvars {
    Netvars();

    std::optional<uintptr_t> find_by_hash(uintptr_t hash);
  private:
    using HashOffset = std::pair<uintptr_t, uintptr_t>;

    void walk_table(std::string_view network_name,
                    const game::RecvTable *recv_table, uintptr_t offset = 0);

    std::vector<HashOffset> hashed;
  };

  constinit inline std::optional<Netvars> netvars{};
}

#define NETVAR_OFFSET(RetType, func_name, class_name, var_name, offset)        \
  RetType func_name() const {                                                  \
    constexpr auto hash = utils::fnv::hash(class_name "->" var_name);          \
    return *reinterpret_cast<const RetType *>(                                 \
        this + core::netvars->find_by_hash(hash).value() + offset);            \
  }

#define NETVAR(RetType, func_name, class_name, var_name)                       \
  NETVAR_OFFSET(RetType, func_name, class_name, var_name, 0)
