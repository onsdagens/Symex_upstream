use disarmv7::{
    arch::Shift,
    operation::{Stm, Stmdb, StrImmediate, StrRegister, StrbImmediate, StrbRegister, Strbt, StrdImmediate, Strex, Strexb, Strexh, StrhImmediate, StrhRegister, Strht, Strt},
};
use transpiler::pseudo;

use super::{sealed::Into, Decode};

impl Decode for Strt {
    fn decode(&self, _in_it_block: bool) -> Vec<general_assembly::prelude::Operation> {
        let Self { rt, rn, imm } = self;
        let rt = rt.local_into();
        let rn = rn.local_into();
        let imm = imm.unwrap_or(0).local_into();

        pseudo!([
            rn:u32;
            rt:u32;
            let address = rn + imm;
            let data = rt;
            LocalAddress(address,32) = data;
        ])
    }
}

impl Decode for Strht {
    fn decode(&self, _in_it_block: bool) -> Vec<general_assembly::prelude::Operation> {
        let Self { rt, rn, imm } = self;
        let rt = rt.local_into();
        let rn = rn.local_into();
        let imm = imm.unwrap_or(0).local_into();

        pseudo!([
            rt:u32;
            rn:u32;
            let address = rn + imm;
            LocalAddress(address,16) = Resize(rt<15:0>,u16);
        ])
    }
}

impl Decode for StrhRegister {
    fn decode(&self, _in_it_block: bool) -> Vec<general_assembly::prelude::Operation> {
        let Self { rt, rn, rm, shift } = self;

        let rt = rt.local_into();
        let rn = rn.local_into();
        let rm = rm.local_into();
        let shift_n = match shift {
            Some(shift) => {
                assert!(shift.shift_t == Shift::Lsl);
                shift.shift_n as u32
            }
            None => 0,
        }
        .local_into();
        pseudo!([
            rm:u32;
            rt:u32;
            rn:u32;
            let offset = rm << shift_n;
            let address = rn + offset;
            LocalAddress(address,16) = Resize(rt<15:0>, u16);
        ])
    }
}

impl Decode for StrhImmediate {
    fn decode(&self, _in_it_block: bool) -> Vec<general_assembly::prelude::Operation> {
        let Self { index, add, w, rt, rn, imm } = self;
        let rt = rt.local_into();
        let rn = rn.local_into();
        let imm = imm.unwrap_or(0).local_into();

        pseudo!([
            rt:u32;
            rn:u32;
            imm:u32;
            let offset_addr = rn - imm;
            if (*add) {
                offset_addr = rn + imm;
            }
            let address = rn;
            if (*index) {
                address = offset_addr;
            }
            LocalAddress(address,16) = Resize(rt<15:0>,u16);
            if (*w) {
                rn = offset_addr;
            }
        ])
    }
}

impl Decode for Strexh {
    fn decode(&self, _in_it_block: bool) -> Vec<general_assembly::prelude::Operation> {
        let Self { rd, rt, rn } = self;
        let rd = rd.local_into();
        let rt = rt.local_into();
        let rn = rn.local_into();

        pseudo!([
            rn:u32;
            rt:u32;
            rd:u32;
            let address = rn;
            // TODO! Add in exclusive address here
            Abort("Incomplete instruction Strexh");
            LocalAddress(address,16) = Resize(rt<15:0>,u16);
            rd = 0.local_into();
        ])
    }
}

impl Decode for Strexb {
    fn decode(&self, _in_it_block: bool) -> Vec<general_assembly::prelude::Operation> {
        let Self { rd, rt, rn } = self;
        let rd = rd.local_into();
        let rt = rt.local_into();
        let rn = rn.local_into();
        pseudo!([
            rd:u32;
            rt:u32;
            rn:u32;
            let address = rn;
            // TODO! Add in exclusive addresses here
            Abort("Incomplete instruction Strexb");
            LocalAddress(address,8) = Resize(rt<7:0>, u8);
            rd = 0.local_into();
        ])
    }
}

impl Decode for Strex {
    fn decode(&self, _in_it_block: bool) -> Vec<general_assembly::prelude::Operation> {
        let Self { rd, rt, rn, imm } = self;
        let rd = rd.local_into();
        let rt = rt.local_into();
        let rn = rn.local_into();
        let imm = imm.unwrap_or(0).local_into();

        pseudo!([
            rd:u32;
            rn:u32;
            rt:u32;
            let address = rn + imm;
            // TODO! Add in exclusive addresses here
            LocalAddress(address,32) = rt;
            Abort("Incomplete instruction Strex");
            rd = 0.local_into();
        ])
    }
}

