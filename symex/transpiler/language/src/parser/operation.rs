//! Defines parsing rules for the ast
//! [`Operations`](crate::ast::operations::Operation).
use syn::{
    parse::{discouraged::Speculative, Parse, ParseStream, Result},
    Ident,
    Token,
};

use crate::ast::{
    operand::{Operand, Type},
    operations::{Assign, BinOp, BinaryOperation, CompareOperation, UnOp, UnaryOperation},
};
impl Parse for Assign {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut dest: Operand = input.parse()?;
        let ty = if input.peek(Token![:]) {
            let _: Token![:] = input.parse()?;
            let ty: Type = input.parse()?;
            dest.set_type(ty);
            Some(ty)
        } else {
            None
        };

        let _: Token![=] = input.parse()?;
        let mut rhs: Operand = input.parse()?;
        if let Some(ty) = ty {
            rhs.set_type(ty);
        }
        if !input.peek(Token![;]) {
            return Err(input.error("Expected ;"));
        }
        Ok(Self { dest, rhs })
    }
}

impl Operand {
    /// TODO: Docs
    pub fn set_type(&mut self, ty: Type) {
        match self {
            Self::Expr((_, inner)) => *inner = Some(ty),
            Self::Ident((_, inner)) => *inner = Some(ty),
            Self::FieldExtract((_, inner)) => *inner = Some(ty),
            Self::DynamicFieldExtract((_, inner)) => *inner = Some(ty),
            Self::WrappedLiteral(_) => {}
        }
    }
}
impl Parse for UnOp {
    fn parse(input: ParseStream) -> Result<Self> {
        let dest: Operand = input.parse()?;
        let mut ty = None;
        if input.peek(Token![:]) {
            let _: Token![:] = input.parse()?;
            let inner_ty: Type = input.parse()?;
            ty = Some(inner_ty);
        }
        let _: Token![=] = input.parse()?;
        let op: UnaryOperation = input.parse()?;
        let rhs: Operand = input.parse()?;
        if !input.peek(syn::token::Semi) {
            return Err(input.error("Expected ;"));
        }
        Ok(Self {
            dest,
            op,
            rhs,
            result_ty: ty,
        })
    }
}
impl Parse for BinOp {
    fn parse(input: ParseStream) -> Result<Self> {
        let dest: Operand = input.parse()?;
        let mut ty = None;
        if input.peek(Token![:]) {
            let _: Token![:] = input.parse()?;
            let inner_ty: Type = input.parse()?;
            ty = Some(inner_ty);
        }
        let _: Token![=] = input.parse()?;

        let lhs: Operand = input.parse()?;

        let op: BinaryOperation = input.parse()?;

        let rhs: Operand = input.parse()?;
        if !input.peek(syn::token::Semi) {
            return Err(input.error("Expected ;"));
        }
        Ok(Self {
            dest,
            op,
            lhs,
            rhs,
            result_ty: ty,
        })
    }
}
impl Parse for UnaryOperation {
    fn parse(input: ParseStream) -> Result<Self> {
        if input.peek(Token![!]) {
            let _: Token![!] = input.parse()?;
            return Ok(Self::BitwiseNot);
        }
        Err(input.error("Expected unary op"))
    }
}
impl Parse for BinaryOperation {
    fn parse(input: ParseStream) -> Result<Self> {
        use BinaryOperation::*;
        if input.peek(Token![+]) {
            let _: Token![+] = input.parse()?;
            return Ok(Add);
        }
        if input.peek(Token![-]) {
            let _: Token![-] = input.parse()?;
            return Ok(Sub);
        }
        if input.peek(Ident) {
            let speculative = input.fork();
            let ident: Ident = speculative.parse()?;
            if ident.to_string().to_lowercase() == "adc" {
                input.advance_to(&speculative);
                return Ok(AddWithCarry);
            }
        }
        if input.peek(syn::token::Slash) {
            let _: syn::token::Slash = input.parse()?;
            return Ok(Self::Div);
        }
        if input.peek(Ident) {
            let speculative = input.fork();
            let ident: Ident = speculative.parse()?;
            if ident.to_string().to_lowercase() == "sadd" {
                input.advance_to(&speculative);
                return Ok(SAdd);
            }
        }
        if input.peek(Ident) {
            let speculative = input.fork();
            let ident: Ident = speculative.parse()?;
            if ident.to_string().to_lowercase() == "ssub" {
                input.advance_to(&speculative);
                return Ok(SAdd);
            }
        }
        if input.peek(Token![*]) {
            let _: Token![*] = input.parse()?;
            return Ok(Self::Mul);
        }
        if input.peek(Token![&]) {
            let _: Token![&] = input.parse()?;
            return Ok(Self::BitwiseAnd);
        }
        if input.peek(Token![|]) {
            let _: Token![|] = input.parse()?;
            return Ok(Self::BitwiseOr);
        }
        if input.peek(Token![^]) {
            let _: Token![^] = input.parse()?;
            return Ok(Self::BitwiseXor);
        }
        if input.peek(Token![>>]) {
            let _: Token![>>] = input.parse()?;
            return Ok(Self::LogicalRightShift);
        }
        if input.peek(Token![<<]) {
            let _: Token![<<] = input.parse()?;
            return Ok(Self::LogicalLeftShift);
        }
        if input.peek(Ident) {
            let ident: Ident = input.parse()?;
            // Revisit this later
            if ident.to_string().to_lowercase() == "asr" {
                return Ok(ArithmeticRightShift);
            }
        }
        let speculative = input.fork();
        if let Ok(val) = speculative.parse() {
            input.advance_to(&speculative);
            return Ok(Self::Compare(val));
        }
        Err(input.error("Expected operation"))
    }
}

impl Parse for CompareOperation {
    fn parse(input: ParseStream) -> Result<Self> {
        if input.peek(Token![==]) {
            let _: Token![==] = input.parse()?;
            return Ok(Self::Eq);
        }
        if input.peek(Token![!=]) {
            let _: Token![!=] = input.parse()?;
            return Ok(Self::Neq);
        }
        if input.peek(Token![>=]) {
            let _: Token![>=] = input.parse()?;
            return Ok(Self::Geq);
        }
        if input.peek(Token![<=]) {
            let _: Token![<=] = input.parse()?;
            return Ok(Self::Leq);
        }
        if input.peek(Token![>]) {
            let _: Token![>] = input.parse()?;
            return Ok(Self::Gt);
        }
        if input.peek(Token![<]) {
            let _: Token![<] = input.parse()?;
            return Ok(Self::Lt);
        }
        if input.peek(Ident) {
            let ident: Ident = input.parse()?;
            return Ok(Self::Runtime(ident));
        }

        Err(input.error("Expected a comparison operation"))
    }
}
