//! Lexing module
//! Elegua source code can be translated into separate tokens.

pub use super::token::Tok;
use std::collections::HashMap;

pub struct Lexer<T: Iterator<Item = char>> {
    location: Location,
}

#[derive(Debug)]
pub enum LexicalError {
    StringError,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Location {
    row usize,
    column usize,
}

impl Location {
    pub fn new(row: usize, column: usize) -> Self {
        Location {
            row: row,
            column: column,
        }
    }

    pub fn get_row(&self) -> usize {
        self.row
    }

    pub fn get_column(&self) -> usize {
        self.column
    }
}

pub fn get_keywords() -> HashMap<String, Tok> {
    let mut keywords: HashMap<String, Tok> = HashMap::new();

    // Keywords (alphabetically):
    keywords.insert(String::from("and"), Tok::And);
    keywords.insert(String::from("break"), Tok::Break);
    keywords.insert(String::from("case"), Tok::Case);
    keywords.insert(String::from("class"), Tok::Class);
    keywords.insert(String::from("continue"), Tok::Continue);
    keywords.insert(String::from("def"), Tok::Def);
    keywords.insert(String::from("do"), Tok::Do);
    keywords.insert(String::from("elif"), Tok::Elif);
    keywords.insert(String::from("else"), Tok::Else);
    keywords.insert(String::from("end"), Tok::End);
    keywords.insert(String::from("false"), Tok::False);
    keywords.insert(String::from("for"), Tok::For);
    keywords.insert(String::from("from"), Tok::From);
    keywords.insert(String::from("global"), Tok::Global);
    keywords.insert(String::from("if"), Tok::If);
    keywords.insert(String::from("import"), Tok::Import);
    keywords.insert(String::from("in"), Tok::In);
    keywords.insert(String::from("module"), Tok::Module);
    keywords.insert(String::from("null"), Tok::Null);
    keywords.insert(String::from("not"), Tok::Not);
    keywords.insert(String::from("or"), Tok::Or);
    keywords.insert(String::from("return"), Tok::Return);
    keywords.insert(String::from("self"), Tok::Self);
    keywords.insert(String::from("super"), Tok::Super);
    keywords.insert(String::from("true"), Tok::True);
    keywords.insert(String::from("while"), Tok::While);
    keywords.insert(String::from("yield"), Tok::Yield);
    keywords
}

