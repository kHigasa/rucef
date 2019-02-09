//! Lexing module
//! Elegua source code can be translated into separate tokens.

pub use super::token::Tok;
use std::collections::HashMap;

pub struct Lexer<T: Iterator<Item = char>> {
    chars: T,
    at_begin_of_line: bool,
    pending: Vec<Spanned<Tok>>,
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
            chars: input,
            at_begin_of_line: true,
            pending: Vec::new(),
            chr0: None,
            chr1: None,
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

    fn next_line(&mut self) {
        self.at_begin_of_line = true;
        self.row += 1;
        self.colomn = 1;
    }

    fn lex_comment(&mut self) {
        self.next_char();
        // Skip everything until end of line.
        loop {
            match self.chr0 {
                Some('\n') => {
                    return;
                }
                Some(_) => {}
                None => return,
            }
            self.next_char();
        }
    }

    fn inner_next(&mut self) -> Option<Spanned<Tok>> {
        if !self.pending.is_empty() {
            return Some(self.pending.remove(0));
        }

        'top_loop: loop {
            if self.at_begin_of_line {
                self.at_begin_of_line = false;
                
                let mut spaces: usize = 0;
                let mut tabs: usize = 0;
                loop {
                    match self.chr0 {
                        Some(' ') => {
                            self.next_char();
                            spaces += 1;
                        }
                        Some('\t') => {
                            if spaces != 0 {
                                // Don't allow tabs after spaces as part of indentation.
                                // Tabs after spaces is even more insane than mixing spaces and tabs.
                                panic!("Tabs not allowed as part of indentation after spaces");
                            }
                            self.next_char();
                            tabs += 1;
                        }
                        Some("//") => {
                            self.lex_comment();
                            self.at_begin_of_line = true;
                            continue 'top_loop;
                        }
                        Some('\n') => {
                            self.next_char();
                            self.next_line();
                            continue 'top_loop;
                        }
                        _ => {
                            break;
                        }
                    }
                }
            }
        }
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

