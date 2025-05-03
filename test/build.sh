x86_64-w64-mingw32-g++ \
  -shared \
  -o testlib.dll \
  testlib.cpp \
  -Wl,--kill-at \
  -static-libgcc \
  -static-libstdc++
