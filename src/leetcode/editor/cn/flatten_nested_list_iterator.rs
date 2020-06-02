//! [341]扁平化嵌套列表迭代器
//给你一个嵌套的整型列表。请你设计一个迭代器，使其能够遍历这个整型列表中的所有整数。 
//
// 列表中的每一项或者为一个整数，或者是另一个列表。其中列表的元素也可能是整数或是其他列表。 
//
// 
//
// 示例 1: 
//
// 输入: [[1,1],2,[1,1]]
//输出: [1,1,2,1,1]
//解释: 通过重复调用 next 直到 hasNext 返回 false，next 返回的元素的顺序应该是: [1,1,2,1,1]。 
//
// 示例 2: 
//
// 输入: [1,[4,[6]]]
//输出: [1,4,6]
//解释: 通过重复调用 next 直到 hasNext 返回 false，next 返回的元素的顺序应该是: [1,4,6]。
// 
// Related Topics 栈 设计

use std::ops::DerefMut;

#[derive(Debug, PartialEq, Eq)]
pub enum NestedInteger {
    Int(i32),
    List(Vec<NestedInteger>),
}

pub struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)

struct NestedIterator {
    stack: std::cell::RefCell<Vec<NestedInteger>>
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NestedIterator {
    fn new(nested_list: Vec<NestedInteger>) -> Self {
        Self {
            stack: std::cell::RefCell::new(vec![NestedInteger::List(nested_list)])
        }
    }

    fn next(&self) -> i32 {
        let stack = self.stack.borrow_mut();
        let mut node = self.stack.borrow_mut().deref_mut().pop().unwrap();
        while true {
            match node {
                NestedInteger::Int(n) => { return n; },
                NestedInteger::List(mut nested_list) => {
                    todo!()
                },
            }
        }
    }

    fn has_next(&self) -> bool {
        todo!()
    }
}

/*
 * Your NestedIterator object will be instantiated and called as such:
 * let obj = NestedIterator::new(nestedList);
 * let ret_1: i32 = obj.next();
 * let ret_2: bool = obj.has_next();
 */
//leetcode submit region end(Prohibit modification and deletion)
