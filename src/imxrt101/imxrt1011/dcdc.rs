#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! DCDC

#[cfg(not(feature = "nosync"))]
pub use crate::imxrt101::peripherals::dcdc::Instance;
pub use crate::imxrt101::peripherals::dcdc::{RegisterBlock, ResetValues};
pub use crate::imxrt101::peripherals::dcdc::{REG0, REG1, REG2, REG3};

/// Access functions for the DCDC peripheral instance
pub mod DCDC {
    use super::ResetValues;
    #[cfg(not(feature = "nosync"))]
    use core::sync::atomic::{AtomicBool, Ordering};

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x40080000,
        _marker: ::core::marker::PhantomData,
        #[cfg(not(feature = "doc"))]
        intrs: &[crate::interrupt::DCDC],
        #[cfg(feature = "doc")]
        intrs: &[],
    };

    /// Reset values for each field in DCDC
    pub const reset: ResetValues = ResetValues {
        REG0: 0x14030111,
        REG1: 0x111BA29C,
        REG2: 0x00000009,
        REG3: 0x0000010E,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static DCDC_TAKEN: AtomicBool = AtomicBool::new(false);

    /// Safe access to DCDC
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
        let taken = DCDC_TAKEN.swap(true, Ordering::SeqCst);
        if taken {
            None
        } else {
            Some(INSTANCE)
        }
    }

    /// Release exclusive access to DCDC
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        assert!(inst.addr == INSTANCE.addr, "Released the wrong instance");

        let taken = DCDC_TAKEN.swap(false, Ordering::SeqCst);
        assert!(taken, "Released a peripheral which was not taken");
    }

    /// Unsafely steal DCDC
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        DCDC_TAKEN.store(true, Ordering::SeqCst);
        INSTANCE
    }

    /// The interrupts associated with DCDC
    #[cfg(not(feature = "doc"))]
    pub const INTERRUPTS: [crate::Interrupt; 1] = [crate::interrupt::DCDC];

    /// The interrupts associated with DCDC
    ///
    /// Note: the values are invalid for a documentation build.
    #[cfg(feature = "doc")]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];
}

/// Raw pointer to DCDC
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const DCDC: *const RegisterBlock = 0x40080000 as *const _;
