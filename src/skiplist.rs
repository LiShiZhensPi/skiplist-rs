use std::mem::MaybeUninit;
use std::ptr::NonNull;

use crate::random::Random;
use crate::skipnode::SkipNode;

const DEFAULT_MAX_LEVL: usize = 16;

pub struct SkipList<T>
where
    T: Ord,
{
    head: SkipNode<T>,
    len: usize,
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

    // returns (node,prevs)
    unsafe fn find_less_than(&mut self, key: &T) -> (&SkipNode<T>, Vec<*mut SkipNode<T>>) {
        let mut prevs =
            vec![MaybeUninit::<*mut SkipNode<T>>::uninit().assume_init(); self.head.level];

        let mut level = self.head.level - 1;
        let mut node = &mut self.head;

        loop {
            while let Some(mut next) = node.next[level] {
                if &next.as_ref().key < key {
                    node = next.as_mut();
                } else {
                    break;
                }
            }

            prevs[level] = &mut *node;

            if level > 0 {
                level -= 1;
            } else {
                break;
            }
        }
        (node, prevs)
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.len
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn insert(&mut self, key: T) {
        unsafe {
            let (node, mut prevs) = self.find_less_than(&key);
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
            let mut p_new_node = NonNull::from(&new_node);

            for l in 0..p_new_node.as_ref().level {
                p_new_node.as_mut().next[l] = (*prevs[l]).next[l];
                (*prevs[l]).next[l] = Some(p_new_node);
            }

            self.len += 1;
        }
    }

    // pub fn find(&self, key: &T) -> Option<&T> {
    //     self.find_greater_or_equal(key).and_then(|node| {
    //         if &node.key == key {
    //             Some(&node.key)
    //         } else {
    //             None
    //         }
    //     })
    // }

    // pub fn delete(&self,key :&T)->bool{

    // }
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
            assert!(i == list.len() as i32);
        }

        // for i in 1..100 {
        //     assert!(list.find(&i) == Some(&i));
        // }

        // for i in 100..101 {
        //     assert!(list.find(&i) == None)
        // }
    }
}
