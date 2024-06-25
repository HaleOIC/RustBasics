use std::collections::HashMap;

use crate::pen::Pen;
enum ArithmeticOperator {
    Add,
    Sub,
    Mul,
    Div,
}

impl ArithmeticOperator {
    fn from_token(op: &str) -> Option<Self> {
        match op {
            "+" => Some(ArithmeticOperator::Add),
            "-" => Some(ArithmeticOperator::Sub),
            "*" => Some(ArithmeticOperator::Mul),
            "/" => Some(ArithmeticOperator::Div),
            _ => None,
        }
    }
}

enum CompareOperator {
    Eq,
    Neq,
    Gt,
    Lt,
    AND,
    OR,
}

impl CompareOperator {
    fn from_token(op: &str) -> Option<Self> {
        match op {
            "EQ" => Some(CompareOperator::Eq),
            "NE" => Some(CompareOperator::Neq),
            "GT" => Some(CompareOperator::Gt),
            "LT" => Some(CompareOperator::Lt),
            "AND" => Some(CompareOperator::AND),
            "OR" => Some(CompareOperator::OR),
            _ => None,
        }
    }
}

pub struct MathExpression {
    arg1: Box<Expression>,
    arg2: Box<Expression>,
    operator: ArithmeticOperator,
}

pub struct CompareExpression {
    arg1: Box<Expression>,
    arg2: Box<Expression>,
    operator: CompareOperator,
}

pub struct QueryExpression {
    command: String, 
}

pub enum Expression {
    Value(f32),
    Variable(String),
    Arithmetic(MathExpression),
    Comparison(CompareExpression),
    Query(QueryExpression),
}

impl Expression {
    pub fn evaluate(&self, values: &HashMap<String, f32>, pen: &Pen) -> Option<f32> {
        match self {
            // case1: return itself
            Expression::Value(v) => Some(*v),
            // case2: return value of variable
            Expression::Variable(v) => match values.get(v) {
                Some(val) => Some(*val),
                None => None
            },
            // case3: evaluate the math expression
            Expression::Arithmetic(e) => {
                let arg1 = e.arg1.evaluate(values, pen)?;
                let arg2 = e.arg2.evaluate(values, pen)?;
                match e.operator {
                    ArithmeticOperator::Add => Some(arg1 + arg2),
                    ArithmeticOperator::Sub => Some(arg1 - arg2),
                    ArithmeticOperator::Mul => Some(arg1 * arg2),
                    ArithmeticOperator::Div => Some(arg1 / arg2),
                }
            }
            // case4: evaluate the comparison expression
            Expression::Comparison(e) => {
                let arg1 = e.arg1.evaluate(values, pen)?;
                let arg2 = e.arg2.evaluate(values, pen)?;
                match e.operator {
                    CompareOperator::Eq => {
                        if arg1 == arg2 {
                            Some(1.0)
                        } else {
                            Some(0.0)
                        }
                    }
                    CompareOperator::Neq => {
                        if arg1 != arg2 {
                            Some(1.0)
                        } else {
                            Some(0.0)
                        }
                    }
                    CompareOperator::Gt => {
                        if arg1 > arg2 {
                            Some(1.0)
                        } else {
                            Some(0.0)
                        }
                    }
                    CompareOperator::Lt => {
                        if arg1 < arg2 {
                            Some(1.0)
                        } else {
                            Some(0.0)
                        }
                    }
                    CompareOperator::AND => {
                        if arg1 != 0.0 && arg2 != 0.0 {
                            Some(1.0)
                        } else {
                            Some(0.0)
                        }
                    }
                    CompareOperator::OR => {
                        if arg1 != 0.0 || arg2 != 0.0 {
                            Some(1.0)
                        } else {
                            Some(0.0)
                        }
                    }
                }
            }
            // Case 5: Query for pen status
            Expression::Query(e) => {
                match e.command.as_str() {
                    "XCOR" => {
                        Some(pen.get_x())
                    }
                    "YCOR" => {
                        Some(pen.get_y())
                    }
                    "HEADING" => {
                        Some(pen.get_degree() as f32)
                    }
                    "COLOR" => {
                        Some(pen.get_color_number() as f32)
                    }
                    _ => {
                        None
                    }
                }
            }
        }
    }
}

