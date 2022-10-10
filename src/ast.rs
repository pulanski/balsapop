// #[derive(Clone, Debug)]
// pub(crate) enum Factor {
//     Int { n: i64 },
//     Var { name: String },
//     StringLiteral { s: String },
// }

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum LiteralExpression {
    Integer { n: i64 },
    Float { f: f64 },
    String { s: String },
    Char { c: char },
    Bool { b: bool },
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum Keyword {
    As,
    Break,
    Const,
    Continue,
    Else,
    Enum,
    False,
    Fn,
    For,
    If,
    Impl,
    In,
    Let,
    Loop,
    Match,
    Mod,
    Mut,
    Pub,
    Return,
    SelfValue,
    SelfType,
    Static,
    Struct,
    Super,
    Trait,
    True,
    Type,
    Use,
    While,
    Reserved(ReservedKeyword),
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum ReservedKeyword {
    Abstract,
    Async,
    Await,
    Become,
    Box,
    Crate,
    Do,
    Final,
    Macro,
    Move,
    Override,
    Priv,
    Proc,
    Pure,
    Ref,
    Typeof,
    Unsafe,
    Unsized,
    Virtual,
    Where,
    Yield,
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum Integer {
    U8 { n: u8 },
    U16 { n: u16 },
    U32 { n: u32 },
    U64 { n: u64 },
    U128 { n: u128 },
    USize { n: usize },
    I8 { n: i8 },
    I16 { n: i16 },
    I32 { n: i32 },
    I64 { n: i64 },
    I128 { n: i128 },
    ISize { n: isize },
}
