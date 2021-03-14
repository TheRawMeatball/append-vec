use std::{cell::UnsafeCell, fmt::Debug};

pub struct AppendVec<T>(UnsafeCell<Vec<Box<T>>>);

impl<T> AppendVec<T> {
    pub fn push(&self, t: T) {
        unsafe {
            (*self.0.get()).push(Box::new(t));
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

    pub fn pop(&mut self) -> Option<T> {
        self.0.get_mut().pop().map(|v| *v)
    }

    pub fn new() -> Self {
        Self(Default::default())
    }
}

impl<T> Drop for AppendVec<T> {
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
