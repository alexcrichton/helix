#include <windows.h>
#include <stdio.h>
#include <assert.h>
#include <setjmp.h>

typedef void(*thunk)(void);

extern int rb_invoke(thunk fn);


int main() {
  HANDLE rust = LoadLibrary("rust.dll");
  assert(rust != NULL);
  thunk (*init)(void) = (thunk(*)(void)) GetProcAddress(rust, "Init_native");
  assert(init != NULL);

  thunk fn = init();
  printf("invoke1: %d\n", rb_invoke(fn));
  printf("invoke2: %d\n", rb_invoke(fn));

  return 0;
}
