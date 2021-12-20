#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! GPIO
//!
//! Used by: imxrt1061, imxrt1062, imxrt1064

#[cfg(not(feature = "nosync"))]
use crate::imxrt106::peripherals::gpio::private;
#[cfg(not(feature = "nosync"))]
pub use crate::imxrt106::peripherals::gpio::{Instance, Valid};
pub use crate::imxrt106::peripherals::gpio::{RegisterBlock, ResetValues};
pub use crate::imxrt106::peripherals::gpio::{
    DR, DR_CLEAR, DR_SET, DR_TOGGLE, EDGE_SEL, GDIR, ICR1, ICR2, IMR, ISR, PSR,
};
#[cfg(not(feature = "nosync"))]
use core::sync::atomic::{AtomicBool, Ordering};

/// The GPIO1 peripheral instance.
#[cfg(all(not(feature = "nosync"), not(feature = "doc")))]
pub type GPIO1 = Instance<1>;

/// The GPIO1 peripheral instance.
///
/// This is a new type only for documentation purposes. When
/// compiling for a target, this is defined as
///
/// ```rust
/// pub type GPIO1 = Instance<1>;
/// ```
#[cfg(all(not(feature = "nosync"), feature = "doc"))]
pub struct GPIO1 {
    #[allow(unused)] // Only for documentation generation.
    addr: u32,
}

#[cfg(not(feature = "nosync"))]
impl private::Sealed for GPIO1 {}
#[cfg(not(feature = "nosync"))]
impl Valid for GPIO1 {}

#[cfg(not(feature = "nosync"))]
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static GPIO1_TAKEN: AtomicBool = AtomicBool::new(false);

/// Access functions for the GPIO1 peripheral instance
#[cfg(not(feature = "nosync"))]
impl GPIO1 {
    const INSTANCE: Self = Self {
        addr: 0x401b8000,
        #[cfg(not(feature = "doc"))]
        intrs: &[
            crate::interrupt::GPIO1_INT0,
            crate::interrupt::GPIO1_INT1,
            crate::interrupt::GPIO1_INT2,
            crate::interrupt::GPIO1_INT3,
            crate::interrupt::GPIO1_INT4,
            crate::interrupt::GPIO1_INT5,
            crate::interrupt::GPIO1_INT6,
            crate::interrupt::GPIO1_INT7,
            crate::interrupt::GPIO1_Combined_0_15,
            crate::interrupt::GPIO1_Combined_16_31,
        ],
    };

    /// Reset values for each field in GPIO1
    pub const reset: ResetValues = ResetValues {
        DR: 0x00000000,
        GDIR: 0x00000000,
        PSR: 0x00000000,
        ICR1: 0x00000000,
        ICR2: 0x00000000,
        IMR: 0x00000000,
        ISR: 0x00000000,
        EDGE_SEL: 0x00000000,
        DR_SET: 0x00000000,
        DR_CLEAR: 0x00000000,
        DR_TOGGLE: 0x00000000,
    };

    /// Safe access to GPIO1
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
        let taken = GPIO1_TAKEN.swap(true, Ordering::SeqCst);
        if taken {
            None
        } else {
            Some(Self::INSTANCE)
        }
    }

    /// Release exclusive access to GPIO1
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[inline]
    pub fn release(_: Self) {
        let taken = GPIO1_TAKEN.swap(false, Ordering::SeqCst);
        assert!(taken, "Released a peripheral which was not taken");
    }

    /// Unsafely steal GPIO1
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[inline]
    pub unsafe fn steal() -> Self {
        GPIO1_TAKEN.store(true, Ordering::SeqCst);
        Self::INSTANCE
    }

    /// The interrupts associated with GPIO1
    #[cfg(not(feature = "doc"))]
    pub const INTERRUPTS: [crate::Interrupt; 10] = [
        crate::interrupt::GPIO1_INT0,
        crate::interrupt::GPIO1_INT1,
        crate::interrupt::GPIO1_INT2,
        crate::interrupt::GPIO1_INT3,
        crate::interrupt::GPIO1_INT4,
        crate::interrupt::GPIO1_INT5,
        crate::interrupt::GPIO1_INT6,
        crate::interrupt::GPIO1_INT7,
        crate::interrupt::GPIO1_Combined_0_15,
        crate::interrupt::GPIO1_Combined_16_31,
    ];

    /// The interrupts associated with GPIO1
    ///
    /// Note: the values are invalid for a documentation build.
    #[cfg(feature = "doc")]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];
}

