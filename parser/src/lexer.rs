//! Lexing module
//! Elegua source code can be translated into separate tokens.

pub use super::token::Tok;
use std::collections::HashMap;

pub struct Lexer<T: Iterator<Item = char>> {
    chars: T,
    chr0: Option<char>,
    chr1: Option<char>,
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
    keywords.insert(String::from("lambda"), Tok::Lambda);
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

pub type Spanned<Tok> = Result<(Location, Tok, Location), LexicalError>;

impl<T> Lexer<T>
where T: Iterator<Item = char>,
{
    pub fn new(input: T) -> Self {
        let mut lxr = Lexer {
            location: Location::new(0, 0),
        };
        lxr.next_char();
        lxr.next_char();
        // Start at top row(=1) and left column(=1)
        lxr.location.row = 1;
        lxr.location.column = 1;
        lxr
    }

    fn next_char(&mut self) -> Option<char> {
        let c = self.chr0;
        let nxt = self.chars.next();
        self.chr0 = self.chr1;
        self.chr1 = nxt;
        self.location.column += 1;
        c
    }

    fn inner_next(&mut self) -> Option<Spanned<Tok>> {
        // ToDo:
    }
}

impl<T> Iterator for Lexer<T>
where T: Iterator<Item = char>,
{
    type Item = Spanned<Tok>;

    fn next(&mut self) -> Option<Self::Item> {
        // Create some sort of hash map for single char tokens:
        // let mut X = HashMap::new();
        // X.insert('=', Tok::Equal);
        let token = self.inner_next();
        trace!(
            "Lex token {:?}", token
        );
        token
    }
}

