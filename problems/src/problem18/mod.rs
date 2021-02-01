use failure::{format_err, Error};
use utils::{result, RetTypes};

#[derive(PartialEq, Clone, Debug)]
enum Token {
    Plus,
    Mult,
    Number(usize),
    OpenParen,
    CloseParen,
}

fn compute_expr(sub_expr: &[Token]) -> Result<usize, Error> {
    if let Token::Number(mut net) = sub_expr[0] {
        let tail = sub_expr[1..].to_vec();

        let mut add_n = |net, n| net + n;
        let mut mult_n = |net, n| net * n;

        let mut last_op: &mut dyn FnMut(usize, usize) -> usize = &mut add_n;

        // evalute forward
        for tok in tail.iter() {
            match tok {
                Token::Number(n) => {
                    net = last_op(net, *n);
                }
                Token::Plus => last_op = &mut add_n,
                Token::Mult => last_op = &mut mult_n,
                _ => {}
            }
        }

        return Ok(net);
    }

    Err(format_err!("wrong expr {:?}", sub_expr))
}

fn eval_parens(mut stack: Vec<Token>) -> Result<Vec<Token>, Error> {
    let mut sub_expr = vec![];

    // back-trace for open paren
    while !stack.is_empty() {
        let tok = stack.pop().unwrap();
        if tok == Token::OpenParen {
            break;
        }
        sub_expr.push(tok);
    }

    sub_expr.reverse();
    stack.push(Token::Number(compute_expr(&sub_expr)?));

    Ok(stack)
}

fn first_star(input: &[Vec<Token>]) -> Result<usize, Error> {
    let mut net = 0;
    for expr in input {
        let mut stack: Vec<Token> = vec![];
        for tok in expr.iter() {
            if tok == &Token::CloseParen {
                stack = eval_parens(stack)?;
            } else {
                stack.push(tok.clone());
            }
        }
        net += compute_expr(&stack)?;
    }
    Ok(net)
}

fn fold_addition(mut stack: Vec<Token>) -> Result<Vec<Token>, Error> {
    if stack.len() > 1 && stack[stack.len() - 2] == Token::Plus {
        let n = compute_expr(&stack[stack.len() - 3..])?;
        stack = stack[..stack.len() - 3].to_vec();
        stack.push(Token::Number(n));
    }

    Ok(stack)
}

fn second_star(input: &[Vec<Token>]) -> Result<usize, Error> {
    let mut net = 0;

    for expr in input {
        let mut stack: Vec<Token> = vec![];

        for tok in expr.iter() {
            match tok {
                Token::CloseParen => {
                    stack = eval_parens(stack)?;
                    stack = fold_addition(stack)?;
                }
                Token::Number(_) => {
                    stack.push(tok.clone());
                    stack = fold_addition(stack)?;
                }
                _ => stack.push(tok.clone()),
            };
        }

        net += compute_expr(&stack)?;
    }

    Ok(net)
}

fn parse(input_raw: &str) -> Result<Vec<Vec<Token>>, Error> {
    let mut res = vec![];

    for line in input_raw.lines() {
        let mut tmp = vec![];
        let normalized = line.replace(' ', "");
        for c in normalized.chars() {
            let token = match c {
                '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                    Token::Number(c.to_digit(10).unwrap() as usize)
                }
                '(' => Token::OpenParen,
                ')' => Token::CloseParen,
                '+' => Token::Plus,
                '*' => Token::Mult,
                _ => return Err(format_err!("wrong input")),
            };
            tmp.push(token);
        }
        res.push(tmp);
    }

    Ok(res)
}

#[allow(clippy::unnecessary_wraps)]
pub(crate) fn solve() -> Result<RetTypes, Error> {
    let input_raw = include_str!("./input");
    let parsed = parse(input_raw)?;

    Ok(RetTypes::Usize(result(
        first_star(&parsed),
        second_star(&parsed),
    )))
}
