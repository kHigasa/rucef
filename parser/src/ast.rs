//! Ast implementation
//!

pub use super::lexer::Location;
use num_bigint::BigInt;

#[derive(Debug, PartialEq)]
pub enum Top {
    Program(Program),
    Statement(LocatedStatement),
    Expression(Expression),
}

#[derive(Debug, PartialEq)]
pub struct Program {
    pub statements: Vec<LocatedStatement>,
}

pub type LocatedStatement = Located<Statement>;

#[derive(Debug, PartialEq)]
pub struct Located<T> {
    pub location: Location,
    pub node: T,
}

#[derive(Debug, PartialEq)]
pub struct SingleImport {
    pub module: String,
    // symbol name in module, name it should be assigned locally
    pub symbol: Option<String>,
    pub alias: Option<String>,
}

/// Ast nodes for elegua statements.
#[derive(Debug, PartialEq)]
pub enum Statement {
    Break,
    Continue,
    Return {
        value: Option<Vec<Expression>>,
    },
    Import {
        import_parts: Vec<SingleImport>,
    },
    Assign {
        targets: Vec<Expression>,
        value: Expression,
    },
    AugAssign {
        target: Box<Expression>,
        op: Operator,
        value: Box<Expression>,
    },
    Expression {
        expression: Expression,
    },
    Global {
        names: Vec<String>,
    },
    NonLocal {
        names: Vec<String>,
    },
    If {
        test: Expression,
        body: Vec<LocatedStatement>,
        orelse: Option<Vec<LocatedStatement>>,
    },
    While {
        test: Expression,
        body: Vec<LocatedStatement>,
        orelse: Option<Vec<LocatedStatement>>,
    },
    For {
        target: Expression,
        iter: Vec<Expression>,
        body: Vec<LocatedStatement>,
        orelse: Option<Vec<LocatedStatement>>,
    },
    ClassDef {
        name: String,
        body: Vec<LocatedStatement>,
        keywords: Vec<Keyword>,
        // ToDo:
    },
    FunctionDef {
        name: String,
        args: Parameters,
        body: Vec<LocatedStatement>,
    },
}

#[derive(Debug, PartialEq)]
pub enum Expression {
    Boolop {
        a: Box<Expression>,
        op: Operator,
        b: Box<Expression>,
    },
    Unop {
        op: UnaryOperator,
        a: Box<Expression>,
    },
    Yield {
        value: Option<Box<Expression>>,
    },
    Compare {
        a: Box<Expression>,
        op: Comparison,
        b: Box<Expression>,
    },
    Call {
        function: Box<Expression>,
        args: Vec<Expression>,
        keywords: Vec<Keyword>,
    },
    Number {
        value: Number,
    },
    Array {
        elements: Vec<Expression>,
    },
    HashMap {
        elements: Vec<(Expression, Expression)>,
    },
    Set {
        elements: Vec<Expression>,
    },
    String {
        elements: Vec<Expression>,
    },
    Identifier {
        name: String,
    },
    Lambda {
        args: Parameters,
        body: Box<Expression>,
    },
    True,
    False,
    None,
}

#[derive(Debug, PartialEq, Default)]
pub struct Parameters {
    pub args: Vec<String>,
    pub defaults: Vec<Expression>,
}

#[derive(Debug, PartialEq)]
pub struct Keyword {
    pub name: Option<String>,
    pub value: Expression,
}

#[derive(Debug, PartialEq)]
pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Pow,
}

#[derive(Debug, PartialEq)]
pub enum BooleanOperator {
    And,
    Or,
}

#[derive(Debug, PartialEq)]
pub enum UnaryOperator {
    Pos,
    Neg,
    Not,
}

#[derive(Debug, PartialEq)]
pub enum UnaryOperator {
    Equal,
    NotEqual,
    Less,
    LessOrEqual,
    Greater,
    GreaterOrEqual,
    Is,
    IsNot,
}

#[derive(Debug, PartialEq)]
pub enum Number {
    Integer { value: BigInt },
    Float { value: f64 },
}

