// --- Day 18: Operation Order ---
// As you look out the window and notice a heavily-forested continent slowly appear over the
// horizon, you are interrupted by the child sitting next to you. They're curious if you could help
// them with their math homework.
//
// Unfortunately, it seems like this "math" follows different rules than you remember.
//
// The homework (your puzzle input) consists of a series of expressions that consist of
// addition (+), multiplication (*), and parentheses ((...)). Just like normal math, parentheses
// indicate that the expression inside must be evaluated before it can be used by the surrounding
// expression. Addition still finds the sum of the numbers on both sides of the operator, and
// multiplication still finds the product.
//
// However, the rules of operator precedence have changed. Rather than evaluating multiplication
// before addition, the operators have the same precedence, and are evaluated left-to-right
// regardless of the order in which they appear.
//
// For example, the steps to evaluate the expression 1 + 2 * 3 + 4 * 5 + 6 are as follows:
//
// 1 + 2 * 3 + 4 * 5 + 6
// 3   * 3 + 4 * 5 + 6
// 9   + 4 * 5 + 6
// 13   * 5 + 6
// 65   + 6
// 71
// Parentheses can override this order; for example, here is what happens if parentheses are added
// to form 1 + (2 * 3) + (4 * (5 + 6)):
//
// 1 + (2 * 3) + (4 * (5 + 6))
// 1 +    6    + (4 * (5 + 6))
// 7      + (4 * (5 + 6))
// 7      + (4 *   11   )
// 7      +     44
// 51
// Here are a few more examples:
//
// 2 * 3 + (4 * 5) becomes 26.
// 5 + (8 * 3 + 9 + 3 * 4 * 3) becomes 437.
// 5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4)) becomes 12240.
// ((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2 becomes 13632.
// Before you can help with the homework, you need to understand it yourself. Evaluate the
// expression on each line of the homework; what is the sum of the resulting values?

use std::borrow::BorrowMut;
use std::str::Chars;

#[derive(PartialEq, Debug, Clone)]
enum Op {
    Add,
    Multiply,
}

#[derive(PartialEq, Debug, Clone)]
enum Precedence {
    None,
    Multiplication,
}

fn eval_expression(precedence: &Precedence, chars: &mut Chars) -> u64 {
    let mut acc = 0_u64;
    let mut op: Option<Op> = None;
    let mut val = 0_u64;

    fn consume_val(acc: &mut u64, op: &mut Option<Op>, val: &mut u64) {
        match op.as_ref() {
            None => return,
            Some(op) => match op {
                Op::Add => *acc += *val,
                Op::Multiply => *acc *= *val,
            },
        }
        *op = None;
        *val = 0;
    }

    loop {
        match chars.next() {
            None => {
                consume_val(&mut acc, &mut op, &mut val);
                return acc;
            }
            Some(char) => match char {
                '(' => match op {
                    None => acc = eval_expression(precedence, chars),
                    Some(_) => val = eval_expression(precedence, chars),
                },
                ' ' => {
                    if op.is_some() && val != 0 {
                        consume_val(&mut acc, &mut op, &mut val)
                    }
                }
                '+' => op = Some(Op::Add),
                '*' => match precedence {
                    Precedence::None => op = Some(Op::Multiply),
                    Precedence::Multiplication => {
                        op = Some(Op::Multiply);
                        val = eval_expression(precedence, chars);
                        consume_val(&mut acc, &mut op, &mut val);
                        return acc;
                    }
                },
                ')' => {
                    consume_val(&mut acc, &mut op, &mut val);
                    return acc;
                }
                char => match op {
                    None => acc = acc * 10 + char.to_digit(10).unwrap() as u64,
                    Some(_) => val = val * 10 + char.to_digit(10).unwrap() as u64,
                },
            },
        }
    }
}

pub fn part1(lines: &[String]) -> u64 {
    lines
        .iter()
        .map(|line| eval_expression(&Precedence::None, line.chars().borrow_mut()))
        .sum()
}

