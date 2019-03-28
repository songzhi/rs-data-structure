//!4 阶斐波那契序列如下：f0=f1=f2=0, f3=1,…,fi=fi-1+fi-2+fi-3+fi-4.
//! 利用容量为 k=4 的循环队列，构造序列的前 n+1 项（f0, f1 , f2 ,… fn ）
//! 要求满足 fn ≤200 而 fn+1 >200。
//!
use crate::deque::Deque;

pub fn fib_4th_order() -> Vec<u16> {
    let mut seq = vec![0, 0, 0, 1];
    let mut queue = Deque::new();
    queue.push_back(0);
    queue.push_back(0);
    queue.push_back(0);
    queue.push_back(1);
    let mut f_nth = 0;
    while f_nth <= 200 {
        f_nth = 0;
        for _ in 0..4 {
            let num = queue.pop_front().unwrap();
            f_nth += num;
            queue.push_back(num);
        }
        queue.pop_front();
        queue.push_back(f_nth);
        seq.push(f_nth);
    }
    seq.pop();
    seq
}

#[cfg(test)]
mod test {
    use super::fib_4th_order;

    #[test]
    fn test_fib() {
        let seq = fib_4th_order();
        assert_eq!(vec![0, 0, 0, 1, 1, 2, 4, 8, 15, 29, 56, 108], seq);
    }
}