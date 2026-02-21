[![Rust](https://img.shields.io/badge/Rust-1.75+-dea584.svg?style=flat-square)](https://www.rust-lang.org/)
[![Version](https://img.shields.io/crates/v/autowrap.svg?style=flat-square)](https://crates.io/crates/autowrap)
[![Docs](https://img.shields.io/badge/docs.rs-latest-blue.svg?style=flat-square)](https://docs.rs/autowrap/latest/autowrap/)
[![License](https://img.shields.io/crates/l/autowrap.svg?style=flat-square)](https://github.com/FreeMan271828/auto-wrap/blob/main/LICENSE)
[![Build Status](https://img.shields.io/github/actions/workflow/status/FreeMan271828/auto-wrap/ci.yml?branch=main&style=flat-square)](https://github.com/FreeMan271828/auto-wrap/actions)

---

# AutoWrap / 自动封装

[English](#autowrap) | [中文](#自动封装)

---

## AutoWrap

A high-performance Rust library providing ergonomic smart pointer and interior mutability extensions with zero-cost abstractions.

### Features

- **Zero-Cost Abstractions**: All methods are inlined at compile time with `#[inline(always)]`
- **Cell Wrapper**: Wrap a Copy type into a `Cell<T>` for interior mutability.
- **RefCell Wrapper**: Wrap a type into a `RefCell<T>` for single-threaded interior mutability (std feature required).
- **Rc Wrapper**: Wrap a type into an `Rc<T>` for reference-counted shared ownership (std feature required).
- **Rc<RefCell> Wrapper**: Wrap a type into `Rc<RefCell<T>>` for shared, mutable ownership (std feature required).
- **Arc Wrapper**: Wrap a type into `Arc<T>` for thread-safe reference-counted ownership (sync feature required).
- **Arc<Mutex> Wrapper**: Wrap a type into `Arc<Mutex<T>>` for shared, mutable, thread-safe ownership (sync feature required).
- **Arc<RwLock> Wrapper**: Wrap a type into `Arc<RwLock<T>>` for shared, read-write lock ownership (sync feature required).
- **OnceLock Wrapper**: Wrap a type into `OnceLock<T>` for lazy initialization (sync feature required).
- **Atomic Wrappers**: Wrap integer types into Atomic types with 7 conversion options (sync feature required).

### Performance

AutoWrap is designed for maximum performance:

- **Full LTO (Link Time Optimization)**: Cross-module inlining optimization
- **Single Codegen Unit**: Maximum optimization opportunities in release builds
- **Zero Abstraction Overhead**: All wrapper calls are inlined to direct constructor calls
- **Macro-Generated Code**: Reduced code duplication for better compiler optimization

#### Release Profile Optimization

```toml
[profile.release]
opt-level = 3      # Maximum optimization
lto = "fat"       # Full Link Time Optimization
codegen-units = 1 # Single unit for better optimization
strip = true      # Remove debug symbols
panic = "abort"   # Smaller binary size
```

### Installation

Add to your Cargo.toml:

```toml
[dependencies]
autowrap = "1.0.1"

# Or with features:
[dependencies.autowrap]
version = "1.0.1"
features = ["sync", "std"]
```

### Usage

#### Basic Wrappers

```rust
use autowrap::WrapExt;

// Interior mutability for Copy types
let cell = 42u32.cell();
cell.set(100);
assert_eq!(cell.get(), 100);

// Reference counting
#[cfg(feature = "std")]
let rc = "hello".to_string().rc();

// Thread-safe sharing
#[cfg(all(feature = "std", feature = "sync"))]
let arc_mutex = 0u32.arc_mutex();
```

#### Thread-Safe Wrappers

```rust
#[cfg(feature = "sync")]
use autowrap::WrapExt;

// Arc<RwLock> for multiple readers/writers
#[cfg(feature = "sync")]
let data = "hello".to_string().arc_rwlock();
{
    let read = data.read().unwrap();
    assert_eq!(*read, "hello");
}

// OnceLock for lazy initialization
#[cfg(feature = "sync")]
let lazy = 42u32.once_lock();
```

#### Atomic Types

```rust
#[cfg(feature = "sync")]
use autowrap::AtomicWrapExt;

// Convert between atomic types easily
#[cfg(feature = "sync")]
let atomic = 100u32.atomic_u32();
atomic.store(200, Ordering::SeqCst);
assert_eq!(atomic.load(Ordering::SeqCst), 200);

// Cross-type conversion
#[cfg(feature = "sync")]
let atomic_usize = 100u32.atomic_usize(); // u32 -> AtomicUsize
```

### Features Flags

- `std` – Enable Rc, RefCell and related wrappers (enabled by default).
- `sync` – Enable Arc, Mutex, RwLock, OnceLock, and atomic wrappers (requires std).

### License

MIT

---

# 自动封装

一个高性能的 Rust 库，提供人体工程学的智能指针和内部可变性扩展，具有零成本抽象。

### 特性

- **零成本抽象**：所有方法在编译时使用 `#[inline(always)]` 内联
- **Cell 封装**：将 Copy 类型封装为 `Cell<T>`，实现内部可变性。
- **RefCell 封装**：将类型封装为 `RefCell<T>`，用于单线程内部可变性（需要 std 特性）。
- **Rc 封装**：将类型封装为 `Rc<T>`，用于引用计数的共享所有权（需要 std 特性）。
- **Rc<RefCell> 封装**：将类型封装为 `Rc<RefCell<T>>`，用于共享可变所有权（需要 std 特性）。
- **Arc 封装**：将类型封装为 `Arc<T>`，用于线程安全的引用计数所有权（需要 sync 特性）。
- **Arc<Mutex> 封装**：将类型封装为 `Arc<Mutex<T>>`，用于共享可变线程安全所有权（需要 sync 特性）。
- **Arc<RwLock> 封装**：将类型封装为 `Arc<RwLock<T>>`，用于共享读写锁所有权（需要 sync 特性）。
- **OnceLock 封装**：将类型封装为 `OnceLock<T>`，用于延迟初始化（需要 sync 特性）。
- **原子类型封装**：将整数类型封装为原子类型，支持 7 种转换选项（需要 sync 特性）。

### 性能

AutoWrap 经过专门设计，以实现最佳性能：

- **完整 LTO（链接时优化）**：跨模块内联优化
- **单一代码生成单元**：发布版本中最大优化机会
- **零抽象开销**：所有封装调用都内联为直接构造函数调用
- **宏生成代码**：减少代码重复，提升编译器优化效果

#### 发布配置优化

```toml
[profile.release]
opt-level = 3      # 最高优化级别
lto = "fat"       # 完整链接时优化
codegen-units = 1 # 单一单元以获得更好优化
strip = true      # 移除调试符号
panic = "abort"   # 更小的二进制体积
```

### 安装

添加到你的 `Cargo.toml`：

```toml
[dependencies]
autowrap = "1.1.0"

# 或使用特性：
[dependencies.autowrap]
version = "1.1.0"
features = ["sync", "std"]
```

### 使用方法

#### 基础封装

```rust
use autowrap::WrapExt;

// Copy 类型的内部可变性
let cell = 42u32.cell();
cell.set(100);
assert_eq!(cell.get(), 100);

// 引用计数
#[cfg(feature = "std")]
let rc = "hello".to_string().rc();

// 线程安全共享
#[cfg(all(feature = "std", feature = "sync"))]
let arc_mutex = 0u32.arc_mutex();
```

#### 线程安全封装

```rust
#[cfg(feature = "sync")]
use autowrap::WrapExt;

// Arc<RwLock> 用于多读单写场景
#[cfg(feature = "sync")]
let data = "hello".to_string().arc_rwlock();
{
    let read = data.read().unwrap();
    assert_eq!(*read, "hello");
}

// OnceLock 用于延迟初始化
#[cfg(feature = "sync")]
let lazy = 42u32.once_lock();
```

#### 原子类型

```rust
#[cfg(feature = "sync")]
use autowrap::AtomicWrapExt;

// 轻松转换原子类型
#[cfg(feature = "sync")]
let atomic = 100u32.atomic_u32();
atomic.store(200, Ordering::SeqCst);
assert_eq!(atomic.load(Ordering::SeqCst), 200);

// 跨类型转换
#[cfg(feature = "sync")]
let atomic_usize = 100u32.atomic_usize(); // u32 -> AtomicUsize
```

### 特性标志

- `std` – 启用 Rc、RefCell 及相关封装（默认启用）。
- `sync` – 启用 Arc、Mutex、RwLock、OnceLock 和原子类型封装（需要 std）。

### 许可证

MIT