impl Decode for StrdImmediate {
    fn decode(&self, _in_it_block: bool) -> Vec<general_assembly::prelude::Operation> {
        let Self { w, index, add, rt, rt2, rn, imm } = self;
        let rt2 = rt2.local_into();
        let rt = rt.local_into();
        let rn = rn.local_into();
        let index = index.unwrap_or(true);
        let imm = imm.unwrap_or(0).local_into();
        let w = w.unwrap_or(false);

        pseudo!([
            rt2:u32;
            rt:u32;
            rn:u32;
            imm:u32;
            let offset_addr = rn - imm;
            if (*add) {
                offset_addr = rn + imm;
            }

            let address = rn;
            if (index) {
                address = offset_addr;
            }
            LocalAddress(address,32) = rt;
            address = address + 4.local_into();
            LocalAddress(address,32) = rt2;

            if (w) {
                rn = offset_addr;
            }
        ])
    }
}

impl Decode for Strbt {
    fn decode(&self, _in_it_block: bool) -> Vec<general_assembly::prelude::Operation> {
        let Self { rt, rn, imm } = self;
        let rt = rt.local_into();
        let rn = rn.local_into();
        let imm = imm.unwrap_or(0).local_into();
        pseudo!([
            rt:u32;
            let address:u32 = rn + imm;
            LocalAddress(address, 8) = Resize(rt,u8);
        ])
    }
}

impl Decode for StrbRegister {
    fn decode(&self, _in_it_block: bool) -> Vec<general_assembly::prelude::Operation> {
        let Self { rt, rn, rm, shift } = self;
        let rt = rt.local_into();
        let rn = rn.local_into();
        let rm = rm.local_into();
        let shift_n = match shift {
            Some(shift) => shift.shift_n as u32,
            None => 0,
        }
        .local_into();
        pseudo!([
                rt:u32;
                // Shift will always be LSL on the v7
                let offset:u32 = rm << shift_n;
                let address:u32 = rn + offset;
                LocalAddress(address, 8) = Resize(rt,u8);
        ])
    }
}

impl Decode for StrbImmediate {
    fn decode(&self, _in_it_block: bool) -> Vec<general_assembly::prelude::Operation> {
        let Self { w, index, add, rt, rn, imm } = self;
        let w = w.unwrap_or(false);
        let index = index.unwrap_or(false);
        let rt = rt.local_into();
        let rn = rn.local_into();
        let imm = imm.local_into();

        pseudo!([
            rn:u32;
            rt:u32;
            let offset_addr:u32 = rn - imm;
            if (*add) {
                offset_addr = rn + imm;
            }

            let address = rn;
            if (index) {
                address = offset_addr;
            }

            LocalAddress(address,8) = rt<7:0>;

            if (w) {
                rn = offset_addr;
            }
        ])
    }
}

impl Decode for StrRegister {
    fn decode(&self, _in_it_block: bool) -> Vec<general_assembly::prelude::Operation> {
        let Self { rt, rn, rm, shift } = self;
        let rt = rt.local_into();
        let rn = rn.local_into();
        let rm = rm.local_into();
        let shift_n = match shift {
            Some(shift) => {
                assert!(shift.shift_t == Shift::Lsl, "Shift must always be lsl on the v7.");
                shift.shift_n as u32
            }
            None => 0,
        }
        .local_into();
        pseudo!([
            shift_n:u32;
            // Shift will always be LSL on the v7
            let offset:u32 = rm << shift_n;
            let address = rn + offset;
            LocalAddress(address, 32):u32 = rt;
        ])
    }
}

impl Decode for StrImmediate {
    fn decode(&self, _in_it_block: bool) -> Vec<general_assembly::prelude::Operation> {
        let Self { w, index, add, rt, rn, imm } = self;
        let w = w.unwrap_or(false);
        let add = *add;
        let index = index.unwrap_or(false);
        let rt = rt.local_into();
        let rn = rn.local_into();
        let imm = imm.local_into();

        pseudo!([
            let offset_addr:u32 = rn - imm;
            if (add) {
                offset_addr = rn + imm;
            }

            let address:u32 = rn;
            if (index) {
                address = offset_addr;
            }

            let intermediate_rt:u32 = rt;

            if (w) {
                rn = offset_addr;
            }

            LocalAddress(address,32) = intermediate_rt;
        ])
    }
}

impl Decode for Stmdb {
    fn decode(&self, _in_it_block: bool) -> Vec<general_assembly::prelude::Operation> {
        let Self { w, rn, registers } = self;
        let w = w.unwrap_or(false);
        let rn = rn.local_into();

        let n = registers.registers.len() as u32;
        pseudo!([
            rn:u32;
            let address:u32 = rn - (4*n).local_into();
            for reg in registers.registers.iter() {
                LocalAddress(address,32) = reg.local_into();
                address += 4.local_into();
            }
            if (w) {
                rn = rn - (4u32 * n).local_into();
            }
        ])
    }
}

impl Decode for Stm {
    fn decode(&self, _in_it_block: bool) -> Vec<general_assembly::prelude::Operation> {
        let Self { w, rn, registers } = self;
        let w = w.unwrap_or(false);
        let rn = rn.local_into();
        let bc = registers.registers.len() as u32;

        pseudo!([
            let address:u32 = rn;
            for reg in registers.registers.iter() {
                LocalAddress(address,32) = reg.local_into();
                address += 4.local_into();
            }
            if (w) {
                rn += (4*bc).local_into();
            }
        ])
    }
}
