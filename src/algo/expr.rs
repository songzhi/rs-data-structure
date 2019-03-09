pub fn infix_expr_to_post(tokens: impl Iterator<Item=char>) -> String {
    let mut post_expr = String::new();
    let mut stack = vec![];
    for token in tokens {
        match token {
            '+' | '-' => {
                while let Some(op) = stack.pop() {
                    if op == '(' {
                        stack.push(op);
                        break;
                    }
                    post_expr.push(op);
                }
                stack.push(token);
            }
            '(' => stack.push(token),
            '*' | '/' => {
                while let Some(op) = stack.pop() {
                    if "+-(".contains(op) {
                        stack.push(op);
                        break;
                    }
                    post_expr.push(op);
                }
                stack.push(token);
            }
            ')' => {
                while let Some(op) = stack.pop() {
                    if op == '(' {
                        break;
                    }
                    post_expr.push(op);
                }
            }
            _ => post_expr.push(token)
        }
    }
    while let Some(op) = stack.pop() {
        post_expr.push(op);
    }
    post_expr
}

#[cfg(test)]
mod test {
    use crate::algo::expr::infix_expr_to_post;

    #[test]
    fn test_infix_expr_to_post() {
        let infix_expr = "a+b*c+(d*e+f)*g";
        let postfix_expr = infix_expr_to_post(infix_expr.chars().into_iter());
        assert_eq!("abc*+de*f+g*+", postfix_expr);
    }
}