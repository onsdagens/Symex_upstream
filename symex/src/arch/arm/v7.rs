use std::fmt::Display;

use anyhow::Context;
use decoder::Convert;
use disarmv7::prelude::{Operation as V7Operation, *};
use general_assembly::{extension::ieee754::RoundingMode, operation::Operation, shift::Shift};

use crate::{
    arch::{ArchError, Architecture, ArchitectureOverride, ParseError, SupportedArchitecture},
    debug,
    executor::{
        hooks::{HookContainer, PCHook},
        instruction::Instruction,
        state::GAState,
    },
    project::dwarf_helper::SubProgramMap,
    smt::{SmtExpr, SmtMap},
    trace,
    GAError,
};

//#[rustfmt::skip]
pub mod compare;
pub mod decoder;
#[cfg(test)]
pub mod test;
pub mod timing;

/// Type level denotation for the ARMV7-EM ISA.
#[derive(Debug, Default, Clone)]
pub struct ArmV7EM {}

impl ArmV7EM {
    fn add_apsr_hooks<C: crate::Composition>(&self, cfg: &mut HookContainer<C>, _map: &mut SubProgramMap) {
        let write_aspr_n = |state: &mut GAState<C>, value: C::SmtExpression| {
            let value = value.resize_unsigned(1).resize_unsigned(32);
            let reg = state.memory.get_register("XPSR")?;
            let mask = state.memory.from_u64((u32::MAX >> 1).into(), 32);
            let shift_steps = state.memory.from_u64(31, 32);
            let mask = reg.and(&mask);
            let reg = value.shift(&shift_steps, Shift::Lsl);
            state.memory.set_register("XPSR", mask.or(&reg))?;
            trace!("WROTE APSR.N, {:?}", value.get_constant());
            Ok(())
        };
        let read_apsr_n = |state: &mut GAState<C>| {
            let reg = state.memory.get_register("XPSR")?;
            let shift_steps = state.memory.from_u64(31, 32);
            let mask = state.memory.from_u64((!(u32::MAX >> 1)).into(), 32);
            let mask = reg.and(&mask);
            let reg_value = mask.shift(&shift_steps, Shift::Lsr).resize_unsigned(1);
            trace!("READ APSR.N{:?}", reg_value.get_constant());

            Ok(reg_value)
        };

        let write_aspr = |state: &mut GAState<C>, value: C::SmtExpression| {
            let keep_mask = 0b1111_1000_0000_1111_0000_0000_0000_0000u64;
            let drop_mask = !keep_mask;
            let reg = state.memory.get_register("XPSR")?;
            let keep_mask = state.memory.from_u64(keep_mask, 32);
            let drop_mask = state.memory.from_u64(drop_mask, 32);
            let mask = reg.and(&drop_mask);
            let value = value.and(&keep_mask);
            state.memory.set_register("XPSR", mask.or(&value))?;
            Ok(())
        };
        let read_apsr = |state: &mut GAState<C>| {
            let keep_mask = 0b1111_1000_0000_1111_0000_0000_0000_0000u64;
            let reg = state.memory.get_register("XPSR")?;

            Ok(reg.and(&state.memory.from_u64(keep_mask, 32)))
        };

        cfg.add_register_write_hook("APSR".to_string(), write_aspr);
        cfg.add_register_read_hook("APSR".to_string(), read_apsr);

        cfg.add_flag_write_hook("APSR.N".to_string(), write_aspr_n);
        cfg.add_flag_read_hook("APSR.N".to_string(), read_apsr_n);
        cfg.add_flag_write_hook("N".to_string(), write_aspr_n);
        cfg.add_flag_read_hook("N".to_string(), read_apsr_n);

        let write_apsr_z = |state: &mut GAState<C>, value: C::SmtExpression| {
            let value = value.resize_unsigned(1).resize_unsigned(32);
            let reg = state.memory.get_register("XPSR")?;
            let mask = state.memory.from_u64(!(1 << 30), 32);
            let shift_steps = state.memory.from_u64(30, 32);
            let mask = reg.and(&mask);
            let reg = value.shift(&shift_steps, Shift::Lsl);
            state.memory.set_register("XPSR", mask.or(&reg))?;
            trace!("WROTE APSR.Z, {:?}", value.get_constant());
            Ok(())
        };
        let read_apsr_z = |state: &mut GAState<C>| {
            let reg = state.memory.get_register("XPSR")?;
            let shift_steps = state.memory.from_u64(30, 32);
            let mask = state.memory.from_u64(1 << 30, 32);
            let mask = reg.and(&mask);
            let reg_val = mask.shift(&shift_steps, Shift::Lsr);
            trace!("READ APSR.Z, {:?}", reg_val.get_constant());

            Ok(reg_val.resize_unsigned(1))
        };

        cfg.add_flag_write_hook("APSR.Z".to_string(), write_apsr_z);
        cfg.add_flag_read_hook("APSR.Z".to_string(), read_apsr_z);
        cfg.add_flag_write_hook("Z".to_string(), write_apsr_z);
        cfg.add_flag_read_hook("Z".to_string(), read_apsr_z);

        let write_apsr_c = |state: &mut GAState<C>, value: C::SmtExpression| {
            let value = value.resize_unsigned(1).resize_unsigned(32);
            let reg = state.memory.get_register("XPSR")?;
            let mask = state.memory.from_u64(!(1 << 29), 32);
            let shift_steps = state.memory.from_u64(29, 32);
            let mask = reg.and(&mask);
            let reg = value.shift(&shift_steps, Shift::Lsl);
            state.memory.set_register("XPSR", mask.or(&reg))?;
            trace!("WROTE APSR.C, {:?}", value.get_constant());
            Ok(())
        };
        let read_apsr_c = |state: &mut GAState<C>| {
            let reg = state.memory.get_register("XPSR")?;
            let shift_steps = state.memory.from_u64(29, 32);
            let mask = state.memory.from_u64(1 << 29, 32);
            let mask = reg.and(&mask);
            let reg = mask.shift(&shift_steps, Shift::Lsr).resize_unsigned(1);
            trace!("READ APSR.C, {:?}", reg.get_constant());

            Ok(reg)
        };

        cfg.add_flag_write_hook("APSR.C".to_string(), write_apsr_c);
        cfg.add_flag_read_hook("APSR.C".to_string(), read_apsr_c);
        cfg.add_flag_write_hook("C".to_string(), write_apsr_c);
        cfg.add_flag_read_hook("C".to_string(), read_apsr_c);

        let write_apsr_v = |state: &mut GAState<C>, value: C::SmtExpression| {
            let value = value.resize_unsigned(1).resize_unsigned(32);
            let reg = state.memory.get_register("XPSR")?;
            let mask = state.memory.from_u64(!(1 << 28), 32);
            let shift_steps = state.memory.from_u64(28, 32);
            let mask = reg.and(&mask);
            let reg = value.shift(&shift_steps, Shift::Lsl);
            state.memory.set_register("XPSR", mask.or(&reg))?;
            trace!("WRITE APSR.V, {:?}", value.get_constant());
            Ok(())
        };
        let read_apsr_v = |state: &mut GAState<C>| {
            let reg = state.memory.get_register("XPSR")?;
            let shift_steps = state.memory.from_u64(28, 32);
            let mask = state.memory.from_u64(1 << 28, 32);
            let mask = reg.and(&mask);
            let reg = mask.shift(&shift_steps, Shift::Lsr).resize_unsigned(1);
            trace!("READ APSR.V, {:?}", reg.get_constant());

            Ok(reg)
        };

        cfg.add_flag_write_hook("APSR.V".to_string(), write_apsr_v);
        cfg.add_flag_read_hook("APSR.V".to_string(), read_apsr_v);
        cfg.add_flag_write_hook("V".to_string(), write_apsr_v);
        cfg.add_flag_read_hook("V".to_string(), read_apsr_v);

        let write_apsr_q = |state: &mut GAState<C>, value: C::SmtExpression| {
            let value = value.resize_unsigned(1).resize_unsigned(32);
            let reg = state.memory.get_register("XPSR")?;
            let mask = state.memory.from_u64(!(1 << 27), 32);
            let shift_steps = state.memory.from_u64(27, 32);
            let mask = reg.and(&mask);
            let reg = value.shift(&shift_steps, Shift::Lsl);
            state.memory.set_register("XPSR", mask.or(&reg))?;
            let reg = state.memory.get_register("XPSR")?;
            trace!("WRITE APSR.Q, {:?}", reg);
            Ok(())
        };
        let read_apsr_q = |state: &mut GAState<C>| {
            let reg = state.memory.get_register("XPSR")?;
            trace!("READ APSR.Q, {:?}", reg);
            let shift_steps = state.memory.from_u64(27, 32);
            let mask = state.memory.from_u64(1 << 27, 32);
            let mask = reg.and(&mask);
            let reg = mask.shift(&shift_steps, Shift::Lsr).resize_unsigned(1);

            Ok(reg)
        };

        cfg.add_flag_write_hook("APSR.Q".to_string(), write_apsr_q);
        cfg.add_flag_read_hook("APSR.Q".to_string(), read_apsr_q);
        cfg.add_flag_write_hook("Q".to_string(), write_apsr_q);
        cfg.add_flag_read_hook("Q".to_string(), read_apsr_q);

        let write_apsr_ge = |state: &mut GAState<C>, value: C::SmtExpression| {
            let value = value.resize_unsigned(4).resize_unsigned(32);
            let reg = state.memory.get_register("XPSR")?;
            let mask = state.memory.from_u64(!(0b1111 << 16), 32);
            let shift_steps = state.memory.from_u64(16, 32);
            let mask = reg.and(&mask);
            let reg = value.shift(&shift_steps, Shift::Lsl);
            state.memory.set_register("XPSR", mask.or(&reg))?;
            Ok(())
        };
        let read_apsr_ge = |state: &mut GAState<C>| {
            let reg = state.memory.get_register("XPSR")?;
            let shift_steps = state.memory.from_u64(16, 32);
            let mask = state.memory.from_u64(0b1111 << 16, 32);
            let mask = reg.and(&mask);
            let reg = mask.shift(&shift_steps, Shift::Lsr).resize_unsigned(4).resize_unsigned(32);
            Ok(reg)
        };

        cfg.add_register_write_hook("APSR.GE".to_string(), write_apsr_ge);
        cfg.add_register_read_hook("APSR.GE".to_string(), read_apsr_ge);
    }

