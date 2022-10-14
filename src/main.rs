#![allow(dead_code, unused_imports)]
mod ast;
mod cli;
mod db;
mod parser_errors;

use std::{fs::read_to_string, path::PathBuf};

use ast::*;

use clap::Parser;
use cli::BalsapopCli;
use miette::Result;

use crate::cli::FileNotFound;

#[derive(Debug)]
struct SourceFile {
    path: PathBuf,
    contents: ProgramSource,
}

// impl SourceFile {
//     fn new(path: PathBuf) -> Result<Self> {
//         let text = read_to_string(&path).into_diagnostic()?;
//         Ok(Self {
//             path,
//             contents: ProgramSource { text },
//         })
//     }
// }

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

    println!("Source file: {:#?}", source_file);
    // parser::LiteralExpressionParser::new().parse("22");
    // let ast = parser::LiteralExpressionParser::new()
    //     .parse("2212312312312.2e+10")
    //     .unwrap();

    // println!("AST: {:?}", ast);

    let float_exponent = parser::FloatExponentParser::new().parse("e+_");
    match float_exponent {
        Ok(_) => println!("Float exponent parsed successfully"),
        Err(err) => {
            println!("Error parsing float exponent: {:?}", err);
            match err {
                lalrpop_util::ParseError::User {
                    error: parser_errors::InternalParserError::InvalidFloatExponent,
                } => return Err(parser_errors::InvalidFloatExponent {}.into()),
                _ => {
                    println!("Other error");
                }
            }
        }
    }

    // collect all of the errors https://lalrpop.github.io/lalrpop/tutorial/008_error_recovery.html
    // then loop through them, match on the error type, and add the corresponding
    // error to the diagnostics report

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
    use crate::parser_errors::InternalParserError::*;
    use lalrpop_util::ParseError;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_parse_string_literals() {
        // let ast = parser::LiteralExpressionParser::new()
        //     .parse(r#""Hello""#)
        //     .unwrap();
        // assert_eq!(ast, ast::LiteralExpression::String {
        //     s: "Hello".to_string(),
        // });

        // let ast = parser::LiteralExpressionParser::new()
        //     .parse(r#""Hello, world!""#)
        //     .unwrap();
        // assert_eq!(ast, ast::LiteralExpression::String {
        //     s: "Hello, world!".to_string(),
        // });

        // Character literals
        // assert_eq!(parser::CharLiteralParser::new().parse('a'), Ok('a'));
        // assert_eq!(parser::CharLiteralParser::new().parse('z'), Ok('z'));
        // assert_eq!(parser::CharLiteralParser::new().parse('A'), Ok('A'));
        // assert_eq!(parser::CharLiteralParser::new().parse('Z'), Ok('Z'));
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
            parser::IntegerLiteralParser::new().parse("0XA_B_C_D_E_F_i16"),
            Ok(0xABCDEF)
        );
        // TODO Error("Expected a digit, but found '_'. Integer literals cannot start
        // assert_eq!(
        //     parser::IntegerLiteralParser::new().parse("0X_A_B_C_D_E_F_i16"),
        //     Ok(0xABCDEF)
        // );
        assert_eq!(parser::IntegerLiteralParser::new().parse("22i32"), Ok(22));
        assert_eq!(parser::IntegerLiteralParser::new().parse("0i64"), Ok(0));
        assert_eq!(
            parser::IntegerLiteralParser::new().parse("1_2_3_i128"),
            Ok(123)
        );
        // TODO Error("Expected a digit, but found '_'. Integer literals cannot start
        // with an underscore.") assert_eq!(
        //     parser::IntegerLiteralParser::new().parse("_1_2_3_i128"),
        //     Ok(123)
        // );
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
        assert_eq!(parser::DecimalLiteralParser::new().parse("1_2_3_"), Ok(123));
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
        // TODO assert that this is an error (decimal literal cannot start with an
        // underscore)
        // assert_eq!(
        //     parser::DecimalLiteralParser::new().parse("_1_2_3_"),
        //     Err(ParseError::User {
        //         error: InvalidDecimalLiteral,
        //     })
        // );

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
    fn test_parse_float_literals() {
        // let ast = parser::LiteralExpressionParser::new()
        //     .parse("22.0")
        //     .unwrap();
        // assert_eq!(ast, ast::LiteralExpression::Float { f: 22.0 });

        // let ast = parser::LiteralExpressionParser::new().parse("0.0").unwrap();
        // assert_eq!(ast, ast::LiteralExpression::Float { f: 0.0 });

        // DEC_LITERAL . DEC_LITERAL FLOAT_EXPONENT?
        assert_eq!(parser::FloatLiteralParser::new().parse("22.0"), Ok(22.0));
        assert_eq!(
            parser::FloatLiteralParser::new().parse("22.0e+1"),
            Ok(22.0e+1)
        );
        assert_eq!(
            parser::FloatLiteralParser::new().parse("1234.5e-6"),
            Ok(1234.5e-6)
        );
        assert_eq!(
            parser::FloatLiteralParser::new().parse("12E+99_"),
            Ok(12E+99)
        );
        assert_eq!(parser::FloatLiteralParser::new().parse("12."), Ok(12.0));

        // Invalid Float Exponents
        assert_eq!(
            parser::FloatExponentParser::new().parse("e+_"),
            Err(ParseError::User {
                error: InvalidFloatExponent,
            })
        );
        assert_eq!(
            parser::FloatExponentParser::new().parse("E-_"),
            Err(ParseError::User {
                error: InvalidFloatExponent,
            })
        );

        // Valid Float Exponents
        assert_eq!(
            parser::FloatExponentParser::new().parse("e+3"),
            Ok(String::from("e+3"))
        );
        assert_eq!(
            parser::FloatExponentParser::new().parse("E-3"),
            Ok(String::from("E-3"))
        );
        assert_eq!(
            parser::FloatExponentParser::new().parse("e+123"),
            Ok(String::from("e+123"))
        );
        assert_eq!(
            parser::FloatExponentParser::new().parse("E-123"),
            Ok(String::from("E-123"))
        );
        assert_eq!(
            parser::FloatExponentParser::new().parse("e+_0_1_2_3_4_5_6_7_8_9"),
            Ok(String::from("e+_0_1_2_3_4_5_6_7_8_9"))
        );
        assert_eq!(
            parser::FloatExponentParser::new().parse("E-_0_1_2_3_4_5_6_7_8_9"),
            Ok(String::from("E-_0_1_2_3_4_5_6_7_8_9"))
        );
        assert!(parser::FloatExponentParser::new().parse("e+").is_err());

        // "+" | "-"
        assert_eq!(parser::PlusOrMinusSignParser::new().parse("+"), Ok('+'));
        assert_eq!(parser::PlusOrMinusSignParser::new().parse("-"), Ok('-'));
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
    use pretty_assertions::assert_eq;

    // TODO maybe in future expand to deserializing parsed nonterminals into
    // well-defined data structures housing tokens along with their lexemes
    // and spans. Similar to how lexing is done in Logos.
    // In contrast, the current implementation is better for having well-defined,
    // and simple to reason about action code within the parser

    // TODO maybe add a redundant lexer implementation via Logos
    // for emitting tokens along with their lexemes, spans, and usage in the
    // language.

    // TODO refactor lexing to more idiomatic Rust data structures
    // (e.g. using enums instead of strings)
    // Example: PlusSignParser::new().parse("+").unwrap() ->
    // Token::Punctuation::Plus

    #[test]
    fn test_lex_punctuation() {
        assert_eq!(parser::PlusParser::new().parse("+"), Ok(Punctuation::Plus));
        assert_eq!(
            parser::MinusParser::new().parse("-"),
            Ok(Punctuation::Minus)
        );
        assert_eq!(parser::StarParser::new().parse("*"), Ok(Punctuation::Star));
        assert_eq!(
            parser::SlashParser::new().parse("/"),
            Ok(Punctuation::Slash)
        );
        assert_eq!(
            parser::BackslashParser::new().parse("\\"),
            Ok(Punctuation::Backslash)
        );
        assert_eq!(
            parser::PercentParser::new().parse("%"),
            Ok(Punctuation::Percent)
        );
        assert_eq!(
            parser::CaretParser::new().parse("^"),
            Ok(Punctuation::Caret)
        );
        assert_eq!(parser::NotParser::new().parse("!"), Ok(Punctuation::Not));
        assert_eq!(parser::AndParser::new().parse("&"), Ok(Punctuation::And));
        assert_eq!(parser::OrParser::new().parse("|"), Ok(Punctuation::Or));
        assert_eq!(
            parser::AndAndParser::new().parse("&&"),
            Ok(Punctuation::AndAnd)
        );
        assert_eq!(parser::OrOrParser::new().parse("||"), Ok(Punctuation::OrOr));
        assert_eq!(
            parser::PlusEqualsParser::new().parse("+="),
            Ok(Punctuation::PlusEquals)
        );
        assert_eq!(
            parser::MinusEqualsParser::new().parse("-="),
            Ok(Punctuation::MinusEquals)
        );
        assert_eq!(
            parser::StarEqualsParser::new().parse("*="),
            Ok(Punctuation::StarEquals)
        );
        assert_eq!(
            parser::SlashEqualsParser::new().parse("/="),
            Ok(Punctuation::SlashEquals)
        );
        assert_eq!(
            parser::PercentEqualsParser::new().parse("%="),
            Ok(Punctuation::PercentEquals)
        );
        assert_eq!(
            parser::CaretEqualsParser::new().parse("^="),
            Ok(Punctuation::CaretEquals)
        );
        assert_eq!(
            parser::AndEqualsParser::new().parse("&="),
            Ok(Punctuation::AndEquals)
        );
        assert_eq!(
            parser::OrEqualsParser::new().parse("|="),
            Ok(Punctuation::OrEquals)
        );
        assert_eq!(
            parser::EqualsParser::new().parse("="),
            Ok(Punctuation::Equals)
        );
        assert_eq!(
            parser::DoubleEqualsParser::new().parse("=="),
            Ok(Punctuation::DoubleEquals)
        );
        assert_eq!(
            parser::NotEqualParser::new().parse("!="),
            Ok(Punctuation::NotEqual)
        );
        assert_eq!(
            parser::NotEqualParser::new().parse("≠"),
            Ok(Punctuation::NotEqual)
        );
        assert_eq!(
            parser::LessThanParser::new().parse("<"),
            Ok(Punctuation::LessThan)
        );
        assert_eq!(
            parser::LessThanEqualParser::new().parse("<="),
            Ok(Punctuation::LessThanEqual)
        );
        assert_eq!(
            parser::LessThanEqualParser::new().parse("≤"),
            Ok(Punctuation::LessThanEqual)
        );
        assert_eq!(
            parser::GreaterThanParser::new().parse(">"),
            Ok(Punctuation::GreaterThan)
        );
        assert_eq!(
            parser::GreaterThanEqualParser::new().parse(">="),
            Ok(Punctuation::GreaterThanEqual)
        );
        assert_eq!(
            parser::GreaterThanEqualParser::new().parse("≥"),
            Ok(Punctuation::GreaterThanEqual)
        );
        assert_eq!(
            parser::UnderscoreParser::new().parse("_"),
            Ok(Punctuation::Underscore)
        );
        assert_eq!(parser::DotParser::new().parse("."), Ok(Punctuation::Dot));
        assert_eq!(
            parser::DotDotParser::new().parse(".."),
            Ok(Punctuation::DotDot)
        );
        assert_eq!(
            parser::DotDotDotParser::new().parse("..."),
            Ok(Punctuation::DotDotDot)
        );
        assert_eq!(
            parser::DotDotEqualsParser::new().parse("..="),
            Ok(Punctuation::DotDotEquals)
        );
        assert_eq!(
            parser::CommaParser::new().parse(","),
            Ok(Punctuation::Comma)
        );
        assert_eq!(
            parser::SemicolonParser::new().parse(";"),
            Ok(Punctuation::Semicolon)
        );
        assert_eq!(
            parser::ColonParser::new().parse(":"),
            Ok(Punctuation::Colon)
        );
        assert_eq!(
            parser::PathSeparatorParser::new().parse("::"),
            Ok(Punctuation::PathSeparator)
        );
        assert_eq!(
            parser::RightArrowParser::new().parse("->"),
            Ok(Punctuation::RightArrow)
        );
        assert_eq!(
            parser::RightArrowParser::new().parse("→"),
            Ok(Punctuation::RightArrow)
        );
        assert_eq!(
            parser::LeftArrowParser::new().parse("<-"),
            Ok(Punctuation::LeftArrow)
        );
        assert_eq!(
            parser::LeftArrowParser::new().parse("←"),
            Ok(Punctuation::LeftArrow)
        );
        assert_eq!(
            parser::FatRightArrowParser::new().parse("=>"),
            Ok(Punctuation::FatRightArrow)
        );
        assert_eq!(
            parser::FatRightArrowParser::new().parse("⇒"),
            Ok(Punctuation::FatRightArrow)
        );
        assert_eq!(
            parser::FatLeftArrowParser::new().parse("⇐"),
            Ok(Punctuation::FatLeftArrow)
        );
        assert_eq!(
            parser::PoundParser::new().parse("#"),
            Ok(Punctuation::Pound)
        );
        assert_eq!(
            parser::DollarParser::new().parse("$"),
            Ok(Punctuation::Dollar)
        );
        assert_eq!(
            parser::QuestionParser::new().parse("?"),
            Ok(Punctuation::Question)
        );
    }

    #[test]
    fn test_lex_delimiters() {
        assert_eq!(
            parser::DelimiterParser::new().parse("("),
            Ok(Delimiter::LeftParen)
        );
        assert_eq!(
            parser::DelimiterParser::new().parse(")"),
            Ok(Delimiter::RightParen)
        );
        assert_eq!(
            parser::DelimiterParser::new().parse("["),
            Ok(Delimiter::LeftBracket)
        );
        assert_eq!(
            parser::DelimiterParser::new().parse("]"),
            Ok(Delimiter::RightBracket)
        );
        assert_eq!(
            parser::DelimiterParser::new().parse("{"),
            Ok(Delimiter::LeftBrace)
        );
        assert_eq!(
            parser::DelimiterParser::new().parse("}"),
            Ok(Delimiter::RightBrace)
        );
    }

    #[test]
    fn test_lex_numeric_literals() {
        // Integer Literal Prefix Terminals
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

        // Integer Literal Suffix Terminals

        // - Unsigned Integer Literal Suffix Terminals
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

        // - Signed Integer Literal Suffix Terminals
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

        // Digit Terminals

        // - Binary Digit Terminals
        assert_eq!(parser::BinaryDigitParser::new().parse("0"), Ok('0'));
        assert_eq!(parser::BinaryDigitParser::new().parse("1"), Ok('1'));

        // - Octal Digit Terminals
        assert_eq!(parser::OctalDigitParser::new().parse("0"), Ok('0'));
        assert_eq!(parser::OctalDigitParser::new().parse("1"), Ok('1'));
        assert_eq!(parser::OctalDigitParser::new().parse("2"), Ok('2'));
        assert_eq!(parser::OctalDigitParser::new().parse("3"), Ok('3'));
        assert_eq!(parser::OctalDigitParser::new().parse("4"), Ok('4'));
        assert_eq!(parser::OctalDigitParser::new().parse("5"), Ok('5'));
        assert_eq!(parser::OctalDigitParser::new().parse("6"), Ok('6'));
        assert_eq!(parser::OctalDigitParser::new().parse("7"), Ok('7'));

        // - Decimal Digit Terminals
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

        // - Hexadecimal Digit Terminals
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

        // Float Literal Suffix Terminals
        assert_eq!(
            parser::FloatSuffixParser::new().parse("f32"),
            Ok(String::from("f32"))
        );
        assert_eq!(
            parser::FloatSuffixParser::new().parse("f64"),
            Ok(String::from("f64"))
        );

        // Float Literal Exponent Symbol Terminals
        assert_eq!(parser::ExponentialSymbolParser::new().parse("e"), Ok('e'));
        assert_eq!(parser::ExponentialSymbolParser::new().parse("E"), Ok('E'));
    }

    #[test]
    fn test_lex_mathematical_constants() {
        ////////////////////////////
        // Mathematical Constants //
        ////////////////////////////

        // Pi Terminals (π)
        assert_eq!(
            parser::PiParser::new().parse("pi"),
            Ok(MathematicalConstant::Pi {
                value: std::f64::consts::PI,
            })
        );
        assert_eq!(
            parser::PiParser::new().parse("π"),
            Ok(MathematicalConstant::Pi {
                value: std::f64::consts::PI,
            })
        );
        assert_eq!(
            parser::PiParser::new().parse("𝜋"),
            Ok(MathematicalConstant::Pi {
                value: std::f64::consts::PI,
            })
        );

        // Euler's Number Terminals (e)
        assert_eq!(
            parser::EulerParser::new().parse("ℯ"),
            Ok(MathematicalConstant::Euler {
                value: std::f64::consts::E,
            })
        );
        assert_eq!(
            parser::EulerParser::new().parse("euler"),
            Ok(MathematicalConstant::Euler {
                value: std::f64::consts::E,
            })
        );

        // Tau Terminals (τ)
        assert_eq!(
            parser::TauParser::new().parse("tau"),
            Ok(MathematicalConstant::Tau {
                value: std::f64::consts::TAU,
            })
        );
        assert_eq!(
            parser::TauParser::new().parse("τ"),
            Ok(MathematicalConstant::Tau {
                value: std::f64::consts::TAU,
            })
        );
        assert_eq!(
            parser::TauParser::new().parse("𝜏"),
            Ok(MathematicalConstant::Tau {
                value: std::f64::consts::TAU,
            })
        );

        // Catalan's Constant Terminals (γ)
        assert_eq!(
            parser::CatalanParser::new().parse("catalan"),
            Ok(MathematicalConstant::Catalan {
                value: 0.91596559417721901505460351493238411077414937428167,
            })
        );

        // Golden Ratio Terminals (φ)
        assert_eq!(
            parser::GoldenRatioParser::new().parse("golden"),
            Ok(MathematicalConstant::GoldenRatio {
                value: 1.618033988749895,
            })
        );
        assert_eq!(
            parser::GoldenRatioParser::new().parse("φ"),
            Ok(MathematicalConstant::GoldenRatio {
                value: 1.618033988749895,
            })
        );
        assert_eq!(
            parser::GoldenRatioParser::new().parse("𝜙"),
            Ok(MathematicalConstant::GoldenRatio {
                value: 1.618033988749895,
            })
        );

        // Euler Mascheroni Constant Terminals (γ)
        assert_eq!(
            parser::EulerMascheroniParser::new().parse("eulermascheroni"),
            Ok(MathematicalConstant::EulerMascheroni {
                value: 0.5772156649015329,
            })
        );
        assert_eq!(
            parser::EulerMascheroniParser::new().parse("eulergamma"),
            Ok(MathematicalConstant::EulerMascheroni {
                value: 0.5772156649015329,
            })
        );
        assert_eq!(
            parser::EulerMascheroniParser::new().parse("γ"),
            Ok(MathematicalConstant::EulerMascheroni {
                value: 0.5772156649015329,
            })
        );
        assert_eq!(
            parser::EulerMascheroniParser::new().parse("𝛾"),
            Ok(MathematicalConstant::EulerMascheroni {
                value: 0.5772156649015329,
            })
        );

        // Infinity Terminals (∞)
        assert_eq!(
            parser::InfinityParser::new().parse("Inf"),
            Ok(MathematicalConstant::Infinity {
                value: std::f64::INFINITY,
            })
        );
        assert_eq!(
            parser::InfinityParser::new().parse("∞"),
            Ok(MathematicalConstant::Infinity {
                value: std::f64::INFINITY,
            })
        );

        // Not a Number Terminals (NaN)
        assert_eq!(
            parser::NotANumberParser::new().parse("NaN"),
            Ok(MathematicalConstant::NotANumber)
        );
    }

    #[test]
    fn test_lex_superscript_numeric_literals() {
        // SuperscriptIntegerLiteral
        assert_eq!(
            parser::SuperscriptIntegerLiteralParser::new().parse("¹"),
            Ok(SuperscriptIntegerLiteral { n: 1 })
        );
        assert_eq!(
            parser::SuperscriptIntegerLiteralParser::new().parse("¹²"),
            Ok(SuperscriptIntegerLiteral { n: 12 })
        );

        // SuperscriptDecimalDigit = "⁰" | "¹" | "²" | "³" | "⁴" | "⁵" | "⁶" | "⁷" | "⁸" | "⁹"
        assert_eq!(
            parser::SuperscriptDecimalDigitParser::new().parse("⁰"),
            Ok(SuperscriptDecimalDigit { digit: 0 })
        );
        assert_eq!(
            parser::SuperscriptDecimalDigitParser::new().parse("¹"),
            Ok(SuperscriptDecimalDigit { digit: 1 })
        );
        assert_eq!(
            parser::SuperscriptDecimalDigitParser::new().parse("²"),
            Ok(SuperscriptDecimalDigit { digit: 2 })
        );
        assert_eq!(
            parser::SuperscriptDecimalDigitParser::new().parse("³"),
            Ok(SuperscriptDecimalDigit { digit: 3 })
        );
        assert_eq!(
            parser::SuperscriptDecimalDigitParser::new().parse("⁴"),
            Ok(SuperscriptDecimalDigit { digit: 4 })
        );
        assert_eq!(
            parser::SuperscriptDecimalDigitParser::new().parse("⁵"),
            Ok(SuperscriptDecimalDigit { digit: 5 })
        );
        assert_eq!(
            parser::SuperscriptDecimalDigitParser::new().parse("⁶"),
            Ok(SuperscriptDecimalDigit { digit: 6 })
        );
        assert_eq!(
            parser::SuperscriptDecimalDigitParser::new().parse("⁷"),
            Ok(SuperscriptDecimalDigit { digit: 7 })
        );
        assert_eq!(
            parser::SuperscriptDecimalDigitParser::new().parse("⁸"),
            Ok(SuperscriptDecimalDigit { digit: 8 })
        );
        assert_eq!(
            parser::SuperscriptDecimalDigitParser::new().parse("⁹"),
            Ok(SuperscriptDecimalDigit { digit: 9 })
        );
    }

    #[test]
    fn test_lex_superscript_punctuation() {
        // SuperscriptPunctuation = "⁺" | "⁻" | "⁽" | "⁾"
        assert_eq!(
            parser::SuperscriptPunctuationParser::new().parse("⁺"),
            Ok(SuperscriptPunctuation::Plus)
        );
        assert_eq!(
            parser::SuperscriptPunctuationParser::new().parse("⁻"),
            Ok(SuperscriptPunctuation::Minus)
        );
        assert_eq!(
            parser::SuperscriptPunctuationParser::new().parse("⁽"),
            Ok(SuperscriptPunctuation::LeftParen)
        );
        assert_eq!(
            parser::SuperscriptPunctuationParser::new().parse("⁾"),
            Ok(SuperscriptPunctuation::RightParen)
        );
    }

    #[test]
    fn test_lex_subscript_symbols() {

    }

    #[test]
    fn test_lex_mathematical_symbols() {
        // Powers (e.g. 2²)
        assert_eq!(
            parser::MathematicalSymbolParser::new().parse("¹²"),
            Ok(MathematicalSymbol::Power { exponent: 12 })
        );
        assert_eq!(
            parser::MathematicalSymbolParser::new().parse("⁻⁵³"),
            Ok(MathematicalSymbol::Power { exponent: -53 })
        );
        assert_eq!(
            parser::MathematicalSymbolParser::new().parse("⁺⁵³"),
            Ok(MathematicalSymbol::Power { exponent: 53 })
        );

        // Roots (e.g. ²√4 = 2)
        assert_eq!(
            parser::MathematicalSymbolParser::new().parse("√"),
            Ok(MathematicalSymbol::Root { exponent: 2 })
        );
        assert_eq!(
            parser::MathematicalSymbolParser::new().parse("²√"),
            Ok(MathematicalSymbol::Root { exponent: 2 })
        );
        assert_eq!(
            parser::MathematicalSymbolParser::new().parse("¹²√"),
            Ok(MathematicalSymbol::Root { exponent: 12 })
        );
        assert_eq!(
            parser::MathematicalSymbolParser::new().parse("⁻⁵³√"),
            Ok(MathematicalSymbol::Root { exponent: -53 })
        );
        assert_eq!(
            parser::MathematicalSymbolParser::new().parse("⁺⁵³√"),
            Ok(MathematicalSymbol::Root { exponent: 53 })
        );
        assert_eq!(
            parser::MathematicalSymbolParser::new().parse("∛"),
            Ok(MathematicalSymbol::Root { exponent: 3 })
        );
        assert_eq!(
            parser::MathematicalSymbolParser::new().parse("∜"),
            Ok(MathematicalSymbol::Root { exponent: 4 })
        );

        // Fractions (e.g. 1//2 = ½)

        // General Mathematical Symbols (e.g. ÷, ∫, etc.)
        assert_eq!(
            parser::MathematicalSymbolParser::new().parse("÷"),
            Ok(MathematicalSymbol::Division)
        );
        assert_eq!(
            parser::MathematicalSymbolParser::new().parse("∝"),
            Ok(MathematicalSymbol::ProportionalTo)
        );
        // assert_eq!(
        //     parser::MathematicalSymbolParser::new().parse("∠"),
        //     Ok(MathematicalSymbol::Angle)
        // );
        // assert_eq!(
        //     parser::MathematicalSymbolParser::new().parse("∧"),
        //     Ok(MathematicalSymbol::LogicalAnd)
        // );
        // assert_eq!(
        //     parser::MathematicalSymbolParser::new().parse("∨"),
        //     Ok(MathematicalSymbol::LogicalOr)
        // );
        assert_eq!(
            parser::MathematicalSymbolParser::new().parse("∩"),
            Ok(MathematicalSymbol::Intersection)
        );
        assert_eq!(
            parser::MathematicalSymbolParser::new().parse("∪"),
            Ok(MathematicalSymbol::Union)
        );
        assert_eq!(
            parser::MathematicalSymbolParser::new().parse("∫"),
            Ok(MathematicalSymbol::Integral)
        );
        assert_eq!(
            parser::MathematicalSymbolParser::new().parse("∴"),
            Ok(MathematicalSymbol::Therefore)
        );
        assert_eq!(
            parser::MathematicalSymbolParser::new().parse("∵"),
            Ok(MathematicalSymbol::Because)
        );
        assert_eq!(
            parser::MathematicalSymbolParser::new().parse("≈"),
            Ok(MathematicalSymbol::ApproximatelyEqual)
        );
        assert_eq!(
            parser::MathematicalSymbolParser::new().parse("≉"),
            Ok(MathematicalSymbol::NotApproximatelyEqual)
        );
        assert_eq!(
            parser::MathematicalSymbolParser::new().parse("≡"),
            Ok(MathematicalSymbol::IdenticalTo)
        );
        assert_eq!(
            parser::MathematicalSymbolParser::new().parse("≢"),
            Ok(MathematicalSymbol::NotIdenticalTo)
        );
        assert_eq!(
            parser::MathematicalSymbolParser::new().parse("⊂"),
            Ok(MathematicalSymbol::SubsetOf)
        );
        assert_eq!(
            parser::MathematicalSymbolParser::new().parse("⊃"),
            Ok(MathematicalSymbol::SupersetOf)
        );
        assert_eq!(
            parser::MathematicalSymbolParser::new().parse("⊆"),
            Ok(MathematicalSymbol::SubsetOfOrEqualTo)
        );
        assert_eq!(
            parser::MathematicalSymbolParser::new().parse("⊇"),
            Ok(MathematicalSymbol::SupersetOfOrEqualTo)
        );
        // assert_eq!(
        //     parser::MathematicalSymbolParser::new().parse("⊕"),
        //     Ok(MathematicalSymbol::Plus)
        // );
        // assert_eq!(
        //     parser::MathematicalSymbolParser::new().parse("⊖"),
        //     Ok(MathematicalSymbol::Minus)
        // );
        // assert_eq!(
        //     parser::MathematicalSymbolParser::new().parse("⊗"),
        //     Ok(MathematicalSymbol::Multiplication)
        // );
        // assert_eq!(
        //     parser::MathematicalSymbolParser::new().parse("⊘"),
        //     Ok(MathematicalSymbol::Division)
        // );
        // assert_eq!(
        //     parser::MathematicalSymbolParser::new().parse("⊙"),
        //     Ok(MathematicalSymbol::Circle)
        // );
        // assert_eq!(
        //     parser::MathematicalSymbolParser::new().parse("⊥"),
        //     Ok(MathematicalSymbol::Perpendicular)
        // );
        // assert_eq!(
        //     parser::MathematicalSymbolParser::new().parse("⋅"),
        //     Ok(MathematicalSymbol::Dot)
        // );
        // assert_eq!(
        //     parser::MathematicalSymbolParser::new().parse("⋯"),
        //     Ok(MathematicalSymbol::HorizontalEllipsis)
        // );
        // assert_eq!(
        //     parser::MathematicalSymbolParser::new().parse("⌈"),
        //     Ok(MathematicalSymbol::LeftCeiling)
        // );
        // assert_eq!(
        //     parser::MathematicalSymbolParser::new().parse("⌉"),
        //     Ok(MathematicalSymbol::RightCeiling)
        // );
        // assert_eq!(
        //     parser::MathematicalSymbolParser::new().parse("⌊"),
        //     Ok(MathematicalSymbol::LeftFloor)
        // );
        // assert_eq!(
        //     parser::MathematicalSymbolParser::new().parse("⌋"),
        //     Ok(MathematicalSymbol::RightFloor)
        // );
        // assert_eq!(
        //     parser::MathematicalSymbolParser::new().parse("〈"),
        //     Ok(MathematicalSymbol::LeftPointingAngleBracket)
        // );
        // assert_eq!(
        //     parser::MathematicalSymbolParser::new().parse("〉"),
        //     Ok(MathematicalSymbol::RightPointingAngleBracket)
        // );

        // TODO refactor to SuperScriptSymbol (e.g. ², ³, ⁴, ⁵, ⁶, ⁷, ⁸, ⁹)
        // and SubScriptSymbol (e.g. ₀, ₁, ₂, ₃, ₄, ₅, ₆, ₇, ₈, ₉)
        // and PowerParser (e.g. 10² -> 10^2, 10⁴³ -> 10^43
    }

    #[test]
    fn test_lex_character_and_string_literals() {
        // Apostrophe
        assert_eq!(parser::ApostropheParser::new().parse("'"), Ok('\''));

        // Quote
        assert_eq!(parser::QuoteParser::new().parse("\""), Ok('"'));
    }

    #[test]
    fn test_lex_keywords() {
        assert_eq!(parser::AsParser::new().parse("as"), Ok(Keyword::As));
        assert_eq!(
            parser::BreakParser::new().parse("break"),
            Ok(Keyword::Break)
        );
        assert_eq!(
            parser::ConstParser::new().parse("const"),
            Ok(Keyword::Const)
        );
        assert_eq!(
            parser::ContinueParser::new().parse("continue"),
            Ok(Keyword::Continue)
        );
        assert_eq!(
            parser::CrateParser::new().parse("crate"),
            Ok(Keyword::Reserved(ReservedKeyword::Crate))
        );
        assert_eq!(parser::ElseParser::new().parse("else"), Ok(Keyword::Else));
        assert_eq!(parser::EnumParser::new().parse("enum"), Ok(Keyword::Enum));
        assert_eq!(
            parser::ExternParser::new().parse("extern"),
            Ok(Keyword::Reserved(ReservedKeyword::Extern))
        );
        assert_eq!(
            parser::FalseParser::new().parse("false"),
            Ok(Keyword::False)
        );
        assert_eq!(
            parser::FalseParser::new().parse("False"),
            Ok(Keyword::False)
        );
        assert_eq!(parser::FnParser::new().parse("fn"), Ok(Keyword::Fn));
        assert_eq!(parser::ForParser::new().parse("for"), Ok(Keyword::For));
        assert_eq!(parser::IfParser::new().parse("if"), Ok(Keyword::If));
        assert_eq!(parser::ImplParser::new().parse("impl"), Ok(Keyword::Impl));
        assert_eq!(parser::InParser::new().parse("in"), Ok(Keyword::In));
        assert_eq!(parser::LoopParser::new().parse("loop"), Ok(Keyword::Loop));
        assert_eq!(
            parser::MatchParser::new().parse("match"),
            Ok(Keyword::Match)
        );
        assert_eq!(
            parser::MissingParser::new().parse("missing"),
            Ok(Keyword::Missing)
        );
        assert_eq!(
            parser::MissingParser::new().parse("Missing"),
            Ok(Keyword::Missing)
        );
        assert_eq!(parser::ModParser::new().parse("mod"), Ok(Keyword::Mod));
        assert_eq!(parser::PubParser::new().parse("pub"), Ok(Keyword::Pub));
        assert_eq!(
            parser::ReturnParser::new().parse("return"),
            Ok(Keyword::Return)
        );
        assert_eq!(
            parser::SelfValueParser::new().parse("self"),
            Ok(Keyword::SelfValue)
        );
        assert_eq!(
            parser::SelfTypeParser::new().parse("Self"),
            Ok(Keyword::SelfType)
        );
        assert_eq!(
            parser::StaticParser::new().parse("static"),
            Ok(Keyword::Static)
        );
        assert_eq!(
            parser::StructParser::new().parse("struct"),
            Ok(Keyword::Struct)
        );
        assert_eq!(
            parser::SuperParser::new().parse("super"),
            Ok(Keyword::Super)
        );
        assert_eq!(
            parser::TraitParser::new().parse("trait"),
            Ok(Keyword::Trait)
        );
        assert_eq!(parser::TrueParser::new().parse("true"), Ok(Keyword::True));
        assert_eq!(parser::TrueParser::new().parse("True"), Ok(Keyword::True));
        assert_eq!(parser::TypeParser::new().parse("type"), Ok(Keyword::Type));
        assert_eq!(parser::UseParser::new().parse("use"), Ok(Keyword::Use));
        assert_eq!(
            parser::WhereParser::new().parse("where"),
            Ok(Keyword::Where)
        );
        assert_eq!(
            parser::WhileParser::new().parse("while"),
            Ok(Keyword::While)
        );
    }

    #[test]
    fn text_lex_reserved_keywords() {
        assert_eq!(
            parser::AbstractParser::new().parse("abstract"),
            Ok(Keyword::Reserved(ReservedKeyword::Abstract))
        );
        assert_eq!(
            parser::AsyncParser::new().parse("async"),
            Ok(Keyword::Reserved(ReservedKeyword::Async))
        );
        assert_eq!(
            parser::AwaitParser::new().parse("await"),
            Ok(Keyword::Reserved(ReservedKeyword::Await))
        );
        assert_eq!(
            parser::CrateParser::new().parse("crate"),
            Ok(Keyword::Reserved(ReservedKeyword::Crate))
        );
        assert_eq!(
            parser::DoParser::new().parse("do"),
            Ok(Keyword::Reserved(ReservedKeyword::Do))
        );
        assert_eq!(
            parser::DynParser::new().parse("dyn"),
            Ok(Keyword::Reserved(ReservedKeyword::Dyn))
        );
        assert_eq!(
            parser::ExportParser::new().parse("export"),
            Ok(Keyword::Reserved(ReservedKeyword::Export))
        );
        assert_eq!(
            parser::ExternParser::new().parse("extern"),
            Ok(Keyword::Reserved(ReservedKeyword::Extern))
        );
        assert_eq!(
            parser::FinalParser::new().parse("final"),
            Ok(Keyword::Reserved(ReservedKeyword::Final))
        );
        assert_eq!(
            parser::ImportParser::new().parse("import"),
            Ok(Keyword::Reserved(ReservedKeyword::Import))
        );
        assert_eq!(
            parser::LetParser::new().parse("let"),
            Ok(Keyword::Reserved(ReservedKeyword::Let))
        );
        assert_eq!(
            parser::MacroParser::new().parse("macro"),
            Ok(Keyword::Reserved(ReservedKeyword::Macro))
        );
        assert_eq!(
            parser::MoveParser::new().parse("move"),
            Ok(Keyword::Reserved(ReservedKeyword::Move))
        );
        assert_eq!(
            parser::MutParser::new().parse("mut"),
            Ok(Keyword::Reserved(ReservedKeyword::Mut))
        );
        assert_eq!(
            parser::OverrideParser::new().parse("override"),
            Ok(Keyword::Reserved(ReservedKeyword::Override))
        );
        assert_eq!(
            parser::PrivParser::new().parse("priv"),
            Ok(Keyword::Reserved(ReservedKeyword::Priv))
        );
        assert_eq!(
            parser::ProcParser::new().parse("proc"),
            Ok(Keyword::Reserved(ReservedKeyword::Proc))
        );
        assert_eq!(
            parser::RefParser::new().parse("ref"),
            Ok(Keyword::Reserved(ReservedKeyword::Ref))
        );
        assert_eq!(
            parser::TypeofParser::new().parse("typeof"),
            Ok(Keyword::Reserved(ReservedKeyword::Typeof))
        );
        assert_eq!(
            parser::UnsafeParser::new().parse("unsafe"),
            Ok(Keyword::Reserved(ReservedKeyword::Unsafe))
        );
        assert_eq!(
            parser::UnsizedParser::new().parse("unsized"),
            Ok(Keyword::Reserved(ReservedKeyword::Unsized))
        );
        assert_eq!(
            parser::VirtualParser::new().parse("virtual"),
            Ok(Keyword::Reserved(ReservedKeyword::Virtual))
        );
        assert_eq!(
            parser::YieldParser::new().parse("yield"),
            Ok(Keyword::Reserved(ReservedKeyword::Yield))
        );
    }

    #[test]
    fn test_lex_comments() {
        // Line comments
        assert_eq!(
            parser::LineCommentParser::new().parse("//"),
            Ok(Comment::LineComment {
                comment: String::from("//"),
            })
        );

        assert_eq!(
            parser::LineCommentParser::new().parse("//   - Only a comment"),
            Ok(Comment::LineComment {
                comment: String::from("//   - Only a comment"),
            })
        );
        assert_eq!(
            parser::LineCommentParser::new().parse("// Hello, world!"),
            Ok(Comment::LineComment {
                comment: String::from("// Hello, world!"),
            })
        );
        assert_eq!(
            parser::LineCommentParser::new().parse("// Hello, \tworld!"),
            Ok(Comment::LineComment {
                comment: String::from("// Hello, \tworld!"),
            })
        );
        assert_eq!(
            parser::LineCommentParser::new().parse(
                "// This is an incredibly long line comment that should be parsed \
                 correctly. It should also be parsed correctly if it contains a \
                 newline"
            ),
            Ok(Comment::LineComment {
                comment: String::from(
                    "// This is an incredibly long line comment that should be \
                     parsed correctly. It should also be parsed correctly if it \
                     contains a newline"
                ),
            })
        );
        assert!(parser::LineCommentParser::new()
            .parse("// Hello, \nworld!")
            .is_err());

        // Block comments
        assert_eq!(
            parser::BlockCommentParser::new().parse("/* */"),
            Ok(Comment::BlockComment {
                comment: String::from("/* */"),
            })
        );
        assert_eq!(
            parser::BlockCommentParser::new().parse("/**/"),
            Ok(Comment::BlockComment {
                comment: String::from("/**/"),
            })
        );
        assert_eq!(
            parser::BlockCommentParser::new().parse("/***/"),
            Ok(Comment::BlockComment {
                comment: String::from("/***/"),
            })
        );
        assert_eq!(
            parser::BlockCommentParser::new().parse("/*   - Only a comment */"),
            Ok(Comment::BlockComment {
                comment: String::from("/*   - Only a comment */"),
            })
        );
        assert_eq!(
            parser::BlockCommentParser::new().parse("/*** - Only a comment */"),
            Ok(Comment::BlockComment {
                comment: String::from("/*** - Only a comment */"),
            })
        );
        assert_eq!(
            parser::BlockCommentParser::new().parse("/* Hello, world! */"),
            Ok(Comment::BlockComment {
                comment: String::from("/* Hello, world! */"),
            })
        );
        assert_eq!(
            parser::BlockCommentParser::new().parse("/* Hello, \tworld! */"),
            Ok(Comment::BlockComment {
                comment: String::from("/* Hello, \tworld! */"),
            })
        );
        assert_eq!(
            parser::BlockCommentParser::new().parse(
                "/* This is an incredibly long\nblock comment that should be \
                 parsed correctly.\nIt should also be parsed correctly if it \
                 contains a newline */"
            ),
            Ok(Comment::BlockComment {
                comment: String::from(
                    "/* This is an incredibly long\nblock comment that should be \
                     parsed correctly.\nIt should also be parsed correctly if it \
                     contains a newline */"
                ),
            })
        );
        // TODO: Add implementation for nested block comments in the grammar
        // assert_eq!(
        //     parser::BlockCommentParser::new()
        //         .parse("/* In Balsapop /* we can /* nest comments */ */ */"),
        //     Ok(Comment::BlockComment {
        //         comment: String::from(
        //             "/* In Balsapop /* we can /* nest comments */ */ */"
        //         ),
        //     })
        // );

        // Doc comments (e.g. outer line doc comments, inner line doc comments, outer
        // block doc comments, inner block doc comments)

        // - Outer line doc comments
        assert_eq!(
            parser::OuterLineDocCommentParser::new().parse("///"),
            Ok(Comment::DocComment(DocComment::OuterLineDocComment {
                comment: String::from("///"),
            }))
        );
        assert_eq!(
            parser::OuterLineDocCommentParser::new()
                .parse("///  - Outer line doc (exactly 3 slashes)"),
            Ok(Comment::DocComment(DocComment::OuterLineDocComment {
                comment: String::from("///  - Outer line doc (exactly 3 slashes)"),
            }))
        );
        assert_eq!(
            parser::OuterLineDocCommentParser::new().parse("/// Hello, world!"),
            Ok(Comment::DocComment(DocComment::OuterLineDocComment {
                comment: String::from("/// Hello, world!"),
            }))
        );
        assert_eq!(
            parser::OuterLineDocCommentParser::new().parse("/// Hello, \tworld!"),
            Ok(Comment::DocComment(DocComment::OuterLineDocComment {
                comment: String::from("/// Hello, \tworld!"),
            }))
        );
        assert_eq!(
            parser::OuterLineDocCommentParser::new().parse(
                "/// This is an incredibly long line comment that should be parsed \
                 correctly. It should also be parsed correctly if it contains a \
                 newline"
            ),
            Ok(Comment::DocComment(DocComment::OuterLineDocComment {
                comment: String::from(
                    "/// This is an incredibly long line comment that should be \
                     parsed correctly. It should also be parsed correctly if it \
                     contains a newline"
                ),
            }))
        );
        assert!(parser::OuterLineDocCommentParser::new()
            .parse("/// Hello, \nworld!")
            .is_err());
        assert!(parser::OuterLineDocCommentParser::new()
            .parse("//// Hello, \nworld!")
            .is_err());

        // - Inner line doc comments
        assert_eq!(
            parser::InnerLineDocCommentParser::new().parse("//!"),
            Ok(Comment::DocComment(DocComment::InnerLineDocComment {
                comment: String::from("//!"),
            }))
        );
        assert_eq!(
            parser::InnerLineDocCommentParser::new().parse("//! Hello, world!"),
            Ok(Comment::DocComment(DocComment::InnerLineDocComment {
                comment: String::from("//! Hello, world!"),
            }))
        );
        assert_eq!(
            parser::InnerLineDocCommentParser::new().parse("//! Hello, \tworld!"),
            Ok(Comment::DocComment(DocComment::InnerLineDocComment {
                comment: String::from("//! Hello, \tworld!"),
            }))
        );
        assert_eq!(
            parser::InnerLineDocCommentParser::new().parse(
                "//! This is an incredibly long line comment that should be parsed \
                 correctly. It should also be parsed correctly if it contains a \
                 newline"
            ),
            Ok(Comment::DocComment(DocComment::InnerLineDocComment {
                comment: String::from(
                    "//! This is an incredibly long line comment that should be \
                     parsed correctly. It should also be parsed correctly if it \
                     contains a newline"
                ),
            }))
        );
        assert!(parser::InnerLineDocCommentParser::new()
            .parse("//! Hello, \nworld!")
            .is_err());

        // - Outer block doc comments
        // /**  - Outer block doc (exactly) 2 asterisks */
    }

    #[test]
    fn test_lex_logic_literals() {
        assert_eq!(
            parser::LogicLiteralParser::new().parse("true"),
            Ok(LogicLiteral::True { value: true })
        );
        assert_eq!(
            parser::LogicLiteralParser::new().parse("True"),
            Ok(LogicLiteral::True { value: true })
        );
        assert_eq!(
            parser::LogicLiteralParser::new().parse("false"),
            Ok(LogicLiteral::False { value: false })
        );
        assert_eq!(
            parser::LogicLiteralParser::new().parse("False"),
            Ok(LogicLiteral::False { value: false })
        );
        assert_eq!(
            parser::LogicLiteralParser::new().parse("missing"),
            Ok(LogicLiteral::Missing)
        );
        assert_eq!(
            parser::LogicLiteralParser::new().parse("Missing"),
            Ok(LogicLiteral::Missing)
        );
    }

    // fn test_lex_paths() {
        // Simple path segments
        // assert_eq!(
        //     parser::SimplePathSegmentParser::new().parse("a"),
        //     Ok(PathSegment::Simple(SimplePathSegment {
        //         name: String::from("a"),
        //     }))
        // );
        // assert_eq!(
        //     parser::SimplePathSegmentParser::new().parse("super"),
        //     Ok(SimplePathSegment::SuperSegment {
        //         segment: String::from("super"),
        //     })
        // );
        // assert_eq!(
        //     parser::SimplePathSegmentParser::new().parse("self"),
        //     Ok(SimplePathSegment::SelfValueSegment {
        //         segment: String::from("self"),
        //     })
        // );
        // assert_eq!(
        //     parser::SimplePathSegmentParser::new().parse("crate"),
        //     Ok(SimplePathSegment::CrateSegment {
        //         segment: String::from("crate"),
        //     })
        // );
    // }
}
