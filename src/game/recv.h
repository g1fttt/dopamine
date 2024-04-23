#pragma once

#include <utils/pad.h>

#include <cstdint>

namespace game
{
  enum struct SendPropType {
    NumSendPropTypes = 6,
  };

  struct RecvTable;

  struct RecvProp {
    const char *var_name;
    SendPropType recv_type;
    PAD(29);
    RecvTable *data_table;
    int32_t offset;
    PAD(12);
  };

  struct RecvTable {
    RecvProp *props;
    int32_t prop_amount;
    PAD(4);
    const char *name;
  };
}
