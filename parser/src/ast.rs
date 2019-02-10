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
    },
    FunctionDef {
        name: String,
        args: Parameters,
        body: Vec<LocatedStatement>,
    },
}

#[derive(Debug, PartialEq)]
pub enum Expression {
    
}

#[derive(Debug, PartialEq, Default)]
pub struct Parameters {
    pub args: Vec<String>,
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

