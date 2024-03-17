#pragma once

#include <cstdint>
#include <optional>
#include <string_view>
#include <utility>
#include <vector>

namespace internal {
  struct RecvTable;
}

namespace utils {
  class Netvars {
  public:
    constexpr Netvars(const Netvars &&) = delete;
    constexpr Netvars(Netvars &&) = delete;

    static Netvars &get() {
      static Netvars self{};
      { self.init_or_nothing(); }
      return self;
    }

    std::optional<uintptr_t> find_by_hash(uintptr_t hash);
  private:
    using Offset = std::pair<uintptr_t, uintptr_t>;
  private:
    constexpr Netvars() = default;

    void init_or_nothing();
    void walk_table(std::string_view network_name,
                    const internal::RecvTable *recv_table,
                    uintptr_t offset = 0);
  private:
    std::vector<Offset> offsets;
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
