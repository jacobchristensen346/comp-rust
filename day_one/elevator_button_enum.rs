// Demonstrate the use of Enum variants
// When it comes to describing different events using data structures.

// In this version, add in more data structures to handle the different
// types of buttons.

#![allow(dead_code)]

// A type alias for a floor number that is an integer.
type Floor = i32;

#[derive(Debug)]
/// An event in the elevator system that the controller must react to.
enum Event {
    Arrive(i32),
    DoorOpen,
    DoorClosed,
    ButtonPress(WhichButton),
}

/// A direction of travel.
#[derive(Debug)]
enum Direction {
    Up,
    Down,
}

// The specific button pressed by the user.
// Each button (variant) has a different set of fields.
#[derive(Debug)]
enum WhichButton {
    FloorButton(Floor),
    CallButton(Floor, Direction),
}

/// The car has arrived on the given floor.
fn car_arrived(floor: i32) -> Event {
    let event: Event = Event::Arrive(floor);
    return event;
}

/// The car doors have opened.
fn car_door_opened() -> Event {
    return Event::DoorOpen;
}

/// The car doors have closed.
fn car_door_closed() -> Event {
    return Event::DoorClosed;
}

/// A directional button was pressed in an elevator lobby on the given floor.
fn lobby_call_button_pressed(floor: i32, dir: Direction) -> Event {
    return Event::ButtonPress(WhichButton::CallButton(floor, dir));
}

/// A floor button was pressed in the elevator car.
fn car_floor_button_pressed(floor: i32) -> Event {
    return Event::ButtonPress(WhichButton::FloorButton(floor));
}

fn main() {
    println!(
        "A ground floor passenger has pressed the up button: {:?}",
        lobby_call_button_pressed(0, Direction::Up)
    );
    println!("The car has arrived on the ground floor: {:?}", car_arrived(0));
    println!("The car door opened: {:?}", car_door_opened());
    println!(
        "A passenger has pressed the 3rd floor button: {:?}",
        car_floor_button_pressed(3)
    );
    println!("The car door closed: {:?}", car_door_closed());
    println!("The car has arrived on the 3rd floor: {:?}", car_arrived(3));
}