/// Raw pointer to GPIO1
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const GPIO1: *const RegisterBlock = 0x401b8000 as *const _;

/// The GPIO2 peripheral instance.
#[cfg(all(not(feature = "nosync"), not(feature = "doc")))]
pub type GPIO2 = Instance<2>;

/// The GPIO2 peripheral instance.
///
/// This is a new type only for documentation purposes. When
/// compiling for a target, this is defined as
///
/// ```rust
/// pub type GPIO2 = Instance<2>;
/// ```
#[cfg(all(not(feature = "nosync"), feature = "doc"))]
pub struct GPIO2 {
    #[allow(unused)] // Only for documentation generation.
    addr: u32,
}

#[cfg(not(feature = "nosync"))]
impl private::Sealed for GPIO2 {}
#[cfg(not(feature = "nosync"))]
impl Valid for GPIO2 {}

#[cfg(not(feature = "nosync"))]
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static GPIO2_TAKEN: AtomicBool = AtomicBool::new(false);

/// Access functions for the GPIO2 peripheral instance
#[cfg(not(feature = "nosync"))]
impl GPIO2 {
    const INSTANCE: Self = Self {
        addr: 0x401bc000,
        #[cfg(not(feature = "doc"))]
        intrs: &[
            crate::interrupt::GPIO2_Combined_0_15,
            crate::interrupt::GPIO2_Combined_16_31,
        ],
    };

    /// Reset values for each field in GPIO2
    pub const reset: ResetValues = ResetValues {
        DR: 0x00000000,
        GDIR: 0x00000000,
        PSR: 0x00000000,
        ICR1: 0x00000000,
        ICR2: 0x00000000,
        IMR: 0x00000000,
        ISR: 0x00000000,
        EDGE_SEL: 0x00000000,
        DR_SET: 0x00000000,
        DR_CLEAR: 0x00000000,
        DR_TOGGLE: 0x00000000,
    };

    /// Safe access to GPIO2
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
        let taken = GPIO2_TAKEN.swap(true, Ordering::SeqCst);
        if taken {
            None
        } else {
            Some(Self::INSTANCE)
        }
    }

    /// Release exclusive access to GPIO2
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[inline]
    pub fn release(_: Self) {
        let taken = GPIO2_TAKEN.swap(false, Ordering::SeqCst);
        assert!(taken, "Released a peripheral which was not taken");
    }

    /// Unsafely steal GPIO2
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[inline]
    pub unsafe fn steal() -> Self {
        GPIO2_TAKEN.store(true, Ordering::SeqCst);
        Self::INSTANCE
    }

    /// The interrupts associated with GPIO2
    #[cfg(not(feature = "doc"))]
    pub const INTERRUPTS: [crate::Interrupt; 2] = [
        crate::interrupt::GPIO2_Combined_0_15,
        crate::interrupt::GPIO2_Combined_16_31,
    ];

    /// The interrupts associated with GPIO2
    ///
    /// Note: the values are invalid for a documentation build.
    #[cfg(feature = "doc")]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];
}

/// Raw pointer to GPIO2
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const GPIO2: *const RegisterBlock = 0x401bc000 as *const _;

/// The GPIO3 peripheral instance.
#[cfg(all(not(feature = "nosync"), not(feature = "doc")))]
pub type GPIO3 = Instance<3>;

/// The GPIO3 peripheral instance.
///
/// This is a new type only for documentation purposes. When
/// compiling for a target, this is defined as
///
/// ```rust
/// pub type GPIO3 = Instance<3>;
/// ```
#[cfg(all(not(feature = "nosync"), feature = "doc"))]
pub struct GPIO3 {
    #[allow(unused)] // Only for documentation generation.
    addr: u32,
}

