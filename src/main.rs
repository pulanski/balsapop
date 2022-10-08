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

    #[test]
    fn test_parse_integer_literals() {
        //     let ast =
        // parser::LiteralExpressionParser::new().parse("22").unwrap();
        //     assert_eq!(ast, ast::LiteralExpression::Integer { n: 22 });

        //     let ast =
        // parser::LiteralExpressionParser::new().parse("0").unwrap();
        //     assert_eq!(ast, ast::LiteralExpression::Integer { n: 0 });

        //     let ast =
        // parser::LiteralExpressionParser::new().parse("-22").unwrap();
        //     assert_eq!(ast, ast::LiteralExpression::Integer { n: -22 });

        // Intermediate Non-Terminals (e.g. DecimalDigitOrUnderscore, etc.)
        // These need to return chars as they are used within other
        // nonterminals which filter out the underscores

        // Integer Literals
        assert_eq!(
            parser::IntegerLiteralParser::new().parse("0x_____u8"),
            Ok(0x0)
        );
        assert_eq!(
            parser::IntegerLiteralParser::new().parse("0X_____u16"),
            Ok(0x0)
        );
        assert_eq!(
            parser::IntegerLiteralParser::new().parse("0x_1_2_3_4_5_6_7_u32"),
            Ok(0x1234567)
        );
        assert_eq!(
            parser::IntegerLiteralParser::new().parse("0X_1_2_3_4_5_6_7_u64"),
            Ok(0x1234567)
        );
        assert_eq!(
            parser::IntegerLiteralParser::new().parse("0x_a_b_c_d_e_f_u128"),
            Ok(0xabcdef)
        );
        assert_eq!(
            parser::IntegerLiteralParser::new().parse("0X_a_b_c_d_e_f_usize"),
            Ok(0xabcdef)
        );
        assert_eq!(
            parser::IntegerLiteralParser::new().parse("0x_A_B_C_D_E_F_i8"),
            Ok(0xABCDEF)
        );
        assert_eq!(
            parser::IntegerLiteralParser::new().parse("0X_A_B_C_D_E_F_i16"),
            Ok(0xABCDEF)
        );
        assert_eq!(parser::IntegerLiteralParser::new().parse("22i32"), Ok(22));
        assert_eq!(parser::IntegerLiteralParser::new().parse("0i64"), Ok(0));
        assert_eq!(
            parser::IntegerLiteralParser::new().parse("_1_2_3_i128"),
            Ok(123)
        );
        assert_eq!(
            parser::IntegerLiteralParser::new().parse("1_000isize"),
            Ok(1_000)
        );
        assert_eq!(
            parser::IntegerLiteralParser::new().parse("1_234_567u8"),
            Ok(1_234_567)
        );
        assert_eq!(
            parser::IntegerLiteralParser::new().parse("1_234_567_890u16"),
            Ok(1_234_567_890)
        );
        assert_eq!(
            parser::IntegerLiteralParser::new().parse("9_876_543_210u32"),
            Ok(9_876_543_210)
        );
        assert_eq!(
            parser::IntegerLiteralParser::new()
                .parse("1_234_567_890_123_456_789_123_456_789u64"),
            Ok(1_234_567_890_123_456_789_123_456_789)
        );
        assert_eq!(
            parser::IntegerLiteralParser::new()
                .parse("9_876_543_210_987_654_321_987_654_321u128"),
            Ok(9_876_543_210_987_654_321_987_654_321)
        );
        assert_eq!(
            parser::IntegerLiteralParser::new().parse("0o_____usize"),
            Ok(0o0)
        );
        assert_eq!(
            parser::IntegerLiteralParser::new().parse("0O_____i8"),
            Ok(0o0)
        );
        assert_eq!(
            parser::IntegerLiteralParser::new().parse("0o_1_2_3_4_5_6_7_i16"),
            Ok(0o1234567)
        );
        assert_eq!(
            parser::IntegerLiteralParser::new().parse("0O_1_2_3_4_5_6_7_i32"),
            Ok(0o1234567)
        );
        assert_eq!(
            parser::IntegerLiteralParser::new().parse("0o70i64"),
            Ok(0o70)
        );
        assert_eq!(
            parser::IntegerLiteralParser::new().parse("0O70i128"),
            Ok(0o70)
        );
        assert!(parser::IntegerLiteralParser::new()
            .parse("0o8isize")
            .is_err());
        assert_eq!(
            parser::IntegerLiteralParser::new().parse("0b_____u8"),
            Ok(0b0)
        );
        assert_eq!(
            parser::IntegerLiteralParser::new().parse("0B_____u16"),
            Ok(0b0)
        );
        assert_eq!(
            parser::IntegerLiteralParser::new().parse("0b_1_0_1_0_1_0_1_0_u32"),
            Ok(0b10101010)
        );
        assert_eq!(
            parser::IntegerLiteralParser::new().parse("0B_1_0_1_0_1_0_1_0_u64"),
            Ok(0b10101010)
        );
        assert_eq!(
            parser::IntegerLiteralParser::new().parse("0b1111_1111_1001_0000_u128"),
            Ok(0b1111_1111_1001_0000)
        );
        assert_eq!(
            parser::IntegerLiteralParser::new().parse("0B1111_1111_1001_0000_usize"),
            Ok(0b1111_1111_1001_0000)
        );

        // Integer Sub-Literals (e.g. DecimalLiteral, HexadecimalLiteral, etc.)

        // - HexadecimalLiteral
        assert_eq!(
            parser::HexadecimalLiteralParser::new().parse("0x_____"),
            Ok(0x0)
        );
        assert_eq!(
            parser::HexadecimalLiteralParser::new().parse("0X_____"),
            Ok(0x0)
        );
        assert_eq!(
            parser::HexadecimalLiteralParser::new().parse("0x_1_2_3_4_5_6_7_"),
            Ok(0x1234567)
        );
        assert_eq!(
            parser::HexadecimalLiteralParser::new().parse("0X_1_2_3_4_5_6_7_"),
            Ok(0x1234567)
        );
        assert_eq!(
            parser::HexadecimalLiteralParser::new().parse("0x_a_b_c_d_e_f_"),
            Ok(0xabcdef)
        );
        assert_eq!(
            parser::HexadecimalLiteralParser::new().parse("0X_a_b_c_d_e_f_"),
            Ok(0xabcdef)
        );
        assert_eq!(
            parser::HexadecimalLiteralParser::new().parse("0x_A_B_C_D_E_F_"),
            Ok(0xABCDEF)
        );
        assert_eq!(
            parser::HexadecimalLiteralParser::new().parse("0X_A_B_C_D_E_F_"),
            Ok(0xABCDEF)
        );
        assert!(parser::HexadecimalLiteralParser::new()
            .parse("0xG")
            .is_err());

        // - DecimalLiteral
        assert_eq!(parser::DecimalLiteralParser::new().parse("22"), Ok(22));
        assert_eq!(parser::DecimalLiteralParser::new().parse("0"), Ok(0));
        assert_eq!(
            parser::DecimalLiteralParser::new().parse("_1_2_3_"),
            Ok(123)
        );
        assert_eq!(
            parser::DecimalLiteralParser::new().parse("1_000"),
            Ok(1_000)
        );
        assert_eq!(
            parser::DecimalLiteralParser::new().parse("1_234_567"),
            Ok(1_234_567)
        );
        assert_eq!(
            parser::DecimalLiteralParser::new().parse("1_234_567_890"),
            Ok(1_234_567_890)
        );
        assert_eq!(
            parser::DecimalLiteralParser::new().parse("9_876_543_210"),
            Ok(9_876_543_210)
        );
        assert_eq!(
            parser::DecimalLiteralParser::new()
                .parse("1_234_567_890_123_456_789_123_456_789"),
            Ok(1_234_567_890_123_456_789_123_456_789)
        );
        assert_eq!(
            parser::DecimalLiteralParser::new()
                .parse("9_876_543_210_987_654_321_987_654_321"),
            Ok(9_876_543_210_987_654_321_987_654_321)
        );

        // - OctalLiteral
        assert_eq!(parser::OctalLiteralParser::new().parse("0o_____"), Ok(0o0));
        assert_eq!(parser::OctalLiteralParser::new().parse("0O_____"), Ok(0o0));
        assert_eq!(
            parser::OctalLiteralParser::new().parse("0o_1_2_3_4_5_6_7_"),
            Ok(0o1234567)
        );
        assert_eq!(
            parser::OctalLiteralParser::new().parse("0O_1_2_3_4_5_6_7_"),
            Ok(0o1234567)
        );
        assert_eq!(parser::OctalLiteralParser::new().parse("0o70"), Ok(0o70));
        assert_eq!(parser::OctalLiteralParser::new().parse("0O70"), Ok(0o70));
        assert!(parser::OctalLiteralParser::new().parse("0o8").is_err());

        // - BinaryLiteral
        assert_eq!(parser::BinaryLiteralParser::new().parse("0b_____"), Ok(0b0));
        assert_eq!(parser::BinaryLiteralParser::new().parse("0B_____"), Ok(0b0));
        assert_eq!(
            parser::BinaryLiteralParser::new().parse("0b________1"),
            Ok(0b1)
        );
        assert_eq!(
            parser::BinaryLiteralParser::new().parse("0B________1"),
            Ok(0b1)
        );
        assert_eq!(
            parser::BinaryLiteralParser::new().parse("0b1111_1111_1001_0000"),
            Ok(0b1111_1111_1001_0000)
        );
        assert_eq!(
            parser::BinaryLiteralParser::new().parse("0B1111_1111_1001_0000"),
            Ok(0b1111_1111_1001_0000)
        );
        assert_eq!(
            parser::BinaryLiteralParser::new()
                .parse("0b_0_1_1_0_1_1_0_0_1_0_0_0_0_0_0_0_"),
            Ok(0b0110_1100_1000_0000)
        );
        assert!(parser::BinaryLiteralParser::new().parse("0b2").is_err());

        // HexadecimalDigitOrUnderscore
        // 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | a | b | c | d | e | f | A | B | C
        // | D | E | F | _
        assert_eq!(
            parser::HexadecimalDigitOrUnderscoreParser::new()
                .parse("0")
                .unwrap(),
            '0'
        );
        assert_eq!(
            parser::HexadecimalDigitOrUnderscoreParser::new()
                .parse("1")
                .unwrap(),
            '1'
        );
        assert_eq!(
            parser::HexadecimalDigitOrUnderscoreParser::new()
                .parse("2")
                .unwrap(),
            '2'
        );
        assert_eq!(
            parser::HexadecimalDigitOrUnderscoreParser::new()
                .parse("3")
                .unwrap(),
            '3'
        );
        assert_eq!(
            parser::HexadecimalDigitOrUnderscoreParser::new()
                .parse("4")
                .unwrap(),
            '4'
        );
        assert_eq!(
            parser::HexadecimalDigitOrUnderscoreParser::new()
                .parse("5")
                .unwrap(),
            '5'
        );
        assert_eq!(
            parser::HexadecimalDigitOrUnderscoreParser::new()
                .parse("6")
                .unwrap(),
            '6'
        );
        assert_eq!(
            parser::HexadecimalDigitOrUnderscoreParser::new()
                .parse("7")
                .unwrap(),
            '7'
        );
        assert_eq!(
            parser::HexadecimalDigitOrUnderscoreParser::new()
                .parse("8")
                .unwrap(),
            '8'
        );
        assert_eq!(
            parser::HexadecimalDigitOrUnderscoreParser::new()
                .parse("9")
                .unwrap(),
            '9'
        );
        assert_eq!(
            parser::HexadecimalDigitOrUnderscoreParser::new()
                .parse("a")
                .unwrap(),
            'a'
        );
        assert_eq!(
            parser::HexadecimalDigitOrUnderscoreParser::new()
                .parse("b")
                .unwrap(),
            'b'
        );
        assert_eq!(
            parser::HexadecimalDigitOrUnderscoreParser::new()
                .parse("c")
                .unwrap(),
            'c'
        );
        assert_eq!(
            parser::HexadecimalDigitOrUnderscoreParser::new()
                .parse("d")
                .unwrap(),
            'd'
        );
        assert_eq!(
            parser::HexadecimalDigitOrUnderscoreParser::new()
                .parse("e")
                .unwrap(),
            'e'
        );
        assert_eq!(
            parser::HexadecimalDigitOrUnderscoreParser::new()
                .parse("f")
                .unwrap(),
            'f'
        );
        assert_eq!(
            parser::HexadecimalDigitOrUnderscoreParser::new()
                .parse("A")
                .unwrap(),
            'A'
        );
        assert_eq!(
            parser::HexadecimalDigitOrUnderscoreParser::new()
                .parse("B")
                .unwrap(),
            'B'
        );
        assert_eq!(
            parser::HexadecimalDigitOrUnderscoreParser::new()
                .parse("C")
                .unwrap(),
            'C'
        );
        assert_eq!(
            parser::HexadecimalDigitOrUnderscoreParser::new()
                .parse("D")
                .unwrap(),
            'D'
        );
        assert_eq!(
            parser::HexadecimalDigitOrUnderscoreParser::new()
                .parse("E")
                .unwrap(),
            'E'
        );
        assert_eq!(
            parser::HexadecimalDigitOrUnderscoreParser::new()
                .parse("F")
                .unwrap(),
            'F'
        );

        // DecimalDigitOrUnderscore
        // 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | _
        assert_eq!(
            parser::DecimalDigitOrUnderscoreParser::new().parse("0"),
            Ok('0')
        );
        assert_eq!(
            parser::DecimalDigitOrUnderscoreParser::new().parse("1"),
            Ok('1')
        );
        assert_eq!(
            parser::DecimalDigitOrUnderscoreParser::new().parse("2"),
            Ok('2')
        );
        assert_eq!(
            parser::DecimalDigitOrUnderscoreParser::new().parse("3"),
            Ok('3')
        );
        assert_eq!(
            parser::DecimalDigitOrUnderscoreParser::new().parse("4"),
            Ok('4')
        );
        assert_eq!(
            parser::DecimalDigitOrUnderscoreParser::new().parse("5"),
            Ok('5')
        );
        assert_eq!(
            parser::DecimalDigitOrUnderscoreParser::new().parse("6"),
            Ok('6')
        );
        assert_eq!(
            parser::DecimalDigitOrUnderscoreParser::new().parse("7"),
            Ok('7')
        );
        assert_eq!(
            parser::DecimalDigitOrUnderscoreParser::new().parse("8"),
            Ok('8')
        );
        assert_eq!(
            parser::DecimalDigitOrUnderscoreParser::new().parse("9"),
            Ok('9')
        );
        assert_eq!(
            parser::DecimalDigitOrUnderscoreParser::new().parse("_"),
            Ok('_')
        );

        // OctalDigitOrUnderscore
        // 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | _
        assert_eq!(
            parser::OctalDigitOrUnderscoreParser::new().parse("0"),
            Ok('0')
        );
        assert_eq!(
            parser::OctalDigitOrUnderscoreParser::new().parse("1"),
            Ok('1')
        );
        assert_eq!(
            parser::OctalDigitOrUnderscoreParser::new().parse("2"),
            Ok('2')
        );
        assert_eq!(
            parser::OctalDigitOrUnderscoreParser::new().parse("3"),
            Ok('3')
        );
        assert_eq!(
            parser::OctalDigitOrUnderscoreParser::new().parse("4"),
            Ok('4')
        );
        assert_eq!(
            parser::OctalDigitOrUnderscoreParser::new().parse("5"),
            Ok('5')
        );
        assert_eq!(
            parser::OctalDigitOrUnderscoreParser::new().parse("6"),
            Ok('6')
        );
        assert_eq!(
            parser::OctalDigitOrUnderscoreParser::new().parse("7"),
            Ok('7')
        );
        assert_eq!(
            parser::OctalDigitOrUnderscoreParser::new().parse("_"),
            Ok('_')
        );

        // BinaryDigitOrUnderscore
        // 0 | 1 | _
        assert_eq!(
            parser::BinaryDigitOrUnderscoreParser::new().parse("0"),
            Ok('0')
        );
        assert_eq!(
            parser::BinaryDigitOrUnderscoreParser::new().parse("1"),
            Ok('1')
        );
        assert_eq!(
            parser::BinaryDigitOrUnderscoreParser::new().parse("_"),
            Ok('_')
        );
    }

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
/// Verifies the correctness of the LALRPOP lexer          //
/// generator when it comes to lexing the source code.     //
///                                                        //
/// At first glance, the APIs here may seem a bit          //
/// odd since they all reference structures including the  //
/// substring "Parser" and this test suite is intended to  //
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
/// within the grammar by the parser to create the AST.    //
/////////////////////////////////////////////////////////////
#[cfg(test)]
mod lexer_test_suite {
    use super::*;

