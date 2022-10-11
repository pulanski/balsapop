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
    Loop,
    Match,
    Mod,
    Pub,
    Reserved(ReservedKeyword),
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
    Where,
    While,
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum ReservedKeyword {
    Abstract,
    Async,
    Await,
    Crate,
    Do,
    Dyn,
    Export,
    Extern,
    Final,
    Import,
    Let,
    Macro,
    Move,
    Mut,
    Override,
    Priv,
    Proc,
    Ref,
    Typeof,
    Unsafe,
    Unsized,
    Virtual,
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
