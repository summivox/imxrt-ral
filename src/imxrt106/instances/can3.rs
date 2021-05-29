#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! CAN
//!
//! Used by: imxrt1061, imxrt1062, imxrt1064

#[cfg(not(feature = "nosync"))]
pub use crate::imxrt106::peripherals::can3::Instance;
pub use crate::imxrt106::peripherals::can3::{RegisterBlock, ResetValues};
pub use crate::imxrt106::peripherals::can3::{
    CBT, CRCR, CS0, CS1, CS10, CS11, CS12, CS13, CS14, CS15, CS16, CS17, CS18, CS19, CS2, CS20,
    CS21, CS22, CS23, CS24, CS25, CS26, CS27, CS28, CS29, CS3, CS30, CS31, CS32, CS33, CS34, CS35,
    CS36, CS37, CS38, CS39, CS4, CS40, CS41, CS42, CS43, CS44, CS45, CS46, CS47, CS48, CS49, CS5,
    CS50, CS51, CS52, CS53, CS54, CS55, CS56, CS57, CS58, CS59, CS6, CS60, CS61, CS62, CS63, CS7,
    CS8, CS9, CTRL1, CTRL2, ECR, EDCBT, ENCBT, EPRS, ERFCR, ERFFEL0, ERFFEL1, ERFFEL10, ERFFEL100,
    ERFFEL101, ERFFEL102, ERFFEL103, ERFFEL104, ERFFEL105, ERFFEL106, ERFFEL107, ERFFEL108,
    ERFFEL109, ERFFEL11, ERFFEL110, ERFFEL111, ERFFEL112, ERFFEL113, ERFFEL114, ERFFEL115,
    ERFFEL116, ERFFEL117, ERFFEL118, ERFFEL119, ERFFEL12, ERFFEL120, ERFFEL121, ERFFEL122,
    ERFFEL123, ERFFEL124, ERFFEL125, ERFFEL126, ERFFEL127, ERFFEL13, ERFFEL14, ERFFEL15, ERFFEL16,
    ERFFEL17, ERFFEL18, ERFFEL19, ERFFEL2, ERFFEL20, ERFFEL21, ERFFEL22, ERFFEL23, ERFFEL24,
    ERFFEL25, ERFFEL26, ERFFEL27, ERFFEL28, ERFFEL29, ERFFEL3, ERFFEL30, ERFFEL31, ERFFEL32,
    ERFFEL33, ERFFEL34, ERFFEL35, ERFFEL36, ERFFEL37, ERFFEL38, ERFFEL39, ERFFEL4, ERFFEL40,
    ERFFEL41, ERFFEL42, ERFFEL43, ERFFEL44, ERFFEL45, ERFFEL46, ERFFEL47, ERFFEL48, ERFFEL49,
    ERFFEL5, ERFFEL50, ERFFEL51, ERFFEL52, ERFFEL53, ERFFEL54, ERFFEL55, ERFFEL56, ERFFEL57,
    ERFFEL58, ERFFEL59, ERFFEL6, ERFFEL60, ERFFEL61, ERFFEL62, ERFFEL63, ERFFEL64, ERFFEL65,
    ERFFEL66, ERFFEL67, ERFFEL68, ERFFEL69, ERFFEL7, ERFFEL70, ERFFEL71, ERFFEL72, ERFFEL73,
    ERFFEL74, ERFFEL75, ERFFEL76, ERFFEL77, ERFFEL78, ERFFEL79, ERFFEL8, ERFFEL80, ERFFEL81,
    ERFFEL82, ERFFEL83, ERFFEL84, ERFFEL85, ERFFEL86, ERFFEL87, ERFFEL88, ERFFEL89, ERFFEL9,
    ERFFEL90, ERFFEL91, ERFFEL92, ERFFEL93, ERFFEL94, ERFFEL95, ERFFEL96, ERFFEL97, ERFFEL98,
    ERFFEL99, ERFIER, ERFSR, ESR1, ESR2, ETDC, FDCBT, FDCRC, FDCTRL, HR_TIME_STAMP0,
    HR_TIME_STAMP1, HR_TIME_STAMP10, HR_TIME_STAMP11, HR_TIME_STAMP12, HR_TIME_STAMP13,
    HR_TIME_STAMP14, HR_TIME_STAMP15, HR_TIME_STAMP16, HR_TIME_STAMP17, HR_TIME_STAMP18,
    HR_TIME_STAMP19, HR_TIME_STAMP2, HR_TIME_STAMP20, HR_TIME_STAMP21, HR_TIME_STAMP22,
    HR_TIME_STAMP23, HR_TIME_STAMP24, HR_TIME_STAMP25, HR_TIME_STAMP26, HR_TIME_STAMP27,
    HR_TIME_STAMP28, HR_TIME_STAMP29, HR_TIME_STAMP3, HR_TIME_STAMP30, HR_TIME_STAMP31,
    HR_TIME_STAMP32, HR_TIME_STAMP33, HR_TIME_STAMP34, HR_TIME_STAMP35, HR_TIME_STAMP36,
    HR_TIME_STAMP37, HR_TIME_STAMP38, HR_TIME_STAMP39, HR_TIME_STAMP4, HR_TIME_STAMP40,
    HR_TIME_STAMP41, HR_TIME_STAMP42, HR_TIME_STAMP43, HR_TIME_STAMP44, HR_TIME_STAMP45,
    HR_TIME_STAMP46, HR_TIME_STAMP47, HR_TIME_STAMP48, HR_TIME_STAMP49, HR_TIME_STAMP5,
    HR_TIME_STAMP50, HR_TIME_STAMP51, HR_TIME_STAMP52, HR_TIME_STAMP53, HR_TIME_STAMP54,
    HR_TIME_STAMP55, HR_TIME_STAMP56, HR_TIME_STAMP57, HR_TIME_STAMP58, HR_TIME_STAMP59,
    HR_TIME_STAMP6, HR_TIME_STAMP60, HR_TIME_STAMP61, HR_TIME_STAMP62, HR_TIME_STAMP63,
    HR_TIME_STAMP7, HR_TIME_STAMP8, HR_TIME_STAMP9, ID0, ID1, ID10, ID11, ID12, ID13, ID14, ID15,
    ID16, ID17, ID18, ID19, ID2, ID20, ID21, ID22, ID23, ID24, ID25, ID26, ID27, ID28, ID29, ID3,
    ID30, ID31, ID32, ID33, ID34, ID35, ID36, ID37, ID38, ID39, ID4, ID40, ID41, ID42, ID43, ID44,
    ID45, ID46, ID47, ID48, ID49, ID5, ID50, ID51, ID52, ID53, ID54, ID55, ID56, ID57, ID58, ID59,
    ID6, ID60, ID61, ID62, ID63, ID7, ID8, ID9, IFLAG1, IFLAG2, IMASK1, IMASK2, MB, MB0,
    MB0_16B_WORD1, MB0_32B_WORD5, MB0_64B_WORD12, MB0_64B_WORD13, MB0_64B_WORD8, MB0_64B_WORD9,
    MB1, MB10_16B_WORD0, MB10_16B_WORD1, MB10_32B_WORD0, MB10_32B_WORD1, MB10_32B_WORD4,
    MB10_32B_WORD5, MB10_64B_WORD0, MB10_64B_WORD1, MB10_64B_WORD12, MB10_64B_WORD13,
    MB10_64B_WORD4, MB10_64B_WORD5, MB10_64B_WORD8, MB10_64B_WORD9, MB10_8B_WORD0, MB10_8B_WORD1,
    MB11_16B_CS, MB11_16B_ID, MB11_16B_WORD2, MB11_16B_WORD3, MB11_32B_CS, MB11_32B_ID,
    MB11_32B_WORD2, MB11_32B_WORD3, MB11_32B_WORD6, MB11_32B_WORD7, MB11_64B_CS, MB11_64B_ID,
    MB11_64B_WORD10, MB11_64B_WORD11, MB11_64B_WORD14, MB11_64B_WORD15, MB11_64B_WORD2,
    MB11_64B_WORD3, MB11_64B_WORD6, MB11_64B_WORD7, MB11_8B_WORD0, MB11_8B_WORD1, MB12_16B_WORD0,
    MB12_16B_WORD1, MB12_32B_WORD0, MB12_32B_WORD1, MB12_32B_WORD4, MB12_32B_WORD5, MB12_64B_WORD0,
    MB12_64B_WORD1, MB12_64B_WORD12, MB12_64B_WORD13, MB12_64B_WORD4, MB12_64B_WORD5,
    MB12_64B_WORD8, MB12_64B_WORD9, MB12_8B_WORD0, MB12_8B_WORD1, MB13_16B_CS, MB13_16B_ID,
    MB13_16B_WORD2, MB13_16B_WORD3, MB13_32B_CS, MB13_32B_ID, MB13_32B_WORD2, MB13_32B_WORD3,
    MB13_32B_WORD6, MB13_32B_WORD7, MB13_64B_CS, MB13_64B_ID, MB13_64B_WORD10, MB13_64B_WORD11,
    MB13_64B_WORD14, MB13_64B_WORD15, MB13_64B_WORD2, MB13_64B_WORD3, MB13_64B_WORD6,
    MB13_64B_WORD7, MB13_8B_WORD0, MB13_8B_WORD1, MB14_16B_WORD0, MB14_16B_WORD1, MB14_32B_WORD0,
    MB14_32B_WORD1, MB14_32B_WORD4, MB14_32B_WORD5, MB14_8B_WORD0, MB14_8B_WORD1, MB15_16B_CS,
    MB15_16B_ID, MB15_16B_WORD2, MB15_16B_WORD3, MB15_32B_CS, MB15_32B_ID, MB15_32B_WORD2,
    MB15_32B_WORD3, MB15_32B_WORD6, MB15_32B_WORD7, MB16_16B_WORD0, MB16_16B_WORD1, MB16_32B_WORD0,
    MB16_32B_WORD1, MB16_32B_WORD4, MB16_32B_WORD5, MB17_32B_CS, MB17_32B_ID, MB17_32B_WORD2,
    MB17_32B_WORD3, MB17_32B_WORD6, MB17_32B_WORD7, MB1_32B_WORD7, MB1_64B_WORD10, MB1_64B_WORD11,
    MB1_64B_WORD14, MB1_64B_WORD15, MB1_64B_WORD2, MB1_64B_WORD3, MB1_64B_WORD6, MB1_64B_WORD7,
    MB2_64B_WORD0, MB2_64B_WORD1, MB63_8B_WORD0, MB63_8B_WORD1, MCR, RX14MASK, RX15MASK, RXFGMASK,
    RXFIR, RXIMR0, RXIMR1, RXIMR10, RXIMR11, RXIMR12, RXIMR13, RXIMR14, RXIMR15, RXIMR16, RXIMR17,
    RXIMR18, RXIMR19, RXIMR2, RXIMR20, RXIMR21, RXIMR22, RXIMR23, RXIMR24, RXIMR25, RXIMR26,
    RXIMR27, RXIMR28, RXIMR29, RXIMR3, RXIMR30, RXIMR31, RXIMR32, RXIMR33, RXIMR34, RXIMR35,
    RXIMR36, RXIMR37, RXIMR38, RXIMR39, RXIMR4, RXIMR40, RXIMR41, RXIMR42, RXIMR43, RXIMR44,
    RXIMR45, RXIMR46, RXIMR47, RXIMR48, RXIMR49, RXIMR5, RXIMR50, RXIMR51, RXIMR52, RXIMR53,
    RXIMR54, RXIMR55, RXIMR56, RXIMR57, RXIMR58, RXIMR59, RXIMR6, RXIMR60, RXIMR61, RXIMR62,
    RXIMR63, RXIMR7, RXIMR8, RXIMR9, RXMGMASK, TIMER,
};

