/// return the first matched index of 'sub' in 's' after 'pos' else None
pub fn index(s: &str, sub: &str, pos: usize) -> Option<usize> {
    let (mut i, mut j) = (pos, pos);
    let s = s.chars().collect::<Vec<_>>();
    let sub = sub.chars().collect::<Vec<_>>();
    let (s_len, sub_len) = (s.len(), sub.len());
    while i < s_len && j < sub_len {
        if s[i] == sub[j] {
            i += 1;
            j += 1;
        } else {
            i = i - j + 1;
            j = 0;
        }
    }
    if j >= sub_len {
        Some(i - sub_len)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_index() {
        let s = "ababcabcacbab";
        let sub = "abcac";
        let i = index(s, sub, 0);
        assert_eq!(Some(5), i);
    }
}