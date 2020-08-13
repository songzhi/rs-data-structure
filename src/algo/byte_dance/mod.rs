mod range_max;
mod max_points;
mod chuan;
mod ab_exchange;
mod char_exchange;
mod push_box;
mod reallocate_room;
mod task_schedule;
mod three_teams_tie;
mod user_likes;
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