/// Access functions for the CAN3 peripheral instance
pub mod CAN3 {
    use super::ResetValues;
    #[cfg(not(feature = "nosync"))]
    use core::sync::atomic::{AtomicBool, Ordering};

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x401d8000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in CAN3
    pub const reset: ResetValues = ResetValues {
        MCR: 0x5980000F,
        CTRL1: 0x00000000,
        TIMER: 0x00000000,
        RXMGMASK: 0x00000000,
        RX14MASK: 0x00000000,
        RX15MASK: 0x00000000,
        ECR: 0x00000000,
        ESR1: 0x00000000,
        IMASK2: 0x00000000,
        IMASK1: 0x00000000,
        IFLAG2: 0x00000000,
        IFLAG1: 0x00000000,
        CTRL2: 0x00800000,
        ESR2: 0x00000000,
        CRCR: 0x00000000,
        RXFGMASK: 0x00000000,
        RXFIR: 0x00000000,
        CBT: 0x00000000,
        CS0: 0x00000000,
        ID0: 0x00000000,
        MB0: 0x00000000,
        MB0_16B_WORD1: 0x00000000,
        CS1: 0x00000000,
        ID1: 0x00000000,
        MB: 0x00000000,
        MB0_32B_WORD5: 0x00000000,
        CS2: 0x00000000,
        ID2: 0x00000000,
        MB0_64B_WORD8: 0x00000000,
        MB0_64B_WORD9: 0x00000000,
        CS3: 0x00000000,
        ID3: 0x00000000,
        MB0_64B_WORD12: 0x00000000,
        MB0_64B_WORD13: 0x00000000,
        CS4: 0x00000000,
        ID4: 0x00000000,
        MB1: 0x00000000,
        MB1_32B_WORD7: 0x00000000,
        CS5: 0x00000000,
        ID5: 0x00000000,
        MB1_64B_WORD2: 0x00000000,
        MB1_64B_WORD3: 0x00000000,
        CS6: 0x00000000,
        ID6: 0x00000000,
        MB1_64B_WORD6: 0x00000000,
        MB1_64B_WORD7: 0x00000000,
        CS7: 0x00000000,
        ID7: 0x00000000,
        MB1_64B_WORD10: 0x00000000,
        MB1_64B_WORD11: 0x00000000,
        CS8: 0x00000000,
        ID8: 0x00000000,
        MB1_64B_WORD14: 0x00000000,
        MB1_64B_WORD15: 0x00000000,
        CS9: 0x00000000,
        ID9: 0x00000000,
        MB2_64B_WORD0: 0x00000000,
        MB2_64B_WORD1: 0x00000000,
        CS10: 0x00000000,
        ID10: 0x00000000,
        MB10_8B_WORD0: 0x00000000,
        MB10_8B_WORD1: 0x00000000,
        CS11: 0x00000000,
        ID11: 0x00000000,
        MB11_8B_WORD0: 0x00000000,
        MB11_8B_WORD1: 0x00000000,
        CS12: 0x00000000,
        ID12: 0x00000000,
        MB12_8B_WORD0: 0x00000000,
        MB12_8B_WORD1: 0x00000000,
        CS13: 0x00000000,
        ID13: 0x00000000,
        MB13_8B_WORD0: 0x00000000,
        MB13_8B_WORD1: 0x00000000,
        CS14: 0x00000000,
        ID14: 0x00000000,
        MB14_8B_WORD0: 0x00000000,
        MB14_8B_WORD1: 0x00000000,
        CS15: 0x00000000,
        ID15: 0x00000000,
        MB10_16B_WORD0: 0x00000000,
        MB10_16B_WORD1: 0x00000000,
        CS16: 0x00000000,
        ID16: 0x00000000,
        MB11_16B_CS: 0x00000000,
        MB11_16B_ID: 0x00000000,
        CS17: 0x00000000,
        ID17: 0x00000000,
        MB11_16B_WORD2: 0x00000000,
        MB11_16B_WORD3: 0x00000000,
        CS18: 0x00000000,
        ID18: 0x00000000,
        MB12_16B_WORD0: 0x00000000,
        MB12_16B_WORD1: 0x00000000,
        CS19: 0x00000000,
        ID19: 0x00000000,
        MB13_16B_CS: 0x00000000,
        MB13_16B_ID: 0x00000000,
        CS20: 0x00000000,
        ID20: 0x00000000,
        MB13_16B_WORD2: 0x00000000,
        MB13_16B_WORD3: 0x00000000,
        CS21: 0x00000000,
        ID21: 0x00000000,
        MB14_16B_WORD0: 0x00000000,
        MB14_16B_WORD1: 0x00000000,
        CS22: 0x00000000,
        ID22: 0x00000000,
        MB15_16B_CS: 0x00000000,
        MB15_16B_ID: 0x00000000,
        CS23: 0x00000000,
        ID23: 0x00000000,
        MB15_16B_WORD2: 0x00000000,
        MB15_16B_WORD3: 0x00000000,
        CS24: 0x00000000,
        ID24: 0x00000000,
        MB16_16B_WORD0: 0x00000000,
        MB16_16B_WORD1: 0x00000000,
        CS25: 0x00000000,
        ID25: 0x00000000,
        MB10_32B_WORD0: 0x00000000,
        MB10_32B_WORD1: 0x00000000,
        CS26: 0x00000000,
        ID26: 0x00000000,
        MB10_32B_WORD4: 0x00000000,
        MB10_32B_WORD5: 0x00000000,
        CS27: 0x00000000,
        ID27: 0x00000000,
        MB11_32B_CS: 0x00000000,
        MB11_32B_ID: 0x00000000,
        CS28: 0x00000000,
        ID28: 0x00000000,
        MB11_32B_WORD2: 0x00000000,
        MB11_32B_WORD3: 0x00000000,
        CS29: 0x00000000,
        ID29: 0x00000000,
        MB11_32B_WORD6: 0x00000000,
        MB11_32B_WORD7: 0x00000000,
        CS30: 0x00000000,
        ID30: 0x00000000,
        MB12_32B_WORD0: 0x00000000,
        MB12_32B_WORD1: 0x00000000,
        CS31: 0x00000000,
        ID31: 0x00000000,
        MB12_32B_WORD4: 0x00000000,
        MB12_32B_WORD5: 0x00000000,
        CS32: 0x00000000,
        ID32: 0x00000000,
        MB13_32B_CS: 0x00000000,
        MB13_32B_ID: 0x00000000,
        CS33: 0x00000000,
        ID33: 0x00000000,
        MB13_32B_WORD2: 0x00000000,
        MB13_32B_WORD3: 0x00000000,
        CS34: 0x00000000,
        ID34: 0x00000000,
        MB13_32B_WORD6: 0x00000000,
        MB13_32B_WORD7: 0x00000000,
        CS35: 0x00000000,
        ID35: 0x00000000,
        MB14_32B_WORD0: 0x00000000,
        MB14_32B_WORD1: 0x00000000,
        CS36: 0x00000000,
        ID36: 0x00000000,
        MB14_32B_WORD4: 0x00000000,
        MB14_32B_WORD5: 0x00000000,
        CS37: 0x00000000,
        ID37: 0x00000000,
        MB15_32B_CS: 0x00000000,
        MB15_32B_ID: 0x00000000,
        CS38: 0x00000000,
        ID38: 0x00000000,
        MB15_32B_WORD2: 0x00000000,
        MB15_32B_WORD3: 0x00000000,
        CS39: 0x00000000,
        ID39: 0x00000000,
        MB15_32B_WORD6: 0x00000000,
        MB15_32B_WORD7: 0x00000000,
        CS40: 0x00000000,
        ID40: 0x00000000,
        MB16_32B_WORD0: 0x00000000,
        MB16_32B_WORD1: 0x00000000,
        CS41: 0x00000000,
        ID41: 0x00000000,
        MB16_32B_WORD4: 0x00000000,
        MB16_32B_WORD5: 0x00000000,
        CS42: 0x00000000,
        ID42: 0x00000000,
        MB17_32B_CS: 0x00000000,
        MB17_32B_ID: 0x00000000,
        CS43: 0x00000000,
        ID43: 0x00000000,
        MB17_32B_WORD2: 0x00000000,
        MB17_32B_WORD3: 0x00000000,
        CS44: 0x00000000,
        ID44: 0x00000000,
        MB17_32B_WORD6: 0x00000000,
        MB17_32B_WORD7: 0x00000000,
        CS45: 0x00000000,
        ID45: 0x00000000,
        MB10_64B_WORD0: 0x00000000,
        MB10_64B_WORD1: 0x00000000,
        CS46: 0x00000000,
        ID46: 0x00000000,
        MB10_64B_WORD4: 0x00000000,
        MB10_64B_WORD5: 0x00000000,
        CS47: 0x00000000,
        ID47: 0x00000000,
        MB10_64B_WORD8: 0x00000000,
        MB10_64B_WORD9: 0x00000000,
        CS48: 0x00000000,
        ID48: 0x00000000,
        MB10_64B_WORD12: 0x00000000,
        MB10_64B_WORD13: 0x00000000,
        CS49: 0x00000000,
        ID49: 0x00000000,
        MB11_64B_CS: 0x00000000,
        MB11_64B_ID: 0x00000000,
        CS50: 0x00000000,
        ID50: 0x00000000,
        MB11_64B_WORD2: 0x00000000,
        MB11_64B_WORD3: 0x00000000,
        CS51: 0x00000000,
        ID51: 0x00000000,
        MB11_64B_WORD6: 0x00000000,
        MB11_64B_WORD7: 0x00000000,
        CS52: 0x00000000,
        ID52: 0x00000000,
        MB11_64B_WORD10: 0x00000000,
        MB11_64B_WORD11: 0x00000000,
        CS53: 0x00000000,
        ID53: 0x00000000,
        MB11_64B_WORD14: 0x00000000,
        MB11_64B_WORD15: 0x00000000,
        CS54: 0x00000000,
        ID54: 0x00000000,
        MB12_64B_WORD0: 0x00000000,
        MB12_64B_WORD1: 0x00000000,
        CS55: 0x00000000,
        ID55: 0x00000000,
        MB12_64B_WORD4: 0x00000000,
        MB12_64B_WORD5: 0x00000000,
        CS56: 0x00000000,
        ID56: 0x00000000,
        MB12_64B_WORD8: 0x00000000,
        MB12_64B_WORD9: 0x00000000,
        CS57: 0x00000000,
        ID57: 0x00000000,
        MB12_64B_WORD12: 0x00000000,
        MB12_64B_WORD13: 0x00000000,
        CS58: 0x00000000,
        ID58: 0x00000000,
        MB13_64B_CS: 0x00000000,
        MB13_64B_ID: 0x00000000,
        CS59: 0x00000000,
        ID59: 0x00000000,
        MB13_64B_WORD2: 0x00000000,
        MB13_64B_WORD3: 0x00000000,
        CS60: 0x00000000,
        ID60: 0x00000000,
        MB13_64B_WORD6: 0x00000000,
        MB13_64B_WORD7: 0x00000000,
        CS61: 0x00000000,
        ID61: 0x00000000,
        MB13_64B_WORD10: 0x00000000,
        MB13_64B_WORD11: 0x00000000,
        CS62: 0x00000000,
        ID62: 0x00000000,
        MB13_64B_WORD14: 0x00000000,
        MB13_64B_WORD15: 0x00000000,
        CS63: 0x00000000,
        ID63: 0x00000000,
        MB63_8B_WORD0: 0x00000000,
        MB63_8B_WORD1: 0x00000000,
        RXIMR0: 0x00000000,
        RXIMR1: 0x00000000,
        RXIMR2: 0x00000000,
        RXIMR3: 0x00000000,
        RXIMR4: 0x00000000,
        RXIMR5: 0x00000000,
        RXIMR6: 0x00000000,
        RXIMR7: 0x00000000,
        RXIMR8: 0x00000000,
        RXIMR9: 0x00000000,
        RXIMR10: 0x00000000,
        RXIMR11: 0x00000000,
        RXIMR12: 0x00000000,
        RXIMR13: 0x00000000,
        RXIMR14: 0x00000000,
        RXIMR15: 0x00000000,
        RXIMR16: 0x00000000,
        RXIMR17: 0x00000000,
        RXIMR18: 0x00000000,
        RXIMR19: 0x00000000,
        RXIMR20: 0x00000000,
        RXIMR21: 0x00000000,
        RXIMR22: 0x00000000,
        RXIMR23: 0x00000000,
        RXIMR24: 0x00000000,
        RXIMR25: 0x00000000,
        RXIMR26: 0x00000000,
        RXIMR27: 0x00000000,
        RXIMR28: 0x00000000,
        RXIMR29: 0x00000000,
        RXIMR30: 0x00000000,
        RXIMR31: 0x00000000,
        RXIMR32: 0x00000000,
        RXIMR33: 0x00000000,
        RXIMR34: 0x00000000,
        RXIMR35: 0x00000000,
        RXIMR36: 0x00000000,
        RXIMR37: 0x00000000,
        RXIMR38: 0x00000000,
        RXIMR39: 0x00000000,
        RXIMR40: 0x00000000,
        RXIMR41: 0x00000000,
        RXIMR42: 0x00000000,
        RXIMR43: 0x00000000,
        RXIMR44: 0x00000000,
        RXIMR45: 0x00000000,
        RXIMR46: 0x00000000,
        RXIMR47: 0x00000000,
        RXIMR48: 0x00000000,
        RXIMR49: 0x00000000,
        RXIMR50: 0x00000000,
        RXIMR51: 0x00000000,
        RXIMR52: 0x00000000,
        RXIMR53: 0x00000000,
        RXIMR54: 0x00000000,
        RXIMR55: 0x00000000,
        RXIMR56: 0x00000000,
        RXIMR57: 0x00000000,
        RXIMR58: 0x00000000,
        RXIMR59: 0x00000000,
        RXIMR60: 0x00000000,
        RXIMR61: 0x00000000,
        RXIMR62: 0x00000000,
        RXIMR63: 0x00000000,
        EPRS: 0x00000000,
        ENCBT: 0x00000000,
        EDCBT: 0x00000000,
        ETDC: 0x00000000,
        FDCTRL: 0x80000100,
        FDCBT: 0x00000000,
        FDCRC: 0x00000000,
        ERFCR: 0x00000000,
        ERFIER: 0x00000000,
        ERFSR: 0x00000000,
        HR_TIME_STAMP0: 0x00000000,
        HR_TIME_STAMP1: 0x00000000,
        HR_TIME_STAMP2: 0x00000000,
        HR_TIME_STAMP3: 0x00000000,
        HR_TIME_STAMP4: 0x00000000,
        HR_TIME_STAMP5: 0x00000000,
        HR_TIME_STAMP6: 0x00000000,
        HR_TIME_STAMP7: 0x00000000,
        HR_TIME_STAMP8: 0x00000000,
        HR_TIME_STAMP9: 0x00000000,
        HR_TIME_STAMP10: 0x00000000,
        HR_TIME_STAMP11: 0x00000000,
        HR_TIME_STAMP12: 0x00000000,
        HR_TIME_STAMP13: 0x00000000,
        HR_TIME_STAMP14: 0x00000000,
        HR_TIME_STAMP15: 0x00000000,
        HR_TIME_STAMP16: 0x00000000,
        HR_TIME_STAMP17: 0x00000000,
        HR_TIME_STAMP18: 0x00000000,
        HR_TIME_STAMP19: 0x00000000,
        HR_TIME_STAMP20: 0x00000000,
        HR_TIME_STAMP21: 0x00000000,
        HR_TIME_STAMP22: 0x00000000,
        HR_TIME_STAMP23: 0x00000000,
        HR_TIME_STAMP24: 0x00000000,
        HR_TIME_STAMP25: 0x00000000,
        HR_TIME_STAMP26: 0x00000000,
        HR_TIME_STAMP27: 0x00000000,
        HR_TIME_STAMP28: 0x00000000,
        HR_TIME_STAMP29: 0x00000000,
        HR_TIME_STAMP30: 0x00000000,
        HR_TIME_STAMP31: 0x00000000,
        HR_TIME_STAMP32: 0x00000000,
        HR_TIME_STAMP33: 0x00000000,
        HR_TIME_STAMP34: 0x00000000,
        HR_TIME_STAMP35: 0x00000000,
        HR_TIME_STAMP36: 0x00000000,
        HR_TIME_STAMP37: 0x00000000,
        HR_TIME_STAMP38: 0x00000000,
        HR_TIME_STAMP39: 0x00000000,
        HR_TIME_STAMP40: 0x00000000,
        HR_TIME_STAMP41: 0x00000000,
        HR_TIME_STAMP42: 0x00000000,
        HR_TIME_STAMP43: 0x00000000,
        HR_TIME_STAMP44: 0x00000000,
        HR_TIME_STAMP45: 0x00000000,
        HR_TIME_STAMP46: 0x00000000,
        HR_TIME_STAMP47: 0x00000000,
        HR_TIME_STAMP48: 0x00000000,
        HR_TIME_STAMP49: 0x00000000,
        HR_TIME_STAMP50: 0x00000000,
        HR_TIME_STAMP51: 0x00000000,
        HR_TIME_STAMP52: 0x00000000,
        HR_TIME_STAMP53: 0x00000000,
        HR_TIME_STAMP54: 0x00000000,
        HR_TIME_STAMP55: 0x00000000,
        HR_TIME_STAMP56: 0x00000000,
        HR_TIME_STAMP57: 0x00000000,
        HR_TIME_STAMP58: 0x00000000,
        HR_TIME_STAMP59: 0x00000000,
        HR_TIME_STAMP60: 0x00000000,
        HR_TIME_STAMP61: 0x00000000,
        HR_TIME_STAMP62: 0x00000000,
        HR_TIME_STAMP63: 0x00000000,
        ERFFEL0: 0x00000000,
        ERFFEL1: 0x00000000,
        ERFFEL2: 0x00000000,
        ERFFEL3: 0x00000000,
        ERFFEL4: 0x00000000,
        ERFFEL5: 0x00000000,
        ERFFEL6: 0x00000000,
        ERFFEL7: 0x00000000,
        ERFFEL8: 0x00000000,
        ERFFEL9: 0x00000000,
        ERFFEL10: 0x00000000,
        ERFFEL11: 0x00000000,
        ERFFEL12: 0x00000000,
        ERFFEL13: 0x00000000,
        ERFFEL14: 0x00000000,
        ERFFEL15: 0x00000000,
        ERFFEL16: 0x00000000,
        ERFFEL17: 0x00000000,
        ERFFEL18: 0x00000000,
        ERFFEL19: 0x00000000,
        ERFFEL20: 0x00000000,
        ERFFEL21: 0x00000000,
        ERFFEL22: 0x00000000,
        ERFFEL23: 0x00000000,
        ERFFEL24: 0x00000000,
        ERFFEL25: 0x00000000,
        ERFFEL26: 0x00000000,
        ERFFEL27: 0x00000000,
        ERFFEL28: 0x00000000,
        ERFFEL29: 0x00000000,
        ERFFEL30: 0x00000000,
        ERFFEL31: 0x00000000,
        ERFFEL32: 0x00000000,
        ERFFEL33: 0x00000000,
        ERFFEL34: 0x00000000,
        ERFFEL35: 0x00000000,
        ERFFEL36: 0x00000000,
        ERFFEL37: 0x00000000,
        ERFFEL38: 0x00000000,
        ERFFEL39: 0x00000000,
        ERFFEL40: 0x00000000,
        ERFFEL41: 0x00000000,
        ERFFEL42: 0x00000000,
        ERFFEL43: 0x00000000,
        ERFFEL44: 0x00000000,
        ERFFEL45: 0x00000000,
        ERFFEL46: 0x00000000,
        ERFFEL47: 0x00000000,
        ERFFEL48: 0x00000000,
        ERFFEL49: 0x00000000,
        ERFFEL50: 0x00000000,
        ERFFEL51: 0x00000000,
        ERFFEL52: 0x00000000,
        ERFFEL53: 0x00000000,
        ERFFEL54: 0x00000000,
        ERFFEL55: 0x00000000,
        ERFFEL56: 0x00000000,
        ERFFEL57: 0x00000000,
        ERFFEL58: 0x00000000,
        ERFFEL59: 0x00000000,
        ERFFEL60: 0x00000000,
        ERFFEL61: 0x00000000,
        ERFFEL62: 0x00000000,
        ERFFEL63: 0x00000000,
        ERFFEL64: 0x00000000,
        ERFFEL65: 0x00000000,
        ERFFEL66: 0x00000000,
        ERFFEL67: 0x00000000,
        ERFFEL68: 0x00000000,
        ERFFEL69: 0x00000000,
        ERFFEL70: 0x00000000,
        ERFFEL71: 0x00000000,
        ERFFEL72: 0x00000000,
        ERFFEL73: 0x00000000,
        ERFFEL74: 0x00000000,
        ERFFEL75: 0x00000000,
        ERFFEL76: 0x00000000,
        ERFFEL77: 0x00000000,
        ERFFEL78: 0x00000000,
        ERFFEL79: 0x00000000,
        ERFFEL80: 0x00000000,
        ERFFEL81: 0x00000000,
        ERFFEL82: 0x00000000,
        ERFFEL83: 0x00000000,
        ERFFEL84: 0x00000000,
        ERFFEL85: 0x00000000,
        ERFFEL86: 0x00000000,
        ERFFEL87: 0x00000000,
        ERFFEL88: 0x00000000,
        ERFFEL89: 0x00000000,
        ERFFEL90: 0x00000000,
        ERFFEL91: 0x00000000,
        ERFFEL92: 0x00000000,
        ERFFEL93: 0x00000000,
        ERFFEL94: 0x00000000,
        ERFFEL95: 0x00000000,
        ERFFEL96: 0x00000000,
        ERFFEL97: 0x00000000,
        ERFFEL98: 0x00000000,
        ERFFEL99: 0x00000000,
        ERFFEL100: 0x00000000,
        ERFFEL101: 0x00000000,
        ERFFEL102: 0x00000000,
        ERFFEL103: 0x00000000,
        ERFFEL104: 0x00000000,
        ERFFEL105: 0x00000000,
        ERFFEL106: 0x00000000,
        ERFFEL107: 0x00000000,
        ERFFEL108: 0x00000000,
        ERFFEL109: 0x00000000,
        ERFFEL110: 0x00000000,
        ERFFEL111: 0x00000000,
        ERFFEL112: 0x00000000,
        ERFFEL113: 0x00000000,
        ERFFEL114: 0x00000000,
        ERFFEL115: 0x00000000,
        ERFFEL116: 0x00000000,
        ERFFEL117: 0x00000000,
        ERFFEL118: 0x00000000,
        ERFFEL119: 0x00000000,
        ERFFEL120: 0x00000000,
        ERFFEL121: 0x00000000,
        ERFFEL122: 0x00000000,
        ERFFEL123: 0x00000000,
        ERFFEL124: 0x00000000,
        ERFFEL125: 0x00000000,
        ERFFEL126: 0x00000000,
        ERFFEL127: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static CAN3_TAKEN: AtomicBool = AtomicBool::new(false);

    /// Safe access to CAN3
    ///
    /// This function returns `Some(Instance)` if this instance is not
    /// currently taken, and `None` if it is. This ensures that if you
    /// do get `Some(Instance)`, you are ensured unique access to
    /// the peripheral and there cannot be data races (unless other
    /// code uses `unsafe`, of course). You can then pass the
    /// `Instance` around to other functions as required. When you're
    /// done with it, you can call `release(instance)` to return it.
    ///
    /// `Instance` itself dereferences to a `RegisterBlock`, which
    /// provides access to the peripheral's registers.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn take() -> Option<Instance> {
        let taken = CAN3_TAKEN.swap(true, Ordering::SeqCst);
        if taken {
            None
        } else {
            Some(INSTANCE)
        }
    }

    /// Release exclusive access to CAN3
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        assert!(inst.addr == INSTANCE.addr, "Released the wrong instance");

        let taken = CAN3_TAKEN.swap(false, Ordering::SeqCst);
        assert!(taken, "Released a peripheral which was not taken");
    }

    /// Unsafely steal CAN3
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        CAN3_TAKEN.store(true, Ordering::SeqCst);
        INSTANCE
    }
}

/// Raw pointer to CAN3
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const CAN3: *const RegisterBlock = 0x401d8000 as *const _;
