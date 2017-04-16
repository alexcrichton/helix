#![allow(bad_style)]

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
        wut();
        unsafe {
            let s1 = mk();
            rb_raise(rb_eRuntimeError,
                     b"%s\0".as_ptr() as *const _,
                     s1.as_ptr() as *const u8)
        }
    }

    #[inline(never)]
    fn wut() {
        drop(std::panic::catch_unwind(|| panic!()));
    }

    #[inline(never)]
    fn mk() -> String {
        String::from("test\0")
    }
}
