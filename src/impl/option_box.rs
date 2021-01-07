//! It is Wrapper Type of `Option<Box<T>>`

#[derive(Clone, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
pub struct OptionBox<T> {
    ptr: Option<Box<T>>,
}

//use std::pin::Pin;
use std::option::{Iter, IterMut};

impl<T> OptionBox<T> {
    /// Create a new object.
    ///
    /// ```rust
    /// use galbi::*;
    ///
    /// let value = OptionBox::new(Some(10));
    /// ```
    pub fn new(x: Option<T>) -> OptionBox<T> {
        OptionBox {
            ptr: if x.is_some() {
                Some(Box::new(x.unwrap()))
            } else {
                None
            },
        }
    }

    /// Create a new object.
    ///
    /// ```rust
    /// use galbi::*;
    ///
    /// let value:OptionBox<i32> = OptionBox::some(10);
    /// ```
    pub fn some(x: T) -> OptionBox<T> {
        OptionBox {
            ptr: Some(Box::new(x)),
        }
    }

    /// Create a new object.
    ///
    /// ```rust
    /// use galbi::*;
    ///
    /// let value:OptionBox<u32> = OptionBox::none();
    /// ```
    pub fn none() -> OptionBox<T> {
        OptionBox { ptr: None }
    }

    /// Returns true if the option is a Some value.
    ///
    /// ```rust
    /// use galbi::*;
    ///
    /// let x: OptionBox<u32> = OptionBox::some(19);
    /// assert_eq!(x.is_some(), true);
    ///
    /// let x: OptionBox<u32> = OptionBox::none();
    /// assert_eq!(x.is_some(), false);
    /// ```
    pub fn is_some(&self) -> bool {
        self.ptr.is_some()
    }

    /// Returns true if the option is a None value.
    ///
    /// ```rust
    /// use galbi::*;
    ///
    /// let x: OptionBox<u32> = OptionBox::some(19);
    /// assert_eq!(x.is_none(), false);
    ///
    /// let x: OptionBox<u32> = OptionBox::none();
    /// assert_eq!(x.is_none(), true);
    /// ```
    pub fn is_none(&self) -> bool {
        self.ptr.is_none()
    }

    /// Converts from &mut OptionBox<T> to Option<&Box<T>>.
    ///
    /// ```rust
    /// use galbi::*;
    ///
    /// let text: OptionBox<String> = OptionBox::some("Hello, world!".to_string());
    /// // First, cast `OptionBox<String>` to `Option<&Box<T>>` with `as_ref`,
    /// // then consume *that* with `map`, leaving `text` on the stack.
    /// let text_length: Option<usize> = text.as_ref().map(|s| s.len());
    /// println!("still can print text: {:?}", text);
    /// ```
    pub fn as_ref(&self) -> Option<&Box<T>> {
        self.ptr.as_ref()
    }

    /// Converts from &mut OptionBox<T> to Option<&mut Box<T>>.
    ///
    /// ```rust
    /// use galbi::*;
    ///
    /// let mut x = OptionBox::some(2);
    /// match x.as_mut() {
    ///    Some(v) => *v = Box::new(42),
    ///    None => {},
    /// }
    /// assert_eq!(x, OptionBox::some(42));
    /// ```
    pub fn as_mut(&mut self) -> Option<&mut Box<T>> {
        self.ptr.as_mut()
    }

    /// Panics if the value is a None with a custom panic message provided by msg.
    ///
    /// ```rust
    /// use galbi::*;
    ///
    /// let x = OptionBox::some("foo".to_string());
    /// let value = x.expect("error");
    /// assert_eq!(value, Box::new("foo".to_string()));
    /// ```
    pub fn expect(self, msg: &str) -> Box<T> {
        self.ptr.expect(msg)
    }

    /// Returns the contained Some value, consuming the self value.
    /// Because this function may panic, its use is generally discouraged. Instead, prefer to use pattern matching and handle the None case explicitly, or call unwrap_or, unwrap_or_else, or unwrap_or_default.
    ///
    /// ```rust
    /// use galbi::*;
    ///
    /// let x = OptionBox::some("foo".to_string());
    /// let value = x.unwrap();
    /// assert_eq!(value, Box::new("foo".to_string()));
    /// ```
    pub fn unwrap(self) -> Box<T> {
        self.ptr.unwrap()
    }

    /// Returns the contained Some value, consuming the self value.
    /// Because this function may panic, its use is generally discouraged. Instead, prefer to use pattern matching and handle the None case explicitly, or call unwrap_or, unwrap_or_else, or unwrap_or_default.
    ///
    /// ```rust
    /// use galbi::*;
    ///
    /// assert_eq!(OptionBox::some(12).unwrap_or(Box::new(13)), Box::new(12));
    /// assert_eq!(OptionBox::none().unwrap_or(Box::new(13)), Box::new(13));
    /// ```
    pub fn unwrap_or(self, default: Box<T>) -> Box<T> {
        self.ptr.unwrap_or(default)
    }

