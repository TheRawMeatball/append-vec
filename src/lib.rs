use std::{cell::UnsafeCell, fmt::Debug};

pub struct AppendVec<T: ?Sized>(UnsafeCell<Vec<Box<T>>>);

impl<T: ?Sized> AppendVec<T> {
    pub fn push(&self, t: Box<T>) {
        unsafe {
            (*self.0.get()).push(t);
        }
    }

    pub fn get<'a>(&'a self, i: usize) -> Option<&'a T> {
        unsafe {
            (*self.0.get())
                .get(i)
                .map(|v| &**v)
                .map(|v| std::mem::transmute::<&T, &'a T>(v))
        }
    }

    pub fn last(&self) -> Option<&T> {
        self.get(self.len() - 1)
    }

    pub fn len(&self) -> usize {
        unsafe { (*self.0.get()).len() }
    }

    pub fn inner(&mut self) -> &mut Vec<Box<T>> {
        self.0.get_mut()
    }

    pub fn pop(&mut self) -> Option<Box<T>> {
        self.0.get_mut().pop()
    }

    pub fn new() -> Self {
        Self(Default::default())
    }
}

impl<T> Clone for AppendVec<T>
where
    Box<T>: Clone,
{
    fn clone(&self) -> Self {
        let vec = AppendVec::new();
        // Safe: this data structure is single-threader, meaning no other code will
        // mutate the inner vec while we iterate it.
        let inner = unsafe { &*self.0.get() };
        for elem in inner.iter() {
            vec.push(elem.clone());
        }
        vec
    }
}

impl<T: ?Sized> Drop for AppendVec<T> {
    fn drop(&mut self) {
        while let Some(_) = self.pop() {}
    }
}

impl<T: Debug> Debug for AppendVec<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[test]
fn test() {
    let vec = AppendVec::<String>::new();
    println!("{:?}", &vec);
    vec.push("hello".into());
    vec.push("hi".into());
    let x = vec.get(0);
    println!("{:?}", &vec);
    vec.push("is this burning yet?".into());
    dbg!(x);
    let mut vec = vec;
    println!("{:?}", vec.pop());

    println!("{:?}", &vec);
}
