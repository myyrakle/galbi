pub struct OptionBox<T> {
    ptr: Option<Box<T>>,
}

impl<T> OptionBox<T> {
    pub fn new(x: Option<T>) -> OptionBox<T> {
        OptionBox {
            ptr: if x.is_some() {
                Some(Box::new(x.unwrap()))
            } else {
                None
            },
        }
    }
}

impl<T> std::ops::Deref for OptionBox<T> {
    type Target = Option<Box<T>>;

    fn deref(&self) -> &Self::Target {
        &self.ptr
    }
}

impl<T> std::ops::DerefMut for OptionBox<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.ptr
    }
}
