use crate::cel_m::CellM;
use std::cell::UnsafeCell;

#[derive(Clone, Copy)]
pub enum RefState {
    UnShared,
    Shared(usize),
    Exclusive,
}

pub struct RefCellM<T> {
    value: UnsafeCell<T>,
    state: CellM<RefState>,
}

impl<T> RefCellM<T> {
    pub fn new(value: T) -> Self {
        Self {
            value: UnsafeCell::new(value),
            state: CellM::new(RefState::UnShared),
        }
    }

    pub fn borrow(&self) -> Option<Refm<'_, T>> {
        match self.state.get() {
            RefState::Exclusive => None,
            RefState::UnShared => {
                self.state.set(RefState::Shared(1));
                // SAFETY
                // No exclusive references given out and UnsafeCell is !Sync
                Some(Refm { refcell: self })
            }
            RefState::Shared(n) => {
                self.state.set(RefState::Shared(n + 1));
                // No exclusive references given out and UnsafeCell is !Sync
                Some(Refm { refcell: self })
            }
        }
    }

    pub fn borrow_mut(&self) -> Option<RefMutm<'_, T>> {
        if let RefState::UnShared = self.state.get() {
            self.state.set(RefState::Exclusive);
            // SAFETY
            // No other references given out it is either Shared or Exlusive
            // and UnsafeCell is !Sync
            Some(RefMutm { refcell: self })
        } else {
            None
        }
    }
}

pub struct Refm<'refcell, T> {
    refcell: &'refcell RefCellM<T>,
}

impl<T> std::ops::Deref for Refm<'_, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        // SAFETY
        // a Refm is created only if no exclusive references given out
        // once it is given out, state is set to shared
        // and no exclusive refrns given out
        unsafe { &*self.refcell.value.get() }
    }
}

impl<T> Drop for Refm<'_, T> {
    fn drop(&mut self) {
        match self.refcell.state.get() {
            RefState::Exclusive | RefState::UnShared => unreachable!(),
            RefState::Shared(1) => self.refcell.state.set(RefState::UnShared),
            RefState::Shared(n) => {
                self.refcell.state.set(RefState::Shared(n - 1));
            }
        }
    }
}

pub struct RefMutm<'refcell, T> {
    refcell: &'refcell RefCellM<T>,
}

impl<T> std::ops::Deref for RefMutm<'_, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        // SAFETY
        // a RefMutm is created only if there is only one exclusive references given out
        // no other references given out
        unsafe { &*self.refcell.value.get() }
    }
}

impl<T> std::ops::DerefMut for RefMutm<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        // SAFETY
        // a RefMutm is created only if there is no other references given out
        unsafe { &mut *self.refcell.value.get() }
    }
}

impl<T> Drop for RefMutm<'_, T> {
    fn drop(&mut self) {
        match self.refcell.state.get() {
            RefState::Shared(_) | RefState::UnShared => unreachable!(),
            RefState::Exclusive => self.refcell.state.set(RefState::UnShared),
        }
    }
}