    /// Returns the contained Some value or computes it from a closure.
    ///
    /// ```rust
    /// use galbi::*;
    ///
    /// assert_eq!(OptionBox::some(12).unwrap_or_else(|| Box::new(13)), Box::new(12));
    /// assert_eq!(OptionBox::none().unwrap_or_else(|| Box::new(13)), Box::new(13));
    /// ```
    pub fn unwrap_or_else<F>(self, f: F) -> Box<T>
    where
        F: FnOnce() -> Box<T>,
    {
        self.ptr.unwrap_or_else(f)
    }

    /// Maps an Option<Box<T>> to Option<U> by applying a function to a contained value.
    ///
    /// ```rust
    /// use galbi::*;
    ///
    /// assert_eq!(OptionBox::some(20).map(|e| (*e)*2), Some(40));
    /// ```
    pub fn map<U, F>(self, f: F) -> Option<U>
    where
        F: FnOnce(Box<T>) -> U,
    {
        self.ptr.map(f)
    }

    /// Applies a function to the contained value (if any), or returns the provided default (if not).
    /// Arguments passed to map_or are eagerly evaluated; if you are passing the result of a function call, it is recommended to use map_or_else, which is lazily evaluated.
    ///
    /// ```rust
    /// use galbi::*;
    ///
    /// let x = OptionBox::some(10);
    /// assert_eq!(x.map_or(111, |e| *e), 10);
    ///
    /// let x: OptionBox<i32> = OptionBox::none();
    /// assert_eq!(x.map_or(10, |e| *e), 10);
    /// ```
    pub fn map_or<U, F>(self, default: U, f: F) -> U
    where
        F: FnOnce(Box<T>) -> U,
    {
        self.ptr.map_or(default, f)
    }

    /// Applies a function to the contained value (if any), or computes a default (if not).
    ///
    /// ```rust
    /// use galbi::*;
    ///
    /// let x = OptionBox::some(10);
    /// assert_eq!(x.map_or_else(||111, |e| *e), 10);
    ///
    /// let x: OptionBox<i32> = OptionBox::none();
    /// assert_eq!(x.map_or_else(||10, |e| *e), 10);
    /// ```
    pub fn map_or_else<U, D, F>(self, default: D, f: F) -> U
    where
        D: FnOnce() -> U,
        F: FnOnce(Box<T>) -> U,
    {
        self.ptr.map_or_else(default, f)
    }

    /// Transforms the `Option<Box<T>>` into a `Result<Box<T>, E>`, mapping Some(v) to Ok(v) and None to Err(err).
    /// Arguments passed to ok_or are eagerly evaluated; if you are passing the result of a function call, it is recommended to use ok_or_else, which is lazily evaluated.
    ///
    /// ```rust
    /// use galbi::*;
    ///
    /// let x = OptionBox::some(10);
    /// assert_eq!(x.ok_or(Box::new(0)), Ok(Box::new(10)));
    ///
    /// let x: OptionBox<i32> = OptionBox::none();
    /// assert_eq!(x.ok_or(Box::new(0)), Err(Box::new(0)));
    /// ```
    pub fn ok_or<E>(self, err: E) -> Result<Box<T>, E> {
        self.ptr.ok_or(err)
    }

    /// Transforms the Option<Box<T>> into a Result<Box<T>, E>, mapping Some(v) to Ok(v) and None to Err(err()).
    ///
    /// ```rust
    /// use galbi::*;
    ///
    /// let x = OptionBox::some(10);
    /// assert_eq!(x.ok_or_else(||Box::new(0)), Ok(Box::new(10)));
    ///
    /// let x: OptionBox<i32> = OptionBox::none();
    /// assert_eq!(x.ok_or_else(||Box::new(0)), Err(Box::new(0)));
    /// ```
    pub fn ok_or_else<E, F>(self, err: F) -> Result<Box<T>, E>
    where
        F: FnOnce() -> E,
    {
        self.ptr.ok_or_else(err)
    }

