use std::marker::PhantomData;

#[cfg(test)] mod tests;

unsafe fn change_lifetime<'a, 'b, T>(a: &'a T) -> &'b T {
    (a as *const T).as_ref().unwrap()
}

pub struct GrowVec<'a, T: 'a> {
    v: Vec<Vec<T>>,
    cap: usize,
    ph: PhantomData<&'a T>,
}

impl<'a, T> GrowVec<'a, T> {
    pub fn new() -> Self {
        Self::with_capacity(16) // TBD
    }

    pub fn with_capacity(cap: usize) -> Self {
        assert!(cap > 0);
        Self {
            v: vec![Vec::with_capacity(cap)],
            cap,
            ph: PhantomData,
        }
    }

    pub fn push(&mut self, item: T) -> &'a T {
        if self.v.last().unwrap().len() == self.cap {
            self.v.push(Vec::with_capacity(self.cap));
        }
        self.v.last_mut().unwrap().push(item);
        unsafe {
            change_lifetime(self.v.last().unwrap().last().unwrap())
        }
    }
}
