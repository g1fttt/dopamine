#pragma once

#include <utils/vmethod.h>

namespace internal {
  struct ClientClass;
}

namespace interfaces {
  struct Client {
    VMETHOD(internal::ClientClass *, get_all_classes, 8, (), (this))
  };
}
