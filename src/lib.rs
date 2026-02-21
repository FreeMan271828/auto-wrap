#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "std")]
extern crate std;

#[cfg(feature = "std")]
use std::{
    cell::RefCell,
    rc::Rc,
};

use core::cell::Cell;
use std::sync::{Arc, Mutex};

/// 扩展 trait：为任意 Sized 类型提供智能封装能力
///
/// 设计原则：
/// - 所有方法 inline
/// - 不产生额外抽象层
/// - 明确 trait bound
/// - feature 控制线程相关类型
pub trait WrapExt: Sized {

    /// 封装为 `Cell<T>`
    ///
    /// 要求 T: Copy
    #[inline(always)]
    fn cell(self) -> Cell<Self>
    where
        Self: Copy,
    {
        Cell::new(self)
    }

    /// 封装为 `RefCell<T>`
    #[cfg(feature = "std")]
    #[inline(always)]
    fn refcell(self) -> RefCell<Self> {
        RefCell::new(self)
    }

    /// 封装为 `Rc<T>`
    #[cfg(feature = "std")]
    #[inline(always)]
    fn rc(self) -> Rc<Self> {
        Rc::new(self)
    }

    /// 封装为 `Rc<RefCell<T>>`
    #[cfg(feature = "std")]
    #[inline(always)]
    fn rc_refcell(self) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(self))
    }

    /// 封装为 `Arc<T>`
    #[cfg(all(feature = "std", feature = "sync"))]
    #[inline(always)]
    fn arc(self) -> Arc<Self> {
        Arc::new(self)
    }

    /// 封装为 `Arc<Mutex<T>>`
    #[cfg(all(feature = "std", feature = "sync"))]
    #[inline(always)]
    fn arc_mutex(self) -> Arc<Mutex<Self>> {
        Arc::new(Mutex::new(self))
    }
}

impl<T> WrapExt for T {}