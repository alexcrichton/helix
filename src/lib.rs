extern crate cslice;

#[doc(hidden)]
pub extern crate libc;

#[doc(hidden)]
pub extern crate libcruby_sys as sys;
// pub use rb;

use std::ffi::CString;
use sys::{VALUE, RubyTag};

#[macro_use]
mod macros;
mod class_definition;
mod coercions;

pub use coercions::*;

pub use class_definition::{ClassDefinition, MethodDefinition};

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Class(sys::VALUE);

pub trait RubyMethod {
    fn install(self, class: VALUE, name: &str);
}

impl RubyMethod for extern "C" fn(VALUE) -> VALUE {
    fn install(self, class: VALUE, name: &str) {
        ruby_try!(sys::safe::rb_define_method(
            class,
            CString::new(name).unwrap().as_ptr(),
            self as *const libc::c_void,
            0
        ));
    }
}

impl RubyMethod for extern "C" fn(VALUE, VALUE) -> VALUE {
    fn install(self, class: VALUE, name: &str) {
        ruby_try!(sys::safe::rb_define_method(
            class,
            CString::new(name).unwrap().as_ptr(),
            self as *const libc::c_void,
            1
        ));
    }
}

#[allow(non_snake_case)]
#[inline]
fn ObjectClass() -> Class {
    Class(unsafe { sys::rb_cObject })
}

impl Class {
    pub fn new(name: &str) -> Class {
        ObjectClass().subclass(name)
    }

    pub fn subclass(&self, name: &str) -> Class {
        unsafe {
            Class(sys::rb_define_class(CString::new(name).unwrap().as_ptr(), self.0))
        }
    }

    pub fn define_method<T: RubyMethod>(&self, name: &str, method: T) {
        method.install(self.0, name);
    }
}

pub fn inspect(val: VALUE) -> String {
    unsafe { CheckedValue::<String>::new(ruby_try!(sys::safe::rb_inspect(val))).to_rust() }
}

pub type Metadata = ::sys::VALUE;


#[derive(Clone, Debug)]
pub enum Exception {
    Library(Class, String), // Ruby class, message
    Ruby(RubyTag)
}

impl Exception {
    pub fn with_message(string: String) -> Exception {
        let class = Class(unsafe { sys::rb_eRuntimeError });
        Exception::Library(class, string)
    }

    pub fn type_error(string: String) -> Exception {
        let class = Class(unsafe { sys::rb_eTypeError });
        Exception::Library(class, string)
    }

    pub fn from_any(any: Box<std::any::Any>) -> Exception {
        match any.downcast_ref::<Exception>() {
            Some(e) => e.clone(),
            None    => match any.downcast_ref::<&'static str>() {
                Some(e) => Exception::with_message(format!("{}", e)),
                None    => match any.downcast_ref::<String>() {
                    Some(e) => Exception::with_message(e.clone()),
                    None    => Exception::with_message(format!("Unknown Error; err={:?}", any))
                }
            }
        }
    }

    pub fn from_state(state: RubyTag) -> Exception {
        Exception::Ruby(state)
    }

    pub fn raise(&self) -> sys::VALUE {
        // Both of these will immediately leave the Rust stack. We need to be careful that nothing is
        // left behind. If there are memory leaks, this is definitely a possible culprit.
        match *self {
            Exception::Library(c, ref m) => {
                unsafe {
                    // We're passing in a Ruby string to this in hopes that Ruby will clean it up.
                    sys::rb_raise(c.0, sys::PRINT_VALUE_STR, m.clone().to_ruby());
                }
            }

            Exception::Ruby(t) => {
                unsafe {
                    sys::rb_jump_tag(t)
                }
            }
        }
        unsafe { sys::Qnil } // Return a Ruby nil
    }
}

unsafe impl Send for Exception {}
unsafe impl Sync for Exception {}
