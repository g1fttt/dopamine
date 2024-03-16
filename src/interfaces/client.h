#pragma once

#include <utils/vmethod.h>

namespace internal {
  struct ClientClass;
}

namespace interfaces {
  class Client {
  public:
    VMETHOD(internal::ClientClass *, get_all_classes, 8, (), (this))
  };
}
