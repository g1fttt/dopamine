#pragma once

#include <mutex>

namespace utils
{
  struct Lock {
  private:
    std::scoped_lock<std::mutex> lock{mutex};
    inline static std::mutex mutex{};
  };
}
