#include <assert.h>
#include <cstdio>
#include <stdlib.h>
#include <string.h>

extern "C" {

typedef void* VALUE;
__declspec(dllimport)
extern VALUE rb_define_class(const char *name, VALUE superclass);
__declspec(dllimport)
extern VALUE rb_define_method(VALUE klass,
                              const char *name,
                              void *func,
                              long long the_arity);
__declspec(dllimport)
extern void rb_raise(VALUE exc, const char *s, ...);
__declspec(dllimport)
extern VALUE rb_cObject;
__declspec(dllimport)
extern VALUE rb_eRuntimeError;

}

static void catch_panic(void(*f)(void)) {
  try {
    f();
  } catch (unsigned long long e) {
    printf("caught\n");
    fflush(stdout);
    // ...
  }
}

static void stuff(void) {
  printf("throwing\n");
  fflush(stdout);
  throw 4ULL;
}

struct Foo {
  char *ptr;

  Foo() {
    this->ptr = (char*) malloc(4);
    assert(ptr != NULL);
    memcpy(ptr, "foo", 4);
  }

  ~Foo() {
    free(this->ptr);
  }

  char* c_str() {
    return this->ptr;
  }
};

extern "C" void my_ruby_method(VALUE me) {
  catch_panic(stuff);
  Foo wut;
  rb_raise(rb_eRuntimeError, "%s", wut.c_str());
}

extern "C"
__declspec(dllexport)
void Init_native() {
  VALUE klass = rb_define_class("Console", rb_cObject);
  rb_define_method(klass,
      "freak_out",
      (void*) my_ruby_method,
      0);
}
