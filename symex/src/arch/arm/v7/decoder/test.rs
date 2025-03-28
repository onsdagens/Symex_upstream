use disarmv7::operation::{TeqImmediate, TeqRegister, TstImmediate, TstRegister};
use transpiler::pseudo;

use super::Decode;
use crate::arch::arm::v7::decoder::sealed::Into;

impl Decode for TstRegister {
    fn decode(&self, _in_it_block: bool) -> Vec<general_assembly::prelude::Operation> {
        let TstRegister { rn, rm, shift } = self;

        let rn = rn.local_into();
        let rm = rm.local_into();
        let mut ret = vec![];
        local!(shifted);
        shift!(ret.shift rm -> shifted set c for rm);
        pseudo!(ret.extend[
                let result:u32 = rn & shifted;
                SetNFlag(result);
                SetZFlag(result);
        ]);
        ret
    }
}

impl Decode for TstImmediate {
    fn decode(&self, _in_it_block: bool) -> Vec<general_assembly::prelude::Operation> {
        let TstImmediate { rn, carry, imm } = self;
        let rn = rn.local_into();
        let imm = imm.local_into();
        pseudo!([
            let result:u32 = rn & imm;
            SetZFlag(result);
            SetNFlag(result);
            if (carry.is_some()){
                Flag("C") = (carry.unwrap() as u32).local_into();
            }
        ])
    }
}

impl Decode for TeqRegister {
    fn decode(&self, _in_it_block: bool) -> Vec<general_assembly::prelude::Operation> {
        let TeqRegister { rn, rm, shift } = self;
        let rn = rn.local_into();
        let rm = rm.local_into();

        let mut ret = vec![];
        local!(intermediate);
        shift!(ret.shift rm -> intermediate set c for rn);
        pseudo!(ret.extend[
            let result:u32 = rn ^ intermediate;
            SetZFlag(result);
            SetNFlag(result);
        ]);
        ret
    }
}

impl Decode for TeqImmediate {
    fn decode(&self, _in_it_block: bool) -> Vec<general_assembly::prelude::Operation> {
        let TeqImmediate { rn, carry, imm } = self;
        let rn = rn.local_into();
        let imm = imm.local_into();

        pseudo!([
            let result:u32 = rn ^ imm;
            SetNFlag(result);
            SetZFlag(result);
            if (carry.is_some()){
                Flag("C") = (carry.unwrap() as u32).local_into();
            }
        ])
    }
}
