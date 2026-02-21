//! # AutoWrap
//!
//! A high-performance Rust library providing ergonomic smart pointer and interior
//! mutability extensions with zero-cost abstractions.
//!
//! ## Design Principles
//!
//! - All methods are inlined at compile time
//! - No additional abstraction layers
//! - Explicit trait bounds
//! - Feature-gated thread-safe types

#![cfg_attr(not(feature = "std"), no_std)]
#![doc(test(attr(allow(unused_literals))))]

#[cfg(feature = "std")]
extern crate std as core;

#[cfg(feature = "std")]
use core::cell::RefCell;

#[cfg(all(feature = "std", feature = "sync"))]
use core::sync::atomic::{AtomicUsize, AtomicIsize, AtomicU32, AtomicI32, AtomicU64, AtomicI64, AtomicBool};

#[cfg(feature = "std")]
use std::rc::Rc;

#[cfg(all(feature = "std", feature = "sync"))]
use std::sync::{Arc, Mutex, RwLock, OnceLock};

use core::cell::Cell;

// ============================================================================
// WrapExt Trait - Smart Pointer Wrappers
// ============================================================================

/// Extension trait providing convenient methods to wrap any Sized type into
/// common smart pointer and interior mutability types.
///
/// # Performance
///
/// All methods are marked with `#[inline(always)]` to ensure zero-cost
/// abstraction. The generated code is equivalent to directly calling
/// the underlying constructor.
///
/// # Example
///
/// ```
/// use autowrap::WrapExt;
///
/// // Interior mutability for Copy types
/// let cell = 42u32.cell();
/// cell.set(100);
///
/// // Reference counting
/// #[cfg(feature = "std")]
/// let rc = "hello".to_string().rc();
///
/// // Thread-safe sharing
/// #[cfg(all(feature = "std", feature = "sync"))]
/// let arc = vec![1, 2, 3].arc();
/// ```
pub trait WrapExt: Sized {
    /// Wraps the value in a `Cell<T>`.
    ///
    /// Requires `T: Copy` for interior mutability.
    #[inline(always)]
    fn cell(self) -> Cell<Self>
    where
        Self: Copy,
    {
        Cell::new(self)
    }

    /// Wraps the value in a `RefCell<T>`.
    #[cfg(feature = "std")]
    #[inline(always)]
    fn refcell(self) -> RefCell<Self> {
        RefCell::new(self)
    }

    /// Wraps the value in an `Rc<T>`.
    #[cfg(feature = "std")]
    #[inline(always)]
    fn rc(self) -> Rc<Self> {
        Rc::new(self)
    }

    /// Wraps the value in an `Rc<RefCell<T>>`.
    #[cfg(feature = "std")]
    #[inline(always)]
    fn rc_refcell(self) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(self))
    }

    /// Wraps the value in an `Arc<T>`.
    #[cfg(all(feature = "std", feature = "sync"))]
    #[inline(always)]
    fn arc(self) -> Arc<Self> {
        Arc::new(self)
    }

    /// Wraps the value in an `Arc<Mutex<T>>`.
    #[cfg(all(feature = "std", feature = "sync"))]
    #[inline(always)]
    fn arc_mutex(self) -> Arc<Mutex<Self>> {
        Arc::new(Mutex::new(self))
    }

    /// Wraps the value in an `Arc<RwLock<T>>`.
    #[cfg(all(feature = "std", feature = "sync"))]
    #[inline(always)]
    fn arc_rwlock(self) -> Arc<RwLock<Self>> {
        Arc::new(RwLock::new(self))
    }

    /// Wraps the value in a `OnceLock<T>`.
    #[cfg(all(feature = "std", feature = "sync"))]
    #[inline(always)]
    fn once_lock(self) -> OnceLock<Self> {
        OnceLock::from(self)
    }
}

impl<T> WrapExt for T {}

// ============================================================================
// AtomicWrapExt Trait - Atomic Type Wrappers
// ============================================================================

