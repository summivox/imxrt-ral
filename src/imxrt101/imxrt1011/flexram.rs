#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! FLEXRAM

use crate::RWRegister;

#[cfg(not(feature = "nosync"))]
use core::sync::atomic::{AtomicBool, Ordering};
/// TCM CRTL Register
pub mod TCM_CTRL {

    /// TCM Write Wait Mode Enable
    pub mod TCM_WWAIT_EN {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: TCM write fast mode: Write RAM accesses are expected to be finished in 1-cycle.
            pub const TCM_WWAIT_EN_0: u32 = 0b0;

            /// 0b1: TCM write wait mode: Write RAM accesses are expected to be finished in 2-cycles.
            pub const TCM_WWAIT_EN_1: u32 = 0b1;
        }
    }

    /// TCM Read Wait Mode Enable
    pub mod TCM_RWAIT_EN {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: TCM read fast mode: Read RAM accesses are expected to be finished in 1-cycle.
            pub const TCM_RWAIT_EN_0: u32 = 0b0;

            /// 0b1: TCM read wait mode: Read RAM accesses are expected to be finished in 2-cycles.
            pub const TCM_RWAIT_EN_1: u32 = 0b1;
        }
    }

    /// Force RAM Clock Always On
    pub mod FORCE_CLK_ON {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// OCRAM Magic Address Register
pub mod OCRAM_MAGIC_ADDR {

    /// OCRAM Write Read Select
    pub mod OCRAM_WR_RD_SEL {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: When OCRAM read access hits magic address, it will generate interrupt.
            pub const OCRAM_WR_RD_SEL_0: u32 = 0b0;

            /// 0b1: When OCRAM write access hits magic address, it will generate interrupt.
            pub const OCRAM_WR_RD_SEL_1: u32 = 0b1;
        }
    }

    /// OCRAM Magic Address
    pub mod OCRAM_MAGIC_ADDR {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (14 bits: 0x3fff << 1)
        pub const mask: u32 = 0x3fff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// DTCM Magic Address Register
pub mod DTCM_MAGIC_ADDR {

    /// DTCM Write Read Select
    pub mod DTCM_WR_RD_SEL {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: When DTCM read access hits magic address, it will generate interrupt.
            pub const DTCM_WR_RD_SEL_0: u32 = 0b0;

            /// 0b1: When DTCM write access hits magic address, it will generate interrupt.
            pub const DTCM_WR_RD_SEL_1: u32 = 0b1;
        }
    }

    /// DTCM Magic Address
    pub mod DTCM_MAGIC_ADDR {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (14 bits: 0x3fff << 1)
        pub const mask: u32 = 0x3fff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// ITCM Magic Address Register
pub mod ITCM_MAGIC_ADDR {

    /// ITCM Write Read Select
    pub mod ITCM_WR_RD_SEL {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: When ITCM read access hits magic address, it will generate interrupt.
            pub const ITCM_WR_RD_SEL_0: u32 = 0b0;

            /// 0b1: When ITCM write access hits magic address, it will generate interrupt.
            pub const ITCM_WR_RD_SEL_1: u32 = 0b1;
        }
    }

    /// ITCM Magic Address
    pub mod ITCM_MAGIC_ADDR {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (14 bits: 0x3fff << 1)
        pub const mask: u32 = 0x3fff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Status Register
pub mod INT_STATUS {

    /// ITCM Magic Address Match Status
    pub mod ITCM_MAM_STATUS {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: ITCM did not access magic address.
            pub const ITCM_MAM_STATUS_0: u32 = 0b0;

            /// 0b1: ITCM accessed magic address.
            pub const ITCM_MAM_STATUS_1: u32 = 0b1;
        }
    }

    /// DTCM Magic Address Match Status
    pub mod DTCM_MAM_STATUS {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: DTCM did not access magic address.
            pub const DTCM_MAM_STATUS_0: u32 = 0b0;

            /// 0b1: DTCM accessed magic address.
            pub const DTCM_MAM_STATUS_1: u32 = 0b1;
        }
    }

    /// OCRAM Magic Address Match Status
    pub mod OCRAM_MAM_STATUS {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: OCRAM did not access magic address.
            pub const OCRAM_MAM_STATUS_0: u32 = 0b0;

            /// 0b1: OCRAM accessed magic address.
            pub const OCRAM_MAM_STATUS_1: u32 = 0b1;
        }
    }

    /// ITCM Access Error Status
    pub mod ITCM_ERR_STATUS {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: ITCM access error does not happen
            pub const ITCM_ERR_STATUS_0: u32 = 0b0;

            /// 0b1: ITCM access error happens.
            pub const ITCM_ERR_STATUS_1: u32 = 0b1;
        }
    }

    /// DTCM Access Error Status
    pub mod DTCM_ERR_STATUS {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: DTCM access error does not happen
            pub const DTCM_ERR_STATUS_0: u32 = 0b0;

            /// 0b1: DTCM access error happens.
            pub const DTCM_ERR_STATUS_1: u32 = 0b1;
        }
    }

    /// OCRAM Access Error Status
    pub mod OCRAM_ERR_STATUS {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: OCRAM access error does not happen
            pub const OCRAM_ERR_STATUS_0: u32 = 0b0;

            /// 0b1: OCRAM access error happens.
            pub const OCRAM_ERR_STATUS_1: u32 = 0b1;
        }
    }
}

