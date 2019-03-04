use nom::*;
use std::dbg;
use std::io::Error;
use std::fmt;
use std::fmt::{Debug, Display};
use nom::Context;
use nom::dbg as debug;
use nom::types::CompleteStr;

#[derive(Debug, Eq, Ord, PartialOrd, PartialEq)]
pub struct InputPos {
    pub sourceLine: u32,
    pub sourceColumn: u32
}

impl Display for InputPos {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "line {}, column {}", self.sourceLine, self.sourceColumn)
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum Tok {
    EQU
}

#[derive(Debug, Eq, PartialEq)]
pub enum Stmt {
    Instruction(String),
    Alias(String, u32),
    Constant(String, u32),
    Segment(String, u32),
    CompilerOption(String, String)
}

named!(pub kw_equ<CompleteStr, Tok>, dbg_dmp!(value!(Tok::EQU, tag_s!("EQU"))));

#[cfg(test)]
mod tests {
    use super::*;
    use std::fmt::Display;

    fn show_context<E: Debug>(ctx: Context<CompleteStr, E>) -> String {
        match ctx {
            Context::Code(input, error_kind) => {
                dbg!(error_kind);
                format!("{}", input)
            },
            _ => unimplemented!()
        }
    }

    fn show_err<E: Debug>(err: Err<CompleteStr, E>) -> String {
        match err {
            Err::Error(ctx) | Err::Failure(ctx) => show_context(ctx),
            Err::Incomplete(_) => "incomplete data".to_string()
        }
    }

    #[test]
    fn test_kw_equ() {
        let res = kw_equ(CompleteStr("eq"));
        match res {
            Result::Ok((_, res)) => assert_eq!(res, Tok::EQU),
            Result::Err(err) => {
                eprintln!("{}", show_err(err));
                panic!("parse error")
            }
        }
    }
}