/// Extension trait providing atomic wrappers for integer types.
///
/// # Performance
///
/// All implementations use `#[inline(always)]` and generate optimal
/// atomic operations with no runtime overhead beyond the atomic itself.
///
/// # Example
///
/// ```
/// #[cfg(feature = "sync")]
/// {
///     use autowrap::AtomicWrapExt;
///
///     let counter = 0u32.atomic_u32();
///     counter.fetch_add(1, core::sync::atomic::Ordering::SeqCst);
/// }
/// ```
#[cfg(all(feature = "std", feature = "sync"))]
#[allow(clippy::redundant_closure_call)]
pub trait AtomicWrapExt {
    /// Creates an `AtomicUsize` from this value.
    fn atomic_usize(self) -> AtomicUsize;

    /// Creates an `AtomicIsize` from this value.
    fn atomic_isize(self) -> AtomicIsize;

    /// Creates an `AtomicU32` from this value.
    fn atomic_u32(self) -> AtomicU32;

    /// Creates an `AtomicI32` from this value.
    fn atomic_i32(self) -> AtomicI32;

    /// Creates an `AtomicU64` from this value.
    fn atomic_u64(self) -> AtomicU64;

    /// Creates an `AtomicI64` from this value.
    fn atomic_i64(self) -> AtomicI64;

    /// Creates an `AtomicBool` from this value.
    fn atomic_bool(self) -> AtomicBool;
}

// Macro to generate AtomicWrapExt implementations with optimal code generation
macro_rules! impl_atomic_wrap {
    (
        $( $ty:ty => {
            usize: $to_usize:expr,
            isize: $to_isize:expr,
            u32: $to_u32:expr,
            i32: $to_i32:expr,
            u64: $to_u64:expr,
            i64: $to_i64:expr,
            bool: $to_bool:expr
        } ),* $(,)?
    ) => {
        $(
            #[cfg(all(feature = "std", feature = "sync"))]
            impl AtomicWrapExt for $ty {
                #[inline(always)]
                fn atomic_usize(self) -> AtomicUsize {
                    AtomicUsize::new($to_usize(self))
                }

                #[inline(always)]
                fn atomic_isize(self) -> AtomicIsize {
                    AtomicIsize::new($to_isize(self))
                }

                #[inline(always)]
                fn atomic_u32(self) -> AtomicU32 {
                    AtomicU32::new($to_u32(self))
                }

                #[inline(always)]
                fn atomic_i32(self) -> AtomicI32 {
                    AtomicI32::new($to_i32(self))
                }

                #[inline(always)]
                fn atomic_u64(self) -> AtomicU64 {
                    AtomicU64::new($to_u64(self))
                }

                #[inline(always)]
                fn atomic_i64(self) -> AtomicI64 {
                    AtomicI64::new($to_i64(self))
                }

                #[inline(always)]
                fn atomic_bool(self) -> AtomicBool {
                    AtomicBool::new($to_bool(self))
                }
            }
        )*
    };
}

