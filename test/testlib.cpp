#include <windows.h>

// A simple exported function
extern "C" __declspec(dllexport) int __stdcall Foo(int x) { return x + 1; }

// Another one, with no parameters
extern "C" __declspec(dllexport) void __stdcall Bar() {
  MessageBoxA(nullptr, "Hello from Bar", "TestLib", MB_OK);
}

// DllMain can be totally empty
BOOL WINAPI DllMain(HINSTANCE, DWORD, LPVOID) { return TRUE; }
