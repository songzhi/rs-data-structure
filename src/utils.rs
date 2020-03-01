use unicode_width::UnicodeWidthStr;

/// ## What Rust std::fmt does:
///
/// Acording to this [doc](https://doc.rust-lang.org/std/fmt/index.html#width),if I want to print some content which is around with spaces,I should write like this.`println!("Hello {:5}!", "x");`
///
/// But actually it has bugs when dealing with like CJK chars.I suppose it is because Rust think `width` as `chars().count()`.So when I want to print a 10-chars -width line,it actually prints longer.Because CJK-like char counts 1 char but displays 2-chars-width.
///
/// ## What I want:
///
/// Here's my solution,It's simple but works:
/// ```rust ignore
/// let total_displayed_width = 10;
/// let chars_count = "你好".chars().count();
/// let content_displayed_width = UnicodeWidthStr::width("你好");
/// let width_in_formatter = total_displayed_width - (content_displayed_width - chars_count);
/// print!("{}{:^width$}{}", '|', "你好", '|', width = width_in_formatter);
/// // It prints '|   你好   |',perfectly 10-chars-width
/// ```
pub fn width_in_fmt(content: &str, width: usize) -> usize {
    let total_displayed_width = width;
    let chars_count = content.chars().count();
    let content_displayed_width = UnicodeWidthStr::width(content);
    total_displayed_width - (content_displayed_width - chars_count)
}