    /// Returns an iterator over the possibly contained value.
    ///
    /// ```rust
    /// use galbi::*;
    ///
    /// let x = OptionBox::some(10);
    /// assert_eq!(x.iter().next(), Some(&Box::new(10)));
    ///
    /// let x: OptionBox<i32> = OptionBox::none();
    /// assert_eq!(x.iter().next(), None);
    /// ```
    pub fn iter(&self) -> Iter<'_, Box<T>> {
        self.ptr.iter()
    }

    /// Returns a mutable iterator over the possibly contained value.
    ///
    /// ```rust
    /// use galbi::*;
    ///
    /// let mut x = OptionBox::some(4);
    /// match x.iter_mut().next() {
    ///    Some(v) => *v = Box::new(42),
    ///    None => {},
    /// }
    /// assert_eq!(x, OptionBox::some(42));
    ///
    /// let mut x: OptionBox<u32> = OptionBox::none();
    /// assert_eq!(x.iter_mut().next(), None);
    /// ```
    pub fn iter_mut(&mut self) -> IterMut<'_, Box<T>> {
        self.ptr.iter_mut()
    }

    /// Returns None if the option is None, otherwise returns optb.
    ///
    /// ```rust
    /// use galbi::*;
    ///
    /// let x = OptionBox::some(2);
    /// let y: Option<String> = None;
    /// assert_eq!(x.and(y), None);
    ///
    /// let x: OptionBox<i32> = OptionBox::none();
    /// let y = Some("foo".to_string());
    /// assert_eq!(x.and(y), None);
    ///
    /// let x = OptionBox::some(2);
    /// let y = Some("foo".to_string());
    /// assert_eq!(x.and(y), Some("foo".to_string()));
    ///
    /// let x: OptionBox<i32> = OptionBox::none();
    /// let y: Option<String> = None;
    /// assert_eq!(x.and(y), None);
    /// ```
    pub fn and<U>(self, optb: Option<U>) -> Option<U> {
        self.ptr.and(optb)
    }

    /// Returns None if the option is None, otherwise calls f with the wrapped value and returns the result.
    /// Some languages call this operation flatmap.
    ///
    /// ```rust
    /// use galbi::*;
    ///
    /// fn sq(x: Box<u32>) -> Option<u32> {
    ///     Some((*x)*(*x))
    /// }
    /// fn nope(x: Box<u32>) -> Option<u32> {
    ///     None
    /// }
    ///
    /// assert_eq!(OptionBox::some(2).and_then(sq), Some(4));
    /// assert_eq!(OptionBox::some(2).and_then(nope), None);
    /// assert_eq!(OptionBox::<u32>::none().and_then(sq), None);
    /// ```
    pub fn and_then<U, F>(self, f: F) -> Option<U>
    where
        F: FnOnce(Box<T>) -> Option<U>,
    {
        self.ptr.and_then(f)
    }

    /// Returns None if the option is None, otherwise calls predicate with the wrapped value and returns:
    /// - [Some(t)] if predicate returns true (where t is the wrapped value), and
    /// - None if predicate returns false.
    /// This function works similar to Iterator::filter(). You can imagine the Option<Box<T>> being an iterator over one or zero elements. filter() lets you decide which elements to keep.
    ///
    /// ```rust
    /// use galbi::*;
    ///
    /// fn is_even(ptr: &Box<i32>) -> bool {
    ///     let num: i32 = **ptr;
    ///     num % 2 == 0
    /// }
    ///
    /// assert_eq!(OptionBox::none().filter(is_even), None);
    /// assert_eq!(OptionBox::some(3).filter(is_even), None);
    /// assert_eq!(OptionBox::some(4).filter(is_even), Some(Box::new(4)));
    /// ```
    pub fn filter<P>(self, predicate: P) -> Option<Box<T>>
    where
        P: FnOnce(&Box<T>) -> bool,
    {
        self.ptr.filter(predicate)
    }

    /// Returns the option if it contains a value, otherwise returns optb.
    /// Arguments passed to or are eagerly evaluated; if you are passing the result of a function call, it is recommended to use or_else, which is lazily evaluated.
    ///
    /// ```rust
    /// use galbi::*;
    ///
    /// let x = OptionBox::some(2);
    /// let y = None;
    /// assert_eq!(x.or(y), Some(Box::new(2)))
    /// ```
    pub fn or(self, optb: Option<Box<T>>) -> Option<Box<T>> {
        self.ptr.or(optb)
    }

    /// Returns the option if it contains a value, otherwise calls f and returns the result.
    ///
    /// ```rust
    /// use galbi::*;
    ///
    /// let x: OptionBox<i32> = OptionBox::none();
    /// assert_eq!(x.or_else(|| Some(Box::new(1234))), Some(Box::new(1234)))
    /// ```
    pub fn or_else<F>(self, f: F) -> Option<Box<T>>
    where
        F: FnOnce() -> Option<Box<T>>,
    {
        self.ptr.or_else(f)
    }

    /// Returns Some if exactly one of self, optb is Some, otherwise returns None.
    ///
    /// ```rust
    /// use galbi::*;
    ///
    /// let x = OptionBox::some(2);
    /// let y = None;
    /// assert_eq!(x.xor(y), Some(Box::new(2)))
    /// ```
    pub fn xor(self, optb: Option<Box<T>>) -> Option<Box<T>> {
        self.ptr.xor(optb)
    }
}
