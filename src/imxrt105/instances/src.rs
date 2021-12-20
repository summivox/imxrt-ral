#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! SRC
//!
//! Used by: imxrt1051, imxrt1052

#[cfg(not(feature = "nosync"))]
use crate::imxrt105::peripherals::src::private;
#[cfg(not(feature = "nosync"))]
pub use crate::imxrt105::peripherals::src::{Instance, Valid};
pub use crate::imxrt105::peripherals::src::{RegisterBlock, ResetValues};
pub use crate::imxrt105::peripherals::src::{
    GPR1, GPR10, GPR2, GPR3, GPR4, GPR5, GPR6, GPR7, GPR8, GPR9, SBMR1, SBMR2, SCR, SRSR,
};
#[cfg(not(feature = "nosync"))]
use core::sync::atomic::{AtomicBool, Ordering};

/// The SRC peripheral instance.
#[cfg(all(not(feature = "nosync"), not(feature = "doc")))]
pub type SRC = Instance<0>;

/// The SRC peripheral instance.
///
/// This is a new type only for documentation purposes. When
/// compiling for a target, this is defined as
///
/// ```rust
/// pub type SRC = Instance<0>;
/// ```
#[cfg(all(not(feature = "nosync"), feature = "doc"))]
pub struct SRC {
    #[allow(unused)] // Only for documentation generation.
    addr: u32,
}

#[cfg(not(feature = "nosync"))]
impl private::Sealed for SRC {}
#[cfg(not(feature = "nosync"))]
impl Valid for SRC {}

#[cfg(not(feature = "nosync"))]
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static SRC_TAKEN: AtomicBool = AtomicBool::new(false);

/// Access functions for the SRC peripheral instance
#[cfg(not(feature = "nosync"))]
impl SRC {
    const INSTANCE: Self = Self {
        addr: 0x400f8000,
        #[cfg(not(feature = "doc"))]
        intrs: &[crate::interrupt::SRC],
    };

    /// Reset values for each field in SRC
    pub const reset: ResetValues = ResetValues {
        SCR: 0xA0480520,
        SBMR1: 0x00000000,
        SRSR: 0x00000001,
        SBMR2: 0x00000000,
        GPR1: 0x00000000,
        GPR2: 0x00000000,
        GPR3: 0x00000000,
        GPR4: 0x00000000,
        GPR5: 0x00000000,
        GPR6: 0x00000000,
        GPR7: 0x00000000,
        GPR8: 0x00000000,
        GPR9: 0x00000000,
        GPR10: 0x00000000,
    };

    /// Safe access to SRC
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
        let taken = SRC_TAKEN.swap(true, Ordering::SeqCst);
        if taken {
            None
        } else {
            Some(Self::INSTANCE)
        }
    }

    /// Release exclusive access to SRC
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[inline]
    pub fn release(_: Self) {
        let taken = SRC_TAKEN.swap(false, Ordering::SeqCst);
        assert!(taken, "Released a peripheral which was not taken");
    }

    /// Unsafely steal SRC
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[inline]
    pub unsafe fn steal() -> Self {
        SRC_TAKEN.store(true, Ordering::SeqCst);
        Self::INSTANCE
    }

    /// The interrupts associated with SRC
    #[cfg(not(feature = "doc"))]
    pub const INTERRUPTS: [crate::Interrupt; 1] = [crate::interrupt::SRC];

    /// The interrupts associated with SRC
    ///
    /// Note: the values are invalid for a documentation build.
    #[cfg(feature = "doc")]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];
}

/// Raw pointer to SRC
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const SRC: *const RegisterBlock = 0x400f8000 as *const _;
