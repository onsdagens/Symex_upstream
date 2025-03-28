mod fp {
    use armv6_m_instruction_parser::registers;
    use disarmv7::{
        arch::{register::IEEE754RoundingMode, Condition},
        operation::{
            ConversionArgument, F32OrF64, IntType, RegisterOrAPSR, VLdmF32, VLdmF64,
            VLdrF32, VLdrF64, VPopF32, VPopF64, VPushF32, VPushF64, VStmF32, VStmF64,
            VStrF32, VStrF64, VabsF32, VabsF64, VaddF32, VaddF64, VcmpF32, VcmpF64,
            VcmpZeroF32, VcmpZeroF64, Vcvt, VcvtCustomRoundingIntF32,
            VcvtCustomRoundingIntF64, VcvtF32, VcvtF32F64, VcvtF64, VcvtF64F32, VdivF32,
            VdivF64, VmaxF32, VmaxF64, VminF32, VminF64, VmlF32, VmlF64,
            VmovImmediateF32, VmovImmediateF64, VmovRegisterF32, VmovRegisterF64,
            VmovRegisterF64Builder, VmoveDoubleF32, VmoveF32, VmoveF64, VmoveHalfWord,
            Vmrs, Vmsr, VmulF32, VmulF64, VnegF32, VnegF64, VnmlF32, VnmlF64, VnmulF32,
            VnmulF64, VrintCustomRoundingF32, VrintCustomRoundingF64, VrintF32, VrintF64,
            VselF32, VselF64, VsqrtF32, VsqrtF64, VsubF32, VsubF64,
        },
    };
    use general_assembly::extension::ieee754::{
        ComparisonMode, Operand, OperandStorage, OperandType, RoundingMode,
    };
    use transpiler::pseudo;
    use super::{sealed::Into, Decode};
    impl Decode for VabsF32 {
        fn decode(
            &self,
            _in_it_block: bool,
        ) -> Vec<general_assembly::prelude::Operation> {
            let Self { sd, sm } = self;
            let sd = sd.local_into();
            let sm = sm.local_into();
            {
                let mut ret = Vec::new();
                let intermediate_0 = general_assembly::extension::ieee754::Operand {
                    ty: general_assembly::extension::ieee754::OperandType::Binary32,
                    value: general_assembly::extension::ieee754::OperandStorage::Local(
                        "intermediate_0".to_string(),
                    ),
                };
                ret.extend([
                    general_assembly::operation::Operation::Nop,
                    general_assembly::operation::Operation::Nop,
                    general_assembly::operation::Operation::Ieee754(general_assembly::extension::ieee754::Operations::Abs {
                        operand: sm.clone(),
                        destination: intermediate_0.clone(),
                    }),
                    general_assembly::operation::Operation::Ieee754(general_assembly::extension::ieee754::Operations::Copy {
                        destination: sd.clone(),
                        source: intermediate_0.clone(),
                    }),
                ]);
                ret
            }
        }
    }
    impl Decode for VabsF64 {
        fn decode(
            &self,
            _in_it_block: bool,
        ) -> Vec<general_assembly::prelude::Operation> {
            let Self { dd, dm } = self;
            let dd = dd.local_into();
            let dm = dm.local_into();
            {
                let mut ret = Vec::new();
                let intermediate_0 = general_assembly::extension::ieee754::Operand {
                    ty: general_assembly::extension::ieee754::OperandType::Binary64,
                    value: general_assembly::extension::ieee754::OperandStorage::Local(
                        "intermediate_0".to_string(),
                    ),
                };
                ret.extend([
                    general_assembly::operation::Operation::Nop,
                    general_assembly::operation::Operation::Nop,
                    general_assembly::operation::Operation::Ieee754(general_assembly::extension::ieee754::Operations::Abs {
                        operand: dm.clone(),
                        destination: intermediate_0.clone(),
                    }),
                    general_assembly::operation::Operation::Ieee754(general_assembly::extension::ieee754::Operations::Copy {
                        destination: dd.clone(),
                        source: intermediate_0.clone(),
                    }),
                ]);
                ret
            }
        }
    }
    impl Decode for VaddF32 {
        fn decode(
            &self,
            _in_it_block: bool,
        ) -> Vec<general_assembly::prelude::Operation> {
            let Self { sd, sn, sm } = self;
            let sn = sn.local_into();
            let sd = sd.local_into().unwrap_or(sn.clone());
            let sm = sm.local_into();
            {
                let mut ret = Vec::new();
                ret.extend([
                    general_assembly::operation::Operation::Nop,
                    general_assembly::operation::Operation::Nop,
                    general_assembly::operation::Operation::Nop,
                    general_assembly::operation::Operation::Ieee754(general_assembly::extension::ieee754::Operations::Addition {
                        destination: sd.clone(),
                        lhs: sn.clone(),
                        rhs: sm.clone(),
                    }),
                ]);
                ret
            }
        }
    }
    impl Decode for VaddF64 {
        fn decode(
            &self,
            _in_it_block: bool,
        ) -> Vec<general_assembly::prelude::Operation> {
            let Self { dd, dn, dm } = self;
            let dn = dn.local_into();
            let dd = dd.local_into().unwrap_or(dn.clone());
            let dm = dm.local_into();
            {
                let mut ret = Vec::new();
                ret.extend([
                    general_assembly::operation::Operation::Nop,
                    general_assembly::operation::Operation::Nop,
                    general_assembly::operation::Operation::Nop,
                    general_assembly::operation::Operation::Ieee754(general_assembly::extension::ieee754::Operations::Addition {
                        destination: dd.clone(),
                        lhs: dn.clone(),
                        rhs: dm.clone(),
                    }),
                ]);
                ret
            }
        }
    }
    impl Decode for VselF32 {
        fn decode(
            &self,
            _in_it_block: bool,
        ) -> Vec<general_assembly::prelude::Operation> {
            let Self { cond, sd, sn, sm } = self;
            let sd = sd.local_into();
            let sn = sn.local_into();
            let sm = sm.local_into();
            let cond: ComparisonMode = cond
                .clone()
                .expect("Cannot compare without a condition")
                .local_into();
            {
                let mut ret = Vec::new();
                let intermediate_0 = general_assembly::operand::Operand::Local(
                    "intermediate_0".to_string(),
                );
                ret.extend([
                    general_assembly::operation::Operation::Nop,
                    general_assembly::operation::Operation::Nop,
                    general_assembly::operation::Operation::Nop,
                    general_assembly::operation::Operation::Nop,
                    general_assembly::operation::Operation::Ieee754(general_assembly::extension::ieee754::Operations::Compare {
                        lhs: sn.clone().clone(),
                        rhs: sm.clone().clone(),
                        operation: cond.clone(),
                        destination: intermediate_0.clone(),
                        signal: false,
                    }),
                    general_assembly::operation::Operation::Ite {
                        condition: intermediate_0.clone(),
                        then: <[_]>::into_vec(
                            ::alloc::boxed::box_new([
                                general_assembly::operation::Operation::Ieee754(general_assembly::extension::ieee754::Operations::Copy {
                                    destination: sd.clone(),
                                    source: sn.clone(),
                                }),
                            ]),
                        ),
                        otherwise: <[_]>::into_vec(
                            ::alloc::boxed::box_new([
                                general_assembly::operation::Operation::Ieee754(general_assembly::extension::ieee754::Operations::Copy {
                                    destination: sd.clone(),
                                    source: sm.clone(),
                                }),
                            ]),
                        ),
                    },
                ]);
                ret
            }
        }
    }
    impl Decode for VselF64 {
        fn decode(
            &self,
            _in_it_block: bool,
        ) -> Vec<general_assembly::prelude::Operation> {
            let Self { cond, dd, dn, dm } = self;
            let dd = dd.local_into();
            let dn = dn.local_into();
            let dm = dm.local_into();
            let cond: ComparisonMode = cond
                .clone()
                .expect("Cannot compare without a condition")
                .local_into();
            {
                let mut ret = Vec::new();
                let intermediate_0 = general_assembly::operand::Operand::Local(
                    "intermediate_0".to_string(),
                );
                ret.extend([
                    general_assembly::operation::Operation::Nop,
                    general_assembly::operation::Operation::Nop,
                    general_assembly::operation::Operation::Nop,
                    general_assembly::operation::Operation::Ieee754(general_assembly::extension::ieee754::Operations::Compare {
                        lhs: dn.clone().clone(),
                        rhs: dm.clone().clone(),
                        operation: cond.clone(),
                        destination: intermediate_0.clone(),
                        signal: false,
                    }),
                    general_assembly::operation::Operation::Ite {
                        condition: intermediate_0.clone(),
                        then: <[_]>::into_vec(
                            ::alloc::boxed::box_new([
                                general_assembly::operation::Operation::Ieee754(general_assembly::extension::ieee754::Operations::Copy {
                                    destination: dd.clone(),
                                    source: dn.clone(),
                                }),
                            ]),
                        ),
                        otherwise: <[_]>::into_vec(
                            ::alloc::boxed::box_new([
                                general_assembly::operation::Operation::Ieee754(general_assembly::extension::ieee754::Operations::Copy {
                                    destination: dd.clone(),
                                    source: dm.clone(),
                                }),
                            ]),
                        ),
                    },
                ]);
                ret
            }
        }
    }
    impl Decode for VmlF32 {
        fn decode(
            &self,
            _in_it_block: bool,
        ) -> Vec<general_assembly::prelude::Operation> {
            let VmlF32 { add, sd, sn, sm } = self;
            let add = *add;
            let sd = sd.local_into();
            let sn = sn.local_into();
            let sm = sm.local_into();
            {
                let mut ret = Vec::new();
                let mul = general_assembly::extension::ieee754::Operand {
                    ty: general_assembly::extension::ieee754::OperandType::Binary32,
                    value: general_assembly::extension::ieee754::OperandStorage::Local(
                        "mul".to_string(),
                    ),
                };
                ret.extend([
                    general_assembly::operation::Operation::Nop,
                    general_assembly::operation::Operation::Ieee754(general_assembly::extension::ieee754::Operations::Multiplication {
                        destination: mul.clone(),
                        lhs: sn.clone(),
                        rhs: sm.clone(),
                    }),
                ]);
                if add {
                    ret.extend([
                        general_assembly::operation::Operation::Ieee754(general_assembly::extension::ieee754::Operations::Addition {
                            destination: sd.clone(),
                            lhs: sd.clone(),
                            rhs: mul.clone(),
                        }),
                    ]);
                } else {
                    ret.extend([
                        general_assembly::operation::Operation::Ieee754(general_assembly::extension::ieee754::Operations::Subtraction {
                            destination: sd.clone(),
                            lhs: sd.clone(),
                            rhs: mul.clone(),
                        }),
                    ]);
                };
                ret
            }
        }
    }
    impl Decode for VmlF64 {
        fn decode(
            &self,
            _in_it_block: bool,
        ) -> Vec<general_assembly::prelude::Operation> {
            let VmlF64 { add, dd, dn, dm } = self;
            let add = *add;
            let dd = dd.local_into();
            let dn = dn.local_into();
            let dm = dm.local_into();
            {
                let mut ret = Vec::new();
                let mul = general_assembly::extension::ieee754::Operand {
                    ty: general_assembly::extension::ieee754::OperandType::Binary64,
                    value: general_assembly::extension::ieee754::OperandStorage::Local(
                        "mul".to_string(),
                    ),
                };
                ret.extend([
                    general_assembly::operation::Operation::Nop,
                    general_assembly::operation::Operation::Ieee754(general_assembly::extension::ieee754::Operations::Multiplication {
                        destination: mul.clone(),
                        lhs: dn.clone(),
                        rhs: dm.clone(),
                    }),
                ]);
                if add {
                    ret.extend([
                        general_assembly::operation::Operation::Ieee754(general_assembly::extension::ieee754::Operations::Addition {
                            destination: dd.clone(),
                            lhs: dd.clone(),
                            rhs: mul.clone(),
                        }),
                    ]);
                } else {
                    ret.extend([
                        general_assembly::operation::Operation::Ieee754(general_assembly::extension::ieee754::Operations::Subtraction {
                            destination: dd.clone(),
                            lhs: dd.clone(),
                            rhs: mul.clone(),
                        }),
                    ]);
                };
                ret
            }
        }
    }
    impl Decode for VnmlF32 {
        fn decode(
            &self,
            _in_it_block: bool,
        ) -> Vec<general_assembly::prelude::Operation> {
            let Self { add, sd, sn, sm } = self;
            let add = *add;
            let sd = sd.local_into();
            let sn = sn.local_into();
            let sm = sm.local_into();
            let zero = (0., OperandType::Binary32).local_into();
            {
                let mut ret = Vec::new();
                let prod = general_assembly::extension::ieee754::Operand {
                    ty: general_assembly::extension::ieee754::OperandType::Binary32,
                    value: general_assembly::extension::ieee754::OperandStorage::Local(
                        "prod".to_string(),
                    ),
                };
                ret.extend([
                    general_assembly::operation::Operation::Nop,
                    general_assembly::operation::Operation::Nop,
                    general_assembly::operation::Operation::Ieee754(general_assembly::extension::ieee754::Operations::Multiplication {
                        destination: prod.clone(),
                        lhs: sn.clone(),
                        rhs: sm.clone(),
                    }),
                    general_assembly::operation::Operation::Ieee754(general_assembly::extension::ieee754::Operations::Subtraction {
                        destination: sd.clone(),
                        lhs: zero.clone(),
                        rhs: sd.clone(),
                    }),
                ]);
                if add {
                    ret.extend([
                        general_assembly::operation::Operation::Ieee754(general_assembly::extension::ieee754::Operations::Subtraction {
                            destination: prod.clone(),
                            lhs: zero.clone(),
                            rhs: prod.clone(),
                        }),
                        general_assembly::operation::Operation::Ieee754(general_assembly::extension::ieee754::Operations::Addition {
                            destination: sd.clone(),
                            lhs: sd.clone(),
                            rhs: prod.clone(),
                        }),
                    ]);
                } else {
                    ret.extend([
                        general_assembly::operation::Operation::Ieee754(general_assembly::extension::ieee754::Operations::Addition {
                            destination: sd.clone(),
                            lhs: sd.clone(),
                            rhs: prod.clone(),
                        }),
                    ]);
                };
                ret
            }
        }
    }
    impl Decode for VnmlF64 {
        fn decode(
            &self,
            _in_it_block: bool,
        ) -> Vec<general_assembly::prelude::Operation> {
            let Self { add, dd, dn, dm } = self;
            let add = *add;
            let dd = dd.local_into();
            let dn = dn.local_into();
            let dm = dm.local_into();
            let zero = (0., OperandType::Binary32).local_into();
            {
                let mut ret = Vec::new();
                let prod = general_assembly::extension::ieee754::Operand {
                    ty: general_assembly::extension::ieee754::OperandType::Binary32,
                    value: general_assembly::extension::ieee754::OperandStorage::Local(
                        "prod".to_string(),
                    ),
                };
                ret.extend([
                    general_assembly::operation::Operation::Nop,
                    general_assembly::operation::Operation::Nop,
                    general_assembly::operation::Operation::Ieee754(general_assembly::extension::ieee754::Operations::Multiplication {
                        destination: prod.clone(),
                        lhs: dn.clone(),
                        rhs: dm.clone(),
                    }),
                    general_assembly::operation::Operation::Ieee754(general_assembly::extension::ieee754::Operations::Subtraction {
                        destination: dd.clone(),
                        lhs: zero.clone(),
                        rhs: dd.clone(),
                    }),
                ]);
                if add {
                    ret.extend([
                        general_assembly::operation::Operation::Ieee754(general_assembly::extension::ieee754::Operations::Subtraction {
                            destination: prod.clone(),
                            lhs: zero.clone(),
                            rhs: prod.clone(),
                        }),
                        general_assembly::operation::Operation::Ieee754(general_assembly::extension::ieee754::Operations::Addition {
                            destination: dd.clone(),
                            lhs: dd.clone(),
                            rhs: prod.clone(),
                        }),
                    ]);
                } else {
                    ret.extend([
                        general_assembly::operation::Operation::Ieee754(general_assembly::extension::ieee754::Operations::Addition {
                            destination: dd.clone(),
                            lhs: dd.clone(),
                            rhs: prod.clone(),
                        }),
                    ]);
                };
                ret
            }
        }
    }
    impl Decode for VnmulF32 {
        fn decode(
            &self,
            _in_it_block: bool,
        ) -> Vec<general_assembly::prelude::Operation> {
            let Self { sd, sn, sm } = self;
            let sn = sn.local_into();
            let sd = sd.local_into().unwrap_or(sn.clone());
            let sm = sm.local_into();
            let zero = (0., OperandType::Binary32).local_into();
            {
                let mut ret = Vec::new();
                let prod = general_assembly::extension::ieee754::Operand {
                    ty: general_assembly::extension::ieee754::OperandType::Binary32,
                    value: general_assembly::extension::ieee754::OperandStorage::Local(
                        "prod".to_string(),
                    ),
                };
                ret.extend([
                    general_assembly::operation::Operation::Ieee754(general_assembly::extension::ieee754::Operations::Multiplication {
                        destination: prod.clone(),
                        lhs: sn.clone(),
                        rhs: sm.clone(),
                    }),
                    general_assembly::operation::Operation::Ieee754(general_assembly::extension::ieee754::Operations::Subtraction {
                        destination: sd.clone(),
                        lhs: zero.clone(),
                        rhs: prod.clone(),
                    }),
                ]);
                ret
            }
        }
    }
    impl Decode for VnmulF64 {
        fn decode(
            &self,
            _in_it_block: bool,
        ) -> Vec<general_assembly::prelude::Operation> {
            let Self { dd, dn, dm } = self;
            let dn = dn.local_into();
            let dd = dd.local_into().unwrap_or(dn.clone());
            let dm = dm.local_into();
            let zero = (0., OperandType::Binary32).local_into();
            {
                let mut ret = Vec::new();
                let prod = general_assembly::extension::ieee754::Operand {
                    ty: general_assembly::extension::ieee754::OperandType::Binary64,
                    value: general_assembly::extension::ieee754::OperandStorage::Local(
                        "prod".to_string(),
                    ),
                };
                ret.extend([
                    general_assembly::operation::Operation::Ieee754(general_assembly::extension::ieee754::Operations::Multiplication {
                        destination: prod.clone(),
                        lhs: dn.clone(),
                        rhs: dm.clone(),
                    }),
                    general_assembly::operation::Operation::Ieee754(general_assembly::extension::ieee754::Operations::Subtraction {
                        destination: dd.clone(),
                        lhs: zero.clone(),
                        rhs: prod.clone(),
                    }),
                ]);
                ret
            }
        }
    }
    impl Decode for VmulF32 {
        fn decode(
            &self,
            _in_it_block: bool,
        ) -> Vec<general_assembly::prelude::Operation> {
            let Self { sd, sn, sm } = self;
            let sn = sn.local_into();
            let sd = sd.local_into().unwrap_or(sn.clone());
            let sm = sm.local_into();
            {
                let mut ret = Vec::new();
                ret.extend([
                    general_assembly::operation::Operation::Nop,
                    general_assembly::operation::Operation::Ieee754(general_assembly::extension::ieee754::Operations::Multiplication {
                        destination: sd.clone(),
                        lhs: sn.clone(),
                        rhs: sm.clone(),
                    }),
                ]);
                ret
            }
        }
    }
    impl Decode for VmulF64 {
        fn decode(
            &self,
            _in_it_block: bool,
        ) -> Vec<general_assembly::prelude::Operation> {
            let Self { dd, dn, dm } = self;
            let dn = dn.local_into();
            let dd = dd.local_into().unwrap_or(dn.clone());
            let dm = dm.local_into();
            {
                let mut ret = Vec::new();
                ret.extend([
                    general_assembly::operation::Operation::Nop,
                    general_assembly::operation::Operation::Ieee754(general_assembly::extension::ieee754::Operations::Multiplication {
                        destination: dd.clone(),
                        lhs: dn.clone(),
                        rhs: dm.clone(),
                    }),
                ]);
                ret
            }
        }
    }
    impl Decode for VsubF32 {
        fn decode(
            &self,
            _in_it_block: bool,
        ) -> Vec<general_assembly::prelude::Operation> {
            let Self { sd, sn, sm } = self;
            let sn = sn.local_into();
            let sd = sd.local_into().unwrap_or(sn.clone());
            let sm = sm.local_into();
            {
                let mut ret = Vec::new();
                ret.extend([
                    general_assembly::operation::Operation::Ieee754(general_assembly::extension::ieee754::Operations::Subtraction {
                        destination: sd.clone(),
                        lhs: sn.clone(),
                        rhs: sm.clone(),
                    }),
                ]);
                ret
            }
        }
    }
    impl Decode for VsubF64 {
        fn decode(
            &self,
            _in_it_block: bool,
        ) -> Vec<general_assembly::prelude::Operation> {
            let Self { dd, dn, dm } = self;
            let dn = dn.local_into();
            let dd = dd.local_into().unwrap_or(dn.clone());
            let dm = dm.local_into();
            {
                let mut ret = Vec::new();
                ret.extend([
                    general_assembly::operation::Operation::Ieee754(general_assembly::extension::ieee754::Operations::Subtraction {
                        destination: dd.clone(),
                        lhs: dn.clone(),
                        rhs: dm.clone(),
                    }),
                ]);
                ret
            }
        }
    }
    impl Decode for VdivF32 {
        fn decode(
            &self,
            _in_it_block: bool,
        ) -> Vec<general_assembly::prelude::Operation> {
            let Self { sd, sn, sm } = self;
            let sn = sn.local_into();
            let sd = sd.local_into().unwrap_or(sn.clone());
            let sm = sm.local_into();
            {
                let mut ret = Vec::new();
                ret.extend([
                    general_assembly::operation::Operation::Ieee754(general_assembly::extension::ieee754::Operations::Division {
                        destination: sd.clone(),
                        nominator: sn.clone(),
                        denominator: sm.clone(),
                    }),
                ]);
                ret
            }
        }
    }
    impl Decode for VdivF64 {
        fn decode(
            &self,
            _in_it_block: bool,
        ) -> Vec<general_assembly::prelude::Operation> {
            let Self { dd, dn, dm } = self;
            let dn = dn.local_into();
            let dd = dd.local_into().unwrap_or(dn.clone());
            let dm = dm.local_into();
            {
                let mut ret = Vec::new();
                ret.extend([
                    general_assembly::operation::Operation::Ieee754(general_assembly::extension::ieee754::Operations::Division {
                        destination: dd.clone(),
                        nominator: dn.clone(),
                        denominator: dm.clone(),
                    }),
                ]);
                ret
            }
        }
    }
    impl Decode for VmaxF32 {
        fn decode(
            &self,
            _in_it_block: bool,
        ) -> Vec<general_assembly::prelude::Operation> {
            let Self { sd, sn, sm } = self;
            let sn = sn.local_into();
            let sd = sd.local_into().unwrap_or(sn.clone());
            let sm = sm.local_into();
            {
                let mut ret = Vec::new();
                let intermediate_0 = general_assembly::operand::Operand::Local(
                    "intermediate_0".to_string(),
                );
                ret.extend([
                    general_assembly::operation::Operation::Nop,
                    general_assembly::operation::Operation::Nop,
                    general_assembly::operation::Operation::Ieee754(general_assembly::extension::ieee754::Operations::Compare {
                        lhs: sn.clone().clone(),
                        rhs: sm.clone().clone(),
                        operation: general_assembly::extension::ieee754::ComparisonMode::Greater
                            .clone(),
                        destination: intermediate_0.clone(),
                        signal: false,
                    }),
                    general_assembly::operation::Operation::Ite {
                        condition: intermediate_0.clone(),
                        then: <[_]>::into_vec(
                            ::alloc::boxed::box_new([
                                general_assembly::operation::Operation::Ieee754(general_assembly::extension::ieee754::Operations::Copy {
                                    destination: sd.clone(),
                                    source: sn.clone(),
                                }),
                            ]),
                        ),
                        otherwise: <[_]>::into_vec(
                            ::alloc::boxed::box_new([
                                general_assembly::operation::Operation::Ieee754(general_assembly::extension::ieee754::Operations::Copy {
                                    destination: sd.clone(),
                                    source: sm.clone(),
                                }),
                            ]),
                        ),
                    },
                ]);
                ret
            }
        }
    }
    impl Decode for VmaxF64 {
        fn decode(
            &self,
            _in_it_block: bool,
        ) -> Vec<general_assembly::prelude::Operation> {
            let Self { dd, dn, dm } = self;
            let dn = dn.local_into();
            let dd = dd.local_into().unwrap_or(dn.clone());
            let dm = dm.local_into();
            {
                let mut ret = Vec::new();
                let intermediate_0 = general_assembly::operand::Operand::Local(
                    "intermediate_0".to_string(),
                );
                ret.extend([
                    general_assembly::operation::Operation::Nop,
                    general_assembly::operation::Operation::Nop,
                    general_assembly::operation::Operation::Ieee754(general_assembly::extension::ieee754::Operations::Compare {
                        lhs: dn.clone().clone(),
                        rhs: dm.clone().clone(),
                        operation: general_assembly::extension::ieee754::ComparisonMode::Greater
                            .clone(),
                        destination: intermediate_0.clone(),
                        signal: false,
                    }),
                    general_assembly::operation::Operation::Ite {
                        condition: intermediate_0.clone(),
                        then: <[_]>::into_vec(
                            ::alloc::boxed::box_new([
                                general_assembly::operation::Operation::Ieee754(general_assembly::extension::ieee754::Operations::Copy {
                                    destination: dd.clone(),
                                    source: dn.clone(),
                                }),
                            ]),
                        ),
                        otherwise: <[_]>::into_vec(
                            ::alloc::boxed::box_new([
                                general_assembly::operation::Operation::Ieee754(general_assembly::extension::ieee754::Operations::Copy {
                                    destination: dd.clone(),
                                    source: dm.clone(),
                                }),
                            ]),
                        ),
                    },
                ]);
                ret
            }
        }
    }
    impl Decode for VminF32 {
        fn decode(
            &self,
            _in_it_block: bool,
        ) -> Vec<general_assembly::prelude::Operation> {
            let Self { sd, sn, sm } = self;
            let sn = sn.local_into();
            let sd = sd.local_into().unwrap_or(sn.clone());
            let sm = sm.local_into();
            {
                let mut ret = Vec::new();
                let intermediate_0 = general_assembly::operand::Operand::Local(
                    "intermediate_0".to_string(),
                );
                ret.extend([
                    general_assembly::operation::Operation::Nop,
                    general_assembly::operation::Operation::Nop,
                    general_assembly::operation::Operation::Ieee754(general_assembly::extension::ieee754::Operations::Compare {
                        lhs: sn.clone().clone(),
                        rhs: sm.clone().clone(),
                        operation: general_assembly::extension::ieee754::ComparisonMode::Less
                            .clone(),
                        destination: intermediate_0.clone(),
                        signal: false,
                    }),
                    general_assembly::operation::Operation::Ite {
                        condition: intermediate_0.clone(),
                        then: <[_]>::into_vec(
                            ::alloc::boxed::box_new([
                                general_assembly::operation::Operation::Ieee754(general_assembly::extension::ieee754::Operations::Copy {
                                    destination: sd.clone(),
                                    source: sn.clone(),
                                }),
                            ]),
                        ),
                        otherwise: <[_]>::into_vec(
                            ::alloc::boxed::box_new([
                                general_assembly::operation::Operation::Ieee754(general_assembly::extension::ieee754::Operations::Copy {
                                    destination: sd.clone(),
                                    source: sm.clone(),
                                }),
                            ]),
                        ),
                    },
                ]);
                ret
            }
        }
    }
    impl Decode for VminF64 {
        fn decode(
            &self,
            _in_it_block: bool,
        ) -> Vec<general_assembly::prelude::Operation> {
            let Self { dd, dn, dm } = self;
            let dn = dn.local_into();
            let dd = dd.local_into().unwrap_or(dn.clone());
            let dm = dm.local_into();
            {
                let mut ret = Vec::new();
                let intermediate_0 = general_assembly::operand::Operand::Local(
                    "intermediate_0".to_string(),
                );
                ret.extend([
                    general_assembly::operation::Operation::Nop,
                    general_assembly::operation::Operation::Nop,
                    general_assembly::operation::Operation::Ieee754(general_assembly::extension::ieee754::Operations::Compare {
                        lhs: dn.clone().clone(),
                        rhs: dm.clone().clone(),
                        operation: general_assembly::extension::ieee754::ComparisonMode::Less
                            .clone(),
                        destination: intermediate_0.clone(),
                        signal: false,
                    }),
                    general_assembly::operation::Operation::Ite {
                        condition: intermediate_0.clone(),
                        then: <[_]>::into_vec(
                            ::alloc::boxed::box_new([
                                general_assembly::operation::Operation::Ieee754(general_assembly::extension::ieee754::Operations::Copy {
                                    destination: dd.clone(),
                                    source: dn.clone(),
                                }),
                            ]),
                        ),
                        otherwise: <[_]>::into_vec(
                            ::alloc::boxed::box_new([
                                general_assembly::operation::Operation::Ieee754(general_assembly::extension::ieee754::Operations::Copy {
                                    destination: dd.clone(),
                                    source: dm.clone(),
                                }),
                            ]),
                        ),
                    },
                ]);
                ret
            }
        }
    }
    impl Decode for VmovImmediateF32 {
        fn decode(
            &self,
            _in_it_block: bool,
        ) -> Vec<general_assembly::prelude::Operation> {
            let Self { sd, imm } = self;
            let sd = sd.local_into();
            let imm = Wrappedu32(*imm).local_into();
            {
                let mut ret = Vec::new();
                ret.extend([
                    general_assembly::operation::Operation::Ieee754(general_assembly::extension::ieee754::Operations::Copy {
                        destination: sd.clone(),
                        source: imm.clone(),
                    }),
                ]);
                ret
            }
        }
    }
    impl Decode for VmovImmediateF64 {
        fn decode(
            &self,
            _in_it_block: bool,
        ) -> Vec<general_assembly::prelude::Operation> {
            let Self { dd, imm } = self;
            let dd = dd.local_into();
            let imm = Wrappedu64(*imm).local_into();
            {
                let mut ret = Vec::new();
                ret.extend([
                    general_assembly::operation::Operation::Ieee754(general_assembly::extension::ieee754::Operations::Copy {
                        destination: dd.clone(),
                        source: imm.clone(),
                    }),
                ]);
                ret
            }
        }
    }
    impl Decode for VmovRegisterF32 {
        fn decode(
            &self,
            _in_it_block: bool,
        ) -> Vec<general_assembly::prelude::Operation> {
            let Self { sd, sm } = self;
            let sd = sd.local_into();
            let sm = sm.local_into();
            {
                let mut ret = Vec::new();
                ret.extend([
                    general_assembly::operation::Operation::Ieee754(general_assembly::extension::ieee754::Operations::Copy {
                        destination: sd.clone(),
                        source: sm.clone(),
                    }),
                ]);
                ret
            }
        }
    }
    impl Decode for VmovRegisterF64 {
        fn decode(
            &self,
            _in_it_block: bool,
        ) -> Vec<general_assembly::prelude::Operation> {
            let Self { dd, dm } = self;
            let dd = dd.local_into();
            let dm = dm.local_into();
            {
                let mut ret = Vec::new();
                ret.extend([
                    general_assembly::operation::Operation::Ieee754(general_assembly::extension::ieee754::Operations::Copy {
                        destination: dd.clone(),
                        source: dm.clone(),
                    }),
                ]);
                ret
            }
        }
    }
    impl Decode for VnegF32 {
        fn decode(
            &self,
            _in_it_block: bool,
        ) -> Vec<general_assembly::prelude::Operation> {
            let Self { sd, sm } = self;
            let sd = sd.local_into();
            let sm = sm.local_into();
            let zero = (0., OperandType::Binary32).local_into();
            {
                let mut ret = Vec::new();
                ret.extend([
                    general_assembly::operation::Operation::Ieee754(general_assembly::extension::ieee754::Operations::Subtraction {
                        destination: sd.clone(),
                        lhs: zero.clone(),
                        rhs: sm.clone(),
                    }),
                ]);
                ret
            }
        }
    }
    impl Decode for VnegF64 {
        fn decode(
            &self,
            _in_it_block: bool,
        ) -> Vec<general_assembly::prelude::Operation> {
            let Self { dd, dm } = self;
            let dd = dd.local_into();
            let dm = dm.local_into();
            let zero = (0., OperandType::Binary64).local_into();
            {
                let mut ret = Vec::new();
                ret.extend([
                    general_assembly::operation::Operation::Ieee754(general_assembly::extension::ieee754::Operations::Subtraction {
                        destination: dd.clone(),
                        lhs: zero.clone(),
                        rhs: dm.clone(),
                    }),
                ]);
                ret
            }
        }
    }
    impl Decode for VsqrtF32 {
        fn decode(
            &self,
            _in_it_block: bool,
        ) -> Vec<general_assembly::prelude::Operation> {
            let Self { sd, sm } = self;
            let sd = sd.local_into();
            let sm = sm.local_into();
            {
                let mut ret = Vec::new();
                let intermediate_0 = general_assembly::extension::ieee754::Operand {
                    ty: general_assembly::extension::ieee754::OperandType::Binary32,
                    value: general_assembly::extension::ieee754::OperandStorage::Local(
                        "intermediate_0".to_string(),
                    ),
                };
                ret.extend([
                    general_assembly::operation::Operation::Nop,
                    general_assembly::operation::Operation::Ieee754(general_assembly::extension::ieee754::Operations::Sqrt {
                        operand: sm.clone(),
                        destination: intermediate_0.clone(),
                    }),
                    general_assembly::operation::Operation::Ieee754(general_assembly::extension::ieee754::Operations::Copy {
                        destination: sd.clone(),
                        source: intermediate_0.clone(),
                    }),
                ]);
                ret
            }
        }
    }
    impl Decode for VsqrtF64 {
        fn decode(
            &self,
            _in_it_block: bool,
        ) -> Vec<general_assembly::prelude::Operation> {
            let Self { dd, dm } = self;
            let dd = dd.local_into();
            let dm = dm.local_into();
            {
                let mut ret = Vec::new();
                let intermediate_0 = general_assembly::extension::ieee754::Operand {
                    ty: general_assembly::extension::ieee754::OperandType::Binary64,
                    value: general_assembly::extension::ieee754::OperandStorage::Local(
                        "intermediate_0".to_string(),
                    ),
                };
                ret.extend([
                    general_assembly::operation::Operation::Nop,
                    general_assembly::operation::Operation::Ieee754(general_assembly::extension::ieee754::Operations::Sqrt {
                        operand: dm.clone(),
                        destination: intermediate_0.clone(),
                    }),
                    general_assembly::operation::Operation::Ieee754(general_assembly::extension::ieee754::Operations::Copy {
                        destination: dd.clone(),
                        source: intermediate_0.clone(),
                    }),
                ]);
                ret
            }
        }
    }
    impl Decode for VcvtF32 {
        fn decode(
            &self,
            _in_it_block: bool,
        ) -> Vec<general_assembly::prelude::Operation> {
            let Self { top, convert_from_half, sd, sm } = self;
            let sd = sd.local_into();
            let sm = sm.local_into();
            let (high_bit, low_bit) = match top {
                true => (31, 16),
                false => (15, 0),
            };
            let mask = (!(((1u32 << 16) - 1) << low_bit)).local_into();
            {
                let mut ret = Vec::new();
                if *convert_from_half {
                    let local = general_assembly::operand::Operand::Local(
                        "local".to_string(),
                    );
                    let intermediate_0 = general_assembly::operand::Operand::Local(
                        "intermediate_0".to_string(),
                    );
                    let intermediate_1 = general_assembly::operand::Operand::Local(
                        "intermediate_1".to_string(),
                    );
                    let local_16 = general_assembly::operand::Operand::Local(
                        "local_16".to_string(),
                    );
                    let intermediate_2 = general_assembly::operand::Operand::Local(
                        "intermediate_2".to_string(),
                    );
                    let fp_16 = general_assembly::extension::ieee754::Operand {
                        ty: general_assembly::extension::ieee754::OperandType::Binary16,
                        value: general_assembly::extension::ieee754::OperandStorage::Local(
                            "fp_16".to_string(),
                        ),
                    };
                    let intermediate_3 = general_assembly::extension::ieee754::Operand {
                        ty: general_assembly::extension::ieee754::OperandType::Binary16,
                        value: general_assembly::extension::ieee754::OperandStorage::Local(
                            "intermediate_3".to_string(),
                        ),
                    };
                    let intermediate_4 = general_assembly::extension::ieee754::Operand {
                        ty: general_assembly::extension::ieee754::OperandType::Binary32,
                        value: general_assembly::extension::ieee754::OperandStorage::Local(
                            "intermediate_4".to_string(),
                        ),
                    };
                    ret.extend([
                        general_assembly::operation::Operation::Nop,
                        general_assembly::operation::Operation::Ieee754(general_assembly::extension::ieee754::Operations::Copy {
                            source: sm.clone(),
                            destination: general_assembly::extension::ieee754::Operand {
                                ty: general_assembly::extension::ieee754::OperandType::Binary32,
                                value: general_assembly::extension::ieee754::OperandStorage::CoreOperand {
                                    operand: intermediate_0.clone(),
                                    ty: general_assembly::extension::ieee754::OperandType::Binary32,
                                    signed: false,
                                },
                            },
                        }),
                        general_assembly::operation::Operation::Move {
                            destination: local.clone(),
                            source: intermediate_0.clone(),
                        },
                        general_assembly::operation::Operation::BitFieldExtract {
                            destination: intermediate_1.clone().clone(),
                            operand: local.clone(),
                            start_bit: low_bit,
                            stop_bit: high_bit,
                        },
                        general_assembly::operation::Operation::Move {
                            destination: local.clone(),
                            source: intermediate_1.clone(),
                        },
                        general_assembly::operation::Operation::Resize {
                            destination: intermediate_2.clone().clone(),
                            operand: local.clone().clone(),
                            bits: 16u32.clone(),
                        },
                        general_assembly::operation::Operation::Move {
                            destination: local_16.clone(),
                            source: intermediate_2.clone(),
                        },
                        general_assembly::operation::Operation::Ieee754(general_assembly::extension::ieee754::Operations::Copy {
                            destination: intermediate_3.clone(),
                            source: general_assembly::extension::ieee754::Operand {
                                ty: general_assembly::extension::ieee754::OperandType::Binary16,
                                value: general_assembly::extension::ieee754::OperandStorage::CoreOperand {
                                    operand: local_16.clone(),
                                    ty: general_assembly::extension::ieee754::OperandType::Binary16,
                                    signed: false,
                                },
                            },
                        }),
                        general_assembly::operation::Operation::Ieee754(general_assembly::extension::ieee754::Operations::Copy {
                            destination: fp_16.clone(),
                            source: intermediate_3.clone(),
                        }),
                        general_assembly::operation::Operation::Ieee754(general_assembly::extension::ieee754::Operations::Convert {
                            destination: intermediate_4.clone().clone(),
                            source: fp_16.clone().clone(),
                            rounding: None,
                        }),
                        general_assembly::operation::Operation::Ieee754(general_assembly::extension::ieee754::Operations::Copy {
                            destination: sd.clone(),
                            source: intermediate_4.clone(),
                        }),
                    ]);
                } else {
                    let bits_of_sd = general_assembly::operand::Operand::Local(
                        "bits_of_sd".to_string(),
                    );
                    let intermediate_5 = general_assembly::operand::Operand::Local(
                        "intermediate_5".to_string(),
                    );
                    let value = general_assembly::extension::ieee754::Operand {
                        ty: general_assembly::extension::ieee754::OperandType::Binary16,
                        value: general_assembly::extension::ieee754::OperandStorage::Local(
                            "value".to_string(),
                        ),
                    };
                    let intermediate_6 = general_assembly::extension::ieee754::Operand {
                        ty: general_assembly::extension::ieee754::OperandType::Binary16,
                        value: general_assembly::extension::ieee754::OperandStorage::Local(
                            "intermediate_6".to_string(),
                        ),
                    };
                    let value_bits = general_assembly::operand::Operand::Local(
                        "value_bits".to_string(),
                    );
                    let intermediate_7 = general_assembly::operand::Operand::Local(
                        "intermediate_7".to_string(),
                    );
                    let value_bits_u32 = general_assembly::operand::Operand::Local(
                        "value_bits_u32".to_string(),
                    );
                    let intermediate_8 = general_assembly::operand::Operand::Local(
                        "intermediate_8".to_string(),
                    );
                    let intermediate_9 = general_assembly::extension::ieee754::Operand {
                        ty: general_assembly::extension::ieee754::OperandType::Binary32,
                        value: general_assembly::extension::ieee754::OperandStorage::Local(
                            "intermediate_9".to_string(),
                        ),
                    };
                    ret.extend([
                        general_assembly::operation::Operation::Nop,
                        general_assembly::operation::Operation::Nop,
                        general_assembly::operation::Operation::Ieee754(general_assembly::extension::ieee754::Operations::Copy {
                            source: sd.clone(),
                            destination: general_assembly::extension::ieee754::Operand {
                                ty: general_assembly::extension::ieee754::OperandType::Binary32,
                                value: general_assembly::extension::ieee754::OperandStorage::CoreOperand {
                                    operand: intermediate_5.clone(),
                                    ty: general_assembly::extension::ieee754::OperandType::Binary32,
                                    signed: false,
                                },
                            },
                        }),
                        general_assembly::operation::Operation::And {
                            destination: bits_of_sd.clone(),
                            operand1: intermediate_5.clone(),
                            operand2: mask.clone(),
                        },
                        general_assembly::operation::Operation::Ieee754(general_assembly::extension::ieee754::Operations::Convert {
                            destination: intermediate_6.clone().clone(),
                            source: sm.clone().clone(),
                            rounding: None,
                        }),
                        general_assembly::operation::Operation::Ieee754(general_assembly::extension::ieee754::Operations::Copy {
                            destination: value.clone(),
                            source: intermediate_6.clone(),
                        }),
                        general_assembly::operation::Operation::Ieee754(general_assembly::extension::ieee754::Operations::Copy {
                            source: value.clone(),
                            destination: general_assembly::extension::ieee754::Operand {
                                ty: general_assembly::extension::ieee754::OperandType::Binary16,
                                value: general_assembly::extension::ieee754::OperandStorage::CoreOperand {
                                    operand: intermediate_7.clone(),
                                    ty: general_assembly::extension::ieee754::OperandType::Binary16,
                                    signed: false,
                                },
                            },
                        }),
                        general_assembly::operation::Operation::Move {
                            destination: value_bits.clone(),
                            source: intermediate_7.clone(),
                        },
                        general_assembly::operation::Operation::Resize {
                            destination: intermediate_8.clone().clone(),
                            operand: value_bits.clone().clone(),
                            bits: 32u32.clone(),
                        },
                        general_assembly::operation::Operation::Sl {
                            destination: value_bits_u32.clone(),
                            operand: intermediate_8.clone(),
                            shift: low_bit.clone().local_into(),
                        },
                        general_assembly::operation::Operation::Or {
                            destination: bits_of_sd.clone(),
                            operand1: bits_of_sd.clone(),
                            operand2: value_bits_u32.clone(),
                        },
                        general_assembly::operation::Operation::Ieee754(general_assembly::extension::ieee754::Operations::Copy {
                            destination: intermediate_9.clone(),
                            source: general_assembly::extension::ieee754::Operand {
                                ty: general_assembly::extension::ieee754::OperandType::Binary32,
                                value: general_assembly::extension::ieee754::OperandStorage::CoreOperand {
                                    operand: bits_of_sd.clone(),
                                    ty: general_assembly::extension::ieee754::OperandType::Binary32,
                                    signed: false,
                                },
                            },
                        }),
                        general_assembly::operation::Operation::Ieee754(general_assembly::extension::ieee754::Operations::Copy {
                            destination: sd.clone(),
                            source: intermediate_9.clone(),
                        }),
                    ]);
                };
                ret
            }
        }
    }
    impl Decode for VcvtF64 {
        fn decode(
            &self,
            _in_it_block: bool,
        ) -> Vec<general_assembly::prelude::Operation> {
            let Self { top, convert_from_half, dd, dm } = self;
            let dd = dd.clone().local_into();
            let dm = dm.clone().local_into();
            let (high_bit, low_bit) = match top {
                true => (31, 16),
                false => (15, 0),
            };
            let mask = (!(((1u32 << 16) - 1) << low_bit)).local_into();
            {
                let mut ret = Vec::new();
                if *convert_from_half {
                    let local = general_assembly::operand::Operand::Local(
                        "local".to_string(),
                    );
                    let intermediate_0 = general_assembly::operand::Operand::Local(
                        "intermediate_0".to_string(),
                    );
                    let intermediate_1 = general_assembly::operand::Operand::Local(
                        "intermediate_1".to_string(),
                    );
                    let local_16 = general_assembly::operand::Operand::Local(
                        "local_16".to_string(),
                    );
                    let intermediate_2 = general_assembly::operand::Operand::Local(
                        "intermediate_2".to_string(),
                    );
                    let fp_16 = general_assembly::extension::ieee754::Operand {
                        ty: general_assembly::extension::ieee754::OperandType::Binary16,
                        value: general_assembly::extension::ieee754::OperandStorage::Local(
                            "fp_16".to_string(),
                        ),
                    };
                    let intermediate_3 = general_assembly::extension::ieee754::Operand {
                        ty: general_assembly::extension::ieee754::OperandType::Binary16,
                        value: general_assembly::extension::ieee754::OperandStorage::Local(
                            "intermediate_3".to_string(),
                        ),
                    };
                    let intermediate_4 = general_assembly::extension::ieee754::Operand {
                        ty: general_assembly::extension::ieee754::OperandType::Binary64,
                        value: general_assembly::extension::ieee754::OperandStorage::Local(
                            "intermediate_4".to_string(),
                        ),
                    };
                    ret.extend([
                        general_assembly::operation::Operation::Nop,
                        general_assembly::operation::Operation::Nop,
                        general_assembly::operation::Operation::Ieee754(general_assembly::extension::ieee754::Operations::Copy {
                            source: dm.clone(),
                            destination: general_assembly::extension::ieee754::Operand {
                                ty: general_assembly::extension::ieee754::OperandType::Binary32,
                                value: general_assembly::extension::ieee754::OperandStorage::CoreOperand {
                                    operand: intermediate_0.clone(),
                                    ty: general_assembly::extension::ieee754::OperandType::Binary32,
                                    signed: false,
                                },
                            },
                        }),
                        general_assembly::operation::Operation::Move {
                            destination: local.clone(),
                            source: intermediate_0.clone(),
                        },
                        general_assembly::operation::Operation::BitFieldExtract {
                            destination: intermediate_1.clone().clone(),
                            operand: local.clone(),
                            start_bit: low_bit,
                            stop_bit: high_bit,
                        },
                        general_assembly::operation::Operation::Move {
                            destination: local.clone(),
                            source: intermediate_1.clone(),
                        },
                        general_assembly::operation::Operation::Resize {
                            destination: intermediate_2.clone().clone(),
                            operand: local.clone().clone(),
                            bits: 16u32.clone(),
                        },
                        general_assembly::operation::Operation::Move {
                            destination: local_16.clone(),
                            source: intermediate_2.clone(),
                        },
                        general_assembly::operation::Operation::Ieee754(general_assembly::extension::ieee754::Operations::Copy {
                            destination: intermediate_3.clone(),
                            source: general_assembly::extension::ieee754::Operand {
                                ty: general_assembly::extension::ieee754::OperandType::Binary16,
                                value: general_assembly::extension::ieee754::OperandStorage::CoreOperand {
                                    operand: local_16.clone(),
                                    ty: general_assembly::extension::ieee754::OperandType::Binary16,
                                    signed: false,
                                },
                            },
                        }),
                        general_assembly::operation::Operation::Ieee754(general_assembly::extension::ieee754::Operations::Copy {
                            destination: fp_16.clone(),
                            source: intermediate_3.clone(),
                        }),
                        general_assembly::operation::Operation::Ieee754(general_assembly::extension::ieee754::Operations::Convert {
                            destination: intermediate_4.clone().clone(),
                            source: fp_16.clone().clone(),
                            rounding: None,
                        }),
                        general_assembly::operation::Operation::Ieee754(general_assembly::extension::ieee754::Operations::Copy {
                            destination: dd.clone(),
                            source: intermediate_4.clone(),
                        }),
                    ]);
                } else {
                    let bits_of_dd = general_assembly::operand::Operand::Local(
                        "bits_of_dd".to_string(),
                    );
                    let intermediate_5 = general_assembly::operand::Operand::Local(
                        "intermediate_5".to_string(),
                    );
                    let value = general_assembly::extension::ieee754::Operand {
                        ty: general_assembly::extension::ieee754::OperandType::Binary16,
                        value: general_assembly::extension::ieee754::OperandStorage::Local(
                            "value".to_string(),
                        ),
                    };
                    let intermediate_6 = general_assembly::extension::ieee754::Operand {
                        ty: general_assembly::extension::ieee754::OperandType::Binary16,
                        value: general_assembly::extension::ieee754::OperandStorage::Local(
                            "intermediate_6".to_string(),
                        ),
                    };
                    let value_bits = general_assembly::operand::Operand::Local(
                        "value_bits".to_string(),
                    );
                    let intermediate_7 = general_assembly::operand::Operand::Local(
                        "intermediate_7".to_string(),
                    );
                    let value_bits_u32 = general_assembly::operand::Operand::Local(
                        "value_bits_u32".to_string(),
                    );
                    let intermediate_8 = general_assembly::operand::Operand::Local(
                        "intermediate_8".to_string(),
                    );
                    let intermediate_9 = general_assembly::extension::ieee754::Operand {
                        ty: general_assembly::extension::ieee754::OperandType::Binary32,
                        value: general_assembly::extension::ieee754::OperandStorage::Local(
                            "intermediate_9".to_string(),
                        ),
                    };
                    ret.extend([
                        general_assembly::operation::Operation::Nop,
                        general_assembly::operation::Operation::Nop,
                        general_assembly::operation::Operation::Ieee754(general_assembly::extension::ieee754::Operations::Copy {
                            source: dd.clone(),
                            destination: general_assembly::extension::ieee754::Operand {
                                ty: general_assembly::extension::ieee754::OperandType::Binary32,
                                value: general_assembly::extension::ieee754::OperandStorage::CoreOperand {
                                    operand: intermediate_5.clone(),
                                    ty: general_assembly::extension::ieee754::OperandType::Binary32,
                                    signed: false,
                                },
                            },
                        }),
                        general_assembly::operation::Operation::And {
                            destination: bits_of_dd.clone(),
                            operand1: intermediate_5.clone(),
                            operand2: mask.clone(),
                        },
                        general_assembly::operation::Operation::Ieee754(general_assembly::extension::ieee754::Operations::Convert {
                            destination: intermediate_6.clone().clone(),
                            source: dm.clone().clone(),
                            rounding: None,
                        }),
                        general_assembly::operation::Operation::Ieee754(general_assembly::extension::ieee754::Operations::Copy {
                            destination: value.clone(),
                            source: intermediate_6.clone(),
                        }),
                        general_assembly::operation::Operation::Ieee754(general_assembly::extension::ieee754::Operations::Copy {
                            source: value.clone(),
                            destination: general_assembly::extension::ieee754::Operand {
                                ty: general_assembly::extension::ieee754::OperandType::Binary16,
                                value: general_assembly::extension::ieee754::OperandStorage::CoreOperand {
                                    operand: intermediate_7.clone(),
                                    ty: general_assembly::extension::ieee754::OperandType::Binary16,
                                    signed: false,
                                },
                            },
                        }),
                        general_assembly::operation::Operation::Move {
                            destination: value_bits.clone(),
                            source: intermediate_7.clone(),
                        },
                        general_assembly::operation::Operation::Resize {
                            destination: intermediate_8.clone().clone(),
                            operand: value_bits.clone().clone(),
                            bits: 32u32.clone(),
                        },
                        general_assembly::operation::Operation::Sl {
                            destination: value_bits_u32.clone(),
                            operand: intermediate_8.clone(),
                            shift: low_bit.clone().local_into(),
                        },
                        general_assembly::operation::Operation::Or {
                            destination: bits_of_dd.clone(),
                            operand1: bits_of_dd.clone(),
                            operand2: value_bits_u32.clone(),
                        },
                        general_assembly::operation::Operation::Ieee754(general_assembly::extension::ieee754::Operations::Copy {
                            destination: intermediate_9.clone(),
                            source: general_assembly::extension::ieee754::Operand {
                                ty: general_assembly::extension::ieee754::OperandType::Binary32,
                                value: general_assembly::extension::ieee754::OperandStorage::CoreOperand {
                                    operand: bits_of_dd.clone(),
                                    ty: general_assembly::extension::ieee754::OperandType::Binary32,
                                    signed: false,
                                },
                            },
                        }),
                        general_assembly::operation::Operation::Ieee754(general_assembly::extension::ieee754::Operations::Copy {
                            destination: dd.clone(),
                            source: intermediate_9.clone(),
                        }),
                    ]);
                };
                ret
            }
        }
    }
    impl Decode for VcmpF32 {
        fn decode(
            &self,
            _in_it_block: bool,
        ) -> Vec<general_assembly::prelude::Operation> {
            let Self { e, sd, sm } = self;
            let e = e.unwrap_or(false);
            let sd = sd.local_into();
            let sm = sm.local_into();
            {
                let mut ret = Vec::new();
                let conditional = general_assembly::operand::Operand::Local(
                    "conditional".to_string(),
                );
                let intermediate_0 = general_assembly::operand::Operand::Local(
                    "intermediate_0".to_string(),
                );
                let intermediate_1 = general_assembly::operand::Operand::Local(
                    "intermediate_1".to_string(),
                );
                ret.extend([
                    general_assembly::operation::Operation::Nop,
                    general_assembly::operation::Operation::Nop,
                    general_assembly::operation::Operation::Ieee754(general_assembly::extension::ieee754::Operations::NonComputational {
                        operand: sm.clone(),
                        operation: general_assembly::extension::ieee754::NonComputational::IsNan,
                        destination: intermediate_0.clone(),
                    }),
                    general_assembly::operation::Operation::Ieee754(general_assembly::extension::ieee754::Operations::NonComputational {
                        operand: sd.clone(),
                        operation: general_assembly::extension::ieee754::NonComputational::IsNan,
                        destination: intermediate_1.clone(),
                    }),
                    general_assembly::operation::Operation::Or {
                        destination: conditional.clone(),
                        operand1: intermediate_1.clone(),
                        operand2: intermediate_0.clone(),
                    },
                ]);
                if e {
                    let intermediate_2 = general_assembly::operand::Operand::Local(
                        "intermediate_2".to_string(),
                    );
                    ret.extend([
                        general_assembly::operation::Operation::Compare {
                            lhs: conditional.clone().clone(),
                            rhs: general_assembly::operand::Operand::Immediate(
                                    general_assembly::prelude::DataWord::Bit(false),
                                )
                                .clone(),
                            operation: general_assembly::condition::Comparison::Neq,
                            destination: intermediate_2.clone(),
                        },
                        general_assembly::operation::Operation::Ite {
                            condition: intermediate_2.clone(),
                            then: <[_]>::into_vec(
                                ::alloc::boxed::box_new([
                                    general_assembly::operation::Operation::Abort {
                                        error: ::alloc::__export::must_use({
                                            let res = ::alloc::fmt::format(
                                                format_args!("Invalid Operation exception"),
                                            );
                                            res
                                        }),
                                    },
                                ]),
                            ),
                            otherwise: ::alloc::vec::Vec::new(),
                        },
                    ]);
                }
                let is_zero = general_assembly::operand::Operand::Local(
                    "is_zero".to_string(),
                );
                let less_than_zero = general_assembly::operand::Operand::Local(
                    "less_than_zero".to_string(),
                );
                let greater_than_zero = general_assembly::operand::Operand::Local(
                    "greater_than_zero".to_string(),
                );
                let c = general_assembly::operand::Operand::Local("c".to_string());
                let intermediate_3 = general_assembly::operand::Operand::Local(
                    "intermediate_3".to_string(),
                );
                ret.extend([
                    general_assembly::operation::Operation::Ieee754(general_assembly::extension::ieee754::Operations::Compare {
                        lhs: sd.clone(),
                        rhs: sm.clone(),
                        operation: general_assembly::extension::ieee754::ComparisonMode::Equal,
                        destination: is_zero.clone(),
                        signal: false,
                    }),
                    general_assembly::operation::Operation::Ieee754(general_assembly::extension::ieee754::Operations::Compare {
                        lhs: sd.clone(),
                        rhs: sm.clone(),
                        operation: general_assembly::extension::ieee754::ComparisonMode::Less,
                        destination: less_than_zero.clone(),
                        signal: false,
                    }),
                    general_assembly::operation::Operation::Ieee754(general_assembly::extension::ieee754::Operations::Compare {
                        lhs: sd.clone(),
                        rhs: sm.clone(),
                        operation: general_assembly::extension::ieee754::ComparisonMode::Greater,
                        destination: greater_than_zero.clone(),
                        signal: false,
                    }),
                    general_assembly::operation::Operation::Or {
                        destination: c.clone(),
                        operand1: is_zero.clone(),
                        operand2: greater_than_zero.clone(),
                    },
                    general_assembly::operation::Operation::Move {
                        destination: general_assembly::operand::Operand::Flag(
                            "FPSCR.N".to_owned(),
                        ),
                        source: less_than_zero.clone(),
                    },
                    general_assembly::operation::Operation::Move {
                        destination: general_assembly::operand::Operand::Flag(
                            "FPSCR.Z".to_owned(),
                        ),
                        source: is_zero.clone(),
                    },
                    general_assembly::operation::Operation::Move {
                        destination: general_assembly::operand::Operand::Flag(
                            "FPSCR.C".to_owned(),
                        ),
                        source: c.clone(),
                    },
                    general_assembly::operation::Operation::Move {
                        destination: general_assembly::operand::Operand::Flag(
                            "FPSCR.V".to_owned(),
                        ),
                        source: general_assembly::operand::Operand::Immediate(
                            general_assembly::prelude::DataWord::Bit(false),
                        ),
                    },
                    general_assembly::operation::Operation::Compare {
                        lhs: conditional.clone().clone(),
                        rhs: general_assembly::operand::Operand::Immediate(
                                general_assembly::prelude::DataWord::Bit(false),
                            )
                            .clone(),
                        operation: general_assembly::condition::Comparison::Neq,
                        destination: intermediate_3.clone(),
                    },
                    general_assembly::operation::Operation::Ite {
                        condition: intermediate_3.clone(),
                        then: <[_]>::into_vec(
                            ::alloc::boxed::box_new([
                                general_assembly::operation::Operation::Move {
                                    destination: general_assembly::operand::Operand::Flag(
                                        "FPSCR.N".to_owned(),
                                    ),
                                    source: general_assembly::operand::Operand::Immediate(
                                        general_assembly::prelude::DataWord::Bit(false),
                                    ),
                                },
                                general_assembly::operation::Operation::Move {
                                    destination: general_assembly::operand::Operand::Flag(
                                        "FPSCR.Z".to_owned(),
                                    ),
                                    source: general_assembly::operand::Operand::Immediate(
                                        general_assembly::prelude::DataWord::Bit(false),
                                    ),
                                },
                                general_assembly::operation::Operation::Move {
                                    destination: general_assembly::operand::Operand::Flag(
                                        "FPSCR.C".to_owned(),
                                    ),
                                    source: general_assembly::operand::Operand::Immediate(
                                        general_assembly::prelude::DataWord::Bit(true),
                                    ),
                                },
                                general_assembly::operation::Operation::Move {
                                    destination: general_assembly::operand::Operand::Flag(
                                        "FPSCR.V".to_owned(),
                                    ),
                                    source: general_assembly::operand::Operand::Immediate(
                                        general_assembly::prelude::DataWord::Bit(true),
                                    ),
                                },
                            ]),
                        ),
                        otherwise: ::alloc::vec::Vec::new(),
                    },
                ]);
                ret
            }
        }
    }
    impl Decode for VcmpZeroF32 {
        fn decode(
            &self,
            _in_it_block: bool,
        ) -> Vec<general_assembly::prelude::Operation> {
            let Self { e, sd } = self;
            let sd = sd.local_into();
            let e = e.unwrap_or(false);
            {
                let mut ret = Vec::new();
                let conditional = general_assembly::operand::Operand::Local(
                    "conditional".to_string(),
                );
                let intermediate_0 = general_assembly::operand::Operand::Local(
                    "intermediate_0".to_string(),
                );
                let result = general_assembly::operand::Operand::Local(
                    "result".to_string(),
                );
                ret.extend([
                    general_assembly::operation::Operation::Nop,
                    general_assembly::operation::Operation::Ieee754(general_assembly::extension::ieee754::Operations::NonComputational {
                        operand: sd.clone(),
                        operation: general_assembly::extension::ieee754::NonComputational::IsNan,
                        destination: intermediate_0.clone(),
                    }),
                    general_assembly::operation::Operation::Move {
                        destination: conditional.clone(),
                        source: intermediate_0.clone(),
                    },
                    general_assembly::operation::Operation::Move {
                        destination: result.clone(),
                        source: 0.local_into(),
                    },
                ]);
                if e {
                    let intermediate_1 = general_assembly::operand::Operand::Local(
                        "intermediate_1".to_string(),
                    );
                    ret.extend([
                        general_assembly::operation::Operation::Compare {
                            lhs: conditional.clone().clone(),
                            rhs: general_assembly::operand::Operand::Immediate(
                                    general_assembly::prelude::DataWord::Bit(false),
                                )
                                .clone(),
                            operation: general_assembly::condition::Comparison::Neq,
                            destination: intermediate_1.clone(),
                        },
                        general_assembly::operation::Operation::Ite {
                            condition: intermediate_1.clone(),
                            then: <[_]>::into_vec(
                                ::alloc::boxed::box_new([
                                    general_assembly::operation::Operation::Abort {
                                        error: ::alloc::__export::must_use({
                                            let res = ::alloc::fmt::format(
                                                format_args!("Invalid Operation exception"),
                                            );
                                            res
                                        }),
                                    },
                                ]),
                            ),
                            otherwise: ::alloc::vec::Vec::new(),
                        },
                    ]);
                }
                let is_zero = general_assembly::operand::Operand::Local(
                    "is_zero".to_string(),
                );
                let less_than_zero = general_assembly::operand::Operand::Local(
                    "less_than_zero".to_string(),
                );
                let greater_than_zero = general_assembly::operand::Operand::Local(
                    "greater_than_zero".to_string(),
                );
                let c = general_assembly::operand::Operand::Local("c".to_string());
                let intermediate_2 = general_assembly::operand::Operand::Local(
                    "intermediate_2".to_string(),
                );
                ret.extend([
                    general_assembly::operation::Operation::Ieee754(general_assembly::extension::ieee754::Operations::Compare {
                        lhs: sd.clone(),
                        rhs: general_assembly::extension::ieee754::Operand {
                            ty: general_assembly::extension::ieee754::OperandType::Binary32,
                            value: general_assembly::extension::ieee754::OperandStorage::Immediate {
                                value: 0.0f32 as f64,
                                ty: general_assembly::extension::ieee754::OperandType::Binary32,
                            },
                        },
                        operation: general_assembly::extension::ieee754::ComparisonMode::Equal,
                        destination: is_zero.clone(),
                        signal: false,
                    }),
                    general_assembly::operation::Operation::Ieee754(general_assembly::extension::ieee754::Operations::Compare {
                        lhs: sd.clone(),
                        rhs: general_assembly::extension::ieee754::Operand {
                            ty: general_assembly::extension::ieee754::OperandType::Binary32,
                            value: general_assembly::extension::ieee754::OperandStorage::Immediate {
                                value: 0.0f32 as f64,
                                ty: general_assembly::extension::ieee754::OperandType::Binary32,
                            },
                        },
                        operation: general_assembly::extension::ieee754::ComparisonMode::Less,
                        destination: less_than_zero.clone(),
                        signal: false,
                    }),
                    general_assembly::operation::Operation::Ieee754(general_assembly::extension::ieee754::Operations::Compare {
                        lhs: sd.clone(),
                        rhs: general_assembly::extension::ieee754::Operand {
                            ty: general_assembly::extension::ieee754::OperandType::Binary32,
                            value: general_assembly::extension::ieee754::OperandStorage::Immediate {
                                value: 0.0f32 as f64,
                                ty: general_assembly::extension::ieee754::OperandType::Binary32,
                            },
                        },
                        operation: general_assembly::extension::ieee754::ComparisonMode::Greater,
                        destination: greater_than_zero.clone(),
                        signal: false,
                    }),
                    general_assembly::operation::Operation::Or {
                        destination: c.clone(),
                        operand1: is_zero.clone(),
                        operand2: greater_than_zero.clone(),
                    },
                    general_assembly::operation::Operation::Move {
                        destination: general_assembly::operand::Operand::Flag(
                            "FPSCR.N".to_owned(),
                        ),
                        source: less_than_zero.clone(),
                    },
                    general_assembly::operation::Operation::Move {
                        destination: general_assembly::operand::Operand::Flag(
                            "FPSCR.Z".to_owned(),
                        ),
                        source: is_zero.clone(),
                    },
                    general_assembly::operation::Operation::Move {
                        destination: general_assembly::operand::Operand::Flag(
                            "FPSCR.C".to_owned(),
                        ),
                        source: c.clone(),
                    },
                    general_assembly::operation::Operation::Move {
                        destination: general_assembly::operand::Operand::Flag(
                            "FPSCR.V".to_owned(),
                        ),
                        source: general_assembly::operand::Operand::Immediate(
                            general_assembly::prelude::DataWord::Bit(false),
                        ),
                    },
                    general_assembly::operation::Operation::Compare {
                        lhs: conditional.clone().clone(),
                        rhs: general_assembly::operand::Operand::Immediate(
                                general_assembly::prelude::DataWord::Bit(false),
                            )
                            .clone(),
                        operation: general_assembly::condition::Comparison::Neq,
                        destination: intermediate_2.clone(),
                    },
                    general_assembly::operation::Operation::Ite {
                        condition: intermediate_2.clone(),
                        then: <[_]>::into_vec(
                            ::alloc::boxed::box_new([
                                general_assembly::operation::Operation::Move {
                                    destination: result.clone(),
                                    source: general_assembly::operand::Operand::Immediate(
                                        general_assembly::prelude::DataWord::Word32(
                                            0b0011u32 as u32,
                                        ),
                                    ),
                                },
                                general_assembly::operation::Operation::Move {
                                    destination: general_assembly::operand::Operand::Flag(
                                        "FPSCR.N".to_owned(),
                                    ),
                                    source: general_assembly::operand::Operand::Immediate(
                                        general_assembly::prelude::DataWord::Bit(false),
                                    ),
                                },
                                general_assembly::operation::Operation::Move {
                                    destination: general_assembly::operand::Operand::Flag(
                                        "FPSCR.Z".to_owned(),
                                    ),
                                    source: general_assembly::operand::Operand::Immediate(
                                        general_assembly::prelude::DataWord::Bit(false),
                                    ),
                                },
                                general_assembly::operation::Operation::Move {
                                    destination: general_assembly::operand::Operand::Flag(
                                        "FPSCR.C".to_owned(),
                                    ),
                                    source: general_assembly::operand::Operand::Immediate(
                                        general_assembly::prelude::DataWord::Bit(true),
                                    ),
                                },
                                general_assembly::operation::Operation::Move {
                                    destination: general_assembly::operand::Operand::Flag(
                                        "FPSCR.V".to_owned(),
                                    ),
                                    source: general_assembly::operand::Operand::Immediate(
                                        general_assembly::prelude::DataWord::Bit(true),
                                    ),
                                },
                            ]),
                        ),
                        otherwise: ::alloc::vec::Vec::new(),
                    },
                ]);
                ret
            }
        }
    }
    impl Decode for VcmpZeroF64 {
        fn decode(
            &self,
            _in_it_block: bool,
        ) -> Vec<general_assembly::prelude::Operation> {
            let Self { e, dd } = self;
            let dd = dd.local_into();
            let e = e.unwrap_or(false);
            {
                let mut ret = Vec::new();
                let conditional = general_assembly::operand::Operand::Local(
                    "conditional".to_string(),
                );
                let intermediate_0 = general_assembly::operand::Operand::Local(
                    "intermediate_0".to_string(),
                );
                ret.extend([
                    general_assembly::operation::Operation::Nop,
                    general_assembly::operation::Operation::Ieee754(general_assembly::extension::ieee754::Operations::NonComputational {
                        operand: dd.clone(),
                        operation: general_assembly::extension::ieee754::NonComputational::IsNan,
                        destination: intermediate_0.clone(),
                    }),
                    general_assembly::operation::Operation::Move {
                        destination: conditional.clone(),
                        source: intermediate_0.clone(),
                    },
                ]);
                if e {
                    let intermediate_1 = general_assembly::operand::Operand::Local(
                        "intermediate_1".to_string(),
                    );
                    ret.extend([
                        general_assembly::operation::Operation::Compare {
                            lhs: conditional.clone().clone(),
                            rhs: general_assembly::operand::Operand::Immediate(
                                    general_assembly::prelude::DataWord::Bit(false),
                                )
                                .clone(),
                            operation: general_assembly::condition::Comparison::Neq,
                            destination: intermediate_1.clone(),
                        },
                        general_assembly::operation::Operation::Ite {
                            condition: intermediate_1.clone(),
                            then: <[_]>::into_vec(
                                ::alloc::boxed::box_new([
                                    general_assembly::operation::Operation::Abort {
                                        error: ::alloc::__export::must_use({
                                            let res = ::alloc::fmt::format(
                                                format_args!("Invalid Operation exception"),
                                            );
                                            res
                                        }),
                                    },
                                ]),
                            ),
                            otherwise: ::alloc::vec::Vec::new(),
                        },
                    ]);
                }
                let is_zero = general_assembly::operand::Operand::Local(
                    "is_zero".to_string(),
                );
                let less_than_zero = general_assembly::operand::Operand::Local(
                    "less_than_zero".to_string(),
                );
                let greater_than_zero = general_assembly::operand::Operand::Local(
                    "greater_than_zero".to_string(),
                );
                let c = general_assembly::operand::Operand::Local("c".to_string());
                let intermediate_2 = general_assembly::operand::Operand::Local(
                    "intermediate_2".to_string(),
                );
                ret.extend([
                    general_assembly::operation::Operation::Ieee754(general_assembly::extension::ieee754::Operations::Compare {
                        lhs: dd.clone(),
                        rhs: general_assembly::extension::ieee754::Operand {
                            ty: general_assembly::extension::ieee754::OperandType::Binary64,
                            value: general_assembly::extension::ieee754::OperandStorage::Immediate {
                                value: 0.0f64 as f64,
                                ty: general_assembly::extension::ieee754::OperandType::Binary64,
                            },
                        },
                        operation: general_assembly::extension::ieee754::ComparisonMode::Equal,
                        destination: is_zero.clone(),
                        signal: false,
                    }),
                    general_assembly::operation::Operation::Ieee754(general_assembly::extension::ieee754::Operations::Compare {
                        lhs: dd.clone(),
                        rhs: general_assembly::extension::ieee754::Operand {
                            ty: general_assembly::extension::ieee754::OperandType::Binary64,
                            value: general_assembly::extension::ieee754::OperandStorage::Immediate {
                                value: 0.0f64 as f64,
                                ty: general_assembly::extension::ieee754::OperandType::Binary64,
                            },
                        },
                        operation: general_assembly::extension::ieee754::ComparisonMode::Less,
                        destination: less_than_zero.clone(),
                        signal: false,
                    }),
                    general_assembly::operation::Operation::Ieee754(general_assembly::extension::ieee754::Operations::Compare {
                        lhs: dd.clone(),
                        rhs: general_assembly::extension::ieee754::Operand {
                            ty: general_assembly::extension::ieee754::OperandType::Binary64,
                            value: general_assembly::extension::ieee754::OperandStorage::Immediate {
                                value: 0.0f64 as f64,
                                ty: general_assembly::extension::ieee754::OperandType::Binary64,
                            },
                        },
                        operation: general_assembly::extension::ieee754::ComparisonMode::Greater,
                        destination: greater_than_zero.clone(),
                        signal: false,
                    }),
                    general_assembly::operation::Operation::Or {
                        destination: c.clone(),
                        operand1: is_zero.clone(),
                        operand2: greater_than_zero.clone(),
                    },
                    general_assembly::operation::Operation::Move {
                        destination: general_assembly::operand::Operand::Flag(
                            "FPSCR.N".to_owned(),
                        ),
                        source: less_than_zero.clone(),
                    },
                    general_assembly::operation::Operation::Move {
                        destination: general_assembly::operand::Operand::Flag(
                            "FPSCR.Z".to_owned(),
                        ),
                        source: is_zero.clone(),
                    },
                    general_assembly::operation::Operation::Move {
                        destination: general_assembly::operand::Operand::Flag(
                            "FPSCR.C".to_owned(),
                        ),
                        source: c.clone(),
                    },
                    general_assembly::operation::Operation::Move {
                        destination: general_assembly::operand::Operand::Flag(
                            "FPSCR.V".to_owned(),
                        ),
                        source: general_assembly::operand::Operand::Immediate(
                            general_assembly::prelude::DataWord::Bit(false),
                        ),
                    },
                    general_assembly::operation::Operation::Compare {
                        lhs: conditional.clone().clone(),
                        rhs: general_assembly::operand::Operand::Immediate(
                                general_assembly::prelude::DataWord::Bit(false),
                            )
                            .clone(),
                        operation: general_assembly::condition::Comparison::Neq,
                        destination: intermediate_2.clone(),
                    },
                    general_assembly::operation::Operation::Ite {
                        condition: intermediate_2.clone(),
                        then: <[_]>::into_vec(
                            ::alloc::boxed::box_new([
                                general_assembly::operation::Operation::Move {
                                    destination: general_assembly::operand::Operand::Flag(
                                        "FPSCR.N".to_owned(),
                                    ),
                                    source: general_assembly::operand::Operand::Immediate(
                                        general_assembly::prelude::DataWord::Bit(false),
                                    ),
                                },
                                general_assembly::operation::Operation::Move {
                                    destination: general_assembly::operand::Operand::Flag(
                                        "FPSCR.Z".to_owned(),
                                    ),
                                    source: general_assembly::operand::Operand::Immediate(
                                        general_assembly::prelude::DataWord::Bit(false),
                                    ),
                                },
                                general_assembly::operation::Operation::Move {
                                    destination: general_assembly::operand::Operand::Flag(
                                        "FPSCR.C".to_owned(),
                                    ),
                                    source: general_assembly::operand::Operand::Immediate(
                                        general_assembly::prelude::DataWord::Bit(true),
                                    ),
                                },
                                general_assembly::operation::Operation::Move {
                                    destination: general_assembly::operand::Operand::Flag(
                                        "FPSCR.V".to_owned(),
                                    ),
                                    source: general_assembly::operand::Operand::Immediate(
                                        general_assembly::prelude::DataWord::Bit(true),
                                    ),
                                },
                            ]),
                        ),
                        otherwise: ::alloc::vec::Vec::new(),
                    },
                ]);
                ret
            }
        }
    }
    impl Decode for VcmpF64 {
        fn decode(
            &self,
            _in_it_block: bool,
        ) -> Vec<general_assembly::prelude::Operation> {
            let Self { e, dd, dm } = self;
            let e = e.unwrap_or(false);
            let dd = dd.local_into();
            let dm = dm.local_into();
            {
                let mut ret = Vec::new();
                let conditional = general_assembly::operand::Operand::Local(
                    "conditional".to_string(),
                );
                let intermediate_0 = general_assembly::operand::Operand::Local(
                    "intermediate_0".to_string(),
                );
                let intermediate_1 = general_assembly::operand::Operand::Local(
                    "intermediate_1".to_string(),
                );
                ret.extend([
                    general_assembly::operation::Operation::Nop,
                    general_assembly::operation::Operation::Nop,
                    general_assembly::operation::Operation::Ieee754(general_assembly::extension::ieee754::Operations::NonComputational {
                        operand: dm.clone(),
                        operation: general_assembly::extension::ieee754::NonComputational::IsNan,
                        destination: intermediate_0.clone(),
                    }),
                    general_assembly::operation::Operation::Ieee754(general_assembly::extension::ieee754::Operations::NonComputational {
                        operand: dd.clone(),
                        operation: general_assembly::extension::ieee754::NonComputational::IsNan,
                        destination: intermediate_1.clone(),
                    }),
                    general_assembly::operation::Operation::Or {
                        destination: conditional.clone(),
                        operand1: intermediate_1.clone(),
                        operand2: intermediate_0.clone(),
                    },
                ]);
                if e {
                    let intermediate_2 = general_assembly::operand::Operand::Local(
                        "intermediate_2".to_string(),
                    );
                    ret.extend([
                        general_assembly::operation::Operation::Compare {
                            lhs: conditional.clone().clone(),
                            rhs: general_assembly::operand::Operand::Immediate(
                                    general_assembly::prelude::DataWord::Bit(false),
                                )
                                .clone(),
                            operation: general_assembly::condition::Comparison::Neq,
                            destination: intermediate_2.clone(),
                        },
                        general_assembly::operation::Operation::Ite {
                            condition: intermediate_2.clone(),
                            then: <[_]>::into_vec(
                                ::alloc::boxed::box_new([
                                    general_assembly::operation::Operation::Abort {
                                        error: ::alloc::__export::must_use({
                                            let res = ::alloc::fmt::format(
                                                format_args!("Invalid Operation exception"),
                                            );
                                            res
                                        }),
                                    },
                                ]),
                            ),
                            otherwise: ::alloc::vec::Vec::new(),
                        },
                    ]);
                }
                let is_zero = general_assembly::operand::Operand::Local(
                    "is_zero".to_string(),
                );
                let less_than_zero = general_assembly::operand::Operand::Local(
                    "less_than_zero".to_string(),
                );
                let greater_than_zero = general_assembly::operand::Operand::Local(
                    "greater_than_zero".to_string(),
                );
                let c = general_assembly::operand::Operand::Local("c".to_string());
                let intermediate_3 = general_assembly::operand::Operand::Local(
                    "intermediate_3".to_string(),
                );
                ret.extend([
                    general_assembly::operation::Operation::Ieee754(general_assembly::extension::ieee754::Operations::Compare {
                        lhs: dd.clone(),
                        rhs: dm.clone(),
                        operation: general_assembly::extension::ieee754::ComparisonMode::Equal,
                        destination: is_zero.clone(),
                        signal: false,
                    }),
                    general_assembly::operation::Operation::Ieee754(general_assembly::extension::ieee754::Operations::Compare {
                        lhs: dd.clone(),
                        rhs: dm.clone(),
                        operation: general_assembly::extension::ieee754::ComparisonMode::Less,
                        destination: less_than_zero.clone(),
                        signal: false,
                    }),
                    general_assembly::operation::Operation::Ieee754(general_assembly::extension::ieee754::Operations::Compare {
                        lhs: dd.clone(),
                        rhs: dm.clone(),
                        operation: general_assembly::extension::ieee754::ComparisonMode::Greater,
                        destination: greater_than_zero.clone(),
                        signal: false,
                    }),
                    general_assembly::operation::Operation::Or {
                        destination: c.clone(),
                        operand1: is_zero.clone(),
                        operand2: greater_than_zero.clone(),
                    },
                    general_assembly::operation::Operation::Move {
                        destination: general_assembly::operand::Operand::Flag(
                            "FPSCR.N".to_owned(),
                        ),
                        source: less_than_zero.clone(),
                    },
                    general_assembly::operation::Operation::Move {
                        destination: general_assembly::operand::Operand::Flag(
                            "FPSCR.Z".to_owned(),
                        ),
                        source: is_zero.clone(),
                    },
                    general_assembly::operation::Operation::Move {
                        destination: general_assembly::operand::Operand::Flag(
                            "FPSCR.C".to_owned(),
                        ),
                        source: c.clone(),
                    },
                    general_assembly::operation::Operation::Move {
                        destination: general_assembly::operand::Operand::Flag(
                            "FPSCR.V".to_owned(),
                        ),
                        source: general_assembly::operand::Operand::Immediate(
                            general_assembly::prelude::DataWord::Bit(false),
                        ),
                    },
                    general_assembly::operation::Operation::Compare {
                        lhs: conditional.clone().clone(),
                        rhs: general_assembly::operand::Operand::Immediate(
                                general_assembly::prelude::DataWord::Bit(false),
                            )
                            .clone(),
                        operation: general_assembly::condition::Comparison::Neq,
                        destination: intermediate_3.clone(),
                    },
                    general_assembly::operation::Operation::Ite {
                        condition: intermediate_3.clone(),
                        then: <[_]>::into_vec(
                            ::alloc::boxed::box_new([
                                general_assembly::operation::Operation::Move {
                                    destination: general_assembly::operand::Operand::Flag(
                                        "FPSCR.N".to_owned(),
                                    ),
                                    source: general_assembly::operand::Operand::Immediate(
                                        general_assembly::prelude::DataWord::Bit(false),
                                    ),
                                },
                                general_assembly::operation::Operation::Move {
                                    destination: general_assembly::operand::Operand::Flag(
                                        "FPSCR.Z".to_owned(),
                                    ),
                                    source: general_assembly::operand::Operand::Immediate(
                                        general_assembly::prelude::DataWord::Bit(false),
                                    ),
                                },
                                general_assembly::operation::Operation::Move {
                                    destination: general_assembly::operand::Operand::Flag(
                                        "FPSCR.C".to_owned(),
                                    ),
                                    source: general_assembly::operand::Operand::Immediate(
                                        general_assembly::prelude::DataWord::Bit(true),
                                    ),
                                },
                                general_assembly::operation::Operation::Move {
                                    destination: general_assembly::operand::Operand::Flag(
                                        "FPSCR.V".to_owned(),
                                    ),
                                    source: general_assembly::operand::Operand::Immediate(
                                        general_assembly::prelude::DataWord::Bit(true),
                                    ),
                                },
                            ]),
                        ),
                        otherwise: ::alloc::vec::Vec::new(),
                    },
                ]);
                ret
            }
        }
    }
    impl Decode for VrintF32 {
        fn decode(
            &self,
            _in_it_block: bool,
        ) -> Vec<general_assembly::prelude::Operation> {
            let Self { r, sd, sm } = self;
            let r = r.unwrap_or(false);
            let sd = sd.local_into();
            let sm = sm.local_into();
            {
                let mut ret = Vec::new();
                let destination = general_assembly::operand::Operand::Local(
                    "destination".to_string(),
                );
                ret.extend([
                    general_assembly::operation::Operation::Nop,
                    general_assembly::operation::Operation::Move {
                        destination: destination.clone(),
                        source: 0.local_into(),
                    },
                ]);
                if r {
                    let intermediate_0 = general_assembly::operand::Operand::Local(
                        "intermediate_0".to_string(),
                    );
                    ret.extend([
                        general_assembly::operation::Operation::Ieee754(general_assembly::extension::ieee754::Operations::RoundToInt {
                            destination: general_assembly::extension::ieee754::Operand {
                                ty: general_assembly::extension::ieee754::OperandType::Integral {
                                    size: 32u32,
                                    signed: false,
                                }
                                    .clone(),
                                value: general_assembly::extension::ieee754::OperandStorage::CoreOperand {
                                    operand: intermediate_0.clone().clone(),
                                    ty: general_assembly::extension::ieee754::OperandType::Binary32,
                                    signed: false,
                                },
                            },
                            source: sm.clone().clone(),
                            rounding: Some(
                                general_assembly::extension::ieee754::RoundingMode::TiesTowardZero,
                            ),
                        }),
                        general_assembly::operation::Operation::Move {
                            destination: destination.clone(),
                            source: intermediate_0.clone(),
                        },
                    ]);
                } else {
                    let intermediate_1 = general_assembly::operand::Operand::Local(
                        "intermediate_1".to_string(),
                    );
                    ret.extend([
                        general_assembly::operation::Operation::Ieee754(general_assembly::extension::ieee754::Operations::RoundToInt {
                            destination: general_assembly::extension::ieee754::Operand {
                                ty: general_assembly::extension::ieee754::OperandType::Integral {
                                    size: 32u32,
                                    signed: false,
                                }
                                    .clone(),
                                value: general_assembly::extension::ieee754::OperandStorage::CoreOperand {
                                    operand: intermediate_1.clone().clone(),
                                    ty: general_assembly::extension::ieee754::OperandType::Binary32,
                                    signed: false,
                                },
                            },
                            source: sm.clone().clone(),
                            rounding: None,
                        }),
                        general_assembly::operation::Operation::Move {
                            destination: destination.clone(),
                            source: intermediate_1.clone(),
                        },
                    ]);
                };
                let intermediate_2 = general_assembly::extension::ieee754::Operand {
                    ty: general_assembly::extension::ieee754::OperandType::Binary32,
                    value: general_assembly::extension::ieee754::OperandStorage::Local(
                        "intermediate_2".to_string(),
                    ),
                };
                ret.extend([
                    general_assembly::operation::Operation::Ieee754(general_assembly::extension::ieee754::Operations::Copy {
                        destination: intermediate_2.clone(),
                        source: general_assembly::extension::ieee754::Operand {
                            ty: general_assembly::extension::ieee754::OperandType::Binary32,
                            value: general_assembly::extension::ieee754::OperandStorage::CoreOperand {
                                operand: destination.clone(),
                                ty: general_assembly::extension::ieee754::OperandType::Binary32,
                                signed: false,
                            },
                        },
                    }),
                    general_assembly::operation::Operation::Ieee754(general_assembly::extension::ieee754::Operations::Copy {
                        destination: sd.clone(),
                        source: intermediate_2.clone(),
                    }),
                ]);
                ret
            }
        }
    }
    impl Decode for VrintF64 {
        fn decode(
            &self,
            _in_it_block: bool,
        ) -> Vec<general_assembly::prelude::Operation> {
            let Self { r, dd, dm } = self;
            let r = r.unwrap_or(false);
            let dd = dd.local_into();
            let dm = dm.local_into();
            {
                let mut ret = Vec::new();
                let destination = general_assembly::operand::Operand::Local(
                    "destination".to_string(),
                );
                ret.extend([
                    general_assembly::operation::Operation::Nop,
                    general_assembly::operation::Operation::Move {
                        destination: destination.clone(),
                        source: 0.local_into(),
                    },
                ]);
                if r {
                    let intermediate_0 = general_assembly::operand::Operand::Local(
                        "intermediate_0".to_string(),
                    );
                    ret.extend([
                        general_assembly::operation::Operation::Ieee754(general_assembly::extension::ieee754::Operations::RoundToInt {
                            destination: general_assembly::extension::ieee754::Operand {
                                ty: general_assembly::extension::ieee754::OperandType::Integral {
                                    size: 64u32,
                                    signed: false,
                                }
                                    .clone(),
                                value: general_assembly::extension::ieee754::OperandStorage::CoreOperand {
                                    operand: intermediate_0.clone().clone(),
                                    ty: general_assembly::extension::ieee754::OperandType::Binary64,
                                    signed: false,
                                },
                            },
                            source: dm.clone().clone(),
                            rounding: Some(
                                general_assembly::extension::ieee754::RoundingMode::TiesTowardZero,
                            ),
                        }),
                        general_assembly::operation::Operation::Move {
                            destination: destination.clone(),
                            source: intermediate_0.clone(),
                        },
                    ]);
                } else {
                    let intermediate_1 = general_assembly::operand::Operand::Local(
                        "intermediate_1".to_string(),
                    );
                    ret.extend([
                        general_assembly::operation::Operation::Ieee754(general_assembly::extension::ieee754::Operations::RoundToInt {
                            destination: general_assembly::extension::ieee754::Operand {
                                ty: general_assembly::extension::ieee754::OperandType::Integral {
                                    size: 64u32,
                                    signed: false,
                                }
                                    .clone(),
                                value: general_assembly::extension::ieee754::OperandStorage::CoreOperand {
                                    operand: intermediate_1.clone().clone(),
                                    ty: general_assembly::extension::ieee754::OperandType::Binary64,
                                    signed: false,
                                },
                            },
                            source: dm.clone().clone(),
                            rounding: None,
                        }),
                        general_assembly::operation::Operation::Move {
                            destination: destination.clone(),
                            source: intermediate_1.clone(),
                        },
                    ]);
                };
                let intermediate_2 = general_assembly::extension::ieee754::Operand {
                    ty: general_assembly::extension::ieee754::OperandType::Binary64,
                    value: general_assembly::extension::ieee754::OperandStorage::Local(
                        "intermediate_2".to_string(),
                    ),
                };
                ret.extend([
                    general_assembly::operation::Operation::Ieee754(general_assembly::extension::ieee754::Operations::Copy {
                        destination: intermediate_2.clone(),
                        source: general_assembly::extension::ieee754::Operand {
                            ty: general_assembly::extension::ieee754::OperandType::Binary64,
                            value: general_assembly::extension::ieee754::OperandStorage::CoreOperand {
                                operand: destination.clone(),
                                ty: general_assembly::extension::ieee754::OperandType::Binary64,
                                signed: false,
                            },
                        },
                    }),
                    general_assembly::operation::Operation::Ieee754(general_assembly::extension::ieee754::Operations::Copy {
                        destination: dd.clone(),
                        source: intermediate_2.clone(),
                    }),
                ]);
                ret
            }
        }
    }
    impl Decode for Vcvt {
        fn decode(
            &self,
            _in_it_block: bool,
        ) -> Vec<general_assembly::prelude::Operation> {
            let Self { r, dest, sm, fbits } = self;
            let _ = r;
            let base = (2.0f64).powi(fbits.unwrap_or(0) as i32);
            match (dest, sm) {
                (ConversionArgument::I32(i), ConversionArgument::F32(f)) => {
                    let i = i.local_into();
                    let f = f.local_into();
                    let base = (base, OperandType::Binary32).local_into();
                    {
                        let mut ret = Vec::new();
                        let i_signed = general_assembly::operand::Operand::Local(
                            "i_signed".to_string(),
                        );
                        let intermediate_0 = general_assembly::operand::Operand::Local(
                            "intermediate_0".to_string(),
                        );
                        let i2 = general_assembly::extension::ieee754::Operand {
                            ty: general_assembly::extension::ieee754::OperandType::Binary32,
                            value: general_assembly::extension::ieee754::OperandStorage::Local(
                                "i2".to_string(),
                            ),
                        };
                        let intermediate_1 = general_assembly::extension::ieee754::Operand {
                            ty: general_assembly::extension::ieee754::OperandType::Binary32,
                            value: general_assembly::extension::ieee754::OperandStorage::Local(
                                "intermediate_1".to_string(),
                            ),
                        };
                        ret.extend([
                            general_assembly::operation::Operation::Nop,
                            general_assembly::operation::Operation::Nop,
                            general_assembly::operation::Operation::Nop,
                            general_assembly::operation::Operation::Ieee754(general_assembly::extension::ieee754::Operations::Copy {
                                source: i.clone(),
                                destination: general_assembly::extension::ieee754::Operand {
                                    ty: general_assembly::extension::ieee754::OperandType::Binary32,
                                    value: general_assembly::extension::ieee754::OperandStorage::CoreOperand {
                                        operand: intermediate_0.clone(),
                                        ty: general_assembly::extension::ieee754::OperandType::Binary32,
                                        signed: true,
                                    },
                                },
                            }),
                            general_assembly::operation::Operation::Move {
                                destination: i_signed.clone(),
                                source: intermediate_0.clone(),
                            },
                            general_assembly::operation::Operation::Log {
                                level: general_assembly::operand::LogLevel::Warn,
                                operand: i_signed.clone(),
                                meta: "vcvt to i".to_string(),
                            },
                            general_assembly::operation::Operation::Ieee754(general_assembly::extension::ieee754::Operations::ConvertFromInt {
                                destination: intermediate_1.clone().clone(),
                                operand: general_assembly::extension::ieee754::Operand {
                                    ty: general_assembly::extension::ieee754::OperandType::Integral {
                                        size: 32u32,
                                        signed: true,
                                    },
                                    value: general_assembly::extension::ieee754::OperandStorage::CoreOperand {
                                        operand: i_signed.clone(),
                                        ty: general_assembly::extension::ieee754::OperandType::Integral {
                                            size: 32u32,
                                            signed: true,
                                        },
                                        signed: true,
                                    },
                                },
                            }),
                            general_assembly::operation::Operation::Ieee754(general_assembly::extension::ieee754::Operations::Copy {
                                destination: i2.clone(),
                                source: intermediate_1.clone(),
                            }),
                            general_assembly::operation::Operation::Ieee754(general_assembly::extension::ieee754::Operations::Division {
                                destination: f.clone(),
                                nominator: i2.clone(),
                                denominator: base.clone(),
                            }),
                        ]);
                        ret
                    }
                }
                (ConversionArgument::U32(u), ConversionArgument::F32(f)) => {
                    let u = u.local_into();
                    let f = f.local_into();
                    let base = (base, OperandType::Binary32).local_into();
                    {
                        let mut ret = Vec::new();
                        let u_unsigned = general_assembly::operand::Operand::Local(
                            "u_unsigned".to_string(),
                        );
                        let intermediate_0 = general_assembly::operand::Operand::Local(
                            "intermediate_0".to_string(),
                        );
                        let u2 = general_assembly::extension::ieee754::Operand {
                            ty: general_assembly::extension::ieee754::OperandType::Binary32,
                            value: general_assembly::extension::ieee754::OperandStorage::Local(
                                "u2".to_string(),
                            ),
                        };
                        let intermediate_1 = general_assembly::extension::ieee754::Operand {
                            ty: general_assembly::extension::ieee754::OperandType::Binary32,
                            value: general_assembly::extension::ieee754::OperandStorage::Local(
                                "intermediate_1".to_string(),
                            ),
                        };
                        ret.extend([
                            general_assembly::operation::Operation::Nop,
                            general_assembly::operation::Operation::Nop,
                            general_assembly::operation::Operation::Nop,
                            general_assembly::operation::Operation::Ieee754(general_assembly::extension::ieee754::Operations::Copy {
                                source: u.clone(),
                                destination: general_assembly::extension::ieee754::Operand {
                                    ty: general_assembly::extension::ieee754::OperandType::Binary32,
                                    value: general_assembly::extension::ieee754::OperandStorage::CoreOperand {
                                        operand: intermediate_0.clone(),
                                        ty: general_assembly::extension::ieee754::OperandType::Binary32,
                                        signed: false,
                                    },
                                },
                            }),
                            general_assembly::operation::Operation::Move {
                                destination: u_unsigned.clone(),
                                source: intermediate_0.clone(),
                            },
                            general_assembly::operation::Operation::Log {
                                level: general_assembly::operand::LogLevel::Warn,
                                operand: u_unsigned.clone(),
                                meta: "vcvt to u".to_string(),
                            },
                            general_assembly::operation::Operation::Ieee754(general_assembly::extension::ieee754::Operations::ConvertFromInt {
                                destination: intermediate_1.clone().clone(),
                                operand: general_assembly::extension::ieee754::Operand {
                                    ty: general_assembly::extension::ieee754::OperandType::Integral {
                                        size: 32u32,
                                        signed: false,
                                    },
                                    value: general_assembly::extension::ieee754::OperandStorage::CoreOperand {
                                        operand: u_unsigned.clone(),
                                        ty: general_assembly::extension::ieee754::OperandType::Integral {
                                            size: 32u32,
                                            signed: false,
                                        },
                                        signed: false,
                                    },
                                },
                            }),
                            general_assembly::operation::Operation::Ieee754(general_assembly::extension::ieee754::Operations::Copy {
                                destination: u2.clone(),
                                source: intermediate_1.clone(),
                            }),
                            general_assembly::operation::Operation::Ieee754(general_assembly::extension::ieee754::Operations::Division {
                                destination: f.clone(),
                                nominator: u2.clone(),
                                denominator: base.clone(),
                            }),
                        ]);
                        ret
                    }
                }
                (ConversionArgument::F32(f), ConversionArgument::I32(i)) => {
                    let f = f.local_into();
                    let i = i.local_into();
                    let base = (base, OperandType::Binary32).local_into();
                    {
                        let mut ret = Vec::new();
                        let val = general_assembly::extension::ieee754::Operand {
                            ty: general_assembly::extension::ieee754::OperandType::Binary32,
                            value: general_assembly::extension::ieee754::OperandStorage::Local(
                                "val".to_string(),
                            ),
                        };
                        let rounded = general_assembly::operand::Operand::Local(
                            "rounded".to_string(),
                        );
                        let intermediate_0 = general_assembly::operand::Operand::Local(
                            "intermediate_0".to_string(),
                        );
                        let rounded_f32 = general_assembly::extension::ieee754::Operand {
                            ty: general_assembly::extension::ieee754::OperandType::Binary32,
                            value: general_assembly::extension::ieee754::OperandStorage::Local(
                                "rounded_f32".to_string(),
                            ),
                        };
                        let intermediate_1 = general_assembly::extension::ieee754::Operand {
                            ty: general_assembly::extension::ieee754::OperandType::Binary32,
                            value: general_assembly::extension::ieee754::OperandStorage::Local(
                                "intermediate_1".to_string(),
                            ),
                        };
                        let err = general_assembly::extension::ieee754::Operand {
                            ty: general_assembly::extension::ieee754::OperandType::Binary32,
                            value: general_assembly::extension::ieee754::OperandStorage::Local(
                                "err".to_string(),
                            ),
                        };
                        let round_up = general_assembly::operand::Operand::Local(
                            "round_up".to_string(),
                        );
                        let cond = general_assembly::operand::Operand::Local(
                            "cond".to_string(),
                        );
                        let cond2 = general_assembly::operand::Operand::Local(
                            "cond2".to_string(),
                        );
                        let cond3 = general_assembly::operand::Operand::Local(
                            "cond3".to_string(),
                        );
                        let intermediate_2 = general_assembly::operand::Operand::Local(
                            "intermediate_2".to_string(),
                        );
                        let intermediate_3 = general_assembly::operand::Operand::Local(
                            "intermediate_3".to_string(),
                        );
                        let intermediate_4 = general_assembly::operand::Operand::Local(
                            "intermediate_4".to_string(),
                        );
                        let intermediate_5 = general_assembly::operand::Operand::Local(
                            "intermediate_5".to_string(),
                        );
                        let intermediate_6 = general_assembly::operand::Operand::Local(
                            "intermediate_6".to_string(),
                        );
                        let intermediate_7 = general_assembly::operand::Operand::Local(
                            "intermediate_7".to_string(),
                        );
                        let intermediate_8 = general_assembly::extension::ieee754::Operand {
                            ty: general_assembly::extension::ieee754::OperandType::Binary32,
                            value: general_assembly::extension::ieee754::OperandStorage::Local(
                                "intermediate_8".to_string(),
                            ),
                        };
                        ret.extend([
                            general_assembly::operation::Operation::Nop,
                            general_assembly::operation::Operation::Nop,
                            general_assembly::operation::Operation::Nop,
                            general_assembly::operation::Operation::Ieee754(general_assembly::extension::ieee754::Operations::Multiplication {
                                destination: val.clone(),
                                lhs: f.clone(),
                                rhs: base.clone(),
                            }),
                            general_assembly::operation::Operation::Ieee754(general_assembly::extension::ieee754::Operations::RoundToInt {
                                destination: general_assembly::extension::ieee754::Operand {
                                    ty: general_assembly::extension::ieee754::OperandType::Integral {
                                        size: 32u32,
                                        signed: true,
                                    }
                                        .clone(),
                                    value: general_assembly::extension::ieee754::OperandStorage::CoreOperand {
                                        operand: intermediate_0.clone().clone(),
                                        ty: general_assembly::extension::ieee754::OperandType::Binary32,
                                        signed: true,
                                    },
                                },
                                source: val.clone().clone(),
                                rounding: Some(
                                    general_assembly::extension::ieee754::RoundingMode::TiesTowardZero,
                                ),
                            }),
                            general_assembly::operation::Operation::Move {
                                destination: rounded.clone(),
                                source: intermediate_0.clone(),
                            },
                            general_assembly::operation::Operation::Ieee754(general_assembly::extension::ieee754::Operations::ConvertFromInt {
                                destination: intermediate_1.clone().clone(),
                                operand: general_assembly::extension::ieee754::Operand {
                                    ty: general_assembly::extension::ieee754::OperandType::Integral {
                                        size: 32u32,
                                        signed: true,
                                    },
                                    value: general_assembly::extension::ieee754::OperandStorage::CoreOperand {
                                        operand: rounded.clone(),
                                        ty: general_assembly::extension::ieee754::OperandType::Integral {
                                            size: 32u32,
                                            signed: true,
                                        },
                                        signed: true,
                                    },
                                },
                            }),
                            general_assembly::operation::Operation::Ieee754(general_assembly::extension::ieee754::Operations::Copy {
                                destination: rounded_f32.clone(),
                                source: intermediate_1.clone(),
                            }),
                            general_assembly::operation::Operation::Ieee754(general_assembly::extension::ieee754::Operations::Subtraction {
                                destination: err.clone(),
                                lhs: val.clone(),
                                rhs: rounded_f32.clone(),
                            }),
                            general_assembly::operation::Operation::Move {
                                destination: round_up.clone(),
                                source: general_assembly::operand::Operand::Immediate(
                                    general_assembly::prelude::DataWord::Bit(false),
                                ),
                            },
                            general_assembly::operation::Operation::Move {
                                destination: cond.clone(),
                                source: general_assembly::operand::Operand::Immediate(
                                    general_assembly::prelude::DataWord::Bit(false),
                                ),
                            },
                            general_assembly::operation::Operation::Move {
                                destination: cond2.clone(),
                                source: general_assembly::operand::Operand::Immediate(
                                    general_assembly::prelude::DataWord::Bit(false),
                                ),
                            },
                            general_assembly::operation::Operation::Move {
                                destination: cond3.clone(),
                                source: general_assembly::operand::Operand::Immediate(
                                    general_assembly::prelude::DataWord::Bit(false),
                                ),
                            },
                            general_assembly::operation::Operation::Compare {
                                lhs: general_assembly::operand::Operand::Register(
                                        "FPSCR.RM".to_owned(),
                                    )
                                    .clone(),
                                rhs: general_assembly::operand::Operand::Immediate(
                                        general_assembly::prelude::DataWord::Word32(0b00u32 as u32),
                                    )
                                    .clone(),
                                operation: general_assembly::condition::Comparison::Eq,
                                destination: intermediate_4.clone(),
                            },
                            general_assembly::operation::Operation::Ite {
                                condition: intermediate_4.clone(),
                                then: <[_]>::into_vec(
                                    ::alloc::boxed::box_new([
                                        general_assembly::operation::Operation::Ieee754(general_assembly::extension::ieee754::Operations::Compare {
                                            lhs: err.clone(),
                                            rhs: general_assembly::extension::ieee754::Operand {
                                                ty: general_assembly::extension::ieee754::OperandType::Binary32,
                                                value: general_assembly::extension::ieee754::OperandStorage::Immediate {
                                                    value: 0.5f32 as f64,
                                                    ty: general_assembly::extension::ieee754::OperandType::Binary32,
                                                },
                                            },
                                            operation: general_assembly::extension::ieee754::ComparisonMode::Greater,
                                            destination: cond.clone(),
                                            signal: false,
                                        }),
                                        general_assembly::operation::Operation::Ieee754(general_assembly::extension::ieee754::Operations::Compare {
                                            lhs: err.clone(),
                                            rhs: general_assembly::extension::ieee754::Operand {
                                                ty: general_assembly::extension::ieee754::OperandType::Binary32,
                                                value: general_assembly::extension::ieee754::OperandStorage::Immediate {
                                                    value: 0.5f32 as f64,
                                                    ty: general_assembly::extension::ieee754::OperandType::Binary32,
                                                },
                                            },
                                            operation: general_assembly::extension::ieee754::ComparisonMode::Equal,
                                            destination: cond2.clone(),
                                            signal: false,
                                        }),
                                        general_assembly::operation::Operation::BitFieldExtract {
                                            destination: intermediate_3.clone().clone(),
                                            operand: rounded.clone(),
                                            start_bit: 0,
                                            stop_bit: 0,
                                        },
                                        general_assembly::operation::Operation::Resize {
                                            destination: intermediate_2.clone().clone(),
                                            operand: intermediate_3.clone().clone(),
                                            bits: 1u32.clone(),
                                        },
                                        general_assembly::operation::Operation::Move {
                                            destination: cond3.clone(),
                                            source: intermediate_2.clone(),
                                        },
                                        general_assembly::operation::Operation::And {
                                            destination: cond2.clone(),
                                            operand1: cond2.clone(),
                                            operand2: cond3.clone(),
                                        },
                                        general_assembly::operation::Operation::Or {
                                            destination: cond.clone(),
                                            operand1: cond.clone(),
                                            operand2: cond2.clone(),
                                        },
                                        general_assembly::operation::Operation::Move {
                                            destination: round_up.clone(),
                                            source: cond.clone(),
                                        },
                                    ]),
                                ),
                                otherwise: ::alloc::vec::Vec::new(),
                            },
                            general_assembly::operation::Operation::Compare {
                                lhs: general_assembly::operand::Operand::Register(
                                        "FPSCR.RM".to_owned(),
                                    )
                                    .clone(),
                                rhs: general_assembly::operand::Operand::Immediate(
                                        general_assembly::prelude::DataWord::Word32(0b01u32 as u32),
                                    )
                                    .clone(),
                                operation: general_assembly::condition::Comparison::Eq,
                                destination: intermediate_5.clone(),
                            },
                            general_assembly::operation::Operation::Ite {
                                condition: intermediate_5.clone(),
                                then: <[_]>::into_vec(
                                    ::alloc::boxed::box_new([
                                        general_assembly::operation::Operation::Ieee754(general_assembly::extension::ieee754::Operations::Compare {
                                            lhs: err.clone(),
                                            rhs: general_assembly::extension::ieee754::Operand {
                                                ty: general_assembly::extension::ieee754::OperandType::Binary32,
                                                value: general_assembly::extension::ieee754::OperandStorage::Immediate {
                                                    value: 0.0f32 as f64,
                                                    ty: general_assembly::extension::ieee754::OperandType::Binary32,
                                                },
                                            },
                                            operation: general_assembly::extension::ieee754::ComparisonMode::NotEqual,
                                            destination: cond.clone(),
                                            signal: false,
                                        }),
                                        general_assembly::operation::Operation::Move {
                                            destination: round_up.clone(),
                                            source: cond.clone(),
                                        },
                                    ]),
                                ),
                                otherwise: ::alloc::vec::Vec::new(),
                            },
                            general_assembly::operation::Operation::Compare {
                                lhs: general_assembly::operand::Operand::Register(
                                        "FPSCR.RM".to_owned(),
                                    )
                                    .clone(),
                                rhs: general_assembly::operand::Operand::Immediate(
                                        general_assembly::prelude::DataWord::Word32(0b11u32 as u32),
                                    )
                                    .clone(),
                                operation: general_assembly::condition::Comparison::Eq,
                                destination: intermediate_6.clone(),
                            },
                            general_assembly::operation::Operation::Ite {
                                condition: intermediate_6.clone(),
                                then: <[_]>::into_vec(
                                    ::alloc::boxed::box_new([
                                        general_assembly::operation::Operation::Ieee754(general_assembly::extension::ieee754::Operations::Compare {
                                            lhs: err.clone(),
                                            rhs: general_assembly::extension::ieee754::Operand {
                                                ty: general_assembly::extension::ieee754::OperandType::Binary32,
                                                value: general_assembly::extension::ieee754::OperandStorage::Immediate {
                                                    value: 0.0f32 as f64,
                                                    ty: general_assembly::extension::ieee754::OperandType::Binary32,
                                                },
                                            },
                                            operation: general_assembly::extension::ieee754::ComparisonMode::NotEqual,
                                            destination: cond.clone(),
                                            signal: false,
                                        }),
                                        general_assembly::operation::Operation::Compare {
                                            lhs: rounded.clone(),
                                            rhs: general_assembly::operand::Operand::Immediate(
                                                general_assembly::prelude::DataWord::Word32(
                                                    (0i32).cast_unsigned(),
                                                ),
                                            ),
                                            operation: general_assembly::condition::Comparison::SLt,
                                            destination: cond2.clone(),
                                        },
                                        general_assembly::operation::Operation::And {
                                            destination: cond.clone(),
                                            operand1: cond.clone(),
                                            operand2: cond2.clone(),
                                        },
                                        general_assembly::operation::Operation::Move {
                                            destination: round_up.clone(),
                                            source: cond.clone(),
                                        },
                                    ]),
                                ),
                                otherwise: ::alloc::vec::Vec::new(),
                            },
                            general_assembly::operation::Operation::Compare {
                                lhs: round_up.clone().clone(),
                                rhs: general_assembly::operand::Operand::Immediate(
                                        general_assembly::prelude::DataWord::Bit(false),
                                    )
                                    .clone(),
                                operation: general_assembly::condition::Comparison::Neq,
                                destination: intermediate_7.clone(),
                            },
                            general_assembly::operation::Operation::Ite {
                                condition: intermediate_7.clone(),
                                then: <[_]>::into_vec(
                                    ::alloc::boxed::box_new([
                                        general_assembly::operation::Operation::Add {
                                            destination: rounded.clone(),
                                            operand1: rounded.clone(),
                                            operand2: general_assembly::operand::Operand::Immediate(
                                                general_assembly::prelude::DataWord::Word32(
                                                    (1i32).cast_unsigned(),
                                                ),
                                            ),
                                        },
                                    ]),
                                ),
                                otherwise: ::alloc::vec::Vec::new(),
                            },
                            general_assembly::operation::Operation::Log {
                                level: general_assembly::operand::LogLevel::Warn,
                                operand: rounded.clone(),
                                meta: "vcvt to i".to_string(),
                            },
                            general_assembly::operation::Operation::Ieee754(general_assembly::extension::ieee754::Operations::Copy {
                                destination: intermediate_8.clone(),
                                source: general_assembly::extension::ieee754::Operand {
                                    ty: general_assembly::extension::ieee754::OperandType::Binary32,
                                    value: general_assembly::extension::ieee754::OperandStorage::CoreOperand {
                                        operand: rounded.clone(),
                                        ty: general_assembly::extension::ieee754::OperandType::Binary32,
                                        signed: true,
                                    },
                                },
                            }),
                            general_assembly::operation::Operation::Ieee754(general_assembly::extension::ieee754::Operations::Copy {
                                destination: i.clone(),
                                source: intermediate_8.clone(),
                            }),
                        ]);
                        ret
                    }
                }
                (ConversionArgument::F32(f), ConversionArgument::U32(u)) => {
                    let f = f.local_into();
                    let u = u.local_into();
                    let base = (base, OperandType::Binary32).local_into();
                    {
                        let mut ret = Vec::new();
                        let val = general_assembly::extension::ieee754::Operand {
                            ty: general_assembly::extension::ieee754::OperandType::Binary32,
                            value: general_assembly::extension::ieee754::OperandStorage::Local(
                                "val".to_string(),
                            ),
                        };
                        let rounded = general_assembly::operand::Operand::Local(
                            "rounded".to_string(),
                        );
                        let intermediate_0 = general_assembly::operand::Operand::Local(
                            "intermediate_0".to_string(),
                        );
                        let rounded_f32 = general_assembly::extension::ieee754::Operand {
                            ty: general_assembly::extension::ieee754::OperandType::Binary32,
                            value: general_assembly::extension::ieee754::OperandStorage::Local(
                                "rounded_f32".to_string(),
                            ),
                        };
                        let intermediate_1 = general_assembly::extension::ieee754::Operand {
                            ty: general_assembly::extension::ieee754::OperandType::Binary32,
                            value: general_assembly::extension::ieee754::OperandStorage::Local(
                                "intermediate_1".to_string(),
                            ),
                        };
                        let err = general_assembly::extension::ieee754::Operand {
                            ty: general_assembly::extension::ieee754::OperandType::Binary32,
                            value: general_assembly::extension::ieee754::OperandStorage::Local(
                                "err".to_string(),
                            ),
                        };
                        let round_up = general_assembly::operand::Operand::Local(
                            "round_up".to_string(),
                        );
                        let cond = general_assembly::operand::Operand::Local(
                            "cond".to_string(),
                        );
                        let cond2 = general_assembly::operand::Operand::Local(
                            "cond2".to_string(),
                        );
                        let cond3 = general_assembly::operand::Operand::Local(
                            "cond3".to_string(),
                        );
                        let intermediate_2 = general_assembly::operand::Operand::Local(
                            "intermediate_2".to_string(),
                        );
                        let intermediate_3 = general_assembly::operand::Operand::Local(
                            "intermediate_3".to_string(),
                        );
                        let intermediate_4 = general_assembly::operand::Operand::Local(
                            "intermediate_4".to_string(),
                        );
                        let intermediate_5 = general_assembly::operand::Operand::Local(
                            "intermediate_5".to_string(),
                        );
                        let intermediate_6 = general_assembly::operand::Operand::Local(
                            "intermediate_6".to_string(),
                        );
                        let intermediate_7 = general_assembly::operand::Operand::Local(
                            "intermediate_7".to_string(),
                        );
                        let intermediate_8 = general_assembly::extension::ieee754::Operand {
                            ty: general_assembly::extension::ieee754::OperandType::Binary32,
                            value: general_assembly::extension::ieee754::OperandStorage::Local(
                                "intermediate_8".to_string(),
                            ),
                        };
                        ret.extend([
                            general_assembly::operation::Operation::Nop,
                            general_assembly::operation::Operation::Nop,
                            general_assembly::operation::Operation::Nop,
                            general_assembly::operation::Operation::Ieee754(general_assembly::extension::ieee754::Operations::Multiplication {
                                destination: val.clone(),
                                lhs: f.clone(),
                                rhs: base.clone(),
                            }),
                            general_assembly::operation::Operation::Ieee754(general_assembly::extension::ieee754::Operations::RoundToInt {
                                destination: general_assembly::extension::ieee754::Operand {
                                    ty: general_assembly::extension::ieee754::OperandType::Integral {
                                        size: 32u32,
                                        signed: false,
                                    }
                                        .clone(),
                                    value: general_assembly::extension::ieee754::OperandStorage::CoreOperand {
                                        operand: intermediate_0.clone().clone(),
                                        ty: general_assembly::extension::ieee754::OperandType::Binary32,
                                        signed: false,
                                    },
                                },
                                source: val.clone().clone(),
                                rounding: Some(
                                    general_assembly::extension::ieee754::RoundingMode::TiesTowardZero,
                                ),
                            }),
                            general_assembly::operation::Operation::Move {
                                destination: rounded.clone(),
                                source: intermediate_0.clone(),
                            },
                            general_assembly::operation::Operation::Ieee754(general_assembly::extension::ieee754::Operations::ConvertFromInt {
                                destination: intermediate_1.clone().clone(),
                                operand: general_assembly::extension::ieee754::Operand {
                                    ty: general_assembly::extension::ieee754::OperandType::Integral {
                                        size: 32u32,
                                        signed: false,
                                    },
                                    value: general_assembly::extension::ieee754::OperandStorage::CoreOperand {
                                        operand: rounded.clone(),
                                        ty: general_assembly::extension::ieee754::OperandType::Integral {
                                            size: 32u32,
                                            signed: false,
                                        },
                                        signed: false,
                                    },
                                },
                            }),
                            general_assembly::operation::Operation::Ieee754(general_assembly::extension::ieee754::Operations::Copy {
                                destination: rounded_f32.clone(),
                                source: intermediate_1.clone(),
                            }),
                            general_assembly::operation::Operation::Ieee754(general_assembly::extension::ieee754::Operations::Subtraction {
                                destination: err.clone(),
                                lhs: val.clone(),
                                rhs: rounded_f32.clone(),
                            }),
                            general_assembly::operation::Operation::Move {
                                destination: round_up.clone(),
                                source: general_assembly::operand::Operand::Immediate(
                                    general_assembly::prelude::DataWord::Bit(false),
                                ),
                            },
                            general_assembly::operation::Operation::Move {
                                destination: cond.clone(),
                                source: general_assembly::operand::Operand::Immediate(
                                    general_assembly::prelude::DataWord::Bit(false),
                                ),
                            },
                            general_assembly::operation::Operation::Move {
                                destination: cond2.clone(),
                                source: general_assembly::operand::Operand::Immediate(
                                    general_assembly::prelude::DataWord::Bit(false),
                                ),
                            },
                            general_assembly::operation::Operation::Move {
                                destination: cond3.clone(),
                                source: general_assembly::operand::Operand::Immediate(
                                    general_assembly::prelude::DataWord::Bit(false),
                                ),
                            },
                            general_assembly::operation::Operation::Compare {
                                lhs: general_assembly::operand::Operand::Register(
                                        "FPSCR.RM".to_owned(),
                                    )
                                    .clone(),
                                rhs: general_assembly::operand::Operand::Immediate(
                                        general_assembly::prelude::DataWord::Word32(0b00u32 as u32),
                                    )
                                    .clone(),
                                operation: general_assembly::condition::Comparison::Eq,
                                destination: intermediate_4.clone(),
                            },
                            general_assembly::operation::Operation::Ite {
                                condition: intermediate_4.clone(),
                                then: <[_]>::into_vec(
                                    ::alloc::boxed::box_new([
                                        general_assembly::operation::Operation::Ieee754(general_assembly::extension::ieee754::Operations::Compare {
                                            lhs: err.clone(),
                                            rhs: general_assembly::extension::ieee754::Operand {
                                                ty: general_assembly::extension::ieee754::OperandType::Binary32,
                                                value: general_assembly::extension::ieee754::OperandStorage::Immediate {
                                                    value: 0.5f32 as f64,
                                                    ty: general_assembly::extension::ieee754::OperandType::Binary32,
                                                },
                                            },
                                            operation: general_assembly::extension::ieee754::ComparisonMode::Greater,
                                            destination: cond.clone(),
                                            signal: false,
                                        }),
                                        general_assembly::operation::Operation::Ieee754(general_assembly::extension::ieee754::Operations::Compare {
                                            lhs: err.clone(),
                                            rhs: general_assembly::extension::ieee754::Operand {
                                                ty: general_assembly::extension::ieee754::OperandType::Binary32,
                                                value: general_assembly::extension::ieee754::OperandStorage::Immediate {
                                                    value: 0.5f32 as f64,
                                                    ty: general_assembly::extension::ieee754::OperandType::Binary32,
                                                },
                                            },
                                            operation: general_assembly::extension::ieee754::ComparisonMode::Equal,
                                            destination: cond2.clone(),
                                            signal: false,
                                        }),
                                        general_assembly::operation::Operation::BitFieldExtract {
                                            destination: intermediate_3.clone().clone(),
                                            operand: rounded.clone(),
                                            start_bit: 0,
                                            stop_bit: 0,
                                        },
                                        general_assembly::operation::Operation::Resize {
                                            destination: intermediate_2.clone().clone(),
                                            operand: intermediate_3.clone().clone(),
                                            bits: 1u32.clone(),
                                        },
                                        general_assembly::operation::Operation::Move {
                                            destination: cond3.clone(),
                                            source: intermediate_2.clone(),
                                        },
                                        general_assembly::operation::Operation::And {
                                            destination: cond2.clone(),
                                            operand1: cond2.clone(),
                                            operand2: cond3.clone(),
                                        },
                                        general_assembly::operation::Operation::Or {
                                            destination: cond.clone(),
                                            operand1: cond.clone(),
                                            operand2: cond2.clone(),
                                        },
                                        general_assembly::operation::Operation::Move {
                                            destination: round_up.clone(),
                                            source: cond.clone(),
                                        },
                                    ]),
                                ),
                                otherwise: ::alloc::vec::Vec::new(),
                            },
                            general_assembly::operation::Operation::Compare {
                                lhs: general_assembly::operand::Operand::Register(
                                        "FPSCR.RM".to_owned(),
                                    )
                                    .clone(),
                                rhs: general_assembly::operand::Operand::Immediate(
                                        general_assembly::prelude::DataWord::Word32(0b01u32 as u32),
                                    )
                                    .clone(),
                                operation: general_assembly::condition::Comparison::Eq,
                                destination: intermediate_5.clone(),
                            },
                            general_assembly::operation::Operation::Ite {
                                condition: intermediate_5.clone(),
                                then: <[_]>::into_vec(
                                    ::alloc::boxed::box_new([
                                        general_assembly::operation::Operation::Ieee754(general_assembly::extension::ieee754::Operations::Compare {
                                            lhs: err.clone(),
                                            rhs: general_assembly::extension::ieee754::Operand {
                                                ty: general_assembly::extension::ieee754::OperandType::Binary32,
                                                value: general_assembly::extension::ieee754::OperandStorage::Immediate {
                                                    value: 0.0f32 as f64,
                                                    ty: general_assembly::extension::ieee754::OperandType::Binary32,
                                                },
                                            },
                                            operation: general_assembly::extension::ieee754::ComparisonMode::NotEqual,
                                            destination: cond.clone(),
                                            signal: false,
                                        }),
                                        general_assembly::operation::Operation::Move {
                                            destination: round_up.clone(),
                                            source: cond.clone(),
                                        },
                                    ]),
                                ),
                                otherwise: ::alloc::vec::Vec::new(),
                            },
                            general_assembly::operation::Operation::Compare {
                                lhs: general_assembly::operand::Operand::Register(
                                        "FPSCR.RM".to_owned(),
                                    )
                                    .clone(),
                                rhs: general_assembly::operand::Operand::Immediate(
                                        general_assembly::prelude::DataWord::Word32(0b11u32 as u32),
                                    )
                                    .clone(),
                                operation: general_assembly::condition::Comparison::Eq,
                                destination: intermediate_6.clone(),
                            },
                            general_assembly::operation::Operation::Ite {
                                condition: intermediate_6.clone(),
                                then: <[_]>::into_vec(
                                    ::alloc::boxed::box_new([
                                        general_assembly::operation::Operation::Ieee754(general_assembly::extension::ieee754::Operations::Compare {
                                            lhs: err.clone(),
                                            rhs: general_assembly::extension::ieee754::Operand {
                                                ty: general_assembly::extension::ieee754::OperandType::Binary32,
                                                value: general_assembly::extension::ieee754::OperandStorage::Immediate {
                                                    value: 0.0f32 as f64,
                                                    ty: general_assembly::extension::ieee754::OperandType::Binary32,
                                                },
                                            },
                                            operation: general_assembly::extension::ieee754::ComparisonMode::NotEqual,
                                            destination: cond.clone(),
                                            signal: false,
                                        }),
                                        general_assembly::operation::Operation::Compare {
                                            lhs: rounded.clone(),
                                            rhs: general_assembly::operand::Operand::Immediate(
                                                general_assembly::prelude::DataWord::Word32(0u32 as u32),
                                            ),
                                            operation: general_assembly::condition::Comparison::ULt,
                                            destination: cond2.clone(),
                                        },
                                        general_assembly::operation::Operation::And {
                                            destination: cond.clone(),
                                            operand1: cond.clone(),
                                            operand2: cond2.clone(),
                                        },
                                        general_assembly::operation::Operation::Move {
                                            destination: round_up.clone(),
                                            source: cond.clone(),
                                        },
                                    ]),
                                ),
                                otherwise: ::alloc::vec::Vec::new(),
                            },
                            general_assembly::operation::Operation::Compare {
                                lhs: round_up.clone().clone(),
                                rhs: general_assembly::operand::Operand::Immediate(
                                        general_assembly::prelude::DataWord::Bit(false),
                                    )
                                    .clone(),
                                operation: general_assembly::condition::Comparison::Neq,
                                destination: intermediate_7.clone(),
                            },
                            general_assembly::operation::Operation::Ite {
                                condition: intermediate_7.clone(),
                                then: <[_]>::into_vec(
                                    ::alloc::boxed::box_new([
                                        general_assembly::operation::Operation::Add {
                                            destination: rounded.clone(),
                                            operand1: rounded.clone(),
                                            operand2: general_assembly::operand::Operand::Immediate(
                                                general_assembly::prelude::DataWord::Word32(1u32 as u32),
                                            ),
                                        },
                                    ]),
                                ),
                                otherwise: ::alloc::vec::Vec::new(),
                            },
                            general_assembly::operation::Operation::Ieee754(general_assembly::extension::ieee754::Operations::Copy {
                                destination: intermediate_8.clone(),
                                source: general_assembly::extension::ieee754::Operand {
                                    ty: general_assembly::extension::ieee754::OperandType::Binary32,
                                    value: general_assembly::extension::ieee754::OperandStorage::CoreOperand {
                                        operand: rounded.clone(),
                                        ty: general_assembly::extension::ieee754::OperandType::Binary32,
                                        signed: false,
                                    },
                                },
                            }),
                            general_assembly::operation::Operation::Ieee754(general_assembly::extension::ieee754::Operations::Copy {
                                destination: u.clone(),
                                source: intermediate_8.clone(),
                            }),
                        ]);
                        ret
                    }
                }
                _ => ::core::panicking::panic("not yet implemented"),
            }
        }
    }
    impl Decode for VcvtF32F64 {
        fn decode(
            &self,
            _in_it_block: bool,
        ) -> Vec<general_assembly::prelude::Operation> {
            let Self { sd, dm } = self;
            let sd = sd.local_into();
            let dm = dm.local_into();
            {
                let mut ret = Vec::new();
                let intermediate_0 = general_assembly::extension::ieee754::Operand {
                    ty: general_assembly::extension::ieee754::OperandType::Binary64,
                    value: general_assembly::extension::ieee754::OperandStorage::Local(
                        "intermediate_0".to_string(),
                    ),
                };
                ret.extend([
                    general_assembly::operation::Operation::Nop,
                    general_assembly::operation::Operation::Nop,
                    general_assembly::operation::Operation::Ieee754(general_assembly::extension::ieee754::Operations::Convert {
                        destination: intermediate_0.clone().clone(),
                        source: sd.clone().clone(),
                        rounding: None,
                    }),
                    general_assembly::operation::Operation::Ieee754(general_assembly::extension::ieee754::Operations::Copy {
                        destination: dm.clone(),
                        source: intermediate_0.clone(),
                    }),
                ]);
                ret
            }
        }
    }
    impl Decode for VcvtF64F32 {
        fn decode(
            &self,
            _in_it_block: bool,
        ) -> Vec<general_assembly::prelude::Operation> {
            let Self { dd, sm } = self;
            let dd = dd.local_into();
            let sm = sm.local_into();
            {
                let mut ret = Vec::new();
                let intermediate_0 = general_assembly::extension::ieee754::Operand {
                    ty: general_assembly::extension::ieee754::OperandType::Binary32,
                    value: general_assembly::extension::ieee754::OperandStorage::Local(
                        "intermediate_0".to_string(),
                    ),
                };
                ret.extend([
                    general_assembly::operation::Operation::Nop,
                    general_assembly::operation::Operation::Nop,
                    general_assembly::operation::Operation::Ieee754(general_assembly::extension::ieee754::Operations::Convert {
                        destination: intermediate_0.clone().clone(),
                        source: dd.clone().clone(),
                        rounding: None,
                    }),
                    general_assembly::operation::Operation::Ieee754(general_assembly::extension::ieee754::Operations::Copy {
                        destination: sm.clone(),
                        source: intermediate_0.clone(),
                    }),
                ]);
                ret
            }
        }
    }
    impl Decode for VrintCustomRoundingF32 {
        fn decode(
            &self,
            _in_it_block: bool,
        ) -> Vec<general_assembly::prelude::Operation> {
            let Self { r, sd, sm } = self;
            let r = r.local_into();
            let sd = sd.local_into();
            let sm = sm.local_into();
            {
                let mut ret = Vec::new();
                let result = general_assembly::operand::Operand::Local(
                    "result".to_string(),
                );
                let intermediate_0 = general_assembly::operand::Operand::Local(
                    "intermediate_0".to_string(),
                );
                let intermediate_1 = general_assembly::extension::ieee754::Operand {
                    ty: general_assembly::extension::ieee754::OperandType::Binary32,
                    value: general_assembly::extension::ieee754::OperandStorage::Local(
                        "intermediate_1".to_string(),
                    ),
                };
                ret.extend([
                    general_assembly::operation::Operation::Nop,
                    general_assembly::operation::Operation::Ieee754(general_assembly::extension::ieee754::Operations::RoundToInt {
                        destination: general_assembly::extension::ieee754::Operand {
                            ty: general_assembly::extension::ieee754::OperandType::Integral {
                                size: 32u32,
                                signed: false,
                            }
                                .clone(),
                            value: general_assembly::extension::ieee754::OperandStorage::CoreOperand {
                                operand: intermediate_0.clone().clone(),
                                ty: general_assembly::extension::ieee754::OperandType::Binary32,
                                signed: false,
                            },
                        },
                        source: sm.clone().clone(),
                        rounding: Some(r),
                    }),
                    general_assembly::operation::Operation::Move {
                        destination: result.clone(),
                        source: intermediate_0.clone(),
                    },
                    general_assembly::operation::Operation::Ieee754(general_assembly::extension::ieee754::Operations::Copy {
                        destination: intermediate_1.clone(),
                        source: general_assembly::extension::ieee754::Operand {
                            ty: general_assembly::extension::ieee754::OperandType::Binary32,
                            value: general_assembly::extension::ieee754::OperandStorage::CoreOperand {
                                operand: result.clone(),
                                ty: general_assembly::extension::ieee754::OperandType::Binary32,
                                signed: false,
                            },
                        },
                    }),
                    general_assembly::operation::Operation::Ieee754(general_assembly::extension::ieee754::Operations::Copy {
                        destination: sd.clone(),
                        source: intermediate_1.clone(),
                    }),
                ]);
                ret
            }
        }
    }
    impl Decode for VrintCustomRoundingF64 {
        fn decode(
            &self,
            _in_it_block: bool,
        ) -> Vec<general_assembly::prelude::Operation> {
            let Self { r, dd, dm } = self;
            let r = r.local_into();
            let dd = dd.local_into();
            let dm = dm.local_into();
            {
                let mut ret = Vec::new();
                let result = general_assembly::operand::Operand::Local(
                    "result".to_string(),
                );
                let intermediate_0 = general_assembly::operand::Operand::Local(
                    "intermediate_0".to_string(),
                );
                let intermediate_1 = general_assembly::extension::ieee754::Operand {
                    ty: general_assembly::extension::ieee754::OperandType::Binary64,
                    value: general_assembly::extension::ieee754::OperandStorage::Local(
                        "intermediate_1".to_string(),
                    ),
                };
                ret.extend([
                    general_assembly::operation::Operation::Nop,
                    general_assembly::operation::Operation::Nop,
                    general_assembly::operation::Operation::Ieee754(general_assembly::extension::ieee754::Operations::RoundToInt {
                        destination: general_assembly::extension::ieee754::Operand {
                            ty: general_assembly::extension::ieee754::OperandType::Integral {
                                size: 64u32,
                                signed: false,
                            }
                                .clone(),
                            value: general_assembly::extension::ieee754::OperandStorage::CoreOperand {
                                operand: intermediate_0.clone().clone(),
                                ty: general_assembly::extension::ieee754::OperandType::Binary64,
                                signed: false,
                            },
                        },
                        source: dm.clone().clone(),
                        rounding: Some(r),
                    }),
                    general_assembly::operation::Operation::Move {
                        destination: result.clone(),
                        source: intermediate_0.clone(),
                    },
                    general_assembly::operation::Operation::Ieee754(general_assembly::extension::ieee754::Operations::Copy {
                        destination: intermediate_1.clone(),
                        source: general_assembly::extension::ieee754::Operand {
                            ty: general_assembly::extension::ieee754::OperandType::Binary64,
                            value: general_assembly::extension::ieee754::OperandStorage::CoreOperand {
                                operand: result.clone(),
                                ty: general_assembly::extension::ieee754::OperandType::Binary64,
                                signed: false,
                            },
                        },
                    }),
                    general_assembly::operation::Operation::Ieee754(general_assembly::extension::ieee754::Operations::Copy {
                        destination: dd.clone(),
                        source: intermediate_1.clone(),
                    }),
                ]);
                ret
            }
        }
    }
    impl Decode for VcvtCustomRoundingIntF32 {
        fn decode(
            &self,
            _in_it_block: bool,
        ) -> Vec<general_assembly::prelude::Operation> {
            let Self { r, sd, sm } = self;
            let r = r.local_into();
            let sd_inner = sd.clone().local_into();
            let sm = sm.local_into();
            {
                let mut ret = Vec::new();
                ret.extend([
                    general_assembly::operation::Operation::Nop,
                    general_assembly::operation::Operation::Nop,
                ]);
                if let IntType::U32(_reg) = sd {
                    let result = general_assembly::operand::Operand::Local(
                        "result".to_string(),
                    );
                    let intermediate_0 = general_assembly::operand::Operand::Local(
                        "intermediate_0".to_string(),
                    );
                    let intermediate_1 = general_assembly::extension::ieee754::Operand {
                        ty: general_assembly::extension::ieee754::OperandType::Binary32,
                        value: general_assembly::extension::ieee754::OperandStorage::Local(
                            "intermediate_1".to_string(),
                        ),
                    };
                    ret.extend([
                        general_assembly::operation::Operation::Ieee754(general_assembly::extension::ieee754::Operations::RoundToInt {
                            destination: general_assembly::extension::ieee754::Operand {
                                ty: general_assembly::extension::ieee754::OperandType::Integral {
                                    size: 32u32,
                                    signed: false,
                                }
                                    .clone(),
                                value: general_assembly::extension::ieee754::OperandStorage::CoreOperand {
                                    operand: intermediate_0.clone().clone(),
                                    ty: general_assembly::extension::ieee754::OperandType::Binary32,
                                    signed: false,
                                },
                            },
                            source: sm.clone().clone(),
                            rounding: Some(r),
                        }),
                        general_assembly::operation::Operation::Move {
                            destination: result.clone(),
                            source: intermediate_0.clone(),
                        },
                        general_assembly::operation::Operation::Ieee754(general_assembly::extension::ieee754::Operations::Copy {
                            destination: intermediate_1.clone(),
                            source: general_assembly::extension::ieee754::Operand {
                                ty: general_assembly::extension::ieee754::OperandType::Binary32,
                                value: general_assembly::extension::ieee754::OperandStorage::CoreOperand {
                                    operand: result.clone(),
                                    ty: general_assembly::extension::ieee754::OperandType::Binary32,
                                    signed: false,
                                },
                            },
                        }),
                        general_assembly::operation::Operation::Ieee754(general_assembly::extension::ieee754::Operations::Copy {
                            destination: sd_inner.clone(),
                            source: intermediate_1.clone(),
                        }),
                    ]);
                } else {
                    let result = general_assembly::operand::Operand::Local(
                        "result".to_string(),
                    );
                    let intermediate_2 = general_assembly::operand::Operand::Local(
                        "intermediate_2".to_string(),
                    );
                    let intermediate_3 = general_assembly::extension::ieee754::Operand {
                        ty: general_assembly::extension::ieee754::OperandType::Binary32,
                        value: general_assembly::extension::ieee754::OperandStorage::Local(
                            "intermediate_3".to_string(),
                        ),
                    };
                    ret.extend([
                        general_assembly::operation::Operation::Ieee754(general_assembly::extension::ieee754::Operations::RoundToInt {
                            destination: general_assembly::extension::ieee754::Operand {
                                ty: general_assembly::extension::ieee754::OperandType::Integral {
                                    size: 32u32,
                                    signed: true,
                                }
                                    .clone(),
                                value: general_assembly::extension::ieee754::OperandStorage::CoreOperand {
                                    operand: intermediate_2.clone().clone(),
                                    ty: general_assembly::extension::ieee754::OperandType::Binary32,
                                    signed: true,
                                },
                            },
                            source: sm.clone().clone(),
                            rounding: Some(r),
                        }),
                        general_assembly::operation::Operation::Move {
                            destination: result.clone(),
                            source: intermediate_2.clone(),
                        },
                        general_assembly::operation::Operation::Ieee754(general_assembly::extension::ieee754::Operations::Copy {
                            destination: intermediate_3.clone(),
                            source: general_assembly::extension::ieee754::Operand {
                                ty: general_assembly::extension::ieee754::OperandType::Binary32,
                                value: general_assembly::extension::ieee754::OperandStorage::CoreOperand {
                                    operand: result.clone(),
                                    ty: general_assembly::extension::ieee754::OperandType::Binary32,
                                    signed: true,
                                },
                            },
                        }),
                        general_assembly::operation::Operation::Ieee754(general_assembly::extension::ieee754::Operations::Copy {
                            destination: sd_inner.clone(),
                            source: intermediate_3.clone(),
                        }),
                    ]);
                };
                ret
            }
        }
    }
    impl Decode for VcvtCustomRoundingIntF64 {
        fn decode(
            &self,
            _in_it_block: bool,
        ) -> Vec<general_assembly::prelude::Operation> {
            let Self { r, sd, dm } = self;
            let r = r.local_into();
            let sd_inner = sd.clone().local_into();
            let dm = dm.local_into();
            {
                let mut ret = Vec::new();
                ret.extend([
                    general_assembly::operation::Operation::Nop,
                    general_assembly::operation::Operation::Nop,
                ]);
                if let IntType::U32(_reg) = sd {
                    let result = general_assembly::operand::Operand::Local(
                        "result".to_string(),
                    );
                    let intermediate_0 = general_assembly::operand::Operand::Local(
                        "intermediate_0".to_string(),
                    );
                    let intermediate_1 = general_assembly::extension::ieee754::Operand {
                        ty: general_assembly::extension::ieee754::OperandType::Binary32,
                        value: general_assembly::extension::ieee754::OperandStorage::Local(
                            "intermediate_1".to_string(),
                        ),
                    };
                    ret.extend([
                        general_assembly::operation::Operation::Ieee754(general_assembly::extension::ieee754::Operations::RoundToInt {
                            destination: general_assembly::extension::ieee754::Operand {
                                ty: general_assembly::extension::ieee754::OperandType::Integral {
                                    size: 32u32,
                                    signed: false,
                                }
                                    .clone(),
                                value: general_assembly::extension::ieee754::OperandStorage::CoreOperand {
                                    operand: intermediate_0.clone().clone(),
                                    ty: general_assembly::extension::ieee754::OperandType::Binary64,
                                    signed: false,
                                },
                            },
                            source: dm.clone().clone(),
                            rounding: Some(r),
                        }),
                        general_assembly::operation::Operation::Move {
                            destination: result.clone(),
                            source: intermediate_0.clone(),
                        },
                        general_assembly::operation::Operation::Ieee754(general_assembly::extension::ieee754::Operations::Copy {
                            destination: intermediate_1.clone(),
                            source: general_assembly::extension::ieee754::Operand {
                                ty: general_assembly::extension::ieee754::OperandType::Binary32,
                                value: general_assembly::extension::ieee754::OperandStorage::CoreOperand {
                                    operand: result.clone(),
                                    ty: general_assembly::extension::ieee754::OperandType::Binary32,
                                    signed: false,
                                },
                            },
                        }),
                        general_assembly::operation::Operation::Ieee754(general_assembly::extension::ieee754::Operations::Copy {
                            destination: sd_inner.clone(),
                            source: intermediate_1.clone(),
                        }),
                    ]);
                } else {
                    let result = general_assembly::operand::Operand::Local(
                        "result".to_string(),
                    );
                    let intermediate_2 = general_assembly::operand::Operand::Local(
                        "intermediate_2".to_string(),
                    );
                    let intermediate_3 = general_assembly::extension::ieee754::Operand {
                        ty: general_assembly::extension::ieee754::OperandType::Binary32,
                        value: general_assembly::extension::ieee754::OperandStorage::Local(
                            "intermediate_3".to_string(),
                        ),
                    };
                    ret.extend([
                        general_assembly::operation::Operation::Ieee754(general_assembly::extension::ieee754::Operations::RoundToInt {
                            destination: general_assembly::extension::ieee754::Operand {
                                ty: general_assembly::extension::ieee754::OperandType::Integral {
                                    size: 32u32,
                                    signed: true,
                                }
                                    .clone(),
                                value: general_assembly::extension::ieee754::OperandStorage::CoreOperand {
                                    operand: intermediate_2.clone().clone(),
                                    ty: general_assembly::extension::ieee754::OperandType::Binary64,
                                    signed: true,
                                },
                            },
                            source: dm.clone().clone(),
                            rounding: Some(r),
                        }),
                        general_assembly::operation::Operation::Move {
                            destination: result.clone(),
                            source: intermediate_2.clone(),
                        },
                        general_assembly::operation::Operation::Ieee754(general_assembly::extension::ieee754::Operations::Copy {
                            destination: intermediate_3.clone(),
                            source: general_assembly::extension::ieee754::Operand {
                                ty: general_assembly::extension::ieee754::OperandType::Binary32,
                                value: general_assembly::extension::ieee754::OperandStorage::CoreOperand {
                                    operand: result.clone(),
                                    ty: general_assembly::extension::ieee754::OperandType::Binary32,
                                    signed: true,
                                },
                            },
                        }),
                        general_assembly::operation::Operation::Ieee754(general_assembly::extension::ieee754::Operations::Copy {
                            destination: sd_inner.clone(),
                            source: intermediate_3.clone(),
                        }),
                    ]);
                };
                ret
            }
        }
    }
    impl Decode for VStmF32 {
        fn decode(
            &self,
            _in_it_block: bool,
        ) -> Vec<general_assembly::prelude::Operation> {
            let Self { add, wback, imm32, rn, registers } = self;
            let registers: Vec<_> = registers.iter().map(|el| el.local_into()).collect();
            let rn = rn.local_into();
            let imm32 = imm32.local_into();
            {
                let mut ret = Vec::new();
                let address = general_assembly::operand::Operand::Local(
                    "address".to_string(),
                );
                ret.extend([
                    general_assembly::operation::Operation::Nop,
                    general_assembly::operation::Operation::Move {
                        destination: address.clone(),
                        source: rn.clone(),
                    },
                ]);
                if *add {
                    ret.extend([
                        general_assembly::operation::Operation::Add {
                            destination: address.clone(),
                            operand1: address.clone(),
                            operand2: imm32.clone(),
                        },
                    ]);
                } else {
                    ret.extend([
                        general_assembly::operation::Operation::Sub {
                            destination: address.clone(),
                            operand1: address.clone(),
                            operand2: imm32.clone(),
                        },
                    ]);
                };
                if *wback {
                    if *add {
                        ret.extend([
                            general_assembly::operation::Operation::Add {
                                destination: rn.clone(),
                                operand1: rn.clone(),
                                operand2: imm32.clone(),
                            },
                        ]);
                    } else {
                        ret.extend([
                            general_assembly::operation::Operation::Sub {
                                destination: rn.clone(),
                                operand1: rn.clone(),
                                operand2: imm32.clone(),
                            },
                        ]);
                    };
                }
                for register in registers.into_iter() {
                    let intermediate_0 = general_assembly::operand::Operand::Local(
                        "intermediate_0".to_string(),
                    );
                    ret.extend([
                        general_assembly::operation::Operation::Nop,
                        general_assembly::operation::Operation::Ieee754(general_assembly::extension::ieee754::Operations::Copy {
                            source: register.clone(),
                            destination: general_assembly::extension::ieee754::Operand {
                                ty: general_assembly::extension::ieee754::OperandType::Binary32,
                                value: general_assembly::extension::ieee754::OperandStorage::CoreOperand {
                                    operand: intermediate_0.clone(),
                                    ty: general_assembly::extension::ieee754::OperandType::Binary32,
                                    signed: false,
                                },
                            },
                        }),
                        general_assembly::operation::Operation::Move {
                            destination: general_assembly::operand::Operand::AddressInLocal(
                                "address".to_owned(),
                                32u32,
                            ),
                            source: intermediate_0.clone(),
                        },
                        general_assembly::operation::Operation::Add {
                            destination: address.clone(),
                            operand1: address.clone(),
                            operand2: 4.local_into(),
                        },
                    ]);
                }
                ret
            }
        }
    }
    impl Decode for VStmF64 {
        fn decode(
            &self,
            _in_it_block: bool,
        ) -> Vec<general_assembly::prelude::Operation> {
            let Self { add, wback, imm32, rn, registers } = self;
            let registers: Vec<_> = registers.iter().map(|el| el.local_into()).collect();
            let rn = rn.local_into();
            let imm32 = imm32.local_into();
            {
                let mut ret = Vec::new();
                let address = general_assembly::operand::Operand::Local(
                    "address".to_string(),
                );
                ret.extend([
                    general_assembly::operation::Operation::Nop,
                    general_assembly::operation::Operation::Move {
                        destination: address.clone(),
                        source: rn.clone(),
                    },
                ]);
                if *add {
                    ret.extend([
                        general_assembly::operation::Operation::Add {
                            destination: address.clone(),
                            operand1: address.clone(),
                            operand2: imm32.clone(),
                        },
                    ]);
                } else {
                    ret.extend([
                        general_assembly::operation::Operation::Sub {
                            destination: address.clone(),
                            operand1: address.clone(),
                            operand2: imm32.clone(),
                        },
                    ]);
                };
                if *wback {
                    if *add {
                        ret.extend([
                            general_assembly::operation::Operation::Add {
                                destination: rn.clone(),
                                operand1: rn.clone(),
                                operand2: imm32.clone(),
                            },
                        ]);
                    } else {
                        ret.extend([
                            general_assembly::operation::Operation::Sub {
                                destination: rn.clone(),
                                operand1: rn.clone(),
                                operand2: imm32.clone(),
                            },
                        ]);
                    };
                }
                for register in registers.into_iter() {
                    let intermediate_0 = general_assembly::operand::Operand::Local(
                        "intermediate_0".to_string(),
                    );
                    ret.extend([
                        general_assembly::operation::Operation::Nop,
                        general_assembly::operation::Operation::Ieee754(general_assembly::extension::ieee754::Operations::Copy {
                            source: register.clone(),
                            destination: general_assembly::extension::ieee754::Operand {
                                ty: general_assembly::extension::ieee754::OperandType::Binary64,
                                value: general_assembly::extension::ieee754::OperandStorage::CoreOperand {
                                    operand: intermediate_0.clone(),
                                    ty: general_assembly::extension::ieee754::OperandType::Binary64,
                                    signed: false,
                                },
                            },
                        }),
                        general_assembly::operation::Operation::Move {
                            destination: general_assembly::operand::Operand::AddressInLocal(
                                "address".to_owned(),
                                64u32,
                            ),
                            source: intermediate_0.clone(),
                        },
                        general_assembly::operation::Operation::Add {
                            destination: address.clone(),
                            operand1: address.clone(),
                            operand2: 8.local_into(),
                        },
                    ]);
                }
                ret
            }
        }
    }
    impl Decode for VStrF32 {
        fn decode(
            &self,
            _in_it_block: bool,
        ) -> Vec<general_assembly::prelude::Operation> {
            let Self { add, imm32, rn, sd } = self;
            let imm32 = imm32.local_into();
            let rn = rn.local_into();
            let sd = sd.local_into();
            {
                let mut ret = Vec::new();
                let address = general_assembly::operand::Operand::Local(
                    "address".to_string(),
                );
                ret.extend([
                    general_assembly::operation::Operation::Nop,
                    general_assembly::operation::Operation::Nop,
                    general_assembly::operation::Operation::Move {
                        destination: address.clone(),
                        source: rn.clone(),
                    },
                ]);
                if *add {
                    ret.extend([
                        general_assembly::operation::Operation::Add {
                            destination: address.clone(),
                            operand1: address.clone(),
                            operand2: imm32.clone(),
                        },
                    ]);
                } else {
                    ret.extend([
                        general_assembly::operation::Operation::Sub {
                            destination: address.clone(),
                            operand1: address.clone(),
                            operand2: imm32.clone(),
                        },
                    ]);
                };
                let intermediate_0 = general_assembly::operand::Operand::Local(
                    "intermediate_0".to_string(),
                );
                ret.extend([
                    general_assembly::operation::Operation::Ieee754(general_assembly::extension::ieee754::Operations::Copy {
                        source: sd.clone(),
                        destination: general_assembly::extension::ieee754::Operand {
                            ty: general_assembly::extension::ieee754::OperandType::Binary32,
                            value: general_assembly::extension::ieee754::OperandStorage::CoreOperand {
                                operand: intermediate_0.clone(),
                                ty: general_assembly::extension::ieee754::OperandType::Binary32,
                                signed: false,
                            },
                        },
                    }),
                    general_assembly::operation::Operation::Move {
                        destination: general_assembly::operand::Operand::AddressInLocal(
                            "address".to_owned(),
                            32u32,
                        ),
                        source: intermediate_0.clone(),
                    },
                ]);
                ret
            }
        }
    }
    impl Decode for VStrF64 {
        fn decode(
            &self,
            _in_it_block: bool,
        ) -> Vec<general_assembly::prelude::Operation> {
            let Self { add, imm32, rn, dd } = self;
            let imm32 = imm32.local_into();
            let rn = rn.local_into();
            let dd = dd.local_into();
            {
                let mut ret = Vec::new();
                let address = general_assembly::operand::Operand::Local(
                    "address".to_string(),
                );
                ret.extend([
                    general_assembly::operation::Operation::Nop,
                    general_assembly::operation::Operation::Nop,
                    general_assembly::operation::Operation::Move {
                        destination: address.clone(),
                        source: rn.clone(),
                    },
                ]);
                if *add {
                    ret.extend([
                        general_assembly::operation::Operation::Add {
                            destination: address.clone(),
                            operand1: address.clone(),
                            operand2: imm32.clone(),
                        },
                    ]);
                } else {
                    ret.extend([
                        general_assembly::operation::Operation::Sub {
                            destination: address.clone(),
                            operand1: address.clone(),
                            operand2: imm32.clone(),
                        },
                    ]);
                };
                let intermediate_0 = general_assembly::operand::Operand::Local(
                    "intermediate_0".to_string(),
                );
                ret.extend([
                    general_assembly::operation::Operation::Ieee754(general_assembly::extension::ieee754::Operations::Copy {
                        source: dd.clone(),
                        destination: general_assembly::extension::ieee754::Operand {
                            ty: general_assembly::extension::ieee754::OperandType::Binary64,
                            value: general_assembly::extension::ieee754::OperandStorage::CoreOperand {
                                operand: intermediate_0.clone(),
                                ty: general_assembly::extension::ieee754::OperandType::Binary64,
                                signed: false,
                            },
                        },
                    }),
                    general_assembly::operation::Operation::Move {
                        destination: general_assembly::operand::Operand::AddressInLocal(
                            "address".to_owned(),
                            64u32,
                        ),
                        source: intermediate_0.clone(),
                    },
                ]);
                ret
            }
        }
    }
    impl Decode for VPushF32 {
        fn decode(
            &self,
            _in_it_block: bool,
        ) -> Vec<general_assembly::prelude::Operation> {
            let Self { imm32, registers } = self;
            let imm32 = imm32.local_into();
            let registers: Vec<_> = registers.iter().map(|el| el.local_into()).collect();
            {
                let mut ret = Vec::new();
                let address = general_assembly::operand::Operand::Local(
                    "address".to_string(),
                );
                ret.extend([
                    general_assembly::operation::Operation::Sub {
                        destination: address.clone(),
                        operand1: general_assembly::operand::Operand::Register(
                            "SP&".to_owned(),
                        ),
                        operand2: imm32.clone(),
                    },
                    general_assembly::operation::Operation::Sub {
                        destination: general_assembly::operand::Operand::Register(
                            "SP&".to_owned(),
                        ),
                        operand1: general_assembly::operand::Operand::Register(
                            "SP&".to_owned(),
                        ),
                        operand2: imm32.clone(),
                    },
                ]);
                for register in registers.into_iter() {
                    let intermediate_0 = general_assembly::operand::Operand::Local(
                        "intermediate_0".to_string(),
                    );
                    ret.extend([
                        general_assembly::operation::Operation::Nop,
                        general_assembly::operation::Operation::Ieee754(general_assembly::extension::ieee754::Operations::Copy {
                            source: register.clone(),
                            destination: general_assembly::extension::ieee754::Operand {
                                ty: general_assembly::extension::ieee754::OperandType::Binary32,
                                value: general_assembly::extension::ieee754::OperandStorage::CoreOperand {
                                    operand: intermediate_0.clone(),
                                    ty: general_assembly::extension::ieee754::OperandType::Binary32,
                                    signed: false,
                                },
                            },
                        }),
                        general_assembly::operation::Operation::Move {
                            destination: general_assembly::operand::Operand::AddressInLocal(
                                "address".to_owned(),
                                32u32,
                            ),
                            source: intermediate_0.clone(),
                        },
                        general_assembly::operation::Operation::Add {
                            destination: address.clone(),
                            operand1: address.clone(),
                            operand2: 4.local_into(),
                        },
                    ]);
                }
                ret
            }
        }
    }
    impl Decode for VPushF64 {
        fn decode(
            &self,
            _in_it_block: bool,
        ) -> Vec<general_assembly::prelude::Operation> {
            let Self { imm32, registers } = self;
            let imm32 = imm32.local_into();
            let registers: Vec<_> = registers.iter().map(|el| el.local_into()).collect();
            {
                let mut ret = Vec::new();
                let address = general_assembly::operand::Operand::Local(
                    "address".to_string(),
                );
                ret.extend([
                    general_assembly::operation::Operation::Sub {
                        destination: address.clone(),
                        operand1: general_assembly::operand::Operand::Register(
                            "SP&".to_owned(),
                        ),
                        operand2: imm32.clone(),
                    },
                    general_assembly::operation::Operation::Sub {
                        destination: general_assembly::operand::Operand::Register(
                            "SP&".to_owned(),
                        ),
                        operand1: general_assembly::operand::Operand::Register(
                            "SP&".to_owned(),
                        ),
                        operand2: imm32.clone(),
                    },
                ]);
                for register in registers.into_iter() {
                    let intermediate_0 = general_assembly::operand::Operand::Local(
                        "intermediate_0".to_string(),
                    );
                    ret.extend([
                        general_assembly::operation::Operation::Nop,
                        general_assembly::operation::Operation::Ieee754(general_assembly::extension::ieee754::Operations::Copy {
                            source: register.clone(),
                            destination: general_assembly::extension::ieee754::Operand {
                                ty: general_assembly::extension::ieee754::OperandType::Binary64,
                                value: general_assembly::extension::ieee754::OperandStorage::CoreOperand {
                                    operand: intermediate_0.clone(),
                                    ty: general_assembly::extension::ieee754::OperandType::Binary64,
                                    signed: false,
                                },
                            },
                        }),
                        general_assembly::operation::Operation::Move {
                            destination: general_assembly::operand::Operand::AddressInLocal(
                                "address".to_owned(),
                                64u32,
                            ),
                            source: intermediate_0.clone(),
                        },
                        general_assembly::operation::Operation::Add {
                            destination: address.clone(),
                            operand1: address.clone(),
                            operand2: 8.local_into(),
                        },
                    ]);
                }
                ret
            }
        }
    }
    impl Decode for VLdrF32 {
        fn decode(
            &self,
            _in_it_block: bool,
        ) -> Vec<general_assembly::prelude::Operation> {
            let Self { add, imm32, rn, sd } = self;
            let imm32 = imm32.local_into();
            let rn = rn.local_into();
            let sd = sd.local_into();
            {
                let mut ret = Vec::new();
                let base = general_assembly::operand::Operand::Local("base".to_string());
                let address = general_assembly::operand::Operand::Local(
                    "address".to_string(),
                );
                ret.extend([
                    general_assembly::operation::Operation::Nop,
                    general_assembly::operation::Operation::Nop,
                    general_assembly::operation::Operation::Move {
                        destination: base.clone(),
                        source: rn.clone(),
                    },
                    general_assembly::operation::Operation::Move {
                        destination: address.clone(),
                        source: base.clone(),
                    },
                ]);
                if *add {
                    ret.extend([
                        general_assembly::operation::Operation::Add {
                            destination: address.clone(),
                            operand1: address.clone(),
                            operand2: imm32.clone(),
                        },
                    ]);
                } else {
                    ret.extend([
                        general_assembly::operation::Operation::Sub {
                            destination: address.clone(),
                            operand1: address.clone(),
                            operand2: imm32.clone(),
                        },
                    ]);
                };
                let val = general_assembly::operand::Operand::Local("val".to_string());
                let intermediate_0 = general_assembly::extension::ieee754::Operand {
                    ty: general_assembly::extension::ieee754::OperandType::Binary32,
                    value: general_assembly::extension::ieee754::OperandStorage::Local(
                        "intermediate_0".to_string(),
                    ),
                };
                ret.extend([
                    general_assembly::operation::Operation::Move {
                        destination: val.clone(),
                        source: general_assembly::operand::Operand::AddressInLocal(
                            "address".to_owned(),
                            32u32,
                        ),
                    },
                    general_assembly::operation::Operation::Ieee754(general_assembly::extension::ieee754::Operations::Copy {
                        destination: intermediate_0.clone(),
                        source: general_assembly::extension::ieee754::Operand {
                            ty: general_assembly::extension::ieee754::OperandType::Binary32,
                            value: general_assembly::extension::ieee754::OperandStorage::CoreOperand {
                                operand: val.clone(),
                                ty: general_assembly::extension::ieee754::OperandType::Binary32,
                                signed: false,
                            },
                        },
                    }),
                    general_assembly::operation::Operation::Ieee754(general_assembly::extension::ieee754::Operations::Copy {
                        destination: sd.clone(),
                        source: intermediate_0.clone(),
                    }),
                ]);
                ret
            }
        }
    }
    impl Decode for VLdrF64 {
        fn decode(
            &self,
            _in_it_block: bool,
        ) -> Vec<general_assembly::prelude::Operation> {
            let Self { add, imm32, rn, dd } = self;
            let imm32 = imm32.local_into();
            let rn = rn.local_into();
            let dd = dd.local_into();
            {
                let mut ret = Vec::new();
                let base = general_assembly::operand::Operand::Local("base".to_string());
                let address = general_assembly::operand::Operand::Local(
                    "address".to_string(),
                );
                ret.extend([
                    general_assembly::operation::Operation::Nop,
                    general_assembly::operation::Operation::Nop,
                    general_assembly::operation::Operation::Move {
                        destination: base.clone(),
                        source: rn.clone(),
                    },
                    general_assembly::operation::Operation::Move {
                        destination: address.clone(),
                        source: base.clone(),
                    },
                ]);
                if *add {
                    ret.extend([
                        general_assembly::operation::Operation::Add {
                            destination: address.clone(),
                            operand1: address.clone(),
                            operand2: imm32.clone(),
                        },
                    ]);
                } else {
                    ret.extend([
                        general_assembly::operation::Operation::Sub {
                            destination: address.clone(),
                            operand1: address.clone(),
                            operand2: imm32.clone(),
                        },
                    ]);
                };
                let val = general_assembly::operand::Operand::Local("val".to_string());
                let intermediate_0 = general_assembly::extension::ieee754::Operand {
                    ty: general_assembly::extension::ieee754::OperandType::Binary64,
                    value: general_assembly::extension::ieee754::OperandStorage::Local(
                        "intermediate_0".to_string(),
                    ),
                };
                ret.extend([
                    general_assembly::operation::Operation::Move {
                        destination: val.clone(),
                        source: general_assembly::operand::Operand::AddressInLocal(
                            "address".to_owned(),
                            64u32,
                        ),
                    },
                    general_assembly::operation::Operation::Ieee754(general_assembly::extension::ieee754::Operations::Copy {
                        destination: intermediate_0.clone(),
                        source: general_assembly::extension::ieee754::Operand {
                            ty: general_assembly::extension::ieee754::OperandType::Binary64,
                            value: general_assembly::extension::ieee754::OperandStorage::CoreOperand {
                                operand: val.clone(),
                                ty: general_assembly::extension::ieee754::OperandType::Binary64,
                                signed: false,
                            },
                        },
                    }),
                    general_assembly::operation::Operation::Ieee754(general_assembly::extension::ieee754::Operations::Copy {
                        destination: dd.clone(),
                        source: intermediate_0.clone(),
                    }),
                ]);
                ret
            }
        }
    }
    impl Decode for VPopF32 {
        fn decode(
            &self,
            _in_it_block: bool,
        ) -> Vec<general_assembly::prelude::Operation> {
            let Self { imm32, registers } = self;
            let imm32 = imm32.local_into();
            let registers: Vec<_> = registers.iter().map(|el| el.local_into()).collect();
            {
                let mut ret = Vec::new();
                let address = general_assembly::operand::Operand::Local(
                    "address".to_string(),
                );
                ret.extend([
                    general_assembly::operation::Operation::Move {
                        destination: address.clone(),
                        source: general_assembly::operand::Operand::Register(
                            "SP&".to_owned(),
                        ),
                    },
                    general_assembly::operation::Operation::Add {
                        destination: general_assembly::operand::Operand::Register(
                            "SP&".to_owned(),
                        ),
                        operand1: general_assembly::operand::Operand::Register(
                            "SP&".to_owned(),
                        ),
                        operand2: imm32.clone(),
                    },
                ]);
                for register in registers.into_iter() {
                    let value = general_assembly::operand::Operand::Local(
                        "value".to_string(),
                    );
                    let intermediate_0 = general_assembly::extension::ieee754::Operand {
                        ty: general_assembly::extension::ieee754::OperandType::Binary32,
                        value: general_assembly::extension::ieee754::OperandStorage::Local(
                            "intermediate_0".to_string(),
                        ),
                    };
                    ret.extend([
                        general_assembly::operation::Operation::Move {
                            destination: value.clone(),
                            source: general_assembly::operand::Operand::AddressInLocal(
                                "address".to_owned(),
                                32u32,
                            ),
                        },
                        general_assembly::operation::Operation::Ieee754(general_assembly::extension::ieee754::Operations::Copy {
                            destination: intermediate_0.clone(),
                            source: general_assembly::extension::ieee754::Operand {
                                ty: general_assembly::extension::ieee754::OperandType::Binary32,
                                value: general_assembly::extension::ieee754::OperandStorage::CoreOperand {
                                    operand: value.clone(),
                                    ty: general_assembly::extension::ieee754::OperandType::Binary32,
                                    signed: false,
                                },
                            },
                        }),
                        general_assembly::operation::Operation::Ieee754(general_assembly::extension::ieee754::Operations::Copy {
                            destination: register.clone(),
                            source: intermediate_0.clone(),
                        }),
                        general_assembly::operation::Operation::Add {
                            destination: address.clone(),
                            operand1: address.clone(),
                            operand2: 4.local_into(),
                        },
                    ]);
                }
                ret
            }
        }
    }
    impl Decode for VPopF64 {
        fn decode(
            &self,
            _in_it_block: bool,
        ) -> Vec<general_assembly::prelude::Operation> {
            let Self { imm32, registers } = self;
            let imm32 = imm32.local_into();
            let registers: Vec<_> = registers.iter().map(|el| el.local_into()).collect();
            {
                let mut ret = Vec::new();
                let address = general_assembly::operand::Operand::Local(
                    "address".to_string(),
                );
                ret.extend([
                    general_assembly::operation::Operation::Move {
                        destination: address.clone(),
                        source: general_assembly::operand::Operand::Register(
                            "SP&".to_owned(),
                        ),
                    },
                    general_assembly::operation::Operation::Add {
                        destination: general_assembly::operand::Operand::Register(
                            "SP&".to_owned(),
                        ),
                        operand1: general_assembly::operand::Operand::Register(
                            "SP&".to_owned(),
                        ),
                        operand2: imm32.clone(),
                    },
                ]);
                for register in registers.into_iter() {
                    let value = general_assembly::operand::Operand::Local(
                        "value".to_string(),
                    );
                    let intermediate_0 = general_assembly::extension::ieee754::Operand {
                        ty: general_assembly::extension::ieee754::OperandType::Binary64,
                        value: general_assembly::extension::ieee754::OperandStorage::Local(
                            "intermediate_0".to_string(),
                        ),
                    };
                    ret.extend([
                        general_assembly::operation::Operation::Move {
                            destination: value.clone(),
                            source: general_assembly::operand::Operand::AddressInLocal(
                                "address".to_owned(),
                                64u32,
                            ),
                        },
                        general_assembly::operation::Operation::Ieee754(general_assembly::extension::ieee754::Operations::Copy {
                            destination: intermediate_0.clone(),
                            source: general_assembly::extension::ieee754::Operand {
                                ty: general_assembly::extension::ieee754::OperandType::Binary64,
                                value: general_assembly::extension::ieee754::OperandStorage::CoreOperand {
                                    operand: value.clone(),
                                    ty: general_assembly::extension::ieee754::OperandType::Binary64,
                                    signed: false,
                                },
                            },
                        }),
                        general_assembly::operation::Operation::Ieee754(general_assembly::extension::ieee754::Operations::Copy {
                            destination: register.clone(),
                            source: intermediate_0.clone(),
                        }),
                        general_assembly::operation::Operation::Add {
                            destination: address.clone(),
                            operand1: address.clone(),
                            operand2: 8.local_into(),
                        },
                    ]);
                }
                ret
            }
        }
    }
    impl Decode for VLdmF32 {
        fn decode(
            &self,
            _in_it_block: bool,
        ) -> Vec<general_assembly::prelude::Operation> {
            let Self { add, wback, imm32, rn, registers } = self;
            let imm32 = imm32.local_into();
            let rn = rn.local_into();
            let registers: Vec<_> = registers.iter().map(|el| el.local_into()).collect();
            {
                let mut ret = Vec::new();
                let address = general_assembly::operand::Operand::Local(
                    "address".to_string(),
                );
                ret.extend([
                    general_assembly::operation::Operation::Nop,
                    general_assembly::operation::Operation::Nop,
                    general_assembly::operation::Operation::Move {
                        destination: address.clone(),
                        source: rn.clone(),
                    },
                ]);
                if *add {
                    ret.extend([
                        general_assembly::operation::Operation::Sub {
                            destination: address.clone(),
                            operand1: address.clone(),
                            operand2: imm32.clone(),
                        },
                    ]);
                }
                if *wback {
                    if *add {
                        ret.extend([
                            general_assembly::operation::Operation::Add {
                                destination: rn.clone(),
                                operand1: rn.clone(),
                                operand2: imm32.clone(),
                            },
                        ]);
                    } else {
                        ret.extend([
                            general_assembly::operation::Operation::Sub {
                                destination: rn.clone(),
                                operand1: rn.clone(),
                                operand2: imm32.clone(),
                            },
                        ]);
                    };
                }
                for register in registers.iter() {
                    let value = general_assembly::operand::Operand::Local(
                        "value".to_string(),
                    );
                    let intermediate_0 = general_assembly::extension::ieee754::Operand {
                        ty: general_assembly::extension::ieee754::OperandType::Binary32,
                        value: general_assembly::extension::ieee754::OperandStorage::Local(
                            "intermediate_0".to_string(),
                        ),
                    };
                    ret.extend([
                        general_assembly::operation::Operation::Nop,
                        general_assembly::operation::Operation::Move {
                            destination: value.clone(),
                            source: general_assembly::operand::Operand::AddressInLocal(
                                "address".to_owned(),
                                32u32,
                            ),
                        },
                        general_assembly::operation::Operation::Add {
                            destination: address.clone(),
                            operand1: address.clone(),
                            operand2: 4.local_into(),
                        },
                        general_assembly::operation::Operation::Ieee754(general_assembly::extension::ieee754::Operations::Copy {
                            destination: intermediate_0.clone(),
                            source: general_assembly::extension::ieee754::Operand {
                                ty: general_assembly::extension::ieee754::OperandType::Binary32,
                                value: general_assembly::extension::ieee754::OperandStorage::CoreOperand {
                                    operand: value.clone(),
                                    ty: general_assembly::extension::ieee754::OperandType::Binary32,
                                    signed: false,
                                },
                            },
                        }),
                        general_assembly::operation::Operation::Ieee754(general_assembly::extension::ieee754::Operations::Copy {
                            destination: register.clone(),
                            source: intermediate_0.clone(),
                        }),
                    ]);
                }
                ret
            }
        }
    }
    impl Decode for VLdmF64 {
        fn decode(
            &self,
            _in_it_block: bool,
        ) -> Vec<general_assembly::prelude::Operation> {
            let Self { add, wback, imm32, rn, registers } = self;
            let imm32 = imm32.local_into();
            let rn = rn.local_into();
            let registers: Vec<_> = registers.iter().map(|el| el.local_into()).collect();
            {
                let mut ret = Vec::new();
                let address = general_assembly::operand::Operand::Local(
                    "address".to_string(),
                );
                ret.extend([
                    general_assembly::operation::Operation::Nop,
                    general_assembly::operation::Operation::Nop,
                    general_assembly::operation::Operation::Move {
                        destination: address.clone(),
                        source: rn.clone(),
                    },
                ]);
                if *add {
                    ret.extend([
                        general_assembly::operation::Operation::Sub {
                            destination: address.clone(),
                            operand1: address.clone(),
                            operand2: imm32.clone(),
                        },
                    ]);
                }
                if *wback {
                    if *add {
                        ret.extend([
                            general_assembly::operation::Operation::Add {
                                destination: rn.clone(),
                                operand1: rn.clone(),
                                operand2: imm32.clone(),
                            },
                        ]);
                    } else {
                        ret.extend([
                            general_assembly::operation::Operation::Sub {
                                destination: rn.clone(),
                                operand1: rn.clone(),
                                operand2: imm32.clone(),
                            },
                        ]);
                    };
                }
                for register in registers.iter() {
                    let value = general_assembly::operand::Operand::Local(
                        "value".to_string(),
                    );
                    let intermediate_0 = general_assembly::extension::ieee754::Operand {
                        ty: general_assembly::extension::ieee754::OperandType::Binary64,
                        value: general_assembly::extension::ieee754::OperandStorage::Local(
                            "intermediate_0".to_string(),
                        ),
                    };
                    ret.extend([
                        general_assembly::operation::Operation::Nop,
                        general_assembly::operation::Operation::Move {
                            destination: value.clone(),
                            source: general_assembly::operand::Operand::AddressInLocal(
                                "address".to_owned(),
                                64u32,
                            ),
                        },
                        general_assembly::operation::Operation::Add {
                            destination: address.clone(),
                            operand1: address.clone(),
                            operand2: 8.local_into(),
                        },
                        general_assembly::operation::Operation::Ieee754(general_assembly::extension::ieee754::Operations::Copy {
                            destination: intermediate_0.clone(),
                            source: general_assembly::extension::ieee754::Operand {
                                ty: general_assembly::extension::ieee754::OperandType::Binary64,
                                value: general_assembly::extension::ieee754::OperandStorage::CoreOperand {
                                    operand: value.clone(),
                                    ty: general_assembly::extension::ieee754::OperandType::Binary64,
                                    signed: false,
                                },
                            },
                        }),
                        general_assembly::operation::Operation::Ieee754(general_assembly::extension::ieee754::Operations::Copy {
                            destination: register.clone(),
                            source: intermediate_0.clone(),
                        }),
                    ]);
                }
                ret
            }
        }
    }
    impl Decode for VmoveF32 {
        fn decode(
            &self,
            _in_it_block: bool,
        ) -> Vec<general_assembly::prelude::Operation> {
            let Self { to_core, sn, rt } = self;
            let sn = sn.local_into();
            let rt = rt.local_into();
            {
                let mut ret = Vec::new();
                ret.extend([
                    general_assembly::operation::Operation::Nop,
                    general_assembly::operation::Operation::Nop,
                ]);
                if *to_core {
                    let intermediate_0 = general_assembly::operand::Operand::Local(
                        "intermediate_0".to_string(),
                    );
                    ret.extend([
                        general_assembly::operation::Operation::Ieee754(general_assembly::extension::ieee754::Operations::Copy {
                            source: sn.clone(),
                            destination: general_assembly::extension::ieee754::Operand {
                                ty: general_assembly::extension::ieee754::OperandType::Binary32,
                                value: general_assembly::extension::ieee754::OperandStorage::CoreOperand {
                                    operand: intermediate_0.clone(),
                                    ty: general_assembly::extension::ieee754::OperandType::Binary32,
                                    signed: false,
                                },
                            },
                        }),
                        general_assembly::operation::Operation::Move {
                            destination: rt.clone(),
                            source: intermediate_0.clone(),
                        },
                    ]);
                } else {
                    let intermediate_1 = general_assembly::extension::ieee754::Operand {
                        ty: general_assembly::extension::ieee754::OperandType::Binary32,
                        value: general_assembly::extension::ieee754::OperandStorage::Local(
                            "intermediate_1".to_string(),
                        ),
                    };
                    ret.extend([
                        general_assembly::operation::Operation::Ieee754(general_assembly::extension::ieee754::Operations::Copy {
                            destination: intermediate_1.clone(),
                            source: general_assembly::extension::ieee754::Operand {
                                ty: general_assembly::extension::ieee754::OperandType::Binary32,
                                value: general_assembly::extension::ieee754::OperandStorage::CoreOperand {
                                    operand: rt.clone(),
                                    ty: general_assembly::extension::ieee754::OperandType::Binary32,
                                    signed: false,
                                },
                            },
                        }),
                        general_assembly::operation::Operation::Ieee754(general_assembly::extension::ieee754::Operations::Copy {
                            destination: sn.clone(),
                            source: intermediate_1.clone(),
                        }),
                    ]);
                };
                ret
            }
        }
    }
    impl Decode for VmoveF64 {
        fn decode(
            &self,
            _in_it_block: bool,
        ) -> Vec<general_assembly::prelude::Operation> {
            let Self { to_core, rt, rt2, dm } = self;
            let dm = dm.local_into();
            let rt = rt.local_into();
            let rt2 = rt2.local_into();
            {
                let mut ret = Vec::new();
                ret.extend([
                    general_assembly::operation::Operation::Nop,
                    general_assembly::operation::Operation::Nop,
                    general_assembly::operation::Operation::Nop,
                ]);
                if *to_core {
                    let value = general_assembly::operand::Operand::Local(
                        "value".to_string(),
                    );
                    let intermediate_0 = general_assembly::operand::Operand::Local(
                        "intermediate_0".to_string(),
                    );
                    let intermediate = general_assembly::operand::Operand::Local(
                        "intermediate".to_string(),
                    );
                    let intermediate_1 = general_assembly::operand::Operand::Local(
                        "intermediate_1".to_string(),
                    );
                    let intermediate_2 = general_assembly::operand::Operand::Local(
                        "intermediate_2".to_string(),
                    );
                    let intermediate_3 = general_assembly::operand::Operand::Local(
                        "intermediate_3".to_string(),
                    );
                    let intermediate_4 = general_assembly::operand::Operand::Local(
                        "intermediate_4".to_string(),
                    );
                    ret.extend([
                        general_assembly::operation::Operation::Ieee754(general_assembly::extension::ieee754::Operations::Copy {
                            source: dm.clone(),
                            destination: general_assembly::extension::ieee754::Operand {
                                ty: general_assembly::extension::ieee754::OperandType::Binary64,
                                value: general_assembly::extension::ieee754::OperandStorage::CoreOperand {
                                    operand: intermediate_0.clone(),
                                    ty: general_assembly::extension::ieee754::OperandType::Binary64,
                                    signed: false,
                                },
                            },
                        }),
                        general_assembly::operation::Operation::Move {
                            destination: value.clone(),
                            source: intermediate_0.clone(),
                        },
                        general_assembly::operation::Operation::BitFieldExtract {
                            destination: intermediate_1.clone().clone(),
                            operand: value.clone(),
                            start_bit: 0,
                            stop_bit: 31,
                        },
                        general_assembly::operation::Operation::Move {
                            destination: intermediate.clone(),
                            source: intermediate_1.clone(),
                        },
                        general_assembly::operation::Operation::Resize {
                            destination: intermediate_2.clone().clone(),
                            operand: intermediate.clone().clone(),
                            bits: 32u32.clone(),
                        },
                        general_assembly::operation::Operation::Move {
                            destination: rt.clone(),
                            source: intermediate_2.clone(),
                        },
                        general_assembly::operation::Operation::BitFieldExtract {
                            destination: intermediate_3.clone().clone(),
                            operand: value.clone(),
                            start_bit: 32,
                            stop_bit: 63,
                        },
                        general_assembly::operation::Operation::Move {
                            destination: intermediate.clone(),
                            source: intermediate_3.clone(),
                        },
                        general_assembly::operation::Operation::Resize {
                            destination: intermediate_4.clone().clone(),
                            operand: intermediate.clone().clone(),
                            bits: 32u32.clone(),
                        },
                        general_assembly::operation::Operation::Move {
                            destination: rt2.clone(),
                            source: intermediate_4.clone(),
                        },
                    ]);
                } else {
                    let value = general_assembly::operand::Operand::Local(
                        "value".to_string(),
                    );
                    let intermediate_5 = general_assembly::operand::Operand::Local(
                        "intermediate_5".to_string(),
                    );
                    let intermediate = general_assembly::operand::Operand::Local(
                        "intermediate".to_string(),
                    );
                    let intermediate_6 = general_assembly::operand::Operand::Local(
                        "intermediate_6".to_string(),
                    );
                    let intermediate_7 = general_assembly::extension::ieee754::Operand {
                        ty: general_assembly::extension::ieee754::OperandType::Binary64,
                        value: general_assembly::extension::ieee754::OperandStorage::Local(
                            "intermediate_7".to_string(),
                        ),
                    };
                    ret.extend([
                        general_assembly::operation::Operation::Resize {
                            destination: intermediate_5.clone().clone(),
                            operand: rt.clone().clone(),
                            bits: 64u32.clone(),
                        },
                        general_assembly::operation::Operation::Move {
                            destination: value.clone(),
                            source: intermediate_5.clone(),
                        },
                        general_assembly::operation::Operation::Resize {
                            destination: intermediate_6.clone().clone(),
                            operand: rt2.clone().clone(),
                            bits: 64u32.clone(),
                        },
                        general_assembly::operation::Operation::Sl {
                            destination: intermediate.clone(),
                            operand: intermediate_6.clone(),
                            shift: 32.local_into(),
                        },
                        general_assembly::operation::Operation::Or {
                            destination: value.clone(),
                            operand1: value.clone(),
                            operand2: intermediate.clone(),
                        },
                        general_assembly::operation::Operation::Ieee754(general_assembly::extension::ieee754::Operations::Copy {
                            destination: intermediate_7.clone(),
                            source: general_assembly::extension::ieee754::Operand {
                                ty: general_assembly::extension::ieee754::OperandType::Binary64,
                                value: general_assembly::extension::ieee754::OperandStorage::CoreOperand {
                                    operand: value.clone(),
                                    ty: general_assembly::extension::ieee754::OperandType::Binary64,
                                    signed: false,
                                },
                            },
                        }),
                        general_assembly::operation::Operation::Ieee754(general_assembly::extension::ieee754::Operations::Copy {
                            destination: dm.clone(),
                            source: intermediate_7.clone(),
                        }),
                    ]);
                };
                ret
            }
        }
    }
    impl Decode for VmoveDoubleF32 {
        fn decode(
            &self,
            _in_it_block: bool,
        ) -> Vec<general_assembly::prelude::Operation> {
            let Self { to_core, rt, rt2, sm, sm1 } = self;
            let rt = rt.local_into();
            let rt2 = rt2.local_into();
            let sm = sm.local_into();
            let sm1 = sm1.local_into();
            {
                let mut ret = Vec::new();
                ret.extend([
                    general_assembly::operation::Operation::Nop,
                    general_assembly::operation::Operation::Nop,
                    general_assembly::operation::Operation::Nop,
                    general_assembly::operation::Operation::Nop,
                ]);
                if *to_core {
                    let intermediate_0 = general_assembly::operand::Operand::Local(
                        "intermediate_0".to_string(),
                    );
                    let intermediate_1 = general_assembly::operand::Operand::Local(
                        "intermediate_1".to_string(),
                    );
                    ret.extend([
                        general_assembly::operation::Operation::Ieee754(general_assembly::extension::ieee754::Operations::Copy {
                            source: sm.clone(),
                            destination: general_assembly::extension::ieee754::Operand {
                                ty: general_assembly::extension::ieee754::OperandType::Binary32,
                                value: general_assembly::extension::ieee754::OperandStorage::CoreOperand {
                                    operand: intermediate_0.clone(),
                                    ty: general_assembly::extension::ieee754::OperandType::Binary32,
                                    signed: false,
                                },
                            },
                        }),
                        general_assembly::operation::Operation::Move {
                            destination: rt.clone(),
                            source: intermediate_0.clone(),
                        },
                        general_assembly::operation::Operation::Ieee754(general_assembly::extension::ieee754::Operations::Copy {
                            source: sm1.clone(),
                            destination: general_assembly::extension::ieee754::Operand {
                                ty: general_assembly::extension::ieee754::OperandType::Binary32,
                                value: general_assembly::extension::ieee754::OperandStorage::CoreOperand {
                                    operand: intermediate_1.clone(),
                                    ty: general_assembly::extension::ieee754::OperandType::Binary32,
                                    signed: false,
                                },
                            },
                        }),
                        general_assembly::operation::Operation::Move {
                            destination: rt2.clone(),
                            source: intermediate_1.clone(),
                        },
                    ]);
                } else {
                    let intermediate_2 = general_assembly::extension::ieee754::Operand {
                        ty: general_assembly::extension::ieee754::OperandType::Binary32,
                        value: general_assembly::extension::ieee754::OperandStorage::Local(
                            "intermediate_2".to_string(),
                        ),
                    };
                    let intermediate_3 = general_assembly::extension::ieee754::Operand {
                        ty: general_assembly::extension::ieee754::OperandType::Binary32,
                        value: general_assembly::extension::ieee754::OperandStorage::Local(
                            "intermediate_3".to_string(),
                        ),
                    };
                    ret.extend([
                        general_assembly::operation::Operation::Ieee754(general_assembly::extension::ieee754::Operations::Copy {
                            destination: intermediate_2.clone(),
                            source: general_assembly::extension::ieee754::Operand {
                                ty: general_assembly::extension::ieee754::OperandType::Binary32,
                                value: general_assembly::extension::ieee754::OperandStorage::CoreOperand {
                                    operand: rt.clone(),
                                    ty: general_assembly::extension::ieee754::OperandType::Binary32,
                                    signed: false,
                                },
                            },
                        }),
                        general_assembly::operation::Operation::Ieee754(general_assembly::extension::ieee754::Operations::Copy {
                            destination: sm.clone(),
                            source: intermediate_2.clone(),
                        }),
                        general_assembly::operation::Operation::Ieee754(general_assembly::extension::ieee754::Operations::Copy {
                            destination: intermediate_3.clone(),
                            source: general_assembly::extension::ieee754::Operand {
                                ty: general_assembly::extension::ieee754::OperandType::Binary32,
                                value: general_assembly::extension::ieee754::OperandStorage::CoreOperand {
                                    operand: rt2.clone(),
                                    ty: general_assembly::extension::ieee754::OperandType::Binary32,
                                    signed: false,
                                },
                            },
                        }),
                        general_assembly::operation::Operation::Ieee754(general_assembly::extension::ieee754::Operations::Copy {
                            destination: sm1.clone(),
                            source: intermediate_3.clone(),
                        }),
                    ]);
                };
                ret
            }
        }
    }
    impl Decode for Vmrs {
        fn decode(
            &self,
            _in_it_block: bool,
        ) -> Vec<general_assembly::prelude::Operation> {
            let Self { rt } = self;
            match rt {
                RegisterOrAPSR::APSR => {
                    let mut ret = Vec::new();
                    ret.extend([
                        general_assembly::operation::Operation::Move {
                            destination: general_assembly::operand::Operand::Flag(
                                "APSR.N".to_owned(),
                            ),
                            source: general_assembly::operand::Operand::Flag(
                                "FPSCR.N".to_owned(),
                            ),
                        },
                        general_assembly::operation::Operation::Move {
                            destination: general_assembly::operand::Operand::Flag(
                                "APSR.Z".to_owned(),
                            ),
                            source: general_assembly::operand::Operand::Flag(
                                "FPSCR.Z".to_owned(),
                            ),
                        },
                        general_assembly::operation::Operation::Move {
                            destination: general_assembly::operand::Operand::Flag(
                                "APSR.C".to_owned(),
                            ),
                            source: general_assembly::operand::Operand::Flag(
                                "FPSCR.C".to_owned(),
                            ),
                        },
                        general_assembly::operation::Operation::Move {
                            destination: general_assembly::operand::Operand::Flag(
                                "APSR.V".to_owned(),
                            ),
                            source: general_assembly::operand::Operand::Flag(
                                "FPSCR.V".to_owned(),
                            ),
                        },
                    ]);
                    ret
                }
                RegisterOrAPSR::Register(r) => {
                    let r = r.local_into();
                    {
                        let mut ret = Vec::new();
                        ret.extend([
                            general_assembly::operation::Operation::Move {
                                destination: r.clone(),
                                source: general_assembly::operand::Operand::Register(
                                    "FPSCR".to_owned(),
                                ),
                            },
                        ]);
                        ret
                    }
                }
            }
        }
    }
    impl Decode for Vmsr {
        fn decode(
            &self,
            _in_it_block: bool,
        ) -> Vec<general_assembly::prelude::Operation> {
            let Self { rt } = self;
            let rt = rt.local_into();
            {
                let mut ret = Vec::new();
                ret.extend([
                    general_assembly::operation::Operation::Move {
                        destination: rt.clone(),
                        source: general_assembly::operand::Operand::Register(
                            "FPSCR".to_owned(),
                        ),
                    },
                ]);
                ret
            }
        }
    }
    impl super::sealed::Into<Operand> for F32OrF64 {
        fn local_into(self) -> Operand {
            match self {
                F32OrF64::F32(f) => f.local_into(),
                F32OrF64::F64(f) => f.local_into(),
            }
        }
    }
    impl super::sealed::Into<Operand> for disarmv7::arch::register::F32Register {
        fn local_into(self) -> Operand {
            Operand {
                ty: OperandType::Binary32,
                value: OperandStorage::Register {
                    id: self.name(),
                    ty: OperandType::Binary32,
                },
            }
        }
    }
    impl super::sealed::Into<Operand> for disarmv7::arch::register::F64Register {
        fn local_into(self) -> Operand {
            Operand {
                ty: OperandType::Binary64,
                value: OperandStorage::Register {
                    id: self.name(),
                    ty: OperandType::Binary64,
                },
            }
        }
    }
    impl super::sealed::Into<ComparisonMode> for Condition {
        fn local_into(self) -> ComparisonMode {
            match self {
                Self::Eq => ComparisonMode::Equal,
                Self::Ne => ComparisonMode::NotEqual,
                Self::Lt => ComparisonMode::Less,
                Self::Le => ComparisonMode::LessOrEqual,
                Self::Gt => ComparisonMode::Greater,
                Self::Ge => ComparisonMode::GreaterOrEqual,
                _ => ::core::panicking::panic("not yet implemented"),
            }
        }
    }
    struct Wrappedu64(u64);
    struct Wrappedu32(u32);
    impl super::sealed::Into<Operand> for Wrappedu64 {
        fn local_into(self) -> Operand {
            let val_ptr: *const u64 = (&self.0) as *const u64;
            Operand {
                ty: OperandType::Binary64,
                value: OperandStorage::Immediate {
                    value: unsafe { core::ptr::read(val_ptr as *const f64) },
                    ty: OperandType::Binary64,
                },
            }
        }
    }
    impl super::sealed::Into<Operand> for Wrappedu32 {
        fn local_into(self) -> Operand {
            let val_ptr: *const u32 = (&self.0) as *const u32;
            Operand {
                ty: OperandType::Binary32,
                value: OperandStorage::Immediate {
                    value: unsafe { core::ptr::read(val_ptr as *const f32) } as f64,
                    ty: OperandType::Binary32,
                },
            }
        }
    }
    impl super::sealed::Into<Operand> for (f64, OperandType) {
        fn local_into(self) -> Operand {
            Operand {
                ty: self.1.clone(),
                value: OperandStorage::Immediate {
                    value: self.0,
                    ty: self.1.clone(),
                },
            }
        }
    }
    impl super::sealed::Into<RoundingMode> for IEEE754RoundingMode {
        fn local_into(self) -> RoundingMode {
            match self {
                Self::RN => RoundingMode::TiesToEven,
                Self::RP => RoundingMode::TiesTowardPositive,
                Self::RM => RoundingMode::TiesTowardNegative,
                Self::RZ => RoundingMode::TiesTowardZero,
            }
        }
    }
    impl super::sealed::Into<Operand> for IntType {
        fn local_into(self) -> Operand {
            match self {
                Self::U32(f) => f.local_into(),
                Self::I32(f) => f.local_into(),
            }
        }
    }
}
