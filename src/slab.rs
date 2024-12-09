use std::mem::{swap, transmute, MaybeUninit};
use std::ops::{Deref, DerefMut};

pub struct Slab<const CAP: usize, T> {
    data: [MaybeUninit<T>; CAP],
    len: usize,
}

impl<const CAP: usize, T> Default for Slab<CAP, T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<const CAP: usize, T> Slab<CAP, T> {
    pub const fn new() -> Self {
        Self {
            data: [const { MaybeUninit::uninit() }; CAP],
            len: 0,
        }
    }

    pub fn push_back(&mut self, value: T) {
        assert!(self.len < CAP);
        self.data[self.len].write(value);
        self.len += 1;
    }

    pub fn pop_back(&mut self) -> Option<T> {
        if self.len == 0 {
            None
        } else {
            self.len -= 1;
            let mut v = MaybeUninit::uninit();
            swap(&mut v, &mut self.data[self.len]);
            Some(unsafe { v.assume_init() })
        }
    }
}

impl<const CAP: usize, T> Deref for Slab<CAP, T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        unsafe { transmute(&self.data[..self.len]) }
    }
}

impl<const CAP: usize, T> DerefMut for Slab<CAP, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { transmute(&mut self.data[..self.len]) }
    }
}

impl<const CAP: usize, T> Drop for Slab<CAP, T> {
    fn drop(&mut self) {
        self.data
            .iter_mut()
            .take(self.len)
            .for_each(|slot| unsafe { slot.assume_init_drop() });
        self.len = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::AtomicUsize;
    use std::sync::atomic::Ordering::SeqCst;

    #[test]
    fn test_slab_push_pop() {
        let mut slab = Slab::<5, _>::new();
        slab.push_back(1);
        slab.push_back(2);
        slab.push_back(3);
        slab.push_back(4);
        slab.push_back(5);

        assert_eq!(slab.pop_back(), Some(5));
        assert_eq!(slab.pop_back(), Some(4));
        assert_eq!(slab.pop_back(), Some(3));
        assert_eq!(slab.pop_back(), Some(2));
        assert_eq!(slab.pop_back(), Some(1));
        assert_eq!(slab.pop_back(), None);
        assert_eq!(slab.pop_back(), None);
    }

    #[test]
    #[should_panic]
    fn test_slab_push_out_of_capacity() {
        let mut slab = Slab::<1, _>::new();
        slab.push_back(1);
        slab.push_back(2);
    }

    #[test]
    fn test_drop() {
        struct Foo<'a>(&'a AtomicUsize);

        impl Drop for Foo<'_> {
            fn drop(&mut self) {
                self.0.fetch_add(1, SeqCst);
            }
        }

        let counter = AtomicUsize::new(0);

        let mut slab = Slab::<5, _>::new();

        slab.push_back(Foo(&counter));
        slab.push_back(Foo(&counter));
        slab.push_back(Foo(&counter));
        slab.push_back(Foo(&counter));
        slab.push_back(Foo(&counter));

        assert_eq!(counter.load(SeqCst), 0);
        let _ = slab.pop_back();
        assert_eq!(counter.load(SeqCst), 1);
        let _ = slab.pop_back();
        assert_eq!(counter.load(SeqCst), 2);
        drop(slab);
        assert_eq!(counter.load(SeqCst), 5);
    }

    #[test]
    fn test_as_slice() {
        let mut slab = Slab::<5, _>::new();
        slab.push_back(1);
        slab.push_back(2);
        slab.push_back(3);
        slab.push_back(4);
        slab.push_back(5);

        assert_eq!(&*slab, &[1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_as_slice_mut() {
        let mut slab = Slab::<5, _>::new();
        slab.push_back(1);
        slab.push_back(2);
        slab.push_back(3);
        slab.push_back(4);
        slab.push_back(5);

        let slice = &mut *slab;
        slice[0] = 5;
        slice[1] = 4;
        slice[2] = 3;

        assert_eq!(&*slab, &[5, 4, 3, 4, 5]);
    }
}