#[cfg(not(feature = "nosync"))]
impl private::Sealed for GPIO3 {}
#[cfg(not(feature = "nosync"))]
impl Valid for GPIO3 {}

#[cfg(not(feature = "nosync"))]
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static GPIO3_TAKEN: AtomicBool = AtomicBool::new(false);

/// Access functions for the GPIO3 peripheral instance
#[cfg(not(feature = "nosync"))]
impl GPIO3 {
    const INSTANCE: Self = Self {
        addr: 0x401c0000,
        #[cfg(not(feature = "doc"))]
        intrs: &[
            crate::interrupt::GPIO3_Combined_0_15,
            crate::interrupt::GPIO3_Combined_16_31,
        ],
    };

    /// Reset values for each field in GPIO3
    pub const reset: ResetValues = ResetValues {
        DR: 0x00000000,
        GDIR: 0x00000000,
        PSR: 0x00000000,
        ICR1: 0x00000000,
        ICR2: 0x00000000,
        IMR: 0x00000000,
        ISR: 0x00000000,
        EDGE_SEL: 0x00000000,
        DR_SET: 0x00000000,
        DR_CLEAR: 0x00000000,
        DR_TOGGLE: 0x00000000,
    };

    /// Safe access to GPIO3
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
        let taken = GPIO3_TAKEN.swap(true, Ordering::SeqCst);
        if taken {
            None
        } else {
            Some(Self::INSTANCE)
        }
    }

    /// Release exclusive access to GPIO3
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[inline]
    pub fn release(_: Self) {
        let taken = GPIO3_TAKEN.swap(false, Ordering::SeqCst);
        assert!(taken, "Released a peripheral which was not taken");
    }

    /// Unsafely steal GPIO3
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[inline]
    pub unsafe fn steal() -> Self {
        GPIO3_TAKEN.store(true, Ordering::SeqCst);
        Self::INSTANCE
    }

    /// The interrupts associated with GPIO3
    #[cfg(not(feature = "doc"))]
    pub const INTERRUPTS: [crate::Interrupt; 2] = [
        crate::interrupt::GPIO3_Combined_0_15,
        crate::interrupt::GPIO3_Combined_16_31,
    ];

    /// The interrupts associated with GPIO3
    ///
    /// Note: the values are invalid for a documentation build.
    #[cfg(feature = "doc")]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];
}

/// Raw pointer to GPIO3
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const GPIO3: *const RegisterBlock = 0x401c0000 as *const _;

/// The GPIO4 peripheral instance.
#[cfg(all(not(feature = "nosync"), not(feature = "doc")))]
pub type GPIO4 = Instance<4>;

/// The GPIO4 peripheral instance.
///
/// This is a new type only for documentation purposes. When
/// compiling for a target, this is defined as
///
/// ```rust
/// pub type GPIO4 = Instance<4>;
/// ```
#[cfg(all(not(feature = "nosync"), feature = "doc"))]
pub struct GPIO4 {
    #[allow(unused)] // Only for documentation generation.
    addr: u32,
}

#[cfg(not(feature = "nosync"))]
impl private::Sealed for GPIO4 {}
#[cfg(not(feature = "nosync"))]
impl Valid for GPIO4 {}

#[cfg(not(feature = "nosync"))]
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static GPIO4_TAKEN: AtomicBool = AtomicBool::new(false);

/// Access functions for the GPIO4 peripheral instance
#[cfg(not(feature = "nosync"))]
impl GPIO4 {
    const INSTANCE: Self = Self {
        addr: 0x401c4000,
        #[cfg(not(feature = "doc"))]
        intrs: &[
            crate::interrupt::GPIO4_Combined_0_15,
            crate::interrupt::GPIO4_Combined_16_31,
        ],
    };

