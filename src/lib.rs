// #![feature(ptr_internals)]
// #![feature(allocator_api)]
// #![feature(alloc_layout_extra)]
// #![feature(fmt_internals)]
// #![feature(vec_remove_item)]
#![allow(unused)]

use std::io::{BufRead, Read, Write};

pub mod algo;
// pub mod deque;
// pub mod list;
// pub mod utils;
// pub mod vec;
// //pub mod hash;
// pub mod expr;
// pub mod graph;
// pub mod heap;
// pub mod tree;
// generalized list
// pub mod concurrent;
// pub mod glist;
pub mod leetcode;
// pub mod skiplist;
// pub mod sync;

fn test_it(f: impl Fn(&[u8], &mut Vec<u8>), input: &str, output: &str) {
    let mut buf: Vec<u8> = vec![];
    f(input.as_bytes(), &mut buf);
    assert_eq!(output, unsafe { String::from_utf8_unchecked(buf) });
}
