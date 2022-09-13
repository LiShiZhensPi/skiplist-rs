use std::{mem::MaybeUninit, ptr::NonNull};

type Link<T> = Option<NonNull<SkipNode<T>>>;
struct SkipNode<T> {
    item: T,
    level: usize,
    prev: Link<T>,
    next: Vec<Link<T>>,
}

impl<T> SkipNode<T>
where
    T: Ord,
{
    fn new(item: T, level: usize) -> Self {
        SkipNode {
            item,
            level,
            prev: None,
            next: vec![None; level],
        }
    }
}

struct SkipList<T> {
    head: SkipNode<T>,
    max_level: usize,
}

impl<T> SkipList<T>
where
    T: Ord,
{
    fn new() -> Self {
        const DEFAULT_MAX_LEVL: usize = 8;
        unsafe {
            SkipList {
                head: SkipNode::new(MaybeUninit::<T>::uninit().assume_init(), DEFAULT_MAX_LEVL),
                max_level: DEFAULT_MAX_LEVL,
            }
        }
    }
    fn new_with_level(max_level: usize) -> Self {
        unsafe {
            SkipList {
                head: SkipNode::new(MaybeUninit::<T>::uninit().assume_init(), max_level),
                max_level,
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn skip_list_test() {
        let list = SkipList::<i32>::new();
        const DEFAULT_MAX_LEVL: usize = 8;
        assert!(list.max_level == DEFAULT_MAX_LEVL);
    }
}
