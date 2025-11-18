use std::{cell::{Cell, UnsafeCell}, ops::{Deref, DerefMut}};

pub struct RefCell<T> {
    inner: UnsafeCell<T>,
    state: Cell<BorrowState>,
}

pub struct Ref<'a, T> {
    borrow: &'a T,
    state: &'a Cell<BorrowState>,
}

pub struct RefMut<'a, T> {
    borrow: &'a mut T,
    state: &'a Cell<BorrowState>,
}

#[derive(Copy, Clone)]
enum BorrowState {
    Unborrowed,
    Shared,
    Exclusive,
}

impl<T> RefCell<T> {
    pub fn new(inner: T) -> Self {
        Self {
            inner: UnsafeCell::new(inner),
            state: Cell::new(BorrowState::Unborrowed),
        }
    }

    pub fn borrow(&self) -> Ref<T> {
        unsafe {
            match self.state.get() {
                BorrowState::Unborrowed | BorrowState::Shared => {
                    self.state.set(BorrowState::Shared);

                    Ref {
                        borrow: & *self.inner.get(),
                        state: &self.state,
                    }
                }
                BorrowState::Exclusive => {
                    panic!("tried to shared borrow while holding exclusive borrow");
                }
            }
        }
    }

    pub fn borrow_mut(&self) -> RefMut<T> {
        unsafe {
            match self.state.get() {
                BorrowState::Unborrowed => {
                    self.state.set(BorrowState::Exclusive);

                    RefMut {
                        borrow: &mut *self.inner.get(),
                        state: &self.state,
                    }
                }
                BorrowState::Shared | BorrowState::Exclusive => {
                    panic!("tried to exclusive borrow while holding borrow");
                }
            }
        }
    }
}

impl<T> Deref for Ref<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.borrow
    }
}

impl<T> Deref for RefMut<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.borrow
    }
}

impl<T> DerefMut for RefMut<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.borrow
    }
}

impl<T> Drop for Ref<'_, T> {
    fn drop(&mut self) {
        self.state.set(BorrowState::Unborrowed);
    }
}

impl<T> Drop for RefMut<'_, T> {
    fn drop(&mut self) {
        self.state.set(BorrowState::Unborrowed);
    }
}