/// Interrupt Status Enable Register
pub mod INT_STAT_EN {

    /// ITCM Magic Address Match Status Enable
    pub mod ITCM_MAM_STAT_EN {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Masked
            pub const ITCM_MAM_STAT_EN_0: u32 = 0b0;

            /// 0b1: Enabled
            pub const ITCM_MAM_STAT_EN_1: u32 = 0b1;
        }
    }

    /// DTCM Magic Address Match Status Enable
    pub mod DTCM_MAM_STAT_EN {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Masked
            pub const DTCM_MAM_STAT_EN_0: u32 = 0b0;

            /// 0b1: Enabled
            pub const DTCM_MAM_STAT_EN_1: u32 = 0b1;
        }
    }

    /// OCRAM Magic Address Match Status Enable
    pub mod OCRAM_MAM_STAT_EN {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Masked
            pub const OCRAM_MAM_STAT_EN_0: u32 = 0b0;

            /// 0b1: Enabled
            pub const OCRAM_MAM_STAT_EN_1: u32 = 0b1;
        }
    }

    /// ITCM Access Error Status Enable
    pub mod ITCM_ERR_STAT_EN {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Masked
            pub const ITCM_ERR_STAT_EN_0: u32 = 0b0;

            /// 0b1: Enabled
            pub const ITCM_ERR_STAT_EN_1: u32 = 0b1;
        }
    }

    /// DTCM Access Error Status Enable
    pub mod DTCM_ERR_STAT_EN {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Masked
            pub const DTCM_ERR_STAT_EN_0: u32 = 0b0;

            /// 0b1: Enabled
            pub const DTCM_ERR_STAT_EN_1: u32 = 0b1;
        }
    }

    /// OCRAM Access Error Status Enable
    pub mod OCRAM_ERR_STAT_EN {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Masked
            pub const OCRAM_ERR_STAT_EN_0: u32 = 0b0;

            /// 0b1: Enabled
            pub const OCRAM_ERR_STAT_EN_1: u32 = 0b1;
        }
    }
}

/// Interrupt Enable Register
pub mod INT_SIG_EN {

    /// ITCM Magic Address Match Interrupt Enable
    pub mod ITCM_MAM_SIG_EN {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Masked
            pub const ITCM_MAM_SIG_EN_0: u32 = 0b0;

            /// 0b1: Enabled
            pub const ITCM_MAM_SIG_EN_1: u32 = 0b1;
        }
    }

    /// DTCM Magic Address Match Interrupt Enable
    pub mod DTCM_MAM_SIG_EN {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Masked
            pub const DTCM_MAM_SIG_EN_0: u32 = 0b0;

            /// 0b1: Enabled
            pub const DTCM_MAM_SIG_EN_1: u32 = 0b1;
        }
    }

    /// OCRAM Magic Address Match Interrupt Enable
    pub mod OCRAM_MAM_SIG_EN {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Masked
            pub const OCRAM_MAM_SIG_EN_0: u32 = 0b0;

            /// 0b1: Enabled
            pub const OCRAM_MAM_SIG_EN_1: u32 = 0b1;
        }
    }

    /// ITCM Access Error Interrupt Enable
    pub mod ITCM_ERR_SIG_EN {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Masked
            pub const ITCM_ERR_SIG_EN_0: u32 = 0b0;

            /// 0b1: Enabled
            pub const ITCM_ERR_SIG_EN_1: u32 = 0b1;
        }
    }

    /// DTCM Access Error Interrupt Enable
    pub mod DTCM_ERR_SIG_EN {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Masked
            pub const DTCM_ERR_SIG_EN_0: u32 = 0b0;

            /// 0b1: Enabled
            pub const DTCM_ERR_SIG_EN_1: u32 = 0b1;
        }
    }

    /// OCRAM Access Error Interrupt Enable
    pub mod OCRAM_ERR_SIG_EN {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Masked
            pub const OCRAM_ERR_SIG_EN_0: u32 = 0b0;

            /// 0b1: Enabled
            pub const OCRAM_ERR_SIG_EN_1: u32 = 0b1;
        }
    }
}
#[repr(C)]
pub struct RegisterBlock {
    /// TCM CRTL Register
    pub TCM_CTRL: RWRegister<u32>,

    /// OCRAM Magic Address Register
    pub OCRAM_MAGIC_ADDR: RWRegister<u32>,

