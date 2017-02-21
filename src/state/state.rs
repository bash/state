pub trait State {
    fn print();
}

pub enum Open {}

impl State for Open {
    fn print() {
        println!("open")
    }
}

pub enum Closed {}

impl State for Closed {
    fn print() {
        println!("closed")
    }
}

pub enum Locked {}

impl State for Locked {
    fn print() {
        println!("locked")
    }
}
