use std::marker::PhantomData;
use super::state::{Open, Closed, Locked, State};

#[derive(Debug)]
pub enum Color {
    DarkBrown,
    LightBrown,
    Yellow,
    Green,
    Blue
}

pub struct Door<T: State = Open> {
    color: Color,
    _data: PhantomData<T>
}

impl<T: State> Door<T> {
    pub fn new(color: Color) -> Self {
        Door { color: color, _data: PhantomData }
    }

    pub fn print(&self) {
        println!("{:?} door is {}", self.color, T::name());
    }
}

impl Door<Open> {
    pub fn close(self) -> Door<Closed> {
        Door::new(self.color)
    }
}

impl Door<Closed> {
    pub fn open(self) -> Door<Open> {
        Door::new(self.color)
    }

    pub fn lock(self) -> Door<Locked> {
        Door::new(self.color)
    }
}

impl Door<Locked> {
    pub fn unlock(self) -> Door<Closed> {
        Door::new(self.color)
    }
}
