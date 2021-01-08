/*!
galbi is a library that provides shortcut types for idiomatic nested types.

## install
If cargo-edit is installed, you can install it like this:
```sh
cargo add galbi
```

If not, you have to manually add the dependency to Cargo.toml.
```toml
[dependencies]
galbi = "0.2.1"
```

## use
It can be used in the following format.
Since the automatic dereferencing trait is implemented, you can use the inner methods right away.
```rust
use galbi::*;

fn main()
{
    let shared = ArcMutex::new(15);
    let get = shared.lock().unwrap();
    println!("{}", *get);
}
```

## features
- Rc<RefCell&lt;T>> -> RcCell<T>
- Arc<Mutex&lt;T>> -> ArcMutex<T>
- Option<Box&lt;T>> -> OptionBox<T>
- ... more later

## link
- [document](https://docs.rs/galbi)
- [repository](https://github.com/myyrakle/galbi)
*/

#[path = "./impl/option_box.rs"]
pub mod option_box;
pub use option_box::*;

#[path = "./impl/rc_cell.rs"]
pub mod rc_cell;
pub use rc_cell::*;

#[path = "./impl/arc_mutex.rs"]
pub mod arc_mutex;
pub use arc_mutex::*;
