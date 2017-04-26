#![allow(bad_style)]

type c_string = *const u8;

#[link(name = "ruby")]
extern {
    fn rb_raise(string: c_string) -> !;
}

#[no_mangle]
pub extern fn Init_native() -> extern fn() {
    extern fn test() {
        wut();
        unsafe {
            let s1 = mk();
            rb_raise(s1.as_ptr());
        }
    }

    return test;

    #[inline(never)]
    fn wut() {
        drop(std::panic::catch_unwind(|| panic!()));
    }

    #[inline(never)]
    fn mk() -> String {
        String::from("test\0")
    }
}

