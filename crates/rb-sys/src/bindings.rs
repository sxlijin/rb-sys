//! Raw bindings to libruby, generated by bindgen.
//!
//! This module contains the raw bindings to libruby, generated by bindgen.

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unknown_lints)]
#![allow(deref_nullptr)]
#![warn(unknown_lints)]
#![allow(unaligned_references)]
#![allow(clippy::all)]

use crate::os::raw::*;

/// Type that represents a Ruby object.  It is an unsigned integer of some kind,
/// depending on platforms.
///
/// ```c
/// VALUE value = rb_eval_string("ARGF.readlines.map.with_index");
/// ```
///
/// @warning  ::VALUE is not a pointer.
/// @warning  ::VALUE can be wider than `long`.
pub type VALUE = ::libc::uintptr_t;

/// Type that represents a Ruby identifier such as a variable name.
///
/// ```CXX
/// ID method = rb_intern("method");
/// VALUE result = rb_funcall(obj, method, 0);
/// ```
///
/// @note  ::rb_cSymbol is a Ruby-level data type for the same thing.
pub type ID = ::libc::uintptr_t;

include!(env!("RB_SYS_BINDINGS_PATH"));
