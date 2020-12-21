struct OptionBox<T> {
    ptr: Option<Box<T>>,
}

struct RcCell<T> {
    ptr: std::rc::Rc<std::cell::RefCell<T>>,
}

struct ArcMutex<T> {
    ptr: std::sync::Arc<std::sync::Mutex<T>>,
}
