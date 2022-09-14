use std::{mem::MaybeUninit, ptr::NonNull};

use crate::random::Random;

const DEFAULT_MAX_LEVL: usize = 12;

type Link<T> = Option<NonNull<SkipNode<T>>>;
struct SkipNode<T> {
    key: T,
    level: usize,
    next: Vec<Link<T>>,
}

impl<T> SkipNode<T>
where
    T: Ord,
{
    fn new(key: T, level: usize) -> Self {
        SkipNode {
            key,
            level,
            next: vec![None; level],
        }
    }
}

pub struct SkipList<T> {
    head: SkipNode<T>,
    len: usize,
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
                len: 0,
                max_level: DEFAULT_MAX_LEVL,
                rnd: Random::new(0xdeadbeef),
            }
        }
    }

    fn random_level(&mut self) -> usize {
        // Increase height with probability 1 in kBranching
        const K_BRANCHING: u32 = 4;
        let mut level: usize = 1;
        while level < self.max_level && self.rnd.one_in(K_BRANCHING) {
            level += 1;
        }
        level
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn insert(&mut self, key: T) {
        unsafe {
            let mut node = &self.head;
            let mut level = self.max_level - 1;

            loop {
                while let Some(next) = node.next[level] {
                    if next.as_ref().key < key {
                        node = next.as_ref();
                    } else {
                        break;
                    }
                }
                if level > 0 {
                    level -= 1;
                } else {
                    break;
                }
            }

            if let Some(mut next) = node.next[level] {
                if next.as_ref().key == key {
                    //Replace the item by key.
                    //Note that item and key are equal but not necessarily the same.
                    next.as_mut().key = key;
                    return;
                }
            }

            //insert

            //convert SkipNode to NonNull,maybe use 'from()' and 'into()' is better?
            let new_node = SkipNode::new(key, self.random_level());
            let mut p_new_node = NonNull::new_unchecked(Box::leak(Box::new(new_node)));

            for l in 0..p_new_node.as_ref().level {
                let mut node = &mut self.head;
                while let Some(mut next) = node.next[l] {
                    //operator '<' means 'pub fn lt(&self, other: &Rhs) -> bool', so the new_node.key will not be moved
                    if next.as_ref().key < p_new_node.as_ref().key {
                        node = next.as_mut();
                    } else {
                        break;
                    }
                }
                p_new_node.as_mut().next[l] = node.next[l];
                node.next[l] = Some(p_new_node);
            }

            self.len += 1;
        }
    }

    pub fn find_greater_or_equal(&self, key: &T) -> Option<&T> {
        unsafe {
            let mut node = &self.head;
            let mut level = self.max_level - 1;

            loop {
                while let Some(next) = node.next[level] {
                    if &next.as_ref().key < key {
                        node = next.as_ref();
                    } else {
                        break;
                    }
                }
                if level > 0 {
                    level -= 1;
                } else {
                    break;
                }
            }
            if let Some(next) = node.next[level] {
                Some(&next.as_ref().key)
            } else {
                None
            }
        }
    }

    pub fn find(&self, key: &T) -> Option<&T> {
        self.find_greater_or_equal(key)
            .and_then(|res| if res == key { Some(res) } else { None })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn skip_list_test() {
        let mut list = SkipList::<i32>::new();
        assert!(list.max_level == DEFAULT_MAX_LEVL);

        for i in 1..100 {
            list.insert(i);
            assert!(i == list.len() as i32);
        }

        for i in 1..100 {
            assert!(list.find(&i) == Some(&i));
        }

        for i in 100..101 {
            assert!(list.find(&i) == None)
        }
    }
}
