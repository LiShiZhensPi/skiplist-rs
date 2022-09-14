use std::mem::MaybeUninit;
use std::ptr::NonNull;

use crate::random::Random;
use crate::skipnode::SkipNode;

const DEFAULT_MAX_LEVL: usize = 16;

pub struct SkipList<T> {
    head: SkipNode<T>,
    len: usize,
    rnd: Random,
}

impl<T> SkipList<T> {
    pub fn new() -> Self {
        unsafe {
            SkipList {
                head: SkipNode::new(MaybeUninit::<T>::uninit().assume_init(), DEFAULT_MAX_LEVL),
                len: 0,
                rnd: Random::new(0xdeadbeef),
            }
        }
    }

    fn random_level(&mut self) -> usize {
        // Increase height with probability 1 in kBranching
        const K_BRANCHING: u32 = 4;
        let mut level: usize = 1;
        while level < self.head.level && self.rnd.one_in(K_BRANCHING) {
            level += 1;
        }
        level
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.len
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
}

impl<T> SkipList<T>
where
    T: Ord,
{
    // returns (node,prevs)
    fn find_less_than(&self, key: &T) -> (&SkipNode<T>, Vec<NonNull<SkipNode<T>>>) {
        unsafe {
            let mut prevs = vec![NonNull::dangling(); self.head.level];

            let mut level = self.head.level - 1;
            let mut node = &self.head;

            loop {
                while let Some(next) = node.next[level] {
                    if &next.as_ref().key < key {
                        node = next.as_ref();
                    } else {
                        break;
                    }
                }

                // How to convert &SkipNode<T> to NonNull<...>?
                // https://doc.rust-lang.org/src/core/ptr/non_null.rs.html#792-802
                prevs[level] = NonNull::from(node);

                if level > 0 {
                    level -= 1;
                } else {
                    break;
                }
            }
            (node, prevs)
        }
    }

    pub fn insert(&mut self, key: T) {
        unsafe {
            let (node, prevs) = self.find_less_than(&key);
            if let Some(mut next) = node.next[0] {
                if next.as_ref().key == key {
                    //Replace the item by key.
                    //Note that item and key are equal but not necessarily the same.
                    next.as_mut().key = key;
                    return;
                }
            }

            //insert

            //convert SkipNode to NonNull
            let new_node = SkipNode::new(key, self.random_level());
            let mut p_new_node = NonNull::new_unchecked(Box::leak(Box::new(new_node)));

            for l in 0..p_new_node.as_ref().level {
                let mut p_prev = prevs[l];
                p_new_node.as_mut().next[l] = p_prev.as_ref().next[l];
                p_prev.as_mut().next[l] = Some(p_new_node);
            }

            self.len += 1;
        }
    }

    pub fn find(&self, key: &T) -> Option<&T> {
        unsafe {
            self.find_less_than(key).0.next[0].and_then(|node| {
                if &node.as_ref().key == key {
                    Some(&node.as_ref().key)
                } else {
                    None
                }
            })
        }
    }

    pub fn delete(&mut self, key: &T) -> bool {
        unsafe {
            let (node, mut prevs) = self.find_less_than(key);
            if let Some(next) = node.next[0] {
                if &next.as_ref().key == key {
                    // delete
                    for l in 0..next.as_ref().level {
                        prevs[l].as_mut().next[l] = next.as_ref().next[l];
                    }

                    // drop
                    Box::from_raw(next.as_ptr());

                    self.len -= 1;

                    return true;
                }
            }
            return false;
        }
    }
}

impl<T> Drop for SkipList<T> {
    fn drop(&mut self) {
        println!("list drop");
        unsafe {
            let mut node = self.head.next[0];
            while let Some(next) = node {
                node = next.as_ref().next[0];
                // drop
                Box::from_raw(next.as_ptr());
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn skip_list_test() {
        let mut list = SkipList::<i32>::new();
        assert!(list.head.level == DEFAULT_MAX_LEVL);

        for i in 1..100 {
            list.insert(i);
            assert!(list.len() == i as usize);
        }

        for i in 1..100 {
            assert!(list.find(&i) == Some(&i));
        }

        for i in 100..200 {
            assert!(list.find(&i) == None)
        }

        for i in 1..100 {
            assert!(list.delete(&i));
        }

        assert!(list.is_empty());
    }
}
