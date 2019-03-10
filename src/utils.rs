use unicode_width::UnicodeWidthStr;

pub fn width_in_fmt(content: &str, width: usize) -> usize {
    let total_displayed_width = width;
    let chars_count = content.chars().count();
    let content_displayed_width = UnicodeWidthStr::width(content);
    total_displayed_width - (content_displayed_width - chars_count)
}