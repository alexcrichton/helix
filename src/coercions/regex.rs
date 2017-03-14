use libc;
use std;
use sys;
use regex::Regex;
use sys::{VALUE};
use std::ffi::CString;

use super::{UncheckedValue, CheckResult, CheckedValue, ToRust, ToRuby};

// VALUE -> to_coercible_rust<String> -> CheckResult<String> -> unwrap() -> Coercible<String> -> to_rust() -> String

impl UncheckedValue<Regex> for VALUE {
    fn to_checked(self) -> CheckResult<Regex> {
        if unsafe { sys::RB_TYPE_P(self, sys::T_REGEXP) } {
            Ok(unsafe { CheckedValue::<Regex>::new(self) })
        } else {
            let val = unsafe { CheckedValue::<String>::new(sys::rb_inspect(self)) };
            Err(CString::new(format!("No implicit conversion of {} into Regex", val.to_rust())).unwrap())
        }
    }
}

impl ToRust<Regex> for CheckedValue<Regex> {
    fn to_rust(self) -> Regex {
        unsafe {
          let ruby_str = sys::rb_funcall(self.inner, sys::rb_intern(CString::new("to_s").unwrap().as_ptr()), 0);
          let size = sys::RSTRING_LEN(ruby_str);
          let ptr = sys::RSTRING_PTR(ruby_str);
          let slice = std::slice::from_raw_parts(ptr as *const u8, size as usize);
          Regex::new(std::str::from_utf8(slice).unwrap()).unwrap()
        }
    }
}

impl ToRuby for Regex {
    fn to_ruby(self) -> VALUE {
        let string = self.to_string();
        let ptr = string.as_ptr();
        let len = string.len();
        let ruby_str = unsafe { sys::rb_utf8_str_new(ptr as *const libc::c_char, len as libc::c_long) };
        let class_id = unsafe { sys::rb_intern(CString::new("Regexp").unwrap().as_ptr()) };
        let klass = unsafe { sys::rb_const_get(sys::rb_cObject, class_id) };
        let args = [ruby_str];
        unsafe { sys::rb_class_new_instance(args.len() as isize, args.as_ptr(), klass) }
    }
}
