#![feature(ptr_internals)]
#![feature(allocator_api)]
#![feature(alloc_layout_extra)]
#![feature(fmt_internals)]
#![feature(vec_remove_item)]

pub mod vec;
pub mod list;
pub mod deque;
pub mod algo;
pub mod utils;
//pub mod hash;
pub mod heap;
pub mod graph;
pub mod tree;
pub mod expr;
// generalized list
pub mod glist;
pub mod skiplist;
pub mod concurrent;