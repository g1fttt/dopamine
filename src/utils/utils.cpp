#include "utils.h"

#include <Windows.h>

#include <cstdint>

void *utils::interface_base(const char *module_name,
                            const char *interface_name) {
  const auto module = GetModuleHandleA(module_name);

  using CreateInterface = void *(*)(const char *, int32_t *);
  const auto create_interface = reinterpret_cast<CreateInterface>(
      GetProcAddress(module, "CreateInterface"));

  return create_interface(interface_name, nullptr);
}