    /// Reset values for each field in GPIO4
    pub const reset: ResetValues = ResetValues {
        DR: 0x00000000,
        GDIR: 0x00000000,
        PSR: 0x00000000,
        ICR1: 0x00000000,
        ICR2: 0x00000000,
        IMR: 0x00000000,
        ISR: 0x00000000,
        EDGE_SEL: 0x00000000,
        DR_SET: 0x00000000,
        DR_CLEAR: 0x00000000,
        DR_TOGGLE: 0x00000000,
    };

    /// Safe access to GPIO4
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
        let taken = GPIO4_TAKEN.swap(true, Ordering::SeqCst);
        if taken {
            None
        } else {
            Some(Self::INSTANCE)
        }
    }

    /// Release exclusive access to GPIO4
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[inline]
    pub fn release(_: Self) {
        let taken = GPIO4_TAKEN.swap(false, Ordering::SeqCst);
        assert!(taken, "Released a peripheral which was not taken");
    }

    /// Unsafely steal GPIO4
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[inline]
    pub unsafe fn steal() -> Self {
        GPIO4_TAKEN.store(true, Ordering::SeqCst);
        Self::INSTANCE
    }

    /// The interrupts associated with GPIO4
    #[cfg(not(feature = "doc"))]
    pub const INTERRUPTS: [crate::Interrupt; 2] = [
        crate::interrupt::GPIO4_Combined_0_15,
        crate::interrupt::GPIO4_Combined_16_31,
    ];

    /// The interrupts associated with GPIO4
    ///
    /// Note: the values are invalid for a documentation build.
    #[cfg(feature = "doc")]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];
}

/// Raw pointer to GPIO4
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const GPIO4: *const RegisterBlock = 0x401c4000 as *const _;

/// The GPIO5 peripheral instance.
#[cfg(all(not(feature = "nosync"), not(feature = "doc")))]
pub type GPIO5 = Instance<5>;

/// The GPIO5 peripheral instance.
///
/// This is a new type only for documentation purposes. When
/// compiling for a target, this is defined as
///
/// ```rust
/// pub type GPIO5 = Instance<5>;
/// ```
#[cfg(all(not(feature = "nosync"), feature = "doc"))]
pub struct GPIO5 {
    #[allow(unused)] // Only for documentation generation.
    addr: u32,
}

#[cfg(not(feature = "nosync"))]
impl private::Sealed for GPIO5 {}
#[cfg(not(feature = "nosync"))]
impl Valid for GPIO5 {}

#[cfg(not(feature = "nosync"))]
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static GPIO5_TAKEN: AtomicBool = AtomicBool::new(false);

/// Access functions for the GPIO5 peripheral instance
#[cfg(not(feature = "nosync"))]
impl GPIO5 {
    const INSTANCE: Self = Self {
        addr: 0x400c0000,
        #[cfg(not(feature = "doc"))]
        intrs: &[
            crate::interrupt::GPIO5_Combined_0_15,
            crate::interrupt::GPIO5_Combined_16_31,
        ],
    };

    /// Reset values for each field in GPIO5
    pub const reset: ResetValues = ResetValues {
        DR: 0x00000000,
        GDIR: 0x00000000,
        PSR: 0x00000000,
        ICR1: 0x00000000,
        ICR2: 0x00000000,
        IMR: 0x00000000,
        ISR: 0x00000000,
        EDGE_SEL: 0x00000000,
        DR_SET: 0x00000000,
        DR_CLEAR: 0x00000000,
        DR_TOGGLE: 0x00000000,
    };

    /// Safe access to GPIO5
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
        let taken = GPIO5_TAKEN.swap(true, Ordering::SeqCst);
        if taken {
            None
        } else {
            Some(Self::INSTANCE)
        }
    }

    /// Release exclusive access to GPIO5
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[inline]
    pub fn release(_: Self) {
        let taken = GPIO5_TAKEN.swap(false, Ordering::SeqCst);
        assert!(taken, "Released a peripheral which was not taken");
    }

    /// Unsafely steal GPIO5
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[inline]
    pub unsafe fn steal() -> Self {
        GPIO5_TAKEN.store(true, Ordering::SeqCst);
        Self::INSTANCE
    }

    /// The interrupts associated with GPIO5
    #[cfg(not(feature = "doc"))]
    pub const INTERRUPTS: [crate::Interrupt; 2] = [
        crate::interrupt::GPIO5_Combined_0_15,
        crate::interrupt::GPIO5_Combined_16_31,
    ];

    /// The interrupts associated with GPIO5
    ///
    /// Note: the values are invalid for a documentation build.
    #[cfg(feature = "doc")]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];
}

