//! Lexing module
//! Elegua source code can be translated into separate tokens.

pub use super::token::Tok;
use num_bigint::BigInt;
use num_traits::Num;
use std::collections::HashMap;
use std::str::FromStr;

pub struct Lexer<T: Iterator<Item = char>> {
    chars: T,
    at_begin_of_line: bool,
    nesting: usize,
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
    row: usize,
    column: usize,
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
    keywords.insert(String::from("none"), Tok::None);
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
            nesting: 0,
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

    // Traverse helper functions:
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

    fn get_loc(&self) -> Location {
        self.location.clone()
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
            match self.chr0 {
                Some('0'...'9') => return Some(self.lex_number()),
                Some('_') | Some('a'...'z') | Some('A'...'Z') => return Some(self.lex_identifier()),
                Some("//") => {
                    self.lex_comment();
                    continue;
                }
                Some('"') => {
                    return Some(self.lex_string());
                }
                Some('=') => {
                    let tok_start = self.get_loc();
                    self.next_char();
                    match self.chr0 {
                        Some('=') => {
                            self.next_char();
                            let tok_end = self.get_loc();
                            return Some(Ok((tok_start, Tok::DoubleEqual, tok_end)));
                        }
                        _ => {
                            let tok_end = self.get_loc();
                            return Some(Ok((tok_start, Tok::Equal, tok_end)));
                        }
                    }
                }
                Some('+') => {
                    let tok_start = self.get_loc();
                    self.next_char();
                    match self.chr0 {
                        Some('=') => {
                            self.next_char();
                            let tok_end = self.get_loc();
                            return Some(Ok((tok_start, Tok::PlusEqual, tok_end)));
                        }
                        _ => {
                            let tok_end = self.get_loc();
                            return Some(Ok((tok_start, Tok::Plus, tok_end)));
                        }
                    }
                }
                Some('-') => {
                    let tok_start = self.get_loc();
                    self.next_char();
                    match self.chr0 {
                        Some('=') => {
                            self.next_char();
                            let tok_end = self.get_loc();
                            return Some(Ok((tok_start, Tok::MinusEqual, tok_end)));
                        }
                        _ => {
                            let token_end = self.get_loc();
                            return Some(Ok((tok_start, Tok::Minus, tok_end)));
                        }
                    }
                }
                Some('*') => {
                    let tok_start = self.get_loc();
                    self.next_char();
                    match self.chr0 {
                        Some('=') => {
                            self.next_char();
                            let tok_end = self.get_loc();
                            return Some(Ok((tok_start, Tok::StarEqual, tok_end)));
                        }
                        Some('*') => {
                            self.next_char();
                            match self.chr0 {
                                Some('=') => {
                                    self.next_char();
                                    let tok_end = self.get_loc();
                                    return Some(Ok((tok_start, Tok::DoubleStarEqual, tok_end)));
                                }
                                _ => {
                                    let token_end = self.get_loc();
                                    return Some(Ok((tok_start, Tok::DoubleStar, tok_end)));
                                }
                            }
                        }
                        _ => {
                            let token_end = self.get_loc();
                            return Some(Ok((tok_start, Tok::Star, tok_end)));
                        }
                    }
                }
                Some('/') => {
                    let tok_start = self.get_loc();
                    self.next_char();
                    match self.chr0 {
                        Some('=') => {
                            self.next_char();
                            let tok_end = self.get_loc();
                            return Some(Ok((tok_start, Tok::SlashEqual, tok_end)));
                        }
                        _ => {
                            let token_end = self.get_loc();
                            return Some(Ok((tok_start, Tok::Slash, tok_end)));
                        }
                    }
                }
                Some('%') => {
                    let tok_start = self.get_loc();
                    self.next_char();
                    match self.chr0 {
                        Some('=') => {
                            self.next_char();
                            let tok_end = self.get_loc();
                            return Some(Ok((tok_start, Tok::PercentEqual, tok_end)));
                        }
                        _ => {
                            let token_end = self.get_loc();
                            return Some(Ok((tok_start, Tok::Percent, tok_end)));
                        }
                    }
                }
                Some('<') => {
                    let tok_start = self.get_loc();
                    self.next_char();
                    match self.chr0 {
                        Some('=') => {
                            self.next_char();
                            let tok_end = self.get_loc();
                            return Some(Ok((tok_start, Tok::LessEqual, tok_end)));
                        }
                        _ => {
                            let token_end = self.get_loc();
                            return Some(Ok((tok_start, Tok::Less, tok_end)));
                        }
                    }
                }
                Some('>') => {
                    let tok_start = self.get_loc();
                    self.next_char();
                    match self.chr0 {
                        Some('=') => {
                            self.next_char();
                            let tok_end = self.get_loc();
                            return Some(Ok((tok_start, Tok::GreaterEqual, tok_end)));
                        }
                        _ => {
                            let token_end = self.get_loc();
                            return Some(Ok((tok_start, Tok::Greater, tok_end)));
                        }
                    }
                }
                Some('!') => {
                    let tok_start = self.get_loc();
                    self.next_char();
                    match self.chr0 {
                        Some('=') => {
                            self.next_char();
                            let tok_end = self.get_loc();
                            return Some(Ok((tok_start, Tok::NotEqual, tok_end)));
                        }
                        _ => panic!("Invalid token '!'"),
                    }
                }
                Some('(') => {
                    let result = self.eat_single_char(Tok::Lparen);
                    self.nesting += 1;
                    return Some(result);
                }
                Some(')') => {
                    let result = self.eat_single_char(Tok::Rparen);
                    self.nesting -= 1;
                    return Some(result);
                }
                Some('{') => {
                    let result = self.eat_single_char(Tok::Lbrace);
                    self.nesting += 1;
                    return Some(result);
                }
                Some('}') => {
                    let result = self.eat_single_char(Tok::Rbrace);
                    self.nesting -= 1;
                    return Some(result);
                }
                Some('[') => {
                    let result = self.eat_single_char(Tok::Lbracket);
                    self.nesting += 1;
                    return Some(result);
                }
                Some(']') => {
                    let result = self.eat_single_char(Tok::Rbracket);
                    self.nesting -= 1;
                    return Some(result);
                }
                Some(',') => {
                    return Some(self.eat_single_char(Tok::Comma));
                }
                Some('.') => {
                    return Some(self.eat_single_char(Tok::Dot));
                }
                Some('\n') => {
                    let tok_start = self.get_loc();
                    self.next_char();
                    let tok_end = self.get_loc();
                    self.new_line();
                    // Whether we emit newline or not is depending on the nesting level.
                    if self.nesting == 0 {
                        self.at_begin_of_line = true;
                        return Some(Ok((tok_start, Tok::Newline, tok_end)));
                    } else {
                        continue;
                    }
                }
                Some(' ') => {
                    // Skip whitespaces.
                    self.next_char();
                    continue;
                }
                None => return None,
                _ => {
                    // Ignore all the rest.
                    let c = self.next_char();
                    panic!("Not impl {:?}", c);
                }
            }
        }
    }

    fn eat_single_char(&mut self, tok: Tok) -> Spanned<Tok> {
        let tok_start = self.get_loc();
        self.next_char();
        let tok_end = self.get_loc();
        Ok((tok_start, tok, tok_end))
    }

    // Lexer helper functions:
    fn lex_identifier(&mut self) -> Spanned<Tok> {
        let mut ident = String::new();
        let start_loc = self.get_loc();

        // Take up char into identifier.
        while self.is_char() {
            ident.push(self.next_char().unwrap());
        }

        let end_loc = self.get_loc();
        let mut keywords = get_keywords();

        if keywords.contains_key(&ident) {
            Ok((start_loc, keywords.remove(&ident).unwrap(), end_loc))
        } else {
            Ok((start_loc, Tok::Ident { ident }, end_loc))
        }
    }

    fn lex_number(&mut self) -> Spanned<Tok> {
        let start_loc = self.get_loc();
        if self.chr0 == Some('0') {
            if self.chr1 == Some('x') || self.chr1 == Some('X') {
                // Hex:
                self.next_char();
                self.next_char();
                self.lex_number_with_radix(start_loc, 16)
            } else if self.chr1 == Some('o') || self.chr1 == Some('O') {
                // Oct:
                self.next_char();
                self.next_char();
                self.lex_number_with_radix(start_loc, 8)
            } else if self.chr1 == Some('b') || self.chr1 == Some('B') {
                // Binary:
                self.next_char();
                self.next_char();
                self.lex_number_with_radix(start_loc, 2)
            } else {
                self.lex_normal_number()
            }
        } else {
            self.lex_normal_number()
        }
    }

    fn lex_normal_number(&mut self) -> Spanned<Tok> {
        let start_loc = self.get_loc();

        let mut value_str = String::new();

        // Integer:
        while self.is_number(10) {
            value_str.push(self.next_char.unwrap());
        }

        // Float:
        if self.chr0 == Some('.') || self.chr0 == Some('e') {
            // Take '.':
            if self.chr0 == Some('.') {
                value_str.push(self.next_char().unwrap());
                while self.is_number(10) {
                    value_str.push(self.next_char().unwrap());
                }
            }

            // Take 'e':
            if self.chr0 == Some('e') {
                value_str.push(self.next_char().unwrap());

                if self.chr0 == Some('+') || self.chr0 == Some('-') {
                    value_str.push(self.next_char().unwrap());
                }

                while self.is_number(10) {
                    value_str.push(self.next_char().unwrap());
                }
            }

            let value = f64::from_str(&value_str).unwrap();

            // ToDo: Complex

            let end_loc = self.get_loc();
            Ok((start_loc, Tok::Float { value }, end_loc))
        } else {
            let end_loc = self.get_loc();
            let value = value_str.parse::<BigInt>().unwrap();
            Ok((start_loc, Tok::Int { value }, end_loc))
        }
    }

    fn lex_number_with_radix(&mut self, start_loc: Location, radix: u32) -> Spanned<Tok> {
        let mut value_str = String::new();

        loop {
            if self.is_number(radix) {
                value_str.push(self.next_char().unwrap());
            } else if self.chr0 == Some('_') {
                self.next_char();
            } else {
                break;
            }
        }

        let end_loc = self.get_pos();
        let value = BigInt::from_str_radix(&value_str, radix).unwrap();
        Ok((start_loc, Tok::Int { value }, end_loc))
    }

    fn lex_string(&mut self) -> Spanned<Tok> {
        let mut string_content = String::new();
        let start_loc = self.get_loc();

        loop {
            match self.next_char() {
                Some(c) => {
                    if c == '\n' {
                        self.new_line();
                    }
                    string_content.push(c);
                }
                None => {
                    return Err(LexicalError::StringError);
                }
            }
        }

        let end_loc = self.get_loc();
        let tok = Tok::String {
            value: string_content,
        };
        Ok((start_loc, tok, end_loc))
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

    fn is_char(&self) -> bool {
        match self.chr0 {
            Some('a'...'z') | Some('A'...'Z') | Some('_') | Some('0'...'9') => true,
            _ => false,
        }
    }

    fn is_number(&self, radix: u32) -> bool {
        match radix {
            2 => match self.chr0 {
                Some('0'...'1') => true,
                _ => false,
            },
            8 => match self.chr0 {
                Some('0'...'7') => true,
                _ => false,
            },
            10 => match self.chr0 {
                Some('0'...'9') => true,
                _ => false,
            },
            16 => match self.chr0 {
                Some('0'...'9') | Some('a'...'f') | Some('A'...'F') => true,
                _ => false,
            },
            x => unimplemented!("Radix {} is not implemented.", x),
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

