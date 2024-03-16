#pragma once

#include <utils/pad.h>

namespace internal {
  struct RecvTable;

  struct ClientClass {
    PAD(8);
    const char *network_name;
    RecvTable *recv_table;
    ClientClass *next;
    // int32_t class_id;
  };
}
