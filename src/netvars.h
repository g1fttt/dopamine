#pragma once

// Include here so we don't have to include it everywhere along with "netvars.h"
// header
#include <utils/fnv_hash.h>
#include <utils/ptr.h>

#include <optional>
#include <vector>

namespace game
{
  struct RecvTable;
}

namespace core
{
  struct Interfaces;
}

namespace core
{
  struct Netvars {
    Netvars(const core::Interfaces &interfaces);

    std::optional<uintptr_t> find_by_hash(uintptr_t hash);
  private:
    using HashOffset = std::pair<uintptr_t, uintptr_t>;

    void walk_table(std::string_view network_name,
                    const game::RecvTable *recv_table, uintptr_t offset = 0);

    std::vector<HashOffset> hashed;
  };

  std::optional<uintptr_t> find_netvar_by_hash(uintptr_t hash);
}

#define NETVAR_OFFSET(RetType, func_name, class_name, var_name, offset)        \
  RetType func_name() {                                                        \
    constexpr auto hash = utils::fnv::hash(class_name "->" var_name);          \
    return *utils::Ptr{this}                                                   \
                .byte_add(*core::find_netvar_by_hash(hash) + offset)           \
                .cast<RetType>();                                              \
  }

#define NETVAR(RetType, func_name, class_name, var_name)                       \
  NETVAR_OFFSET(RetType, func_name, class_name, var_name, 0)
