#include "color.h"

#include <imgui.h>

namespace utils
{
  int32_t Color::im_u32() const {
    return ImGui::ColorConvertFloat4ToU32(
        *reinterpret_cast<const ImVec4 *>(float_array()));
  }
}
