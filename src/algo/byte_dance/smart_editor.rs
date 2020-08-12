fn fix_string(raw: &str) -> String {
    let mut chars = raw.chars().peekable();
    let mut ans = String::with_capacity(raw.len());
    let mut flag = false;
    while let Some(c) = chars.next() {
        if let Some(&next) = chars.peek().filter(|&&next| next == c) {
            chars.next();
            // 修复三个同样字母的情况
            while chars.peek().filter(|&&next| next == c).is_some() {
                chars.next();
            }
            if flag {
                flag = false;
                ans.push(c);
            } else {
                flag = true;
                ans.push(c);
                ans.push(c);
            }
        } else {
            flag = false;
            ans.push(c);
        }
    }

    ans
}

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).ok();
    let n: usize = buf.trim().parse().unwrap();
    for _ in 0..n {
        buf.clear();
        std::io::stdin().read_line(&mut buf).ok();
        println!("{}", fix_string(buf.as_str().trim()));
    }
}

#[test]
fn basic() {
    let data = ["helloo", "wooooooow"];
    let ans = ["hello", "woow"];
    for (&line, &result) in data.iter().zip(ans.iter()) {
        assert_eq!(fix_string(line), result);
    }
}
