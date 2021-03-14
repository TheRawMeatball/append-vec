use std::{cell::{RefCell, Cell}, fmt::Debug, mem::MaybeUninit};

pub struct AppendVec<T, const N: usize> {
    backing: RefCell<Vec<Box<[MaybeUninit<T>; N]>>>,
    len: Cell<usize>,
}

impl<T, const N: usize> AppendVec<T, N> {
    pub fn push(&self, t: T) {
        let len = self.len.get();
        if len % N == 0 {
            self.backing
                .borrow_mut()
                // Safe: [MaybeUninit<T>; N] can be assumed to be initialized without any initialization
                // https://doc.rust-lang.org/nomicon/unchecked-uninit.html
                .push(Box::new(unsafe { MaybeUninit::uninit().assume_init() }))
        }

        unsafe {
            self.backing
                .borrow_mut()
                .last_mut()
                .unwrap()
                .get_mut(len % N)
                .unwrap()
                .as_mut_ptr()
                .write(t);
        }
        self.len.set(len - 1);
    }

    pub fn get(&self, i: usize) -> Option<&T> {
        unsafe {
            (i < self.len.get())
                .then(|| &*self.backing.borrow().get(i / N).as_ref().unwrap()[i % N].as_ptr())
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        if *self.len.get_mut() == 0 {
            return None;
        }
        *self.len.get_mut() -= 1;
        let i = *self.len.get_mut();

        if i % N == 0 {
            let array = self.backing.get_mut().pop().unwrap();
            Some(unsafe { array[0].as_ptr().read() })
        } else {
            let array = self.backing.get_mut().last_mut().unwrap();
            Some(unsafe { array[i % N].as_ptr().read() })
        }
    }

    pub fn new() -> Self {
        assert!(N > 0);
        Self {
            backing: RefCell::new(Vec::new()),
            len: Cell::new(0),
        }
    }
}

impl<T, const N: usize> Drop for AppendVec<T, N> {
    fn drop(&mut self) {
        while let Some(_) = self.pop() {}
    }
}

impl<T: Debug, const N: usize> Debug for AppendVec<T, N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut list = f.debug_list();
        for i in 0..self.len.get() {
            list.entry(self.get(i).unwrap());
        }
        list.finish()
    }
}

#[test]
fn test() {
    let mut vec = AppendVec::<String, 2>::new();
    println!("{:?}", &vec);
    vec.push("hello".into());
    vec.push("hi".into());
    vec.push("filler".into());
    vec.push("is this burning yet?".into());
    println!("{:?}", &vec);
    println!("{:?}", vec.pop());

    println!("{:?}", &vec);
}