    fn add_fpscr_hooks<C: crate::Composition>(&self, cfg: &mut HookContainer<C>, _map: &mut SubProgramMap) {
        let write_fpscr_n = |state: &mut GAState<C>, value: C::SmtExpression| {
            let value = value.resize_unsigned(1).resize_unsigned(32);
            let reg = state.memory.get_register("FPSCR")?;
            let mask = state.memory.from_u64((u32::MAX >> 1).into(), 32);
            let shift_steps = state.memory.from_u64(31, 32);
            let mask = reg.and(&mask);
            let reg = value.shift(&shift_steps, Shift::Lsl);
            state.memory.set_register("FPSCR", mask.or(&reg))?;
            Ok(())
        };
        let read_fpscr_n = |state: &mut GAState<C>| {
            let reg = state.memory.get_register("FPSCR")?;
            let shift_steps = state.memory.from_u64(31, 32);
            let mask = state.memory.from_u64((!(u32::MAX >> 1)).into(), 32);
            let mask = reg.and(&mask);
            let reg = mask.shift(&shift_steps, Shift::Lsr).resize_unsigned(1);

            Ok(reg)
        };

        cfg.add_flag_write_hook("FPSCR.N".to_string(), write_fpscr_n);
        cfg.add_flag_read_hook("FPSCR.N".to_string(), read_fpscr_n);

        let write_fpscr_z = |state: &mut GAState<C>, value: C::SmtExpression| {
            let value = value.resize_unsigned(1).resize_unsigned(32);
            let reg = state.memory.get_register("FPSCR")?;
            let mask = state.memory.from_u64(!(1 << 30), 32);
            let shift_steps = state.memory.from_u64(30, 32);
            let mask = reg.and(&mask);
            let reg = value.shift(&shift_steps, Shift::Lsl);
            state.memory.set_register("FPSCR", mask.or(&reg))?;
            Ok(())
        };
        let read_fpscr_z = |state: &mut GAState<C>| {
            let reg = state.memory.get_register("FPSCR")?;
            let shift_steps = state.memory.from_u64(30, 32);
            let mask = state.memory.from_u64(1 << 30, 32);
            let mask = reg.and(&mask);
            let reg = mask.shift(&shift_steps, Shift::Lsr).resize_unsigned(1);

            Ok(reg)
        };

        cfg.add_flag_write_hook("FPSCR.Z".to_string(), write_fpscr_z);
        cfg.add_flag_read_hook("FPSCR.Z".to_string(), read_fpscr_z);

        let write_fpscr_c = |state: &mut GAState<C>, value: C::SmtExpression| {
            let value = value.resize_unsigned(1).resize_unsigned(32);
            let reg = state.memory.get_register("FPSCR")?;
            let mask = state.memory.from_u64(!(1 << 29), 32);
            let shift_steps = state.memory.from_u64(29, 32);
            let mask = reg.and(&mask);
            let reg = value.shift(&shift_steps, Shift::Lsl);
            state.memory.set_register("FPSCR", mask.or(&reg))?;
            Ok(())
        };
        let read_fpscr_c = |state: &mut GAState<C>| {
            let reg = state.memory.get_register("FPSCR")?;
            let shift_steps = state.memory.from_u64(29, 32);
            let mask = state.memory.from_u64(1 << 29, 32);
            let mask = reg.and(&mask);
            let reg = mask.shift(&shift_steps, Shift::Lsr).resize_unsigned(1);

            Ok(reg)
        };

        cfg.add_flag_write_hook("FPSCR.C".to_string(), write_fpscr_c);
        cfg.add_flag_read_hook("FPSCR.C".to_string(), read_fpscr_c);

        let write_fpscr_c = |state: &mut GAState<C>, value: C::SmtExpression| {
            let value = value.resize_unsigned(1).resize_unsigned(32);
            let reg = state.memory.get_register("FPSCR")?;
            let mask = state.memory.from_u64(!(1 << 28), 32);
            let shift_steps = state.memory.from_u64(28, 32);
            let mask = reg.and(&mask);
            let reg = value.shift(&shift_steps, Shift::Lsl);
            state.memory.set_register("FPSCR", mask.or(&reg))?;
            Ok(())
        };
        let read_fpscr_c = |state: &mut GAState<C>| {
            let reg = state.memory.get_register("FPSCR")?;
            let shift_steps = state.memory.from_u64(28, 32);
            let mask = state.memory.from_u64(1 << 28, 32);
            let mask = reg.and(&mask);
            let reg = mask.shift(&shift_steps, Shift::Lsr).resize_unsigned(1);

            Ok(reg)
        };

        cfg.add_flag_write_hook("FPSCR.V".to_string(), write_fpscr_c);
        cfg.add_flag_read_hook("FPSCR.V".to_string(), read_fpscr_c);

        let write_fpscr_rm = |state: &mut GAState<C>, value: C::SmtExpression| {
            let value = value.resize_unsigned(2).resize_unsigned(32);
            state.fp_state.rounding_mode = match value.get_constant() {
                Some(0b00) => RoundingMode::TiesToEven,
                Some(0b01) => RoundingMode::TiesTowardPositive,
                Some(0b10) => RoundingMode::TiesTowardNegative,
                Some(0b11) => RoundingMode::TiesTowardZero,
                Some(_) => return Err(GAError::InvalidRoundingMode).context("While writing to FPSCR"),
                None => return Err(GAError::InvalidRoundingMode).context("While writing to FPSCR, non constant."),
            };
            let reg = state.memory.get_register("FPSCR")?;
            let mask = state.memory.from_u64(!(0b11 << 22), 32);
            let shift_steps = state.memory.from_u64(22, 32);
            let mask = reg.and(&mask);
            let reg = value.shift(&shift_steps, Shift::Lsl);
            state.memory.set_register("FPSCR", mask.or(&reg))?;
            Ok(())
        };
        let read_fpscr_rm = |state: &mut GAState<C>| {
            let reg = state.memory.get_register("FPSCR")?;
            let shift_steps = state.memory.from_u64(22, 32);
            let mask = state.memory.from_u64(0b11 << 22, 32);
            let mask = reg.and(&mask);
            let reg = mask.shift(&shift_steps, Shift::Lsr).resize_unsigned(2).resize_unsigned(32);

            Ok(reg)
        };

        cfg.add_register_write_hook("FPSCR.RM".to_string(), write_fpscr_rm);
        cfg.add_register_read_hook("FPSCR.RM".to_string(), read_fpscr_rm);
    }
}

