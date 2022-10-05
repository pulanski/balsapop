mod ast;
mod cli;
mod db;

use std::{fs::read_to_string, path::PathBuf};

use clap::Parser;
use cli::BalsapopCli;
use miette::{IntoDiagnostic, Result};

use crate::cli::FileNotFound;

#[derive(Debug)]
struct SourceFile {
    path: PathBuf,
    contents: ProgramSource,
}

impl SourceFile {
    fn new(path: PathBuf) -> Result<Self> {
        let text = read_to_string(&path).into_diagnostic()?;
        Ok(Self {
            path,
            contents: ProgramSource { text },
        })
    }
}

#[derive(Debug)]
struct ProgramSource {
    text: String,
}

// // ANCHOR: jar_struct
// #[salsa::jar(db = Db)]
// pub struct Jar(
//     // crate::compile::compile,
//     // crate::ir::SourceProgram,
//     // crate::ir::Program,
//     // crate::ir::VariableId,
//     // crate::ir::FunctionId,
//     // crate::ir::Function,
//     // crate::ir::Diagnostics,
//     // crate::ir::Span,
//     // crate::parser::parse_statements,
//     // crate::type_check::type_check_program,
//     // crate::type_check::type_check_function,
//     // crate::type_check::find_function,
// );
// // ANCHOR_END: jar_struct

// // ANCHOR: jar_db
// pub trait Db: salsa::DbWithJar<Jar> {}
// // ANCHOR_END: jar_db

// // ANCHOR: jar_db_impl
// impl<DB> Db for DB where DB: ?Sized + salsa::DbWithJar<Jar> {}
// // ANCHOR_END: jar_db_impl

#[macro_use]
extern crate lalrpop_util;

lalrpop_mod!(pub parser);

fn main() -> Result<()> {
    let cli_args = BalsapopCli::parse();

    let source_file = match get_source_file(cli_args.source_path) {
        Ok(source_file) => source_file,
        Err(err) => return err,
    };

    println!(
        "Source file: {:?}, {:?}",
        source_file.path, source_file.contents.text
    );

    // parser::LiteralExpressionParser::new().parse("22");
    let ast = parser::LiteralExpressionParser::new()
        .parse("2212312312312.2e+10")
        .unwrap();

    println!("AST: {:?}", ast);

    // parser::LiteralExpressionParser::new().parse(&source_file.contents.text)?;

    Ok(())
}

pub(crate) fn get_source_file(
    source_path: Option<PathBuf>,
) -> Result<Box<SourceFile>, Result<(), miette::ErrReport>> {
    let src = match source_path {
        Some(source_path) => {
            let source_str = match read_to_string(source_path.clone()) {
                Ok(source_str) => source_str,
                Err(_) => {
                    return Err(Err(FileNotFound {}.into()));
                }
            };
            // TODO refactor to more idiomatic Rust
            // Handle the error in the new function
            // new should return a Result
            // if error, propagate the error to the caller
            // e.g. let source_file =
            // SourceFile::new(source_path).into_diagnostic()?; <-- this maybe?
            // Ok(Box::new(SourceFile::new(source_path).unwrap_err(
            // //     path:     source_path,
            //     contents: ProgramSource { text: source_str },
            // })
            // )))

            Box::new(SourceFile {
                path: source_path,
                contents: ProgramSource { text: source_str },
            })
        }
        None => {
            return Err(Err(FileNotFound {}.into()));
        }
    };
    Ok(src)
}

#[cfg(test)]
mod cli_usage_test_suite {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_get_source_file() {
        let source_file =
            get_source_file(Some(PathBuf::from("src/main.rs"))).unwrap();
        assert_eq!(source_file.path, PathBuf::from("src/main.rs"));
    }

    #[test]
    fn test_get_source_file_file_does_not_exist() {
        let source_file = get_source_file(Some(PathBuf::from("src/doesntexist")));
        assert!(source_file.is_err());
    }

