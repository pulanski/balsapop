mod ast;
mod cli;
mod db;

use std::{
    fs::read_to_string,
    path::PathBuf,
};

use clap::Parser as ClapParser;
use cli::BalsapopCli;
use miette::Result;

use crate::cli::FileNotFound;

#[derive(Debug)]
struct SourceFile {
    path:     PathBuf,
    contents: ProgramSource,
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
            Box::new(SourceFile {
                path:     source_path,
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
mod parser_test_suite {
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

    #[test]
    fn test_parse_string_literal_expression() {
        let ast = parser::LiteralExpressionParser::new()
            .parse(r#""Hello""#)
            .unwrap();
        assert_eq!(ast, ast::LiteralExpression::String {
            s: "Hello".to_string(),
        });

        let ast = parser::LiteralExpressionParser::new()
            .parse(r#""Hello, world!""#)
            .unwrap();
        assert_eq!(ast, ast::LiteralExpression::String {
            s: "Hello, world!".to_string(),
        });
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

#[cfg(test)]
mod lexer_test_suite {
    use super::*;

    // TODO maybe in future expand to housing tokens along with their lexemes
    // and spans in well-defined data structures

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
        assert!(parser::UnderscoreParser::new().parse("_").is_ok());
        // assert!(parser::EqualsParser::new().parse("=").is_ok());
    }
}
