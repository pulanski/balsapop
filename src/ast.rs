// #[derive(Clone, Debug)]
// pub(crate) enum Factor {
//     Int { n: i64 },
//     Var { name: String },
//     StringLiteral { s: String },
// }

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum LiteralExpression {
    NumericLiteral(NumericLiteral),
    // String(StringLiteral),
    // Char(CharacterLiteral),
    // Bool(BooleanLiteral),
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
    Missing,
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
pub enum Comment {
    LineComment { comment: String },
    BlockComment { comment: String },
    DocComment(DocComment),
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum DocComment {
    InnerLineDocComment { comment: String },
    InnerBlockDocComment { comment: String },
    OuterLineDocComment { comment: String },
    OuterBlockDocComment { comment: String },
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum NumericLiteral {
    Integer(Integer),
    Float(Float),
    MathematicalConstant(MathematicalConstant),
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

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum Float {
    F32 { f: f32 },
    F64 { f: f64 },
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum MathematicalConstant {
    Pi { value: f64 },
    Euler { value: f64 },
    EulerMascheroni { value: f64 },
    Tau { value: f64 },
    Catalan { value: f64 },
    GoldenRatio { value: f64 },
    Infinity { value: f64 },
    NotANumber,
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum Punctuation {
    Plus,
    Minus,
    Star,
    Slash,
    Backslash,
    Percent,
    Caret,
    Not,
    And,
    Or,
    AndAnd,
    OrOr,
    PlusEquals,
    MinusEquals,
    StarEquals,
    SlashEquals,
    PercentEquals,
    CaretEquals,
    AndEquals,
    OrEquals,
    Equals,
    DoubleEquals,
    NotEqual,
    LessThan,
    LessThanEqual,
    GreaterThan,
    GreaterThanEqual,
    Underscore,
    Dot,
    DotDot,
    DotDotDot,
    DotDotEquals,
    Comma,
    Colon,
    Semicolon,
    PathSeparator,
    RightArrow,
    LeftArrow,
    FatRightArrow,
    FatLeftArrow,
    Pound,
    Dollar,
    Question,
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum Delimiter {
    LeftParen,
    RightParen,
    LeftBracket,
    RightBracket,
    LeftBrace,
    RightBrace,
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum MathematicalSymbol {
    Root { exponent: i8 },
    Power { exponent: i8 },
    Division,
    ProportionalTo,
    Intersection,
    Union,
    Integral,
    Sum,
    Therefore,
    Because,
    ApproximatelyEqual,
    NotApproximatelyEqual,
    IdenticalTo,
    NotIdenticalTo,
    SubsetOf,
    SubsetOfOrEqualTo,
    NotSubsetOf,
    SupersetOf,
    NotSupersetOf,
    SupersetOfOrEqualTo,
    Logarithm,
    NaturalLogarithm,
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Exponent {
    pub value: i8,
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SuperscriptIntegerLiteral {
    pub n: i8,
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SuperscriptDecimalDigit {
    pub digit: i8,
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum SuperscriptPunctuation {
    Plus,
    Minus,
    LeftParen,
    RightParen,
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SimplePath {
    pub segments: Vec<SimplePathSegment>,
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum SimplePathSegment {
    /// NOTE: naming is done in this manner to avoid conflicts with the
    /// keywords "crate", "self", and "super"
    IdentifierSegment { segment: String },
    SuperSegment { segment: String },
    SelfValueSegment { segment: String },
    CrateSegment { segment: String },
    // DollarCrate { segment: String },
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum LogicLiteral {
    True { value: bool },
    False { value: bool },
    Missing
}

// #[derive(Clone, Debug, PartialEq, PartialOrd)]
// pub enum Whitespace {
//     Space,
//     Tab,
//     Newline,
//     CarriageReturn,
//     LineFeed,
// }