    #[test]
    fn test_get_source_file_no_path() {
        let source_file = get_source_file(None);
        assert!(source_file.is_err());
    }
}

// TODO Have sections like the following
// NonTerminals (e.g. Expression, Statement, etc.)
// Terminals (e.g. Identifier, Number, etc.)
#[cfg(test)]
mod parser_test_suite {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_parse_string_literal_expression() {
        let ast = parser::LiteralExpressionParser::new()
            .parse(r#""Hello""#)
            .unwrap();
        assert_eq!(
            ast,
            ast::LiteralExpression::String {
                s: "Hello".to_string(),
            }
        );

        let ast = parser::LiteralExpressionParser::new()
            .parse(r#""Hello, world!""#)
            .unwrap();
        assert_eq!(
            ast,
            ast::LiteralExpression::String {
                s: "Hello, world!".to_string(),
            }
        );
    }

    // #[test]
    // fn test_parse_integer_literal_expression() {
    //     let ast = parser::LiteralExpressionParser::new().parse("22").unwrap();
    //     assert_eq!(ast, ast::LiteralExpression::Integer { n: 22 });

    //     let ast = parser::LiteralExpressionParser::new().parse("0").unwrap();
    //     assert_eq!(ast, ast::LiteralExpression::Integer { n: 0 });

    //     let ast = parser::LiteralExpressionParser::new().parse("-22").unwrap();
    //     assert_eq!(ast, ast::LiteralExpression::Integer { n: -22 });
    // }

    #[test]
    fn test_parse_float_literal_expression() {
        let ast = parser::LiteralExpressionParser::new()
            .parse("22.0")
            .unwrap();
        assert_eq!(ast, ast::LiteralExpression::Float { f: 22.0 });

        let ast = parser::LiteralExpressionParser::new().parse("0.0").unwrap();
        assert_eq!(ast, ast::LiteralExpression::Float { f: 0.0 });

        // let ast = parser::LiteralExpressionParser::new()
        //     .parse("-22.0")
        //     .unwrap();
        // assert_eq!(ast, ast::LiteralExpression::Float { f: -22.0 });

        // e notation
        // let ast = parser::LiteralExpressionParser::new()
        //     .parse("22e+0")
        //     .unwrap();
        // assert_eq!(ast, ast::LiteralExpression::Float { f: 22.0 });
    }

    // #[test]
    // fn test_parse_boolean_literal_expression() {
    //     let ast =
    // parser::LiteralExpressionParser::new().parse("true").unwrap();
    //     assert_eq!(ast, ast::LiteralExpression::Boolean { b: true });

    //     let ast =
    // parser::LiteralExpressionParser::new().parse("false").unwrap();
    //     assert_eq!(ast, ast::LiteralExpression::Boolean { b: false });
    // }
}

/////////////////////////////////////////////////////////////
/// Verifies the correctness of the parser                 //
/// when it comes to both lexing tokens as well as         //
/// parsing nonterminals used in the grammar.              //
///                                                        //
/// At first glance, the APIs here may seem a bit          //
/// odd since they all include the substring               //
/// "Parser" and this test suite is intended to            //
/// test the correctness of the Lexer.                     //
///                                                        //
/// While this may be the case, it is a result of the      //
/// fact that we're using LALRPOP to generate the parser,  //
/// compared to other parsing methods, since by definition //
/// an LR(1) parser can accept a superset of the grammars  //
/// accepted by an LL(1) parser. However, what comes with  //
/// this is that LALRPOP tightly couples the lexer and     //
/// parser for ease of use. In the future, this            //
/// implementation detail may be refactored in favor of    //
/// using a different lexer (e.g. Logos or something else) //
/// and decoupling the lexing phase from that of parsing.  //
/// Regardless, the following test suite is written with   //
/// the intention of testing the lexing capabilities of    //
/// the compiler when it comes to producing tokens used    //
/// in the grammar to create the AST.                      //
/////////////////////////////////////////////////////////////
#[cfg(test)]
mod lexer_test_suite {
    use super::*;

