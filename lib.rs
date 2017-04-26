#![feature(lang_items, panic_unwind, compiler_builtins_lib)]
#![allow(bad_style)]
#![no_std]

extern crate panic_unwind;
extern crate compiler_builtins;

type VALUE = *const u8;
type c_string = *const i8;
type c_func = *const u8;

#[link(name = "msvcrt-ruby230")]
extern {
    static rb_cObject: VALUE;
    static rb_eRuntimeError: VALUE;

    fn rb_define_class(name: c_string, superclass: VALUE) -> VALUE;
    fn rb_define_method(class: VALUE, name: c_string, func: c_func, arity: isize);
    fn rb_raise(exc: VALUE, string: c_string, ...) -> !;
}

extern {
    fn malloc(amt: usize) -> *mut u8;
    fn free(ptr: *mut u8);
}

unsafe fn print(s: &str) {
    extern "system" {
        fn GetStdHandle(a: u32) -> *mut u8;
        fn WriteConsoleA(output: *mut u8,
                         buffer: *mut u8,
                         chars: u32,
                         written: *mut u32,
                         reserved: *mut u8) -> i32;
        fn WriteFile(output: *mut u8,
                     buffer: *mut u8,
                     chars: u32,
                     written: *mut u32,
                     reserved: *mut u8) -> i32;
    }
    const STD_OUTPUT_HANDLE: u32 = (-12i32) as u32;

    let h = GetStdHandle(STD_OUTPUT_HANDLE);
    let mut a = 0;
    WriteFile(h,
              s.as_ptr() as *mut u8,
              s.len() as u32,
              &mut a,
              0 as *mut _);
}

#[no_mangle]
pub extern fn Init_native() {
    unsafe {
        let raw_class = rb_define_class(b"Console\0".as_ptr() as *const _,
                                        rb_cObject);
        rb_define_method(
            raw_class,
            b"freak_out\0".as_ptr() as *const _,
            __ruby_method__ as *const _,
            0,
            );
    }

    extern fn __ruby_method__(_rb_self: VALUE) -> VALUE {
        let mut data_ptr = 1;
        let mut vtable_ptr = 1;
        // drop(std::panic::catch_unwind(|| panic!()));
        unsafe {
            let n = panic_unwind::__rust_maybe_catch_panic(stuff,
                                                   2 as *mut _,
                                                   &mut data_ptr,
                                                   &mut vtable_ptr);
            assert_eq!(n, 1);
            assert_eq!(data_ptr, 0);
            assert_eq!(vtable_ptr, 0);
            print("caught a panic\n");
            let s1 = Foo::new();
            rb_raise(rb_eRuntimeError,
                     b"%s\0".as_ptr() as *const _,
                     s1.ptr);
        }
    }
}

fn stuff(a: *mut u8) {
    assert_eq!(a as usize, 2);
    panic!()
}

#[lang = "panic_fmt"]
pub extern fn rust_begin_panic(_msg: core::fmt::Arguments,
                               _file: &'static str,
                               _line: u32) -> ! {
    unsafe {
        print("starting a panic\n");
        panic_unwind::__rust_start_panic(0, 0);
        loop {}
    }
}

struct Foo {
    ptr: *mut u8,
}

extern "system" {
    fn ExitProcess(code: u32);
}

extern {
    fn memcpy(dst: *mut u8,
              src: *const u8,
              amt: usize) -> *mut u8;
}

impl Foo {
    fn new() -> Foo {
        unsafe {
            let ptr = malloc(4);
            if ptr.is_null() {
                ExitProcess(4);
            }
            memcpy(ptr, b"foo\0".as_ptr(), 4);
            Foo { ptr: ptr }
        }
    }
}

impl Drop for Foo {
    fn drop(&mut self) {
        unsafe {
            free(self.ptr);
        }
    }
}
