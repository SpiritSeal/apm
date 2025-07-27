#[cfg(feature = "unstable_grab")]
use rdev::{grab, Event};

#[cfg(feature = "unstable_grab")]
fn callback(event: Event) -> Option<Event> {
    println!("Event: {:?}", event);
    Some(event) // Return None to suppress the event
}

fn main() {
    #[cfg(feature = "unstable_grab")]
    if let Err(error) = grab(callback) {
        println!("Error: {:?}", error);
    }
    println!("Done");
}


