#![allow(dead_code)]

type Floor = i32;

#[derive(Debug)]
/// An event in the elevator system that the controller must react to.
enum Event {
    Arrived(Floor),
    DoorOpen,
    DoorClosed,
    ButtonPressed(Button)
}

/// A direction of travel.
#[derive(Debug)]
enum Direction {
    Up,
    Down,
}

#[derive(Debug)]
enum Button {
    LobbyCall(Floor, Direction),
    CarCall(Floor)
}

/// The car has arrived on the given floor.
fn car_arrived(floor: i32) -> Event {
    let event = Event::Arrived(floor);
    event
}

/// The car doors have opened.
fn car_door_opened() -> Event {
    let event = Event::DoorOpen;
    event
}

/// The car doors have closed.
fn car_door_closed() -> Event {
    let event = Event::DoorClosed;
    event
}

/// A directional button was pressed in an elevator lobby on the given floor.
fn lobby_call_button_pressed(floor: i32, dir: Direction) -> Event {
    let button = Button::LobbyCall(floor, dir);
    let event = Event::ButtonPressed(button);
    event
}

/// A floor button was pressed in the elevator car.
fn car_floor_button_pressed(floor: i32) -> Event {
    let button = Button::CarCall(floor);
    let event = Event::ButtonPressed(button);
    event
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