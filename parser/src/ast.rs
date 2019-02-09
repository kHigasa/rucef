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

pub struct Located<T> {
    pub location: Location,
    pub node: T,
}

pub struct Located<T> {
    pub location: Location,
    pub node: T,
}



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
}

