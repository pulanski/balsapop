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
