//! Defines transpiling rules for the ast
//! [`Operations`](crate::ast::operations::Operation).
use proc_macro2::TokenStream;
use quote::quote;

use crate::{
    ast::{
        operand::{Operand, Type},
        operations::*,
    },
    Compile,
    Error,
};

impl Compile for Assign {
    type Output = TokenStream;

    fn compile(
        &self,
        state: &mut crate::TranspilerState<Self::Output>,
    ) -> Result<Self::Output, Error> {
        match (&self.dest, &self.rhs) {
            (Operand::Expr((i1, _)), Operand::Expr((i2, _))) if *i1 == *i2 => {
                return Ok(quote! {general_assembly::operation::Operation::Nop})
            }
            (Operand::Ident((i1, _)), Operand::Ident((i2, _))) if *i1 == *i2 => {
                return Ok(quote! {general_assembly::operation::Operation::Nop})
            }
            (Operand::FieldExtract((i1, _)), Operand::FieldExtract((i2, _))) if *i1 == *i2 => {
                return Ok(quote! {general_assembly::operation::Operation::Nop})
            }

            _ => {}
        };
        let dst: TokenStream = self.dest.compile(state)?;
        let target_ty = self.dest.get_type();
        let rhs: TokenStream = self.rhs.compile(state)?;
        let rhs_ty = self.rhs.get_type();
        let to_insert = state.to_insert_above.drain(..);
        if let Type::F16 | Type::F32 | Type::F64 | Type::F128 = target_ty {
            let target_ty = target_ty.fp_name();
            if let Type::U(_size) = rhs_ty {
                return Ok(quote! {
                    #(#to_insert,)*
                    general_assembly::operation::Operation::Ieee754(general_assembly::extension::ieee754::Operations::Copy{ destination: #dst, source: general_assembly::extension::ieee754::OperandStorage::CoreOperand{operand:#rhs, ty:#target_ty, signed:false }})
                });
            }
            if let Type::U(_size) = rhs_ty {
                return Ok(quote! {
                    #(#to_insert,)*
                    general_assembly::operation::Operation::Ieee754(general_assembly::extension::ieee754::Operations::Copy{ destination: #dst, source: general_assembly::extension::ieee754::OperandStorage::CoreOperand{operand:#rhs, ty:#target_ty, signed:true }})
                });
            }

            return Ok(quote! {
                #(#to_insert,)*
                general_assembly::operation::Operation::Ieee754(general_assembly::extension::ieee754::Operations::Copy{ destination: #dst, source: #rhs })
            });
        }
        if let Type::F16 | Type::F32 | Type::F64 | Type::F128 = rhs_ty {
            let rhs_ty = rhs_ty.fp_name();
            if let Type::U(_size) = target_ty {
                return Ok(quote! {
                    #(#to_insert,)*
                    general_assembly::operation::Operation::Ieee754(general_assembly::extension::ieee754::Operations::Copy{ source: #dst, destination: general_assembly::extension::ieee754::OperandStorage::CoreOperand{operand:#rhs, ty:#rhs_ty, signed:false }})
                });
            }
            if let Type::U(_size) = target_ty {
                return Ok(quote! {
                    #(#to_insert,)*
                    general_assembly::operation::Operation::Ieee754(general_assembly::extension::ieee754::Operations::Copy{ source: #dst, destination: general_assembly::extension::ieee754::OperandStorage::CoreOperand{operand:#rhs, ty:#rhs_ty, signed:true }})
                });
            }
        }
        Ok(quote! {
            #(#to_insert,)*
            general_assembly::operation::Operation::Move { destination: #dst, source: #rhs }
        })
    }
}

impl Compile for UnOp {
    type Output = TokenStream;

    fn compile(
        &self,
        state: &mut crate::TranspilerState<Self::Output>,
    ) -> Result<Self::Output, Error> {
        let dst: TokenStream = self.dest.compile(state)?;
        let rhs: TokenStream = self.rhs.compile(state)?;
        let ret = match self.op {
            UnaryOperation::BitwiseNot => quote!(
                general_assembly::operation::Operation::Not { destination: #dst, operand: #rhs }
            ),
        };

        let to_insert = state.to_insert_above.drain(..);
        Ok(quote!(
        #(#to_insert,)*
        #ret
        ))
    }
}

impl Compile for BinOp {
    type Output = TokenStream;

    fn compile(
        &self,
        state: &mut crate::TranspilerState<Self::Output>,
    ) -> Result<Self::Output, Error> {
        let ty = self.dest.get_type();

        let dst: TokenStream = self.dest.compile(state)?;
        let rhs: TokenStream = self.rhs.compile(state)?;
        state.access_operand(self.rhs.clone());
        let lhs: TokenStream = self.lhs.compile(state)?;

        state.access_operand(self.lhs.clone());
        let ret = match &self.op {
            BinaryOperation::Sub => ty.sub(lhs, rhs, dst),
            BinaryOperation::SSub => ty.ssub(lhs, rhs, dst),
            BinaryOperation::Add => ty.add(lhs, rhs, dst),
            BinaryOperation::SAdd => ty.sadd(lhs, rhs, dst),
            // Not quite sure we should keep this in.
            BinaryOperation::AddWithCarry => ty.adc(lhs, rhs, dst),
            BinaryOperation::Div => ty.div(lhs, rhs, dst),
            BinaryOperation::Mul => ty.mul(lhs, rhs, dst),
            BinaryOperation::BitwiseOr => ty.bvor(lhs, rhs, dst),
            BinaryOperation::BitwiseAnd => ty.bvand(lhs, rhs, dst),
            BinaryOperation::BitwiseXor => ty.bvxor(lhs, rhs, dst),
            BinaryOperation::LogicalLeftShift => ty.bvsl(lhs, rhs, dst),
            BinaryOperation::LogicalRightShift => ty.bvlsr(lhs, rhs, dst),
            BinaryOperation::ArithmeticRightShift => ty.bvasr(lhs, rhs, dst),
            BinaryOperation::Compare(c) => {
                let ty = self.lhs.get_type();
                let op = (c.clone(), ty).compile(state)?;
                match ty {
                    Type::I(_) | Type::U(_) => quote! {
                        general_assembly::operation::Operation::Compare {
                            lhs: #lhs,
                            rhs: #rhs,
                            operation: #op,
                            destination: #dst,
                        }
                    },
                    Type::F16 | Type::F32 | Type::F64 | Type::F128 => {
                        quote! {
                            general_assembly::operation::Operation::Ieee754 (general_assembly::extension::ieee754::Operations::Compare {
                                lhs: #lhs,
                                rhs: #rhs,
                                operation: #op,
                                destination: #dst,
                                signal:false,
                            })
                        }
                    }
                    Type::Unit => {
                        quote! {compile_error!("Cannot compare unit types.")}
                    }
                }
            }
        };
        let to_insert = state.to_insert_above.drain(..);
        Ok(quote!(
        #(#to_insert,)*
        #ret
        ))
    }
}

impl Type {
    fn add(&self, lhs: TokenStream, rhs: TokenStream, dest: TokenStream) -> TokenStream {
        match self {
            Self::I(_) | Self::U(_) => quote! {
                    general_assembly::operation::Operation::Add {
                        destination: #dest,
                        operand1: #lhs,
                        operand2: #rhs
                    }
            },
            Self::F16 | Self::F32 | Self::F64 | Self::F128 => quote! {
                general_assembly::operation::Operation::Ieee754(general_assembly::extension::ieee754::Operations::Addition {
                    destination: #dest,
                    lhs: #lhs,
                    rhs: #rhs
                })
            },
            Self::Unit => quote! {compile_error!("Cannot use unit types for expressions")},
        }
    }

    fn adc(&self, lhs: TokenStream, rhs: TokenStream, dest: TokenStream) -> TokenStream {
        match self {
            Self::I(_) | Self::U(_) => quote! {
                    general_assembly::operation::Operation::Adc {
                        destination: #dest,
                        operand1: #lhs,
                        operand2: #rhs
                    }
            },
            Self::F16 | Self::F32 | Self::F64 | Self::F128 => quote! {
                compile_error!("Cannot add float with carry");
            },
            Self::Unit => quote! {compile_error!("Cannot use unit types for expressions")},
        }
    }

    fn sadd(&self, lhs: TokenStream, rhs: TokenStream, dest: TokenStream) -> TokenStream {
        match self {
            Self::I(_) => quote! {
                        general_assembly::operation::Operation::SAdd {
                            destination: #dest,
                            operand1: #lhs,
                            operand2: #rhs,
                            signed:true,
                        }
            },
            Self::U(_) => quote! {
                        general_assembly::operation::Operation::SAdd {
                            destination: #dest,
                            operand1: #lhs,
                            operand2: #rhs,
                            signed:false,
                        }
            },
            Self::F16 | Self::F32 | Self::F64 | Self::F128 => {
                quote!(compile_error!("saturating add makes no sense for floats."))
            }
            Self::Unit => quote! {compile_error!("Cannot use unit types for expressions")},
        }
    }

    fn ssub(&self, lhs: TokenStream, rhs: TokenStream, dest: TokenStream) -> TokenStream {
        match self {
            Self::I(_) => quote! {
                        general_assembly::operation::Operation::SSub {
                            destination: #dest,
                            operand1: #lhs,
                            operand2: #rhs,
                            signed:true,
                        }
            },
            Self::U(_) => quote! {
                        general_assembly::operation::Operation::SSub {
                            destination: #dest,
                            operand1: #lhs,
                            operand2: #rhs,
                            signed:false,
                        }
            },
            Self::F16 | Self::F32 | Self::F64 | Self::F128 => {
                quote!(compile_error!("saturating sub makes no sense for floats."))
            }
            Self::Unit => quote! {compile_error!("Cannot use unit types for expressions")},
        }
    }

    fn bvor(&self, lhs: TokenStream, rhs: TokenStream, dest: TokenStream) -> TokenStream {
        match self {
            Self::I(_) | Self::U(_) => quote! {
                        general_assembly::operation::Operation::Or {
                            destination: #dest,
                            operand1: #lhs,
                            operand2: #rhs
                        }
            },
            Self::F16 | Self::F32 | Self::F64 | Self::F128 => {
                quote!(compile_error!(
                    "Bitwise operations make no sense for floats."
                ))
            }
            Self::Unit => quote! {compile_error!("Cannot use unit types for expressions")},
        }
    }

    fn bvand(&self, lhs: TokenStream, rhs: TokenStream, dest: TokenStream) -> TokenStream {
        match self {
            Self::I(_) | Self::U(_) => quote! {
                        general_assembly::operation::Operation::And {
                            destination: #dest,
                            operand1: #lhs,
                            operand2: #rhs
                        }
            },
            Self::F16 | Self::F32 | Self::F64 | Self::F128 => {
                quote!(compile_error!(
                    "Bitwise operations make no sense for floats."
                ))
            }
            Self::Unit => quote! {compile_error!("Cannot use unit types for expressions")},
        }
    }

    fn bvsl(&self, lhs: TokenStream, rhs: TokenStream, dest: TokenStream) -> TokenStream {
        match self {
            Self::I(_) | Self::U(_) => quote! {
                        general_assembly::operation::Operation::Sl {
                            destination: #dest,
                            operand: #lhs,
                            shift: #rhs
                        }
            },
            Self::F16 | Self::F32 | Self::F64 | Self::F128 => {
                quote!(compile_error!(
                    "Bitwise operations make no sense for floats."
                ))
            }

            Self::Unit => quote! {compile_error!("Cannot use unit types for expressions")},
        }
    }

    fn bvlsr(&self, lhs: TokenStream, rhs: TokenStream, dest: TokenStream) -> TokenStream {
        match self {
            Self::I(_) | Self::U(_) => quote! {
                        general_assembly::operation::Operation::Srl {
                            destination: #dest,
                            operand: #lhs,
                            shift: #rhs
                        }
            },
            Self::F16 | Self::F32 | Self::F64 | Self::F128 => {
                quote!(compile_error!(
                    "Bitwise operations make no sense for floats."
                ))
            }
            Self::Unit => quote! {compile_error!("Cannot use unit types for expressions")},
        }
    }

    fn bvasr(&self, lhs: TokenStream, rhs: TokenStream, dest: TokenStream) -> TokenStream {
        match self {
            Self::I(_) | Self::U(_) => quote! {
                        general_assembly::operation::Operation::Sra {
                            destination: #dest,
                            operand: #lhs,
                            shift: #rhs
                        }
            },
            Self::F16 | Self::F32 | Self::F64 | Self::F128 => {
                quote!(compile_error!(
                    "Bitwise operations make no sense for floats."
                ))
            }
            Self::Unit => quote! {compile_error!("Cannot use unit types for expressions")},
        }
    }

    fn bvxor(&self, lhs: TokenStream, rhs: TokenStream, dest: TokenStream) -> TokenStream {
        match self {
            Self::I(_) | Self::U(_) => quote! {
                        general_assembly::operation::Operation::Xor {
                            destination: #dest,
                            operand1: #lhs,
                            operand2: #rhs
                        }
            },
            Self::F16 | Self::F32 | Self::F64 | Self::F128 => {
                quote!(compile_error!(
                    "Bitwise operations make no sense for floats."
                ))
            }
            Self::Unit => quote! {compile_error!("Cannot use unit types for expressions")},
        }
    }

    fn sub(&self, lhs: TokenStream, rhs: TokenStream, dest: TokenStream) -> TokenStream {
        match self {
            Self::I(_) | Self::U(_) => quote! {
                    general_assembly::operation::Operation::Sub {
                        destination: #dest,
                        operand1: #lhs,
                        operand2: #rhs
                    }
            },
            Self::F16 | Self::F32 | Self::F64 | Self::F128 => quote! {
                general_assembly::operation::Operation::Ieee754(general_assembly::extension::ieee754::Operations::Subtraction {
                    destination: #dest,
                    lhs: #lhs,
                    rhs: #rhs
                })
            },
            Self::Unit => quote! {compile_error!("Cannot use unit types for expressions")},
        }
    }

    fn mul(&self, lhs: TokenStream, rhs: TokenStream, dest: TokenStream) -> TokenStream {
        match self {
            Self::I(_) | Self::U(_) => quote! {
                    general_assembly::operation::Operation::Mul {
                        destination: #dest,
                        operand1: #lhs,
                        operand2: #rhs
                    }
            },
            Self::F16 | Self::F32 | Self::F64 | Self::F128 => quote! {
                general_assembly::operation::Operation::Ieee754(general_assembly::extension::ieee754::Operations::Multiplication {
                    destination: #dest,
                    lhs: #lhs,
                    rhs: #rhs
                })
            },
            Self::Unit => quote! {compile_error!("Cannot use unit types for expressions")},
        }
    }

    fn div(
        &self,
        nominator: TokenStream,
        denominator: TokenStream,
        dest: TokenStream,
    ) -> TokenStream {
        match self {
            Self::I(_) => quote! {
                        general_assembly::operation::Operation::SDiv {
                            destination: #dest,
                            operand1: #nominator,
                            operand2: #denominator
                        }
            },
            Self::U(_) => quote! {
                        general_assembly::operation::Operation::UDiv {
                            destination: #dest,
                            operand1: #nominator,
                            operand2: #denominator
                        }
            },
            Self::F16 | Self::F32 | Self::F64 | Self::F128 => quote! {
                    general_assembly::operation::Operation::Ieee754(general_assembly::extension::ieee754::Operations::Division {
                        destination: #dest,
                        nominator: #nominator,
                        denominator: #denominator
                    })
            },
            Self::Unit => quote! {compile_error!("Cannot use unit types for expressions")},
        }
    }
}
impl Compile for (CompareOperation, crate::ast::operand::Type) {
    type Output = TokenStream;

    fn compile(
        &self,
        _state: &mut crate::TranspilerState<Self::Output>,
    ) -> Result<Self::Output, Error> {
        Ok(match self {
            (CompareOperation::Runtime(id), _) => {
                quote! {#id}
            }
            (CompareOperation::Lt, Type::I(_)) => {
                quote! {general_assembly::condition::Comparison::SLt}
            }
            (CompareOperation::Lt, Type::U(_)) => {
                quote! {general_assembly::condition::Comparison::ULt}
            }
            (CompareOperation::Gt, Type::I(_)) => {
                quote! {general_assembly::condition::Comparison::SGt}
            }
            (CompareOperation::Gt, Type::U(_)) => {
                quote! {general_assembly::condition::Comparison::UGt}
            }
            (CompareOperation::Leq, Type::I(_)) => {
                quote! {general_assembly::condition::Comparison::SLeq}
            }
            (CompareOperation::Leq, Type::U(_)) => {
                quote! {general_assembly::condition::Comparison::ULeq}
            }
            (CompareOperation::Geq, Type::I(_)) => {
                quote! {general_assembly::condition::Comparison::SGeq}
            }
            (CompareOperation::Geq, Type::U(_)) => {
                quote! {general_assembly::condition::Comparison::UGeq}
            }
            (CompareOperation::Lt, Type::F16 | Type::F32 | Type::F64 | Type::F128) => {
                quote! {general_assembly::extension::ieee754::ComparisonMode::Less}
            }
            (CompareOperation::Gt, Type::F16 | Type::F32 | Type::F64 | Type::F128) => {
                quote! {general_assembly::extension::ieee754::ComparisonMode::Greater}
            }
            (CompareOperation::Leq, Type::F16 | Type::F32 | Type::F64 | Type::F128) => {
                quote! {general_assembly::extension::ieee754::ComparisonMode::LessOrEqual}
            }
            (CompareOperation::Geq, Type::F16 | Type::F32 | Type::F64 | Type::F128) => {
                quote! {general_assembly::extension::ieee754::ComparisonMode::GreaterOrEqual}
            }
            (CompareOperation::Eq, Type::F16 | Type::F32 | Type::F64 | Type::F128) => {
                quote! {general_assembly::extension::ieee754::ComparisonMode::Equal}
            }
            (CompareOperation::Neq, Type::F16 | Type::F32 | Type::F64 | Type::F128) => {
                quote! {general_assembly::extension::ieee754::ComparisonMode::NotEqual}
            }
            (CompareOperation::Eq, _) => quote! {general_assembly::condition::Comparison::Eq},
            (CompareOperation::Neq, _) => quote! {general_assembly::condition::Comparison::Neq},
            (_, Type::Unit) => {
                quote! {compile_error!("Cannot use unit types for comparison expressions")}
            }
        })
    }
}
