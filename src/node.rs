use crossbeam::atomic::AtomicCell;
pub struct Node<T: ?Sized> {
    val: AtomicCell<T>,
}