/// parse given token stream into a single expression
impl Expression {
    pub fn parse_expression(tokens: &Vec<String>, cur: usize) -> Option<(Expression, usize)> {
        // incase out of given tokens length
        if cur >= tokens.len() {
            return None;
        }

        // case1: if the first token is a number
        let token = &tokens[cur][..];
        if token.starts_with("\"") {
            return match token[1..].parse::<f32>() {
                Ok(value) => Some((Expression::Value(value), cur)),
                Err(_) => {None}
            };
        }

        // case2: if the first token is a variable
        if token.starts_with(":") {
            return Some((Expression::Variable(String::from(&token[1..])), cur));
        }

        // case3: if the first token is a math expression
        let operator = ArithmeticOperator::from_token(token);
        if operator.is_some() {
            let (exp1, end) = Expression::parse_expression(tokens, cur + 1)?;
            let (exp2, end) = Expression::parse_expression(tokens, end + 1)?;
            return Some((
                Expression::Arithmetic(MathExpression {
                    arg1: Box::new(exp1),
                    arg2: Box::new(exp2),
                    operator: operator.unwrap(),
                }),
                end,
            ));
        }

        // case4: if the first token is a compare expression
        let operator = CompareOperator::from_token(token);
        if operator.is_some() {
            let (exp1, end) = Expression::parse_expression(tokens, cur + 1)?;
            let (exp2, end) = Expression::parse_expression(tokens, end + 1)?;
            return Some((
                Expression::Comparison(CompareExpression {
                    arg1: Box::new(exp1),
                    arg2: Box::new(exp2),
                    operator: operator.unwrap(),
                }),
                end,
            ));
        }

        // case5: if the first token is a query expression
        let strings = ["XCOR", "YCOR", "HEADING", "COLOR"];
        if strings.contains(&token) {
            return Some((Expression::Query(QueryExpression {command: String::from(token)}), cur));
        }

        // default case: None
        None
    }
}

#[cfg(test)]
mod tests {
    use crate::pen;

    use super::*;

    #[test]
    fn test_simple_value_expression() {
        let tokens = vec!["\"3.0".to_string()];
        let (expression, _pos) = Expression::parse_expression(&tokens, 0).unwrap();
        let values = HashMap::new();
        let pen = pen::Pen::new(200, 200);
        assert_eq!(expression.evaluate(&values, &pen).unwrap(), 3.0);
    }

    #[test]
    fn test_variable_expression() {
        let tokens = vec![":x".to_string()];
        let (expression, _pos) = Expression::parse_expression(&tokens, 0).unwrap();
        let mut values = HashMap::new();
        values.insert("x".to_string(), 10.0);
        let pen = pen::Pen::new(200, 200);
        assert_eq!(expression.evaluate(&values, &pen).unwrap(), 10.0);
    }

    #[test]
    fn test_arithmetic_expression() {
        let tokens = vec!["+".to_string(), "\"5.0".to_string(), "\"3.0".to_string()];
        let (expression, _pos) = Expression::parse_expression(&tokens, 0).unwrap();
        let values = HashMap::new();
        let pen = pen::Pen::new(200, 200);

        assert_eq!(expression.evaluate(&values, &pen).unwrap(), 8.0);
    }

    #[test]
    fn test_comparison_expression() {
        let tokens = vec!["GT".to_string(), "\"5.0".to_string(), "\"3.0".to_string()];
        let (expression, _pos) = Expression::parse_expression(&tokens, 0).unwrap();
        let values = HashMap::new();
        let pen = pen::Pen::new(200, 200);

        assert_eq!(expression.evaluate(&values, &pen).unwrap(), 1.0);
    }

    #[test]
    fn test_nested_arithmetic_expression() {
        let tokens = vec![
            "+".to_string(),
            "\"2.0".to_string(),
            "*".to_string(),
            "\"3.0".to_string(),
            "\"4.0".to_string(),
        ];
        let (expression, _pos) = Expression::parse_expression(&tokens, 0).unwrap();
        let values = HashMap::new();
        let pen = pen::Pen::new(200, 200);

        // This should evaluate 2 + (3 * 4) = 14
        assert_eq!(expression.evaluate(&values, &pen).unwrap(), 14.0);
    }

