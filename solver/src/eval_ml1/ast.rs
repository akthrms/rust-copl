#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum Expression {
    Integer(i64),
    Boolean(bool),
    If(Box<Expression>, Box<Expression>, Box<Expression>),
    Add(Box<Expression>, Box<Expression>),
    Sub(Box<Expression>, Box<Expression>),
    Mul(Box<Expression>, Box<Expression>),
    LessThan(Box<Expression>, Box<Expression>),
}
