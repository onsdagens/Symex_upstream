use disarmv7::operation::{Sxtab, Sxtab16, Sxtah, Sxtb, Sxtb16};
use transpiler::pseudo;

use super::{sealed::Into, Decode};

impl Decode for Sxtb16 {
    fn decode(&self, _in_it_block: bool) -> Vec<general_assembly::prelude::Operation> {
        let Sxtb16 { rd, rm, rotation } = self;
        let rm = rm.local_into();
        let rd = rd.local_into().unwrap_or(rm.clone());
        let rotation = rotation.unwrap_or(0).local_into();

        pseudo!([
            rm:i32;
            rd:i32;
            rotation:u32;

            let rotated = Ror(rm,rotation);
            let lsbyte = rotated & ( u8::MAX as u32).local_into();
            rd = SignExtend(lsbyte,16,32) &  (u16::MAX as u32).local_into();

            let msbyte = rotated >> 16.local_into();
            msbyte = msbyte & (u8::MAX as u32).local_into();
            msbyte = SignExtend(msbyte,16,32) & (u16::MAX as u32).local_into();
            msbyte = msbyte << 16.local_into();

            rd = rd | msbyte;
        ])
    }
}

impl Decode for Sxtb {
    fn decode(&self, _in_it_block: bool) -> Vec<general_assembly::prelude::Operation> {
        let Self { rd, rm, rotation } = self;
        let rd = rd.local_into();
        let rm = rm.local_into();
        let rotation = rotation.unwrap_or(0).local_into();
        pseudo!([
            rm:i32;
            rd:i32;
            rotation:u32;
            let rotated = Ror(rm,rotation);
            rotated = rotated & ( u8::MAX as u32).local_into();
            rd = SignExtend(rotated,8,32);
        ])
    }
}

impl Decode for Sxtah {
    fn decode(&self, _in_it_block: bool) -> Vec<general_assembly::prelude::Operation> {
        let Self { rd, rn, rm, rotation } = self;
        let rn = rn.local_into();
        let rm = rm.local_into();
        let rd = rd.local_into().unwrap_or(rn.clone());
        let rotation = rotation.unwrap_or(0).local_into();
        pseudo!([
            rm:i32;
            rm:i32;
            rd:i32;
            let rotated = Ror(rm,rotation);
            rotated = rotated & ( u16::MAX as u32).local_into();
            rd = rn + SignExtend(rotated,16,32);
        ])
    }
}

impl Decode for Sxtab16 {
    fn decode(&self, _in_it_block: bool) -> Vec<general_assembly::prelude::Operation> {
        let Self { rd, rn, rm, rotation } = self;
        let rn = rn.local_into();
        let rd = rd.local_into().unwrap_or(rn.clone());
        let rm = rm.local_into();
        let rotation = rotation.unwrap_or(0);
        let u16max = (u16::MAX as u32).local_into();
        let u8max = (u8::MAX as u32).local_into();
        pseudo!([
            rm:i32;
            rd:i32;
            rn:i32;
            u16max:i32;
            u8max:i32;
            let rotated = Ror(rm, rotation.local_into());


            // Clear the current rd
            rd = 0.local_into();

            let lsh_mask = u16max;

            let rotated_lsbyte = rotated & u8max ;
            rd = rn & lsh_mask;
            // TODO! Make note in the docs for GA that 8 is the msb in the number
            // prior to sign extension
            rd = rd + SignExtend(rotated_lsbyte,8,32);
            rd = rd & lsh_mask;



            //let msh_mask = ((u16::MAX as u32) << 16).local_into();
            let msh_intermediate = rn >> 16.local_into();
            rotated = rotated >> 16.local_into();
            rotated = rotated & (u8::MAX as u32).local_into();
            let intemediate_result = msh_intermediate + SignExtend(rotated,8,32);
            intemediate_result = intemediate_result & lsh_mask;
            intemediate_result = intemediate_result << 16.local_into();

            rd =  rd | intemediate_result;
        ])
    }
}

impl Decode for Sxtab {
    fn decode(&self, _in_it_block: bool) -> Vec<general_assembly::prelude::Operation> {
        let Self { rd, rn, rm, rotation } = self;
        let rn = rn.local_into();
        let rd = rd.local_into().unwrap_or(rn.clone());
        let rm = rm.local_into();
        let rotation = rotation.unwrap_or(0);
        pseudo!([
            rm:i32;
            rn:i32;
            rd:i32;
            let rotated = Ror(rm, rotation.local_into());
            let masked = rotated & (u8::MAX as u32).local_into();
            rd = rn + SignExtend(masked,8,32);
        ])
    }
}