    #[test]
    fn test_combined_arithmetic_and_comparison() {
        let tokens = vec![
            "EQ".to_string(),
            "+".to_string(),
            "\"2.0".to_string(),
            "\"3.0".to_string(),
            "\"5.0".to_string(),
        ];
        let (expression, _pos) = Expression::parse_expression(&tokens, 0).unwrap();
        let values = HashMap::new();
        let pen = pen::Pen::new(200, 200);

        // This should evaluate (2 + 3) == 5 to be true, hence 1.0
        assert_eq!(expression.evaluate(&values, &pen).unwrap(), 1.0);
    }

    #[test]
    fn test_logical_operations() {
        let tokens = vec![
            "AND".to_string(),
            "NE".to_string(),
            "\"5.0".to_string(),
            "\"3.0".to_string(),
            "EQ".to_string(),
            "\"2.0".to_string(),
            "\"2.0".to_string(),
        ];
        let (expression, _pos) = Expression::parse_expression(&tokens, 0).unwrap();
        let values = HashMap::new();
        let pen = pen::Pen::new(200, 200);

        // This should evaluate (5 != 3) AND (2 == 2), both true, hence 1.0
        assert_eq!(expression.evaluate(&values, &pen).unwrap(), 1.0);
    }

    #[test]
    fn test_error_handling_missing_variable() {
        let tokens = vec![":x".to_string()];
        let (expression, _pos) = Expression::parse_expression(&tokens, 0).unwrap();
        let values = HashMap::new();
        let pen = pen::Pen::new(200, 200);

        // Since :x is not in values, this should panic
        let result = expression.evaluate(&values, &pen);
        assert!(result.is_none());
    }

    #[test]
    fn test_error_handling_malformed_expression() {
        let tokens = vec!["+".to_string(), "\"5.0".to_string()];
        let result = Expression::parse_expression(&tokens, 0);

        // The expression is malformed because it lacks a second operand for '+'
        assert!(result.is_none());
    }

    #[test]
    fn test_deep_nested_expressions_with_variables() {
        let tokens = vec![
            "EQ".to_string(),
            "+".to_string(),
            ":x".to_string(),
            "*".to_string(),
            ":y".to_string(),
            "+".to_string(),
            "\"3.0".to_string(),
            "\"2.0".to_string(),
            "\"15.0".to_string(),
        ];
        let (expression, _pos) = Expression::parse_expression(&tokens, 0).unwrap();
        let mut values = HashMap::new();
        values.insert("x".to_string(), 5.0); // x = 5
        values.insert("y".to_string(), 4.0); // y = 4
        let pen = pen::Pen::new(200, 200);

        // The expression is:
        // (x + (y * (3 + 2))) == 15
        // which simplifies to:
        // (5 + (4 * 5)) == 15
        // which evaluates to:
        // (5 + 20) == 15 -> 25 == 15 -> false -> 0.0
        assert_eq!(expression.evaluate(&values, &pen).unwrap(), 0.0);
    }

    #[test]
    fn test_query_expression() {
        let values = HashMap::new();
        let mut pen = pen::Pen::new(200, 300); // Assuming this constructor sets initial x, y, degree, and color index
        pen.set_color(5);
        pen.set_degree(270);

        // Test XCOR query
        let tokens = vec!["XCOR".to_string()];
        let (expression, _pos) = Expression::parse_expression(&tokens, 0).unwrap();
        assert_eq!(expression.evaluate(&values, &pen).unwrap(), 100.0);

        // Test YCOR query
        let tokens = vec!["YCOR".to_string()];
        let (expression, _pos) = Expression::parse_expression(&tokens, 0).unwrap();
        assert_eq!(expression.evaluate(&values, &pen).unwrap(), 150.0);

        // Test HEADING query
        let tokens = vec!["HEADING".to_string()];
        let (expression, _pos) = Expression::parse_expression(&tokens, 0).unwrap();
        assert_eq!(expression.evaluate(&values, &pen).unwrap(), 270.0);

        // Test COLOR query
        let tokens = vec!["COLOR".to_string()];
        let (expression, _pos) = Expression::parse_expression(&tokens, 0).unwrap();
        assert_eq!(expression.evaluate(&values, &pen).unwrap(), 5.0);
    }
}
