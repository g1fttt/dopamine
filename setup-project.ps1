param([string]$mode = "debug")

xmake f -p windows -a x86 -m $mode --toolchain=clang
