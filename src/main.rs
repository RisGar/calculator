use std::env::args;

#[derive(Copy, Clone, PartialEq, Debug)]
enum Token {
  Number(f32),
  Operator(Operator),
  Parenthesis(Parenthesis),
}

#[derive(Copy, Clone, PartialEq, Debug)]
enum Operator {
  Add,
  Subtract,
  Multiply,
  Divide,
  Power,
}

impl Operator {
  fn precedence(&self) -> i32 {
    match self {
      Operator::Add | Operator::Subtract => 1,
      Operator::Multiply | Operator::Divide => 2,
      Operator::Power => 3,
    }
  }

  fn associativity(&self) -> Associativity {
    match self {
      Operator::Add | Operator::Subtract | Operator::Multiply | Operator::Divide => {
        Associativity::Left
      }
      Operator::Power => Associativity::Right,
    }
  }
}

#[derive(Copy, Clone, PartialEq, Debug)]
enum Parenthesis {
  LParen,
  RParen,
}

#[derive(PartialEq)]
enum Associativity {
  Left,
  Right,
}

impl Associativity {
  fn is_left(&self) -> bool {
    match self {
      Associativity::Left => true,
      Associativity::Right => false,
    }
  }

  fn is_right(&self) -> bool {
    !self.is_left()
  }
}

fn shunting_yard(tokens: Vec<Token>) -> Vec<Token> {
  let mut output: Vec<Token> = Vec::new();
  let mut stack: Vec<Token> = Vec::new();

  tokens.iter().for_each(|&token| match token {
    Token::Operator(operator) => {
      while let Some(Token::Operator(top)) = stack.last() {
        if (operator.associativity().is_left() && operator.precedence() <= top.precedence())
          || (operator.associativity().is_right() && operator.precedence() < top.precedence())
        {
          output.push(stack.pop().unwrap());
        } else {
          break;
        }
      }
      stack.push(token);
    }
    Token::Parenthesis(parenthesis) => match parenthesis {
      Parenthesis::LParen => stack.push(token),
      Parenthesis::RParen => loop {
        let popped = stack.pop().expect("Error: Mismatched parentheses");
        if popped == Token::Parenthesis(Parenthesis::LParen) {
          break;
        }
        output.push(popped);
      },
    },
    Token::Number(_) => output.push(token),
  });

  while let Some(token) = stack.pop() {
    output.push(token);
  }

  output
}

fn evaluate_rpn(tokens: Vec<Token>) -> f32 {
  let mut stack: Vec<f32> = Vec::new();

  tokens.iter().for_each(|&token| match token {
    Token::Number(n) => stack.push(n),
    Token::Operator(operator) => {
      let right = stack.pop().expect("Error: Invalid expression");
      let left = stack.pop().expect("Error: Invalid expression");
      let result = match operator {
        Operator::Add => left + right,
        Operator::Subtract => left - right,
        Operator::Multiply => left * right,
        Operator::Divide => left / right,
        Operator::Power => left.powf(right),
      };
      stack.push(result);
    }
    _ => panic!("Error: Invalid expression"),
  });

  stack.pop().expect("Error: Invalid expression")
}

fn tokenise(str: &mut String) -> Vec<Token> {
  let mut tokens = Vec::new();
  let mut number_buffer = String::new();

  str.chars().for_each(|c| match c {
    '0'..='9' | '.' | ',' => number_buffer.push(c),
    '+' => push_non_number(
      &mut tokens,
      &mut number_buffer,
      Token::Operator(Operator::Add),
    ),
    '-' => push_non_number(
      &mut tokens,
      &mut number_buffer,
      Token::Operator(Operator::Subtract),
    ),
    '*' => push_non_number(
      &mut tokens,
      &mut number_buffer,
      Token::Operator(Operator::Multiply),
    ),
    '/' | ':' => push_non_number(
      &mut tokens,
      &mut number_buffer,
      Token::Operator(Operator::Divide),
    ),
    '^' => push_non_number(
      &mut tokens,
      &mut number_buffer,
      Token::Operator(Operator::Power),
    ),
    '(' => push_non_number(
      &mut tokens,
      &mut number_buffer,
      Token::Parenthesis(Parenthesis::LParen),
    ),
    ')' => push_non_number(
      &mut tokens,
      &mut number_buffer,
      Token::Parenthesis(Parenthesis::RParen),
    ),
    _ => (),
  });

  empty_number_buffer(&mut tokens, &mut number_buffer);

  tokens
}

fn push_non_number(tokens: &mut Vec<Token>, number_buffer: &mut String, token: Token) {
  empty_number_buffer(tokens, number_buffer);
  tokens.push(token);
}

fn empty_number_buffer(tokens: &mut Vec<Token>, number_buffer: &mut String) {
  if !number_buffer.is_empty() {
    tokens.push(Token::Number(
      number_buffer
        .parse()
        .expect("Error: Invalid number in expression"),
    ));
    number_buffer.clear();
  }
}

fn main() {
  let expression = args().nth(1).expect("Error: No expression provided");
  let result = evaluate_rpn(shunting_yard(tokenise(&mut expression.clone())));
  println!("{:?}", result);
}

#[cfg(test)]
mod tests {
  use super::*;

  fn evaluate_expression(expression: &str) -> f32 {
    evaluate_rpn(shunting_yard(tokenise(&mut expression.to_string())))
  }

  #[test]
  fn test_basic_operations() {
    assert_eq!(evaluate_expression("2+2"), 4.0);
    assert_eq!(evaluate_expression("5-3"), 2.0);
    assert_eq!(evaluate_expression("4*2"), 8.0);
    assert_eq!(evaluate_expression("8/2"), 4.0);
  }

  #[test]
  fn test_operator_precedence() {
    assert_eq!(evaluate_expression("2+3*4"), 14.0);
    assert_eq!(evaluate_expression("2*3+4"), 10.0);
    assert_eq!(evaluate_expression("2+3*4-5"), 9.0);
  }

  #[test]
  fn test_parentheses_handling() {
    assert_eq!(evaluate_expression("(2+3)*4"), 20.0);
    assert_eq!(evaluate_expression("2*(3+4)"), 14.0);
    assert_eq!(evaluate_expression("2*(3+4)-5"), 9.0);
  }

  #[test]
  fn test_edge_cases() {
    assert_eq!(evaluate_expression("0+0"), 0.0);
    assert_eq!(evaluate_expression("0*1000"), 0.0);
    assert_eq!(evaluate_expression("1000000/1"), 1000000.0);
    assert_eq!(evaluate_expression("1/1000000"), 0.000001);
  }
}
