#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[cfg(feature = "Services_Cortana")]
pub mod Cortana;
#[cfg(feature = "Services_Maps")]
pub mod Maps;
#[cfg(feature = "Services_Store")]
pub mod Store;
#[cfg(feature = "Services_TargetedContent")]
pub mod TargetedContent;
#[cfg(feature = "implement")]
::core::include!("impl.rs");