// --- Part Two ---
// You manage to answer the child's questions and they finish part 1 of their homework, but get
// stuck when they reach the next section: advanced math.
//
// Now, addition and multiplication have different precedence levels, but they're not the ones
// you're familiar with. Instead, addition is evaluated before multiplication.
//
// For example, the steps to evaluate the expression 1 + 2 * 3 + 4 * 5 + 6 are now as follows:
//
// 1 + 2 * 3 + 4 * 5 + 6
// 3   * 3 + 4 * 5 + 6
// 3   *   7   * 5 + 6
// 3   *   7   *  11
// 21       *  11
// 231
// Here are the other examples from above:
//
// 1 + (2 * 3) + (4 * (5 + 6)) still becomes 51.
// 2 * 3 + (4 * 5) becomes 46.
// 5 + (8 * 3 + 9 + 3 * 4 * 3) becomes 1445.
// 5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4)) becomes 669060.
// ((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2 becomes 23340.
// What do you get if you add up the results of evaluating the homework problems using these new
// rules?

pub fn part2(lines: &[String]) -> u64 {
    lines
        .iter()
        .map(|line| eval_expression(&Precedence::Multiplication, line.chars().borrow_mut()))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_eval_expression() {
        assert_eq!(
            eval_expression(&Precedence::None, "2 * 3".chars().borrow_mut()),
            6
        );
        assert_eq!(
            eval_expression(&Precedence::None, "2 + 3".chars().borrow_mut()),
            5
        );
        assert_eq!(
            eval_expression(&Precedence::None, "2 + (2 * 3)".chars().borrow_mut()),
            8
        );

        assert_eq!(
            eval_expression(
                &Precedence::None,
                "1 + (2 * 3) + (4 * (5 + 6))".chars().borrow_mut()
            ),
            51
        );
        assert_eq!(
            eval_expression(&Precedence::None, "2 * 3 + (4 * 5)".chars().borrow_mut()),
            26
        );
        assert_eq!(
            eval_expression(
                &Precedence::None,
                "5 + (8 * 3 + 9 + 3 * 4 * 3)".chars().borrow_mut()
            ),
            437
        );
        assert_eq!(
            eval_expression(
                &Precedence::None,
                "5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"
                    .chars()
                    .borrow_mut()
            ),
            12240
        );
        assert_eq!(
            eval_expression(
                &Precedence::None,
                "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"
                    .chars()
                    .borrow_mut()
            ),
            13632
        );
    }

    #[test]
    pub fn test_eval_expression_pt2() {
        assert_eq!(
            eval_expression(&Precedence::Multiplication, "2 * 3".chars().borrow_mut()),
            6
        );
        assert_eq!(
            eval_expression(&Precedence::Multiplication, "2 + 3".chars().borrow_mut()),
            5
        );
        assert_eq!(
            eval_expression(
                &Precedence::Multiplication,
                "2 * 2 + 3".chars().borrow_mut()
            ),
            10
        );

        assert_eq!(
            eval_expression(
                &Precedence::Multiplication,
                "1 + (2 * 3) + (4 * (5 + 6))".chars().borrow_mut()
            ),
            51
        );
        assert_eq!(
            eval_expression(
                &Precedence::Multiplication,
                "2 * 3 + (4 * 5)".chars().borrow_mut()
            ),
            46
        );
        assert_eq!(
            eval_expression(
                &Precedence::Multiplication,
                "5 + (8 * 3 + 9 + 3 * 4 * 3)".chars().borrow_mut()
            ),
            1445
        );
        assert_eq!(
            eval_expression(
                &Precedence::Multiplication,
                "5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"
                    .chars()
                    .borrow_mut()
            ),
            669060
        );
        assert_eq!(
            eval_expression(
                &Precedence::Multiplication,
                "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"
                    .chars()
                    .borrow_mut()
            ),
            23340
        );
        // Note: Decomposition of the last part that was failing
        assert_eq!(
            eval_expression(
                &Precedence::Multiplication,
                "(2 + 4 * 9)".chars().borrow_mut()
            ),
            54
        );
        assert_eq!(
            eval_expression(
                &Precedence::Multiplication,
                "(6 + 9 * 8 + 6)".chars().borrow_mut()
            ),
            210
        );
        assert_eq!(
            eval_expression(
                &Precedence::Multiplication,
                "(6 + 9 * 8 + 6) + 6".chars().borrow_mut()
            ),
            216
        );
        assert_eq!(
            eval_expression(
                &Precedence::Multiplication,
                "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6)".chars().borrow_mut()
            ),
            11664
        );
        assert_eq!(
            eval_expression(
                &Precedence::Multiplication,
                "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2"
                    .chars()
                    .borrow_mut()
            ),
            11666
        );
        assert_eq!(
            eval_expression(
                &Precedence::Multiplication,
                "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 "
                    .chars()
                    .borrow_mut()
            ),
            11670
        );
    }
}
