#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! LPI2C
//!
//! Used by: imxrt1051, imxrt1052

#[cfg(not(feature = "nosync"))]
pub use crate::imxrt105::peripherals::lpi2c::Instance;
pub use crate::imxrt105::peripherals::lpi2c::{RegisterBlock, ResetValues};
pub use crate::imxrt105::peripherals::lpi2c::{
    MCCR0, MCCR1, MCFGR0, MCFGR1, MCFGR2, MCFGR3, MCR, MDER, MDMR, MFCR, MFSR, MIER, MRDR, MSR,
    MTDR, PARAM, SAMR, SASR, SCFGR1, SCFGR2, SCR, SDER, SIER, SRDR, SSR, STAR, STDR, VERID,
};

/// Access functions for the LPI2C1 peripheral instance
pub mod LPI2C1 {
    use super::ResetValues;
    #[cfg(not(feature = "nosync"))]
    use core::sync::atomic::{AtomicBool, Ordering};

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x403f0000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in LPI2C1
    pub const reset: ResetValues = ResetValues {
        VERID: 0x01000003,
        PARAM: 0x00000202,
        MCR: 0x00000000,
        MSR: 0x00000001,
        MIER: 0x00000000,
        MDER: 0x00000000,
        MCFGR0: 0x00000000,
        MCFGR1: 0x00000000,
        MCFGR2: 0x00000000,
        MCFGR3: 0x00000000,
        MDMR: 0x00000000,
        MCCR0: 0x00000000,
        MCCR1: 0x00000000,
        MFCR: 0x00000000,
        MFSR: 0x00000000,
        MTDR: 0x00000000,
        MRDR: 0x00004000,
        SCR: 0x00000000,
        SSR: 0x00000000,
        SIER: 0x00000000,
        SDER: 0x00000000,
        SCFGR1: 0x00000000,
        SCFGR2: 0x00000000,
        SAMR: 0x00000000,
        SASR: 0x00004000,
        STAR: 0x00000000,
        STDR: 0x00000000,
        SRDR: 0x00004000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static LPI2C1_TAKEN: AtomicBool = AtomicBool::new(false);

    /// Safe access to LPI2C1
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
        let taken = LPI2C1_TAKEN.swap(true, Ordering::SeqCst);
        if taken {
            None
        } else {
            Some(INSTANCE)
        }
    }

    /// Release exclusive access to LPI2C1
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        assert!(inst.addr == INSTANCE.addr, "Released the wrong instance");

        let taken = LPI2C1_TAKEN.swap(false, Ordering::SeqCst);
        assert!(taken, "Released a peripheral which was not taken");
    }

    /// Unsafely steal LPI2C1
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        LPI2C1_TAKEN.store(true, Ordering::SeqCst);
        INSTANCE
    }
}

/// Raw pointer to LPI2C1
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const LPI2C1: *const RegisterBlock = 0x403f0000 as *const _;

/// Access functions for the LPI2C2 peripheral instance
pub mod LPI2C2 {
    use super::ResetValues;
    #[cfg(not(feature = "nosync"))]
    use core::sync::atomic::{AtomicBool, Ordering};

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x403f4000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in LPI2C2
    pub const reset: ResetValues = ResetValues {
        VERID: 0x01000003,
        PARAM: 0x00000202,
        MCR: 0x00000000,
        MSR: 0x00000001,
        MIER: 0x00000000,
        MDER: 0x00000000,
        MCFGR0: 0x00000000,
        MCFGR1: 0x00000000,
        MCFGR2: 0x00000000,
        MCFGR3: 0x00000000,
        MDMR: 0x00000000,
        MCCR0: 0x00000000,
        MCCR1: 0x00000000,
        MFCR: 0x00000000,
        MFSR: 0x00000000,
        MTDR: 0x00000000,
        MRDR: 0x00004000,
        SCR: 0x00000000,
        SSR: 0x00000000,
        SIER: 0x00000000,
        SDER: 0x00000000,
        SCFGR1: 0x00000000,
        SCFGR2: 0x00000000,
        SAMR: 0x00000000,
        SASR: 0x00004000,
        STAR: 0x00000000,
        STDR: 0x00000000,
        SRDR: 0x00004000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static LPI2C2_TAKEN: AtomicBool = AtomicBool::new(false);

    /// Safe access to LPI2C2
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
        let taken = LPI2C2_TAKEN.swap(true, Ordering::SeqCst);
        if taken {
            None
        } else {
            Some(INSTANCE)
        }
    }

    /// Release exclusive access to LPI2C2
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        assert!(inst.addr == INSTANCE.addr, "Released the wrong instance");

