# Auto_Wrap

Auto_Wrap is a Rust trait that provides convenient methods to wrap any Sized type into common smart pointer and interior mutability types, 
such as Cell, RefCell, Rc, Arc, and Mutex. It is designed for ergonomic usage with minimal overhead and conditional feature support for threading.

## Features

- Cell Wrapper: Wrap a Copy type into a Cell<T> for interior mutability.
- RefCell Wrapper: Wrap a type into a RefCell<T> for single-threaded interior mutability (std feature required).
- Rc Wrapper: Wrap a type into an Rc<T> for reference-counted shared ownership (std feature required).
- Rc<RefCell> Wrapper: Wrap a type into Rc<RefCell<T>> for shared, mutable ownership (std feature required).
- Arc Wrapper: Wrap a type into Arc<T> for thread-safe reference-counted ownership (sync feature required).
- Arc<Mutex> Wrapper: Wrap a type into Arc<Mutex<T>> for shared, mutable, thread-safe ownership (sync feature required).

## Installation

Add to your Cargo.toml:
```
[dependencies]
auto_wrap = "0.1"

Optional features:

[dependencies.auto_wrap]
version = "0.1"
features = ["sync", "std"]
```

## Usage
```
use auto_wrap::WrapExt;

let x = 42u32.cell();
x.set(100);
assert_eq!(x.get(), 100);

#[cfg(feature = "std")]
let rc = 42u32.rc();

#[cfg(all(feature = "std", feature = "sync"))]
let arc_mutex = 0u32.arc_mutex();
```
## Features Flags

std – Enable Rc, RefCell and related wrappers.

sync – Enable Arc and Mutex wrappers (requires std).

## License

MIT