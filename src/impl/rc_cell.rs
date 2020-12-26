//! It is Wrapper Type of `Rc<RefCell<T>>`

use std::cell::RefCell;
use std::ops::{Deref, DerefMut};
use std::rc::Rc;

#[derive(Clone)]
pub struct RcCell<T> {
    ptr: Rc<RefCell<T>>,
}

impl<T> RcCell<T> {
    /// Create a new object.
    ///
    /// ```rust
    /// use galbi::*;
    ///
    /// let shared = RcCell::new(15);
    /// let get = shared.borrow();
    /// assert_eq!(15, *get);
    /// ```
    pub fn new(x: T) -> RcCell<T> {
        RcCell {
            ptr: Rc::new(RefCell::new(x)),
        }
    }
}
impl<T> Deref for RcCell<T> {
    type Target = Rc<RefCell<T>>;

    fn deref(&self) -> &Self::Target {
        &self.ptr
    }
}

impl<T> DerefMut for RcCell<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.ptr
    }
}
