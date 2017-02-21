mod state;

use state::{Door, Color};

fn main() {
    let door = Door::new(Color::Green);

    door.print();

    let closed_door = door.close();

    closed_door.print();

    let locked_door = closed_door.lock();

    locked_door.print();

    let unlocked_door = locked_door.unlock();

    unlocked_door.print();

    let opened_door = unlocked_door.open();

    opened_door.print();
}
