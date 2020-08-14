// #![feature(ptr_internals)]
// #![feature(allocator_api)]
// #![feature(alloc_layout_extra)]
// #![feature(fmt_internals)]
// #![feature(vec_remove_item)]
#![allow(unused)]
#![allow(non_snake_case)]

use std::io::{BufRead, Read, Write};

pub mod deque;
pub mod list;
pub mod utils;
pub mod vec;
//pub mod hash;
pub mod expr;
pub mod graph;
pub mod heap;
pub mod tree;
// generalized list
pub mod concurrent;
pub mod glist;
pub mod skiplist;
pub mod sync;

