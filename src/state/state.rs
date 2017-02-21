pub trait State {
    fn name() -> &'static str;
}

pub enum Open {}

impl State for Open {
    fn name() -> &'static str {
        "open"
    }
}

pub enum Closed {}

impl State for Closed {
    fn name() -> &'static str {
        "closed"
    }
}

pub enum Locked {}

impl State for Locked {
    fn name() -> &'static str {
        "locked"
    }
}