    // TODO maybe in future expand to deserializing parsed nonterminals into
    // well-defined data structures housing tokens along with their lexemes
    // and spans. Similar to how lexing is done in Logos.

    // TODO maybe add a redundant lexer implementation via Logos
    // for emitting tokens along with their lexemes, spans, and usage in the
    // language.

    #[test]
    fn test_lex_punctuation() {
        assert_eq!(parser::PlusParser::new().parse("+"), Ok('+'));
        assert_eq!(parser::MinusParser::new().parse("-"), Ok('-'));
        assert_eq!(parser::StarParser::new().parse("*"), Ok('*'));
        assert_eq!(parser::SlashParser::new().parse("/"), Ok('/'));
        assert_eq!(parser::PercentParser::new().parse("%"), Ok('%'));
        assert_eq!(parser::CaretParser::new().parse("^"), Ok('^'));
        assert_eq!(parser::NotParser::new().parse("!"), Ok('!'));
        assert_eq!(parser::AndParser::new().parse("&"), Ok('&'));
        assert_eq!(parser::OrParser::new().parse("|"), Ok('|'));
        assert_eq!(
            parser::AndAndParser::new().parse("&&"),
            Ok(String::from("&&"))
        );
        assert_eq!(
            parser::OrOrParser::new().parse("||"),
            Ok(String::from("||"))
        );
        assert_eq!(
            parser::MinusEqualsParser::new().parse("-="),
            Ok(String::from("-="))
        );
        assert_eq!(
            parser::StarEqualsParser::new().parse("*="),
            Ok(String::from("*="))
        );
        assert_eq!(
            parser::SlashEqualsParser::new().parse("/="),
            Ok(String::from("/="))
        );
        assert_eq!(
            parser::PercentEqualsParser::new().parse("%="),
            Ok(String::from("%="))
        );
        assert_eq!(
            parser::CaretEqualsParser::new().parse("^="),
            Ok(String::from("^="))
        );
        assert_eq!(
            parser::AndEqualsParser::new().parse("&="),
            Ok(String::from("&="))
        );
        assert_eq!(
            parser::OrEqualsParser::new().parse("|="),
            Ok(String::from("|="))
        );
        assert_eq!(parser::EqualsParser::new().parse("="), Ok('='));
        assert_eq!(
            parser::DoubleEqualsParser::new().parse("=="),
            Ok(String::from("=="))
        );
        assert_eq!(
            parser::NotEqualParser::new().parse("!="),
            Ok(String::from("!="))
        );
        assert_eq!(parser::LessThanParser::new().parse("<"), Ok('<'));
        assert_eq!(
            parser::LessThanEqualParser::new().parse("<="),
            Ok(String::from("<="))
        );
        assert_eq!(parser::GreaterThanParser::new().parse(">"), Ok('>'));
        assert_eq!(
            parser::GreaterThanEqualParser::new().parse(">="),
            Ok(String::from(">="))
        );
        assert_eq!(parser::UnderscoreParser::new().parse("_"), Ok('_'));
        assert_eq!(parser::DotParser::new().parse("."), Ok('.'));
        assert_eq!(
            parser::DotDotParser::new().parse(".."),
            Ok(String::from(".."))
        );
        assert_eq!(
            parser::DotDotDotParser::new().parse("..."),
            Ok(String::from("..."))
        );
        assert_eq!(
            parser::DotDotEqualsParser::new().parse("..="),
            Ok(String::from("..="))
        );
        assert_eq!(parser::CommaParser::new().parse(","), Ok(','));
        assert_eq!(parser::SemicolonParser::new().parse(";"), Ok(';'));
        assert_eq!(parser::ColonParser::new().parse(":"), Ok(':'));
        assert_eq!(
            parser::PathSeparatorParser::new().parse("::"),
            Ok(String::from("::"))
        );
        assert_eq!(
            parser::RightArrowParser::new().parse("->"),
            Ok(String::from("->"))
        );
        assert_eq!(
            parser::FatArrowParser::new().parse("=>"),
            Ok(String::from("=>"))
        );
        assert_eq!(parser::PoundParser::new().parse("#"), Ok('#'));
        assert_eq!(parser::DollarParser::new().parse("$"), Ok('$'));
        assert_eq!(parser::QuestionParser::new().parse("?"), Ok('?'));
    }