/// Raw pointer to GPIO5
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const GPIO5: *const RegisterBlock = 0x400c0000 as *const _;

/// The GPIO6 peripheral instance.
#[cfg(all(not(feature = "nosync"), not(feature = "doc")))]
pub type GPIO6 = Instance<6>;

/// The GPIO6 peripheral instance.
///
/// This is a new type only for documentation purposes. When
/// compiling for a target, this is defined as
///
/// ```rust
/// pub type GPIO6 = Instance<6>;
/// ```
#[cfg(all(not(feature = "nosync"), feature = "doc"))]
pub struct GPIO6 {
    #[allow(unused)] // Only for documentation generation.
    addr: u32,
}

#[cfg(not(feature = "nosync"))]
impl private::Sealed for GPIO6 {}
#[cfg(not(feature = "nosync"))]
impl Valid for GPIO6 {}

#[cfg(not(feature = "nosync"))]
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static GPIO6_TAKEN: AtomicBool = AtomicBool::new(false);

/// Access functions for the GPIO6 peripheral instance
#[cfg(not(feature = "nosync"))]
impl GPIO6 {
    const INSTANCE: Self = Self {
        addr: 0x42000000,
        #[cfg(not(feature = "doc"))]
        intrs: &[crate::interrupt::GPIO6_7_8_9],
    };

    /// Reset values for each field in GPIO6
    pub const reset: ResetValues = ResetValues {
        DR: 0x00000000,
        GDIR: 0x00000000,
        PSR: 0x00000000,
        ICR1: 0x00000000,
        ICR2: 0x00000000,
        IMR: 0x00000000,
        ISR: 0x00000000,
        EDGE_SEL: 0x00000000,
        DR_SET: 0x00000000,
        DR_CLEAR: 0x00000000,
        DR_TOGGLE: 0x00000000,
    };

    /// Safe access to GPIO6
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
        let taken = GPIO6_TAKEN.swap(true, Ordering::SeqCst);
        if taken {
            None
        } else {
            Some(Self::INSTANCE)
        }
    }

    /// Release exclusive access to GPIO6
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[inline]
    pub fn release(_: Self) {
        let taken = GPIO6_TAKEN.swap(false, Ordering::SeqCst);
        assert!(taken, "Released a peripheral which was not taken");
    }

    /// Unsafely steal GPIO6
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[inline]
    pub unsafe fn steal() -> Self {
        GPIO6_TAKEN.store(true, Ordering::SeqCst);
        Self::INSTANCE
    }

    /// The interrupts associated with GPIO6
    #[cfg(not(feature = "doc"))]
    pub const INTERRUPTS: [crate::Interrupt; 1] = [crate::interrupt::GPIO6_7_8_9];

    /// The interrupts associated with GPIO6
    ///
    /// Note: the values are invalid for a documentation build.
    #[cfg(feature = "doc")]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];
}

/// Raw pointer to GPIO6
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const GPIO6: *const RegisterBlock = 0x42000000 as *const _;

/// The GPIO7 peripheral instance.
#[cfg(all(not(feature = "nosync"), not(feature = "doc")))]
pub type GPIO7 = Instance<7>;

/// The GPIO7 peripheral instance.
///
/// This is a new type only for documentation purposes. When
/// compiling for a target, this is defined as
///
/// ```rust
/// pub type GPIO7 = Instance<7>;
/// ```
#[cfg(all(not(feature = "nosync"), feature = "doc"))]
pub struct GPIO7 {
    #[allow(unused)] // Only for documentation generation.
    addr: u32,
}