// Optimized implementations using const closures for better inlining
impl_atomic_wrap! {
    usize => {
        usize: |v: usize| v,
        isize: |v: usize| v as isize,
        u32: |v: usize| v as u32,
        i32: |v: usize| v as i32,
        u64: |v: usize| v as u64,
        i64: |v: usize| v as i64,
        bool: |v: usize| v != 0
    },
    isize => {
        usize: |v: isize| v as usize,
        isize: |v: isize| v,
        u32: |v: isize| v as u32,
        i32: |v: isize| v as i32,
        u64: |v: isize| v as u64,
        i64: |v: isize| v as i64,
        bool: |v: isize| v != 0
    },
    u32 => {
        usize: |v: u32| v as usize,
        isize: |v: u32| v as isize,
        u32: |v: u32| v,
        i32: |v: u32| v as i32,
        u64: |v: u32| v as u64,
        i64: |v: u32| v as i64,
        bool: |v: u32| v != 0
    },
    i32 => {
        usize: |v: i32| v as usize,
        isize: |v: i32| v as isize,
        u32: |v: i32| v as u32,
        i32: |v: i32| v,
        u64: |v: i32| v as u64,
        i64: |v: i32| v as i64,
        bool: |v: i32| v != 0
    },
    u64 => {
        usize: |v: u64| v as usize,
        isize: |v: u64| v as isize,
        u32: |v: u64| v as u32,
        i32: |v: u64| v as i32,
        u64: |v: u64| v,
        i64: |v: u64| v as i64,
        bool: |v: u64| v != 0
    },
    i64 => {
        usize: |v: i64| v as usize,
        isize: |v: i64| v as isize,
        u32: |v: i64| v as u32,
        i32: |v: i64| v as i32,
        u64: |v: i64| v as u64,
        i64: |v: i64| v,
        bool: |v: i64| v != 0
    },
    bool => {
        usize: |v: bool| v as usize,
        isize: |v: bool| v as isize,
        u32: |v: bool| v as u32,
        i32: |v: bool| v as i32,
        u64: |v: bool| v as u64,
        i64: |v: bool| v as i64,
        bool: |v: bool| v
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cell_wrap() {
        let c = 10u32.cell();
        assert_eq!(c.get(), 10);
        c.set(20);
        assert_eq!(c.get(), 20);
    }

    #[cfg(feature = "std")]
    mod std_tests {
        use super::*;

        #[test]
        fn test_refcell_wrap() {
            let r = 5u32.refcell();
            *r.borrow_mut() = 8;
            assert_eq!(*r.borrow(), 8);
        }

        #[test]
        fn test_rc_wrap() {
            let rc = 42u32.rc();
            assert_eq!(*rc, 42);
            let rc2 = core::rc::Rc::clone(&rc);
            assert_eq!(core::rc::Rc::strong_count(&rc2), 2);
        }

        #[test]
        fn test_rc_refcell_wrap() {
            let value = 100u32.rc_refcell();
            *value.borrow_mut() = 200;
            assert_eq!(*value.borrow(), 200);
            let cloned = core::rc::Rc::clone(&value);
            assert_eq!(core::rc::Rc::strong_count(&cloned), 2);
        }

        #[cfg(feature = "sync")]
        mod sync_tests {
            use super::*;
            use std::thread;

            #[test]
            fn test_arc_wrap() {
                let arc = 99u32.arc();
                assert_eq!(*arc, 99);
                let arc2 = std::sync::Arc::clone(&arc);
                assert_eq!(std::sync::Arc::strong_count(&arc2), 2);
            }

            #[test]
            fn test_arc_mutex_wrap() {
                let value = 1u32.arc_mutex();
                {
                    let mut lock = value.lock().unwrap();
                    *lock = 10;
                }
                assert_eq!(*value.lock().unwrap(), 10);

                let cloned = std::sync::Arc::clone(&value);
                let handle = thread::spawn(move || {
                    let mut data = cloned.lock().unwrap();
                    *data += 5;
                });
                handle.join().unwrap();
                assert_eq!(*value.lock().unwrap(), 15);
            }

            #[test]
            fn test_arc_rwlock_wrap() {
                let value = 1u32.arc_rwlock();
                {
                    let mut write = value.write().unwrap();
                    *write = 10;
                }
                assert_eq!(*value.read().unwrap(), 10);
            }

            #[test]
            fn test_once_lock_wrap() {
                let once = 42u32.once_lock();
                assert_eq!(*once.get().unwrap(), 42);
            }

            #[test]
            fn test_atomic_wrappers() {
                use core::sync::atomic::Ordering;

                // Test u32 -> various atomic types
                let a_usize = 100u32.atomic_usize();
                a_usize.store(200, Ordering::SeqCst);
                assert_eq!(a_usize.load(Ordering::SeqCst), 200);

                let a_u32 = 100u32.atomic_u32();
                a_u32.store(200, Ordering::SeqCst);
                assert_eq!(a_u32.load(Ordering::SeqCst), 200);

                let a_bool = true.atomic_bool();
                a_bool.store(false, Ordering::SeqCst);
                assert_eq!(a_bool.load(Ordering::SeqCst), false);
            }
        }
    }
}
