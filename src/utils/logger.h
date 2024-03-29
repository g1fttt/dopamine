#pragma once

#include <filesystem>
#include <fstream>
#include <string_view>

namespace utils {
  class Logger {
  public:
    enum class Level : uint8_t {
      Warn,
      Error,
      Fatal,
    };
  public:
    constexpr Logger(const Logger &&) = delete;
    constexpr Logger(const Logger &) = delete;

    static Logger &get() {
      static Logger self{};
      { self.init_or_nothing(); }
      return self;
    }

    template <Level L, typename... Args>
    constexpr void log(std::format_string<Args...> fmt, Args &&...args) {
      file_desc << std::format(
                       "[{}] {}", level_to_string(L),
                       std::vformat(fmt.get(), std::make_format_args(args...)))
                       .data()
                << std::endl;
      file_desc.flush();
    }
  private:
    Logger() = default;

    constexpr static std::string_view level_to_string(Level level) {
      switch (level) {
      case Level::Warn:
        return "WARN";
      case Level::Error:
        return "ERROR";
      case Level::Fatal:
        return "FATAL";
      };
    }

    constexpr void open_log_file(std::filesystem::path file_path) {
      file_desc.open(file_path);
    }

    constexpr void init_or_nothing() {
      open_log_file(L"dopamine.txt");
    }
  private:
    std::ofstream file_desc;
  };
}

using Logger = utils::Logger;
using Level = Logger::Level;
