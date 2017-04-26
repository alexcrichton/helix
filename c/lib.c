#include <setjmpex.h>
#include <stdio.h>
#include <windows.h>

static jmp_buf env;

__declspec(dllexport)
void rb_raise(char *msg) {
  printf("raise: %s\n", msg);
  fflush(stdout);
  longjmp(env, 1);
}

__declspec(dllexport)
int rb_invoke(void(*fn)(void)) {
  if (!setjmp(env)) {
    fn();
    return 0;
  } else {
    return 1;
  }
}
