use std::ptr::NonNull;

type Link<T> = Option<NonNull<SkipNode<T>>>;
pub struct SkipNode<T> {
    pub key: T,
    pub level: usize,
    pub next: Vec<Link<T>>,
}

impl<T> SkipNode<T>
where
    T: Ord,
{
    pub fn new(key: T, level: usize) -> Self {
        SkipNode {
            key,
            level,
            next: vec![None; level],
        }
    }
}

impl<T> Drop for SkipNode<T> {
    //drop nodes recursively
    fn drop(&mut self) {
        unsafe {
            if let Some(next) = self.next[0] {
                Box::from_raw(next.as_ptr());
            }
        }
    }
}
