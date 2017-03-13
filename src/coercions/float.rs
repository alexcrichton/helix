use sys::{self, VALUE, T_FLOAT};
use std::ffi::CString;

use super::{UncheckedValue, CheckResult, CheckedValue, ToRust, ToRuby};

impl UncheckedValue<f64> for VALUE {
    fn to_checked(self) -> CheckResult<f64> {
        if unsafe { sys::RB_TYPE_P(self, T_FLOAT) } {
            Ok(unsafe { CheckedValue::new(self) })
        } else {
            let val = unsafe { CheckedValue::<String>::new(sys::rb_inspect(self)) };
            Err(CString::new(format!("No implicit conversion of {} into Rust f64", val.to_rust())).unwrap())
        }
    }
}

impl ToRust<f64> for CheckedValue<f64> {
    fn to_rust(self) -> f64 {
        unsafe { sys::FLOAT2F64(self.inner) }
    }
}

impl ToRuby for f64 {
    fn to_ruby(self) -> VALUE {
        unsafe { sys::F642FLOAT(self) }
    }
}