        let taken = LPI2C2_TAKEN.swap(false, Ordering::SeqCst);
        assert!(taken, "Released a peripheral which was not taken");
    }

    /// Unsafely steal LPI2C2
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        LPI2C2_TAKEN.store(true, Ordering::SeqCst);
        INSTANCE
    }
}

/// Raw pointer to LPI2C2
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const LPI2C2: *const RegisterBlock = 0x403f4000 as *const _;

/// Access functions for the LPI2C3 peripheral instance
pub mod LPI2C3 {
    use super::ResetValues;
    #[cfg(not(feature = "nosync"))]
    use core::sync::atomic::{AtomicBool, Ordering};

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x403f8000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in LPI2C3
    pub const reset: ResetValues = ResetValues {
        VERID: 0x01000003,
        PARAM: 0x00000202,
        MCR: 0x00000000,
        MSR: 0x00000001,
        MIER: 0x00000000,
        MDER: 0x00000000,
        MCFGR0: 0x00000000,
        MCFGR1: 0x00000000,
        MCFGR2: 0x00000000,
        MCFGR3: 0x00000000,
        MDMR: 0x00000000,
        MCCR0: 0x00000000,
        MCCR1: 0x00000000,
        MFCR: 0x00000000,
        MFSR: 0x00000000,
        MTDR: 0x00000000,
        MRDR: 0x00004000,
        SCR: 0x00000000,
        SSR: 0x00000000,
        SIER: 0x00000000,
        SDER: 0x00000000,
        SCFGR1: 0x00000000,
        SCFGR2: 0x00000000,
        SAMR: 0x00000000,
        SASR: 0x00004000,
        STAR: 0x00000000,
        STDR: 0x00000000,
        SRDR: 0x00004000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static LPI2C3_TAKEN: AtomicBool = AtomicBool::new(false);

    /// Safe access to LPI2C3
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
        let taken = LPI2C3_TAKEN.swap(true, Ordering::SeqCst);
        if taken {
            None
        } else {
            Some(INSTANCE)
        }
    }

    /// Release exclusive access to LPI2C3
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        assert!(inst.addr == INSTANCE.addr, "Released the wrong instance");

        let taken = LPI2C3_TAKEN.swap(false, Ordering::SeqCst);
        assert!(taken, "Released a peripheral which was not taken");
    }

    /// Unsafely steal LPI2C3
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        LPI2C3_TAKEN.store(true, Ordering::SeqCst);
        INSTANCE
    }
}

/// Raw pointer to LPI2C3
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const LPI2C3: *const RegisterBlock = 0x403f8000 as *const _;

/// Access functions for the LPI2C4 peripheral instance
pub mod LPI2C4 {
    use super::ResetValues;
    #[cfg(not(feature = "nosync"))]
    use core::sync::atomic::{AtomicBool, Ordering};

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x403fc000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in LPI2C4
    pub const reset: ResetValues = ResetValues {
        VERID: 0x01000003,
        PARAM: 0x00000202,
        MCR: 0x00000000,
        MSR: 0x00000001,
        MIER: 0x00000000,
        MDER: 0x00000000,
        MCFGR0: 0x00000000,
        MCFGR1: 0x00000000,
        MCFGR2: 0x00000000,
        MCFGR3: 0x00000000,
        MDMR: 0x00000000,
        MCCR0: 0x00000000,
        MCCR1: 0x00000000,
        MFCR: 0x00000000,
        MFSR: 0x00000000,
        MTDR: 0x00000000,
        MRDR: 0x00004000,
        SCR: 0x00000000,
        SSR: 0x00000000,
        SIER: 0x00000000,
        SDER: 0x00000000,
        SCFGR1: 0x00000000,
        SCFGR2: 0x00000000,
        SAMR: 0x00000000,
        SASR: 0x00004000,
        STAR: 0x00000000,
        STDR: 0x00000000,
        SRDR: 0x00004000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static LPI2C4_TAKEN: AtomicBool = AtomicBool::new(false);

    /// Safe access to LPI2C4
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
        let taken = LPI2C4_TAKEN.swap(true, Ordering::SeqCst);
        if taken {
            None
        } else {
            Some(INSTANCE)
        }
    }

    /// Release exclusive access to LPI2C4
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        assert!(inst.addr == INSTANCE.addr, "Released the wrong instance");

        let taken = LPI2C4_TAKEN.swap(false, Ordering::SeqCst);
        assert!(taken, "Released a peripheral which was not taken");
    }

    /// Unsafely steal LPI2C4
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        LPI2C4_TAKEN.store(true, Ordering::SeqCst);
        INSTANCE
    }
}

/// Raw pointer to LPI2C4
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const LPI2C4: *const RegisterBlock = 0x403fc000 as *const _;