    // TODO maybe in future expand to deserializing parsed nonterminals into
    // well-defined data structures housing tokens along with their lexemes
    // and spans. Similar to how lexing is done in Logos.

    #[test]
    fn test_lex_punctuation() {
        assert!(parser::PlusParser::new().parse("+").is_ok());
        assert!(parser::MinusParser::new().parse("-").is_ok());
        assert!(parser::StarParser::new().parse("*").is_ok());
        assert!(parser::SlashParser::new().parse("/").is_ok());
        assert!(parser::PercentParser::new().parse("%").is_ok());
        assert!(parser::CaretParser::new().parse("^").is_ok());
        assert!(parser::NotParser::new().parse("!").is_ok());
        assert!(parser::AndParser::new().parse("&").is_ok());
        assert!(parser::OrParser::new().parse("|").is_ok());
        assert!(parser::AndAndParser::new().parse("&&").is_ok());
        assert!(parser::OrOrParser::new().parse("||").is_ok());
        assert!(parser::MinusEqualsParser::new().parse("-=").is_ok());
        //

        assert!(parser::UnderscoreParser::new().parse("_").is_ok());
        // assert!(parser::)
        // assert!(parser::EqualsParser::new().parse("=").is_ok());
    }

    #[test]
    fn test_lex_numeric_literals() {
        // Integer Literal Prefix Non-Terminals
        assert!(parser::BinaryLiteralPrefixParser::new().parse("0b").is_ok());
        assert!(parser::OctalLiteralPrefixParser::new().parse("0o").is_ok());
        assert!(parser::HexadecimalLiteralPrefixParser::new()
            .parse("0x")
            .is_ok());

        // Integer Literal Suffix Non-Terminals

        // - Unsigned Integer Literal Suffix Non-Terminals
        assert!(parser::IntegerSuffixParser::new().parse("u8").is_ok());
        assert!(parser::IntegerSuffixParser::new().parse("u16").is_ok());
        assert!(parser::IntegerSuffixParser::new().parse("u32").is_ok());
        assert!(parser::IntegerSuffixParser::new().parse("u64").is_ok());
        assert!(parser::IntegerSuffixParser::new().parse("u128").is_ok());
        assert!(parser::IntegerSuffixParser::new().parse("usize").is_ok());

        // - Signed Integer Literal Suffix Non-Terminals
        assert!(parser::IntegerSuffixParser::new().parse("i8").is_ok());
        assert!(parser::IntegerSuffixParser::new().parse("i16").is_ok());
        assert!(parser::IntegerSuffixParser::new().parse("i32").is_ok());
        assert!(parser::IntegerSuffixParser::new().parse("i64").is_ok());
        assert!(parser::IntegerSuffixParser::new().parse("i128").is_ok());
        assert!(parser::IntegerSuffixParser::new().parse("isize").is_ok());

        // Float Literal Suffix Non-Terminals
        assert!(parser::FloatSuffixParser::new().parse("f32").is_ok());
        assert!(parser::FloatSuffixParser::new().parse("f64").is_ok());

        // Float Literal Exponent Non-Terminals

        // Intermediate Non-Terminals (e.g. DecimalDigitOrUnderscore, etc.)
        // These need to return chars as they are used within other
        // nonterminals which can filter out the underscores

        // DecimalDigitOrUnderscore
        let ast = parser::DecimalDigitOrUnderscoreParser::new()
            .parse("1")
            .unwrap();
        assert_eq!(ast, '1'); // <-- TODO maybe change this to be a string
                              // rather than a char          |
                              //                             V
        let ast = parser::DecimalDigitOrUnderscoreParser::new()
            .parse("_")
            .unwrap();
        assert_eq!(ast, '_');

        // Binary Literals
        // let ast = parser::BinaryLiteralParser::new().parse("0b0").unwrap();
    }
}
