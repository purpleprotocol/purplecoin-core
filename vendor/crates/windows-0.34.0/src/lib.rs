/*!
Learn more about Rust for Windows here: <https://github.com/microsoft/windows-rs>
*/

#![doc(html_no_source)]
#![allow(non_snake_case, non_upper_case_globals, non_camel_case_types, dead_code, unused_variables)]

extern crate self as windows;
mod Windows;
pub mod core;
pub use Windows::*;
