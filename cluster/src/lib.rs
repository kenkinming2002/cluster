#![feature(new_uninit)]
#![feature(coroutines)]
#![feature(iter_from_coroutine)]
#![feature(type_alias_impl_trait)]
#![feature(test)]
#![feature(vec_push_within_capacity)]

#![allow(clippy::needless_range_loop)]
#![allow(clippy::map_flatten)]
#![allow(clippy::manual_memcpy)]
#![allow(clippy::type_complexity)]

extern crate test;

pub mod utils;
pub mod math;
pub mod model;