impl<Override: ArchitectureOverride> Architecture<Override> for ArmV7EM {
    type ISA = disarmv7::operation::Operation;

    fn add_hooks<C: crate::Composition>(&self, cfg: &mut HookContainer<C>, map: &mut SubProgramMap) {
        let symbolic_sized = |state: &mut GAState<C>| {
            let value_ptr = state.memory.get_register("R0")?;
            let size = state.memory.get_register("R1")?.get_constant().unwrap() * 8;
            let name = state.label_new_symbolic("any");
            let symb_value = state.memory.unconstrained(&name, size as usize);
            // We should be able to do this now!
            // TODO: We need to label this with proper variable names if possible.
            //state.marked_symbolic.push(Variable {
            //    name: Some(name),
            //    value: symb_value.clone(),
            //    ty: ExpressionType::Integer(size as usize),
            //});
            state.memory.set(&value_ptr, symb_value)?;

            let lr = state.get_register("LR".to_owned())?;
            state.set_register("PC".to_owned(), lr)?;
            Ok(())
        };

        let _ = cfg.add_pc_hook_regex(map, r"^symbolic_size<.+>$", PCHook::Intrinsic(symbolic_sized));

        // §B1.4 Specifies that R[15] => Addr(Current instruction) + 4
        //
        // This can be translated in to
        //
        // PC - Size(prev instruction) / 8 + 4
        // as PC points to the next instruction, we
        //
        //
        // Or we can simply take the previous PC + 4.
        let read_pc = |state: &mut GAState<C>| {
            let size = state.current_instruction.as_ref().unwrap().instruction_size / 8;
            let register = state.memory.get_pc()?.get_constant().unwrap();
            let new_pc = state.memory.from_u64(register - size as u64 + 4, state.memory.get_word_size()).simplify();
            Ok(new_pc)
        };
        let read_primask = |state: &mut GAState<C>| {
            let primask: C::SmtExpression = state.memory.from_u64(0, state.memory.get_word_size()).simplify();
            Ok(primask)
        };

        let write_primask = |_state: &mut GAState<C>, _| {
            panic!("Cannot write to PRIMASK");
        };

        let read_any = |state: &mut GAState<C>| Ok(state.memory.unconstrained_unnamed(32));

        let read_sp = |state: &mut GAState<C>| {
            let two = state.memory.from_u64((!(0b11u32)) as u64, 32);
            let sp = state.get_register("SP".to_owned()).unwrap();
            let sp = sp.simplify();
            Ok(sp.and(&two))
        };

        let write_pc = |state: &mut GAState<C>, value| state.set_register("PC".to_owned(), value);
        let write_sp = |state: &mut GAState<C>, value: C::SmtExpression| {
            //state.set_register("SP".to_string(),
            // value.and(&state.memory.from_u64((!(0b11u32)) as u64, 32)))?; let
            // sp = state.get_register("SP".to_owned()).unwrap(); let sp = sp.
            // simplify();
            state.set_register("SP".to_owned(), value)
        };

        cfg.add_register_read_hook("PC+".to_string(), read_pc);
        cfg.add_register_read_hook("PRIMASK".to_string(), read_primask);
        cfg.add_register_write_hook("PRIMASK".to_string(), write_primask);
        cfg.add_register_write_hook("PC+".to_owned(), write_pc);
        cfg.add_register_read_hook("SP&".to_owned(), read_sp);
        cfg.add_register_write_hook("SP&".to_owned(), write_sp);
        cfg.add_register_read_hook("ANY".to_owned(), read_any);

        self.add_apsr_hooks(cfg, map);
        self.add_fpscr_hooks(cfg, map);

        // reset always done
        let read_reset_done = |state: &mut GAState<C>, _addr| {
            let value = state.memory.from_u64(0xffff_ffff, 32);
            Ok(value)
        };
        cfg.add_memory_read_hook(0x4000c008, read_reset_done);
    }

