pub mod lexer;
pub mod token;

pub trait Expr {
    fn eval(&self) -> Option<i64>;
    fn from_str() -> Self;
}

pub struct PrefixExpr(String);

pub struct InfixExpr(String);

pub struct PostfixExpr(String);


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

pub fn eval_post_expr(expr: impl Iterator<Item=char>) -> Option<i64> {
    let mut stack: Vec<i64> = vec![];
    for ch in expr {
        match ch {
            '+' => {
                let (x, y) = (stack.pop()?, stack.pop()?);
                stack.push(y + x);
            }
            '-' => {
                let (x, y) = (stack.pop()?, stack.pop()?);
                stack.push(y - x);
            }
            '*' => {
                let (x, y) = (stack.pop()?, stack.pop()?);
                stack.push(y * x);
            }
            '/' => {
                let (x, y) = (stack.pop()?, stack.pop()?);
                stack.push(y / x);
            }
            '0'..='9' => {
                stack.push(ch.to_digit(10)?.into());
            }
            _ => { return None; }
        }
    }
    stack.pop()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_infix_expr_to_post() {
        let infix_expr = "a+b*c+(d*e+f)*g";
        let postfix_expr = infix_expr_to_post(infix_expr.chars().into_iter());
        assert_eq!("abc*+de*f+g*+", postfix_expr);
    }

    #[test]
    fn test_eval_post_expr() {
        let expr = "1+2*(5-3)";
        let post_expr = infix_expr_to_post(expr.chars().into_iter());
        assert_eq!(Some(5), eval_post_expr(post_expr.chars().into_iter()));
    }
}