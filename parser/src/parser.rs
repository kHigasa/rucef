extern crate lalrpop_util;

use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::iter;
use std::path::Path;

use super::ast;
use super::elegua;
use super::lexer;
use super::token;

pub fn read_file(filename: &Path) -> Result<String, String> {
    info!("Loading file {:?}", filename);
    match File::open(&filename) {
        Ok(mut file) => {
            let mut s = String::new();

            match file.read_to_string(&mut s) {
                Err(why) => Err(String::from("Reading file failed: ") + why.description()),
                Ok(_) => Ok(s),
            }
        }
        Err(why) => Err(String::from("Opening file failed: ") + why.description()),
    }
}

// Parse elegua code.
pub fn parse(filename: &Path) -> Result<ast::Program, String> {
    info!("Parsing file: {}", filename.display());
    let txt = read_file(filename)?;
    debug!("Read contents of file: {}", txt);
    parse_program(&txt)
}

macro_rules! do_lalr_parsing {
    ($input: expr, $pat: ident, $tok: ident) => {{
        let lxr = lexer::make_tokenizer($input);
        let marker_token = (Default::default(), token::Tok::$tok, Default::default());
        let tokenizer = iter::once(Ok(marker_token)).chain(lxr);

        match elegua::TopParser::new()::parse(tokenizer) {
            Err(why) => Err(format!("{:?}", why)),
            Ok(top) => {
                if let ast::Top::$pat(x) = top {
                    Ok(x)
                } else {
                    unreachable!()
                }
            }
        }
    }};
}

pub fn parse_program(source: &str) -> Result<ast::Program, String> {
    do_lalr_parsing!(source, Program, StartProgram)
}

pub fn parse_statement(source: &str) -> Result<ast::LocatedStatement, String> {
    do_lalr_parsing!(source, Statement, StartStatement)
}

pub fn parse_expression(source: &str) -> Result<ast::Expression, String> {
    do_lalr_parsing!(source, Expression, StartExpression)
}

