

use rdev::{listen, Event, EventType, Button, Key};
use core::time;
use std::sync::mpsc::{Sender, Receiver};
use std::thread;

use std::collections::HashMap;
use std::sync::Mutex;
use std::sync::Arc;

use std::sync::mpsc::channel;


#[derive(Debug, Hash, Eq, PartialEq)]
enum InputKey {
    Mouse(Button),
    Keyboard(Key),
}

#[derive(Debug, Hash, Eq, PartialEq)]
enum TrackableEvent {
    ButtonPress(InputKey),
    KeyPress(InputKey),
}



fn monitor_loop(m: Arc::<Mutex::<HashMap::<TrackableEvent, i32>>>) {   
    loop {
        thread::sleep(time::Duration::from_secs(1));
        let mut hm = m.lock().unwrap();
        dbg!(&hm);
        hm.clear();
    }
}

fn keyboard_listener(tx: Sender<TrackableEvent>) -> impl Fn() {
    move || {
        if let Err(error) = listen(callback_generator(tx.clone())) {
            println!("Error: {:?}", error);
        };
    }
}


fn callback_generator(tx: Sender<TrackableEvent>) -> impl Fn(Event) {
   
    move |event: Event| {
        let tracked = match event.event_type {
            EventType::ButtonPress(b) => Some(TrackableEvent::ButtonPress(InputKey::Mouse(b))),
            EventType::KeyPress(k) => Some(TrackableEvent::KeyPress(InputKey::Keyboard(k))),
            _ => None,
        };
        if let Some(tr) = tracked {
            match tx.send(tr) {
                Ok(_) => (),
                Err(e) => {dbg!(e);},
            };
        };

    }
}

fn signal_consumer(m: Arc<Mutex<HashMap<TrackableEvent, i32>>>, rx: Receiver<TrackableEvent>) {
    while let Ok(tr) = rx.recv() {
        println!("{tr:?}");
        let m_clone = m.clone();
        let mut hm = m_clone.lock().unwrap();
        *hm.entry(tr).or_insert(0) += 1;
    }
}


fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let mut curState = HashMap::new();

    let m = Arc::new(Mutex::new(HashMap::<TrackableEvent, i32>::new()));

    let (tx, rx) = channel();

    // let m_clone2 = Arc::clone(&m);
    let handle2 = thread::spawn(keyboard_listener(tx.clone()));
    
    let m_clone3 = Arc::clone(&m);
    let handle3 = thread::spawn(move || signal_consumer(m_clone3, rx));

    let m_clone = Arc::clone(&m);
    // let handle = thread::spawn(move || monitor_loop(m_clone));
    monitor_loop(m_clone);

    // let _ = handle.join();
    let _ = handle2.join();
    let _ = handle3.join();
    Ok(())
}

