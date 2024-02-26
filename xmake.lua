add_rules('mode.release', 'mode.debug')

set_exceptions('no-cxx')
set_languages('c99', 'c++23')

target('dopamine')
  set_kind('shared')
  add_files('src/*.cpp', 'src/hooks/*.cpp', 'src/utils/*.cpp')
  add_defines('STDCALL=__stdcall', 'THISCALL=__thiscall')
  add_includedirs('src/')