#[cfg(not(feature = "nosync"))]
impl private::Sealed for GPIO7 {}
#[cfg(not(feature = "nosync"))]
impl Valid for GPIO7 {}

#[cfg(not(feature = "nosync"))]
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static GPIO7_TAKEN: AtomicBool = AtomicBool::new(false);

/// Access functions for the GPIO7 peripheral instance
#[cfg(not(feature = "nosync"))]
impl GPIO7 {
    const INSTANCE: Self = Self {
        addr: 0x42004000,
        #[cfg(not(feature = "doc"))]
        intrs: &[crate::interrupt::GPIO6_7_8_9],
    };

    /// Reset values for each field in GPIO7
    pub const reset: ResetValues = ResetValues {
        DR: 0x00000000,
        GDIR: 0x00000000,
        PSR: 0x00000000,
        ICR1: 0x00000000,
        ICR2: 0x00000000,
        IMR: 0x00000000,
        ISR: 0x00000000,
        EDGE_SEL: 0x00000000,
        DR_SET: 0x00000000,
        DR_CLEAR: 0x00000000,
        DR_TOGGLE: 0x00000000,
    };

    /// Safe access to GPIO7
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
        let taken = GPIO7_TAKEN.swap(true, Ordering::SeqCst);
        if taken {
            None
        } else {
            Some(Self::INSTANCE)
        }
    }

    /// Release exclusive access to GPIO7
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[inline]
    pub fn release(_: Self) {
        let taken = GPIO7_TAKEN.swap(false, Ordering::SeqCst);
        assert!(taken, "Released a peripheral which was not taken");
    }

    /// Unsafely steal GPIO7
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[inline]
    pub unsafe fn steal() -> Self {
        GPIO7_TAKEN.store(true, Ordering::SeqCst);
        Self::INSTANCE
    }

    /// The interrupts associated with GPIO7
    #[cfg(not(feature = "doc"))]
    pub const INTERRUPTS: [crate::Interrupt; 1] = [crate::interrupt::GPIO6_7_8_9];

    /// The interrupts associated with GPIO7
    ///
    /// Note: the values are invalid for a documentation build.
    #[cfg(feature = "doc")]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];
}

/// Raw pointer to GPIO7
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const GPIO7: *const RegisterBlock = 0x42004000 as *const _;

/// The GPIO8 peripheral instance.
#[cfg(all(not(feature = "nosync"), not(feature = "doc")))]
pub type GPIO8 = Instance<8>;

/// The GPIO8 peripheral instance.
///
/// This is a new type only for documentation purposes. When
/// compiling for a target, this is defined as
///
/// ```rust
/// pub type GPIO8 = Instance<8>;
/// ```
#[cfg(all(not(feature = "nosync"), feature = "doc"))]
pub struct GPIO8 {
    #[allow(unused)] // Only for documentation generation.
    addr: u32,
}

#[cfg(not(feature = "nosync"))]
impl private::Sealed for GPIO8 {}
#[cfg(not(feature = "nosync"))]
impl Valid for GPIO8 {}

#[cfg(not(feature = "nosync"))]
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static GPIO8_TAKEN: AtomicBool = AtomicBool::new(false);

/// Access functions for the GPIO8 peripheral instance
#[cfg(not(feature = "nosync"))]
impl GPIO8 {
    const INSTANCE: Self = Self {
        addr: 0x42008000,
        #[cfg(not(feature = "doc"))]
        intrs: &[crate::interrupt::GPIO6_7_8_9],
    };

    /// Reset values for each field in GPIO8
    pub const reset: ResetValues = ResetValues {
        DR: 0x00000000,
        GDIR: 0x00000000,
        PSR: 0x00000000,
        ICR1: 0x00000000,
        ICR2: 0x00000000,
        IMR: 0x00000000,
        ISR: 0x00000000,
        EDGE_SEL: 0x00000000,
        DR_SET: 0x00000000,
        DR_CLEAR: 0x00000000,
        DR_TOGGLE: 0x00000000,
    };