    fn translate<C: crate::Composition>(&self, buff: &[u8], state: &GAState<C>) -> Result<Instruction<C>, ArchError> {
        trace!("decoding, buff : {:?}", buff);
        let mut buff: disarmv7::buffer::PeekableBuffer<u8, _> = buff.iter().cloned().into();

        let instr = V7Operation::parse(&mut buff).map_err(|e| ArchError::ParsingError(e.into()));

        trace!("PC{:#x} -> Running {:?}", state.memory.get_pc().unwrap().get_constant().unwrap(), instr);
        let instr = instr?;
        let timing = Self::cycle_count_m4_core(&instr.1);
        let ops: Vec<Operation> = instr.clone().convert(state.get_in_conditional_block());

        Ok(Instruction {
            instruction_size: instr.0 as u32,
            operations: ops,
            max_cycle: timing,
            memory_access: Self::memory_access(&instr.1),
        })
    }

    //fn discover(file: &File<'_>) -> Result<Option<Self>, ArchError> {
    //    let f = match file {
    //        File::Elf32(f) => Ok(f),
    //        _ => Err(ArchError::IncorrectFileType),
    //    }?;
    //    let section = match f.section_by_name(".ARM.attributes") {
    //        Some(section) => Ok(section),
    //        None => Err(ArchError::MissingSection(".ARM.attributes")),
    //    }?;
    //    let isa = arm_isa(&section)?;
    //    match isa {
    //        ArmIsa::ArmV6M => Ok(None),
    //        ArmIsa::ArmV7EM => Ok(Some(ArmV7EM::default())),
    //    }
    //}

