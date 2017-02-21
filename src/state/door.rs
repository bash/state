use std::marker::PhantomData;
use super::state::{Open, Closed, Locked, State};

pub struct Door<T: State = Open> {
    _data: PhantomData<T>
}

impl<T: State> Door<T> {
    pub fn new() -> Self {
        Door { _data: PhantomData }
    }

    pub fn print(&self) {
        T::print();
    }
}

impl Door<Open> {
    pub fn close(self) -> Door<Closed> {
        Door { _data: PhantomData }
    }
}

impl Door<Closed> {
    pub fn open(self) -> Door<Open> {
        Door { _data: PhantomData }
    }

    pub fn lock(self) -> Door<Locked> {
        Door { _data: PhantomData }
    }
}

impl Door<Locked> {
    pub fn unlock(self) -> Door<Closed> {
        Door { _data: PhantomData }
    }
}
