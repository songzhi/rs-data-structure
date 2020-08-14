mod cube;
mod ab_exchange;
mod char_exchange;
mod chuan;
mod dict_order;
mod jump_room;
mod max_points;
mod push_box;
mod questions;
mod range_max;
mod reallocate_room;
mod sum_of_numbers;
mod task_schedule;
mod three_teams_tie;
mod user_likes;
mod xor;
// mod travel;
mod catch_kong_lian_shun;
mod coin_change;
mod feature_extraction;
mod que_hun;
pub mod smart_editor;

fn join<T: std::fmt::Display>(a: &[T]) -> String {
    use std::fmt::Write;
    let mut s = a.iter().fold(String::new(), |mut s, n| {
        write!(s, "{} ", n).ok();
        s
    });
    s.truncate(s.len() - 1);
    s
}