    /// Safe access to GPIO8
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
        let taken = GPIO8_TAKEN.swap(true, Ordering::SeqCst);
        if taken {
            None
        } else {
            Some(Self::INSTANCE)
        }
    }

    /// Release exclusive access to GPIO8
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[inline]
    pub fn release(_: Self) {
        let taken = GPIO8_TAKEN.swap(false, Ordering::SeqCst);
        assert!(taken, "Released a peripheral which was not taken");
    }

    /// Unsafely steal GPIO8
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[inline]
    pub unsafe fn steal() -> Self {
        GPIO8_TAKEN.store(true, Ordering::SeqCst);
        Self::INSTANCE
    }

    /// The interrupts associated with GPIO8
    #[cfg(not(feature = "doc"))]
    pub const INTERRUPTS: [crate::Interrupt; 1] = [crate::interrupt::GPIO6_7_8_9];

    /// The interrupts associated with GPIO8
    ///
    /// Note: the values are invalid for a documentation build.
    #[cfg(feature = "doc")]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];
}

/// Raw pointer to GPIO8
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const GPIO8: *const RegisterBlock = 0x42008000 as *const _;

/// The GPIO9 peripheral instance.
#[cfg(all(not(feature = "nosync"), not(feature = "doc")))]
pub type GPIO9 = Instance<9>;

/// The GPIO9 peripheral instance.
///
/// This is a new type only for documentation purposes. When
/// compiling for a target, this is defined as
///
/// ```rust
/// pub type GPIO9 = Instance<9>;
/// ```
#[cfg(all(not(feature = "nosync"), feature = "doc"))]
pub struct GPIO9 {
    #[allow(unused)] // Only for documentation generation.
    addr: u32,
}

#[cfg(not(feature = "nosync"))]
impl private::Sealed for GPIO9 {}
#[cfg(not(feature = "nosync"))]
impl Valid for GPIO9 {}

#[cfg(not(feature = "nosync"))]
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static GPIO9_TAKEN: AtomicBool = AtomicBool::new(false);

/// Access functions for the GPIO9 peripheral instance
#[cfg(not(feature = "nosync"))]
impl GPIO9 {
    const INSTANCE: Self = Self {
        addr: 0x4200c000,
        #[cfg(not(feature = "doc"))]
        intrs: &[crate::interrupt::GPIO6_7_8_9],
    };

    /// Reset values for each field in GPIO9
    pub const reset: ResetValues = ResetValues {
        DR: 0x00000000,
        GDIR: 0x00000000,
        PSR: 0x00000000,
        ICR1: 0x00000000,
        ICR2: 0x00000000,
        IMR: 0x00000000,
        ISR: 0x00000000,
        EDGE_SEL: 0x00000000,
        DR_SET: 0x00000000,
        DR_CLEAR: 0x00000000,
        DR_TOGGLE: 0x00000000,
    };

    /// Safe access to GPIO9
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
        let taken = GPIO9_TAKEN.swap(true, Ordering::SeqCst);
        if taken {
            None
        } else {
            Some(Self::INSTANCE)
        }
    }

    /// Release exclusive access to GPIO9
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[inline]
    pub fn release(_: Self) {
        let taken = GPIO9_TAKEN.swap(false, Ordering::SeqCst);
        assert!(taken, "Released a peripheral which was not taken");
    }

    /// Unsafely steal GPIO9
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[inline]
    pub unsafe fn steal() -> Self {
        GPIO9_TAKEN.store(true, Ordering::SeqCst);
        Self::INSTANCE
    }

    /// The interrupts associated with GPIO9
    #[cfg(not(feature = "doc"))]
    pub const INTERRUPTS: [crate::Interrupt; 1] = [crate::interrupt::GPIO6_7_8_9];

    /// The interrupts associated with GPIO9
    ///
    /// Note: the values are invalid for a documentation build.
    #[cfg(feature = "doc")]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];
}

/// Raw pointer to GPIO9
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const GPIO9: *const RegisterBlock = 0x4200c000 as *const _;
