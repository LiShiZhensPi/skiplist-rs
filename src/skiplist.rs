use std::{mem::MaybeUninit, ptr::NonNull};

use crate::random::Random;

const DEFAULT_MAX_LEVL: usize = 12;

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

pub struct SkipList<T> {
    head: SkipNode<T>,
    max_level: usize,
    rnd: Random,
}

impl<T> SkipList<T>
where
    T: Ord,
{
    pub fn new() -> Self {
        unsafe {
            SkipList {
                head: SkipNode::new(MaybeUninit::<T>::uninit().assume_init(), DEFAULT_MAX_LEVL),
                max_level: DEFAULT_MAX_LEVL,
                rnd: Random::new(0xdeadbeef),
            }
        }
    }

    fn random_level(&mut self) -> usize {
        // Increase height with probability 1 in kBranching
        const k_branching: u32 = 4;
        let mut level: usize = 1;
        while level < self.max_level && self.rnd.one_in(k_branching) {
            level += 1;
        }
        level
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn skip_list_test() {
        let list = SkipList::<i32>::new();
        assert!(list.max_level == DEFAULT_MAX_LEVL);
    }
}