    fn new() -> Self
    where
        Self: Sized,
    {
        Self {}
    }
}

impl Display for ArmV7EM {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ARMv7-M")
    }
}

impl From<disarmv7::ParseError> for ParseError {
    fn from(value: disarmv7::ParseError) -> Self {
        match value {
            disarmv7::ParseError::Undefined => ParseError::InvalidInstruction,
            disarmv7::ParseError::ArchError(aerr) => match aerr {
                disarmv7::prelude::arch::ArchError::InvalidCondition => ParseError::InvalidCondition,
                disarmv7::prelude::arch::ArchError::InvalidRegister(_) => ParseError::InvalidRegister,
                disarmv7::prelude::arch::ArchError::InvalidField(_) => ParseError::MalfromedInstruction,
            },
            disarmv7::ParseError::Unpredictable => ParseError::Unpredictable,
            disarmv7::ParseError::Invalid16Bit(_) | disarmv7::ParseError::Invalid32Bit(_) => ParseError::InvalidInstruction,
            disarmv7::ParseError::InvalidField(_) => ParseError::MalfromedInstruction,
            disarmv7::ParseError::Incomplete32Bit => ParseError::InsufficientInput,
            disarmv7::ParseError::InternalError(info) => ParseError::Generic(info),
            disarmv7::ParseError::IncompleteParser => ParseError::Generic("Encountered instruction that is not yet supported."),
            disarmv7::ParseError::InvalidCondition => ParseError::InvalidCondition,
            disarmv7::ParseError::IncompleteProgram => ParseError::InsufficientInput,
            disarmv7::ParseError::InvalidRegister(_) => ParseError::InvalidRegister,
            disarmv7::ParseError::PartiallyParsed(error, _) => (*error).into(),
            disarmv7::ParseError::InvalidFloatingPointRegister(_) => ParseError::InvalidRegister,
            disarmv7::ParseError::InvalidRoundingMode(_) => ParseError::InvalidRoundingMode,
        }
    }
}

impl<Override: ArchitectureOverride> From<ArmV7EM> for SupportedArchitecture<Override> {
    fn from(val: ArmV7EM) -> SupportedArchitecture<Override> {
        SupportedArchitecture::Armv7EM(val)
    }
}
