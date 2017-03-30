#[doc(hidden)]
#[macro_export]
macro_rules! ruby_extern_fns {
    { $name:ident($( $argn:ident: $argt:ty ),*) -> $ret:ty; $($rest:tt)* } => {
        #[cfg_attr(windows, link(name="helix-runtime"))]
        extern "C" {
            pub fn $name($($argn: $argt),*) -> $ret;
        }
        ruby_extern_fns! { $($rest)* }
    };
    { $name:ident($( $argn:ident: $argt:ty ),*); $($rest:tt)* } => {
        #[cfg_attr(windows, link(name="helix-runtime"))]
        extern "C" {
            pub fn $name($($argn: $argt),*);
        }
        ruby_extern_fns! { $($rest)* }
    };

    { } => ()
}

#[doc(hidden)]
#[macro_export]
macro_rules! ruby_safe_cb_body {
    { $name:ident, $args_ptr:expr, $( $argn:ident ),* } => {
        unsafe {
            let args: &Args = &*($args_ptr as *const Args);
            $crate::$name($( args.$argn ),*)
        }
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! ruby_safe_cb {
    { $name:ident($( $argn:ident ),*) } => {
        extern "C" fn cb(args_ptr: $crate::void_ptr) -> $crate::void_ptr {
            ruby_safe_cb_body!($name, args_ptr, $( $argn ),*);
            unsafe { $crate::Qnil }.as_ptr()
        }
    };
    { $name:ident($( $argn:ident ),*) -> $ret:ty } => {
        extern "C" fn cb(args_ptr: $crate::void_ptr) -> $crate::void_ptr {
            ruby_safe_cb_body!($name, args_ptr, $( $argn ),*).as_ptr()
        }
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! ruby_safe_call {
    { $args:expr } => {
        {
            let mut state = $crate::EMPTY_TAG;

            let res = unsafe {
                let args_ptr: void_ptr = &$args as *const _ as void_ptr;
                $crate::rb_protect(cb, args_ptr, &mut state)
            };

            (res, state)
        }
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! ruby_safe_fns {
    { $name:ident($( $argn:ident: $argt:ty ),*) -> $ret:ty; $($rest:tt)* } => {
        pub fn $name($( $argn: $argt ),*) -> Result<$ret, $crate::RubyTag> {
            // FIXME: Avoid creating args struct if there are no args
            #[repr(C)]
            #[derive(Copy, Clone, Debug)]
            struct Args {
                pub $($argn: $argt),*
            };

            let args = Args {
                $($argn: $argn),*
            };

            ruby_safe_cb! {
                $name($($argn),*) -> $ret
            }

            let (res, state) = ruby_safe_call!(args);

            if !state.is_empty() {
                return Err(state);
            }

            Ok($crate::VALUE::wrap(res))
        }

        ruby_safe_fns! { $($rest)* }
    };

    { $name:ident($( $argn:ident: $argt:ty ),*); $($rest:tt)* } => {
        pub fn $name($( $argn: $argt ),*) -> Result<(), $crate::RubyTag> {
            // FIXME: Avoid creating args struct if there are no args
            #[repr(C)]
            #[derive(Copy, Clone, Debug)]
            struct Args {
                pub $($argn: $argt),*
            };

            let args = Args {
                $($argn: $argn),*
            };

            ruby_safe_cb! {
                $name($($argn),*)
            }

            let (_, state) = ruby_safe_call!(args);

            if !state.is_empty() {
                return Err(state);
            }

            Ok(())
        }

        ruby_safe_fns! { $($rest)* }
    };

    { } => ()
}

#[macro_export]
macro_rules! ruby_safe_c {
    { $($parts:tt)+ } => {
        ruby_extern_fns! {
            $($parts)+
        }

        pub mod safe {
            use $crate::*;

            ruby_safe_fns! {
                $($parts)+
            }
        }
    }
}