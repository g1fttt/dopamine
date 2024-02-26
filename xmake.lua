add_rules('mode.release', 'mode.debug')

set_exceptions('no-cxx')
set_languages('c99', 'c++23')

target('dopamine')
  set_kind('shared')
  add_files('src/*.cpp', 'src/hooks/*.cpp', 'src/utils/*.cpp')
  add_includedirs('src/')
