//! Defines all valid operand types.

use std::fmt::Display;

use syn::{Expr, Ident, Lit};

use super::function::Function;

/// Enumerates all of the types supported by the language.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Type {
    /// A signed integer.
    I(u32),
    /// A unsigned integer.
    U(u32),
    /// A 16 bit floating point value.
    F16,
    /// A 32 bit floating point value.
    F32,
    /// A 64 bit floating point value.
    F64,
    /// A 128 bit floating point value.
    F128,
    /// The value should `never` be used in an assign statement.
    Unit,
}

impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Self::F16 => "f16".to_string(),
            Self::F32 => "f32".to_string(),
            Self::F64 => "f64".to_string(),
            Self::F128 => "f128".to_string(),
            Self::Unit => "()".to_string(),
            Self::I(bits) => format!("i{bits}"),
            Self::U(bits) => format!("i{bits}"),
        })
    }
}

/// Enumerates all valid operand types.
#[derive(Debug, Clone, PartialEq)]
pub enum Operand {
    /// A general expression [`ExprOperand`].
    Expr((ExprOperand, Option<Type>)),
    /// A plain identifier.
    Ident((IdentOperand, Option<Type>)),
    /// Field extraction.
    FieldExtract((FieldExtract, Option<Type>)),
    /// Field extraction.
    DynamicFieldExtract((DynamicFieldExtract, Option<Type>)),

    /// A wrapped literal.
    WrappedLiteral(WrappedLiteral),
}

#[derive(Debug, Clone, PartialEq)]
/// Enumerates a set of different operands.
///
/// These operands are not new identifiers but can be already defined
/// [`Ident`](struct@Ident)ifiers.
pub enum ExprOperand {
    /// A parenthesis containing an ordinary rust expression.
    ///
    /// This allows inline rust expressions the the DSL.
    Paren(Expr),
    /// A chain like
    /// ```ignore
    /// a.local(<args>).unwrap()
    /// ```
    Chain(Box<ExprOperand>, Vec<(Ident, Vec<Box<Operand>>)>),
    /// A plain identifier.
    Ident(Ident),
    /// A plain literal.
    Literal(Lit),
    /// A function call, this can be either a intrinsic function or a rust
    /// function.
    FunctionCall(Function),
}

#[derive(Debug, Clone, PartialEq)]
/// A wrapped literal.
pub struct WrappedLiteral {
    /// The value of the literal.
    pub val: Lit,
    /// The type of the literal.
    pub ty: Type,
}

/// A (possibly) new identifier.
#[derive(Debug, Clone, PartialEq)]
pub struct IdentOperand {
    /// Whether or not to insert this in to the local scope or not
    pub define: bool,
    /// The identifier used
    pub ident: Ident,
}

#[derive(Debug, Clone, PartialEq)]
/// Valid delimiters for a [`FieldExtract`].
pub enum DelimiterType {
    /// Can be a plain number.
    Const(Lit),
    /// Can be a rust variable.
    Ident(Ident),
}

#[derive(Debug, Clone, PartialEq)]
/// Field extraction.
///
/// This extracts the specified number of bits
/// from the operand and right justifies the result.
pub struct FieldExtract {
    /// The operand to extract from.
    pub operand: Ident,
    /// The first bit to include.
    pub start: u32, //DelimiterType,
    /// The last bit to include.
    pub end: u32, //DelimiterType,
    /// The type for the mask.
    pub ty: Option<syn::Type>,
}

#[derive(Debug, Clone, PartialEq)]
/// Field extraction.
///
/// This extracts the specified number of bits
/// from the operand and right justifies the result.
pub struct DynamicFieldExtract {
    /// The operand to extract from.
    pub operand: Ident,
    /// The first bit to include.
    pub start: DelimiterType,
    /// The last bit to include.
    pub end: DelimiterType,
    /// The type for the mask.
    pub ty: Option<syn::Type>,
}

#[derive(Debug, Clone, PartialEq)]
/// Sets the operand type.
pub struct SetType {
    /// The operand identifier.
    pub operand: Ident,
    /// The operand type.
    pub ty: Type,
}
