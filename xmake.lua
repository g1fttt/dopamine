add_rules('mode.release', 'mode.debug')

set_exceptions('no-cxx')
set_languages('c99', 'c++23')

add_defines(
  'STDCALL=__stdcall',
  'THISCALL=__thiscall')
add_cxflags('-xc++', { force = true }) -- Fix for clangd treating .h files as C files

add_requires('vcpkg::toml11 3.7.1', { alias = 'toml11' })
add_requires('vcpkg::serdepp 0.1.4.1', { alias = 'serdepp' })

add_requires('vcpkg::imgui 1.90', {
  alias = 'imgui',
  configs = {
    features = { 'dx9-binding', 'win32-binding' },
  },
})

target('dopamine')
  set_kind('shared')

  add_files(
    'src/*.cpp',
    'src/hooks/*.cpp',
    'src/utils/*.cpp',
    'src/ui/*.cpp',
    'src/hacks/*.cpp',
    'src/hacks/glow/*.cpp')
  add_packages('imgui', 'toml11', 'serdepp')
  add_links('d3d9')
  add_includedirs('src/')
