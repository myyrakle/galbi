use std::ops::{Deref, DerefMut};
use std::sync::{Arc, Mutex};

pub struct ArcMutex<T> {
    ptr: Arc<Mutex<T>>,
}

impl<T> ArcMutex<T> {
    pub fn new(x: T) -> ArcMutex<T> {
        ArcMutex {
            ptr: Arc::new(Mutex::new(x)),
        }
    }
}
impl<T> Deref for ArcMutex<T> {
    type Target = Arc<Mutex<T>>;

    fn deref(&self) -> &Self::Target {
        &self.ptr
    }
}

impl<T> DerefMut for ArcMutex<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.ptr
    }
}
