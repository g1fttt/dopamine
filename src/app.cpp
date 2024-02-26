#include "app.h"

#include <utils/utils.h>

App &App::get() {
  static App APP;

  if (static bool inited = false; !inited) {
    APP.client = utils::interface_base("client.dll", "VClient017");
    APP.cvar = reinterpret_cast<interfaces::CVar *>(
        utils::interface_base("vstdlib.dll", "VEngineCvar004"));

    APP.client_vmt.init(APP.client);

    inited = true;
  }
  return APP;
}

void App::with(const std::function<void(App &)> &cb) {
  cb(App::get());
}
