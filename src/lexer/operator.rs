use super::lexer::ParseFromConstStr;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Operator {
    Add,
    Sub,
    Div,
    Mul,
    Mod,
    Assign,
    Equal,
    GreaterEqual,
    GreaterThan,
    LessEqual,
    LessThan,
}

impl ParseFromConstStr for Operator {
    fn to_str(&self) -> &'static str {
        match self {
            Operator::Add => "+",
            Operator::Sub => "-",
            Operator::Div => "/",
            Operator::Mul => "*",
            Operator::Mod => "%",
            Operator::Assign => "=",
            Operator::Equal => "==",
            Operator::GreaterEqual => ">=",
            Operator::GreaterThan => ">",
            Operator::LessEqual => "<=",
            Operator::LessThan => "<",
        }
    }

    fn enumarate<'a>() -> &'a [Self]
    where
        Self: Sized,
    {
        &[
            Operator::Add,
            Operator::Sub,
            Operator::Div,
            Operator::Mul,
            Operator::Mod,
            Operator::Equal,
            Operator::Assign,
            Operator::GreaterEqual,
            Operator::GreaterThan,
            Operator::LessEqual,
            Operator::LessThan,
        ]
    }
}
