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