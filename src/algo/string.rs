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

fn set_next(sub: &Vec<char>, next: &mut [usize]) {
    let mut i = 1;
    let mut j = 0;
    let sub_len = sub.len();
    while i < sub_len {
        if j == 0 || sub[i - 1] == sub[j - 1] {
            i += 1;
            j += 1;
            next[i - 1] = j;
        } else {
            j = next[j - 1];
        }
    }
}

/// return the first matched index of 'sub' in 's' after 'pos' else None
/// same function as index but use KMP algorithm
pub fn index_kmp(s: &str, sub: &str, pos: usize) -> Option<usize> {
    let (mut i, mut j) = (pos, 0);
    let s = s.chars().collect::<Vec<_>>();
    let sub = sub.chars().collect::<Vec<_>>();
    let (s_len, sub_len) = (s.len(), sub.len());
    let mut next = vec![0usize; sub_len];
    set_next(&sub, &mut next);
    while i < s_len && j < sub_len {
        if j == 0 || s[i] == sub[j] {
            i += 1;
            j += 1;
        } else {
            j = next[j - 1];
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

    #[test]
    fn test_index_kmp() {
        let s = "ababcabcacbab";
        let sub = "abcac";
        let i = index_kmp(s, sub, 0);
        assert_eq!(Some(5), i);
    }
}
