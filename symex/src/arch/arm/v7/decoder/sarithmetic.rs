use disarmv7::{
    arch::Register,
    operation::{SubSpMinusRegister, Sxth},
};
use transpiler::pseudo;

use super::Decode;
use crate::arch::arm::v7::decoder::sealed::Into;

impl Decode for Sxth {
    fn decode(&self, _in_it_block: bool) -> Vec<general_assembly::prelude::Operation> {
        let Sxth { rd, rm, rotation } = self;
        let rd = rd.local_into();
        let rm = rm.local_into();
        let rotation = rotation.unwrap_or(0).local_into();

        pseudo!([
            rm:u32;
            let rotated = Ror(rm,rotation) & (u16::MAX as u32).local_into();
            rd = SignExtend(rotated, 16,32);
        ])
    }
}