    /// DTCM Magic Address Register
    pub DTCM_MAGIC_ADDR: RWRegister<u32>,

    /// ITCM Magic Address Register
    pub ITCM_MAGIC_ADDR: RWRegister<u32>,

    /// Interrupt Status Register
    pub INT_STATUS: RWRegister<u32>,

    /// Interrupt Status Enable Register
    pub INT_STAT_EN: RWRegister<u32>,

    /// Interrupt Enable Register
    pub INT_SIG_EN: RWRegister<u32>,
}
pub struct ResetValues {
    pub TCM_CTRL: u32,
    pub OCRAM_MAGIC_ADDR: u32,
    pub DTCM_MAGIC_ADDR: u32,
    pub ITCM_MAGIC_ADDR: u32,
    pub INT_STATUS: u32,
    pub INT_STAT_EN: u32,
    pub INT_SIG_EN: u32,
}
pub struct Instance<const N: u8> {
    #[cfg_attr(feature = "nosync", allow(unused))]
    pub(crate) addr: u32,
    #[cfg_attr(feature = "nosync", allow(unused))]
    pub(crate) intrs: &'static [crate::Interrupt],
}

#[cfg(not(feature = "nosync"))]
impl<const N: u8> ::core::ops::Deref for Instance<N> {
    type Target = RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &RegisterBlock {
        unsafe { &*(self.addr as *const _) }
    }
}

#[cfg(not(feature = "nosync"))]
unsafe impl<const N: u8> Send for Instance<N> {}

#[cfg(not(feature = "nosync"))]
impl<const N: u8> Instance<N> {
    /// Return the interrupt signals associated with this
    /// peripheral instance
    ///
    /// Collection may be empty if there is no interrupt signal
    /// associated with the peripheral. There's no guarantee for
    /// interrupt signal ordering in the collection.
    #[inline(always)]
    pub const fn interrupts<'a>(&'a self) -> &'a [crate::Interrupt] {
        self.intrs
    }
}

pub(crate) mod private {
    pub trait Sealed {}
}

/// Describes a valid `Instance<N>` for this peripheral.
pub trait Valid: private::Sealed {}

/// The FLEXRAM peripheral instance.
#[cfg(not(feature = "doc"))]
pub type FLEXRAM = Instance<0>;

/// The FLEXRAM peripheral instance.
///
/// This is a new type only for documentation purposes. When
/// compiling for a target, this is defined as
///
/// ```rust
/// pub type FLEXRAM = Instance<0>;
/// ```
#[cfg(feature = "doc")]
pub struct FLEXRAM {
    #[allow(unused)] // Only for documentation generation.
    addr: u32,
}

impl private::Sealed for FLEXRAM {}
impl Valid for FLEXRAM {}

#[cfg(not(feature = "nosync"))]
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static FLEXRAM_TAKEN: AtomicBool = AtomicBool::new(false);

/// Access functions for the FLEXRAM peripheral instance
#[cfg(not(feature = "nosync"))]
impl FLEXRAM {
    const INSTANCE: Self = Self {
        addr: 0x400b0000,
        #[cfg(not(feature = "doc"))]
        intrs: &[crate::interrupt::FLEXRAM],
    };

    /// Reset values for each field in FLEXRAM
    pub const reset: ResetValues = ResetValues {
        TCM_CTRL: 0x00000000,
        OCRAM_MAGIC_ADDR: 0x00000000,
        DTCM_MAGIC_ADDR: 0x00000000,
        ITCM_MAGIC_ADDR: 0x00000000,
        INT_STATUS: 0x00000000,
        INT_STAT_EN: 0x00000000,
        INT_SIG_EN: 0x00000000,
    };

    /// Safe access to FLEXRAM
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
    #[inline]
    pub fn take() -> Option<Self> {
        let taken = FLEXRAM_TAKEN.swap(true, Ordering::SeqCst);
        if taken {
            None
        } else {
            Some(Self::INSTANCE)
        }
    }

    /// Release exclusive access to FLEXRAM
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[inline]
    pub fn release(_: Self) {
        let taken = FLEXRAM_TAKEN.swap(false, Ordering::SeqCst);
        assert!(taken, "Released a peripheral which was not taken");
    }

    /// Unsafely steal FLEXRAM
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[inline]
    pub unsafe fn steal() -> Self {
        FLEXRAM_TAKEN.store(true, Ordering::SeqCst);
        Self::INSTANCE
    }
}

impl FLEXRAM {
    /// The interrupts associated with FLEXRAM
    #[cfg(not(feature = "doc"))]
    pub const INTERRUPTS: [crate::Interrupt; 1] = [crate::interrupt::FLEXRAM];

    /// The interrupts associated with FLEXRAM
    ///
    /// Note: the values are invalid for a documentation build.
    #[cfg(feature = "doc")]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];
}

/// Raw pointer to FLEXRAM
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const FLEXRAM: *const RegisterBlock = 0x400b0000 as *const _;
