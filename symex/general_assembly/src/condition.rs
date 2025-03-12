//! Defines the [`Condition`] codes used in Symex General Assembly.

#[derive(Debug, PartialEq, Clone, Copy)]
/// Enumerates the condition codes used in Symex General Assembly.
pub enum Condition {
    /// Equal Z = 1
    EQ,

    /// Not Equal Z = 0
    NE,

    /// Carry set C = 1
    CS,

    /// Carry clear C = 0
    CC,

    /// Negative N = 1
    MI,

    /// Positive or zero N = 0
    PL,

    /// Overflow V = 1
    VS,

    /// No overflow V = 0
    VC,

    /// Unsigned higher C = 1 AND Z = 0
    HI,

    /// Unsigned lower or equal C = 0 OR Z = 1
    LS,

    /// Signed higher or equal N = V
    GE,

    /// Signed lower N != V
    LT,

    /// Signed higher Z = 0 AND N = V
    GT,

    /// Signed lower or equal Z = 1 OR N != V
    LE,

    /// No condition always true
    None,
}

/// Enumerates the valid comparison operations.
#[derive(Debug, Clone)]
pub enum Comparison {
    /// The two operands must be equal.
    Eq,
    /// The two operands must not be equal.
    Neq,
    /// The left hand side must be greater than the left side (unsigned).
    UGt,
    /// The left hand side must be greater than or equal to the left side
    /// (unsigned).
    UGeq,
    /// The left hand side must be greater or equal to the left side (unsigned).
    ULt,
    /// The left hand side must be less than or equal to the left side
    /// (unsigned).
    ULeq,
    /// The left hand side must be greater than the left side (signed).
    SGt,
    /// The left hand side must be greater than or equal to the left side
    /// (signed).
    SGeq,
    /// The left hand side must be greater or equal to the left side (signed).
    SLt,
    /// The left hand side must be less than or equal to the left side (signed).
    SLeq,
}
