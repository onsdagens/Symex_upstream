//! Defines the supported arithmetic operations.

use syn::Ident;

use super::operand::Operand;

/// A generic operation,
///
/// This allows syntax like
/// ```ignore
/// let a = b + c + d;
/// ```
pub enum Operation {
    /// A binary operation.
    BinOp(Box<(Operand, BinaryOperation, Operand)>),
    /// A unary operation.
    UnOp(Box<(UnaryOperation, Operand)>),
}

/// Enumerates all valid binary operations.
///
/// This is merely a type-level denotation of
/// operations such as + or -.
#[derive(Debug, Clone, PartialEq)]
#[allow(missing_docs)]
pub enum BinaryOperation {
    Sub,
    /// Saturating sub.
    SSub,
    Add,
    /// Saturating add.
    SAdd,
    AddWithCarry,
    Div,
    Mul,
    BitwiseAnd,
    BitwiseOr,
    BitwiseXor,
    LogicalLeftShift,
    LogicalRightShift,
    ArithmeticRightShift,
    /// Compares two values.
    Compare(CompareOperation),
}

/// Enumerates all supported comparison operations.
#[derive(Debug, Clone, PartialEq)]
#[allow(missing_docs)]
pub enum CompareOperation {
    /// Equal to (==).
    Eq,

    /// Not equal to (!=).
    Neq,

    /// Greater than (>).
    Gt,

    /// Greater or equal to (>=).
    Geq,

    /// Less than (<).
    Lt,

    /// Less than or equal to (<=).
    Leq,

    /// Determined at runtime.
    ///
    /// This cannot and should not be used with signed variables.
    Runtime(Ident),
}

/// Enumerates all valid unary operations.
///
/// This is merely a type-level denotation of
/// operations such as !.
#[derive(Debug, Clone, PartialEq)]
#[allow(missing_docs)]
pub enum UnaryOperation {
    BitwiseNot,
}

/// An assign statement.
///
/// This is syntactically equivalent to
/// ```ignore
/// a = b;
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct Assign {
    /// Where to store the rhs.
    pub dest: Operand,
    /// The value to be copied in to the
    /// destination.
    pub rhs: Operand,
}

/// A unary operation.
///
/// This is syntactically equivalent to
/// ```ignore
/// a = !b;
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct UnOp {
    /// Where to store the result.
    pub dest: Operand,
    /// What operation to apply.
    pub op: UnaryOperation,
    /// The type of the result.
    pub result_ty: Option<crate::ast::operand::Type>,
    /// The operand to apply the operation to.
    pub rhs: Operand,
}

/// A binary operation.
///
/// This is syntactically equivalent to
/// ```ignore
/// a = b + c; // Or any other binary operation
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct BinOp {
    /// Where to store the result.
    pub dest: Operand,
    /// Which operation to apply.
    pub op: BinaryOperation,
    /// The lhs of the operation.
    pub lhs: Operand,
    /// The rhs of the operation.
    pub rhs: Operand,
    /// The type of the result.
    pub result_ty: Option<crate::ast::operand::Type>,
}
