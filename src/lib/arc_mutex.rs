pub struct ArcMutex<T> {
    ptr: std::sync::Arc<std::sync::Mutex<T>>,
}