    #[test]
    fn test_lex_delimiters() {
        assert_eq!(parser::LeftParenParser::new().parse("("), Ok('('));
        assert_eq!(parser::RightParenParser::new().parse(")"), Ok(')'));
        assert_eq!(parser::LeftBracketParser::new().parse("["), Ok('['));
        assert_eq!(parser::RightBracketParser::new().parse("]"), Ok(']'));
        assert_eq!(parser::LeftBraceParser::new().parse("{"), Ok('{'));
        assert_eq!(parser::RightBraceParser::new().parse("}"), Ok('}'));
    }

    #[test]
    fn test_lex_numeric_literals() {
        // Integer Literal Prefix Non-Terminals
        assert_eq!(
            parser::BinaryLiteralPrefixParser::new().parse("0b"),
            Ok(String::from("0b"))
        );
        assert_eq!(
            parser::BinaryLiteralPrefixParser::new().parse("0B"),
            Ok(String::from("0B"))
        );
        assert_eq!(
            parser::OctalLiteralPrefixParser::new().parse("0o"),
            Ok(String::from("0o"))
        );
        assert_eq!(
            parser::OctalLiteralPrefixParser::new().parse("0O"),
            Ok(String::from("0O"))
        );
        assert_eq!(
            parser::HexadecimalLiteralPrefixParser::new().parse("0x"),
            Ok(String::from("0x"))
        );
        assert_eq!(
            parser::HexadecimalLiteralPrefixParser::new().parse("0X"),
            Ok(String::from("0X"))
        );

        // Integer Literal Suffix Non-Terminals

        // - Unsigned Integer Literal Suffix Non-Terminals
        assert_eq!(
            parser::IntegerSuffixParser::new().parse("u8"),
            Ok(String::from("u8"))
        );
        assert_eq!(
            parser::IntegerSuffixParser::new().parse("u16"),
            Ok(String::from("u16"))
        );
        assert_eq!(
            parser::IntegerSuffixParser::new().parse("u32"),
            Ok(String::from("u32"))
        );
        assert_eq!(
            parser::IntegerSuffixParser::new().parse("u64"),
            Ok(String::from("u64"))
        );
        assert_eq!(
            parser::IntegerSuffixParser::new().parse("u128"),
            Ok(String::from("u128"))
        );
        assert_eq!(
            parser::IntegerSuffixParser::new().parse("usize"),
            Ok(String::from("usize"))
        );

        // - Signed Integer Literal Suffix Non-Terminals
        assert_eq!(
            parser::IntegerSuffixParser::new().parse("i8"),
            Ok(String::from("i8"))
        );
        assert_eq!(
            parser::IntegerSuffixParser::new().parse("i16"),
            Ok(String::from("i16"))
        );
        assert_eq!(
            parser::IntegerSuffixParser::new().parse("i32"),
            Ok(String::from("i32"))
        );
        assert_eq!(
            parser::IntegerSuffixParser::new().parse("i64"),
            Ok(String::from("i64"))
        );
        assert_eq!(
            parser::IntegerSuffixParser::new().parse("i128"),
            Ok(String::from("i128"))
        );
        assert_eq!(
            parser::IntegerSuffixParser::new().parse("isize"),
            Ok(String::from("isize"))
        );

        // Digit Non-Terminals

        // - Binary Digit Non-Terminals
        assert_eq!(parser::BinaryDigitParser::new().parse("0"), Ok('0'));
        assert_eq!(parser::BinaryDigitParser::new().parse("1"), Ok('1'));

        // - Octal Digit Non-Terminals
        assert_eq!(parser::OctalDigitParser::new().parse("0"), Ok('0'));
        assert_eq!(parser::OctalDigitParser::new().parse("1"), Ok('1'));
        assert_eq!(parser::OctalDigitParser::new().parse("2"), Ok('2'));
        assert_eq!(parser::OctalDigitParser::new().parse("3"), Ok('3'));
        assert_eq!(parser::OctalDigitParser::new().parse("4"), Ok('4'));
        assert_eq!(parser::OctalDigitParser::new().parse("5"), Ok('5'));
        assert_eq!(parser::OctalDigitParser::new().parse("6"), Ok('6'));
        assert_eq!(parser::OctalDigitParser::new().parse("7"), Ok('7'));

        // - Decimal Digit Non-Terminals
        assert_eq!(parser::DecimalDigitParser::new().parse("0"), Ok('0'));
        assert_eq!(parser::DecimalDigitParser::new().parse("1"), Ok('1'));
        assert_eq!(parser::DecimalDigitParser::new().parse("2"), Ok('2'));
        assert_eq!(parser::DecimalDigitParser::new().parse("3"), Ok('3'));
        assert_eq!(parser::DecimalDigitParser::new().parse("4"), Ok('4'));
        assert_eq!(parser::DecimalDigitParser::new().parse("5"), Ok('5'));
        assert_eq!(parser::DecimalDigitParser::new().parse("6"), Ok('6'));
        assert_eq!(parser::DecimalDigitParser::new().parse("7"), Ok('7'));
        assert_eq!(parser::DecimalDigitParser::new().parse("8"), Ok('8'));
        assert_eq!(parser::DecimalDigitParser::new().parse("9"), Ok('9'));

        // - Hexadecimal Digit Non-Terminals
        assert_eq!(parser::HexadecimalDigitParser::new().parse("0"), Ok('0'));
        assert_eq!(parser::HexadecimalDigitParser::new().parse("1"), Ok('1'));
        assert_eq!(parser::HexadecimalDigitParser::new().parse("2"), Ok('2'));
        assert_eq!(parser::HexadecimalDigitParser::new().parse("3"), Ok('3'));
        assert_eq!(parser::HexadecimalDigitParser::new().parse("4"), Ok('4'));
        assert_eq!(parser::HexadecimalDigitParser::new().parse("5"), Ok('5'));
        assert_eq!(parser::HexadecimalDigitParser::new().parse("6"), Ok('6'));
        assert_eq!(parser::HexadecimalDigitParser::new().parse("7"), Ok('7'));
        assert_eq!(parser::HexadecimalDigitParser::new().parse("8"), Ok('8'));
        assert_eq!(parser::HexadecimalDigitParser::new().parse("9"), Ok('9'));
        assert_eq!(parser::HexadecimalDigitParser::new().parse("a"), Ok('a'));
        assert_eq!(parser::HexadecimalDigitParser::new().parse("b"), Ok('b'));
        assert_eq!(parser::HexadecimalDigitParser::new().parse("c"), Ok('c'));
        assert_eq!(parser::HexadecimalDigitParser::new().parse("d"), Ok('d'));
        assert_eq!(parser::HexadecimalDigitParser::new().parse("e"), Ok('e'));
        assert_eq!(parser::HexadecimalDigitParser::new().parse("f"), Ok('f'));
        assert_eq!(parser::HexadecimalDigitParser::new().parse("A"), Ok('A'));
        assert_eq!(parser::HexadecimalDigitParser::new().parse("B"), Ok('B'));
        assert_eq!(parser::HexadecimalDigitParser::new().parse("C"), Ok('C'));
        assert_eq!(parser::HexadecimalDigitParser::new().parse("D"), Ok('D'));
        assert_eq!(parser::HexadecimalDigitParser::new().parse("E"), Ok('E'));
        assert_eq!(parser::HexadecimalDigitParser::new().parse("F"), Ok('F'));

        /////////////////////////////
        // Floating Point Literals //
        /////////////////////////////

        // Float Literal Suffix Non-Terminals
        assert!(parser::FloatSuffixParser::new().parse("f32").is_ok());
        assert!(parser::FloatSuffixParser::new().parse("f64").is_ok());

        // Float Literal Exponent Non-Terminals
    }
}
