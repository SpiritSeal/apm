

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
        let hm = m.lock().unwrap();
        dbg!(&hm);
    }
}

fn keyboard_listener(m_clone2: Arc<Mutex<HashMap<TrackableEvent, i32>>>, tx: Sender<TrackableEvent>) -> impl Fn() {
    move || {
        let m = m_clone2.clone();
        if let Err(error) = listen(callback_generator(m, tx.clone())) {
            println!("Error: {:?}", error);
        };
    }
}


fn callback_generator(m: Arc::<Mutex::<HashMap::<TrackableEvent, i32>>>, tx: Sender<TrackableEvent>) -> impl Fn(Event) {
   
    move |event: Event| {
        // println!("My callback {:?}", event);
        // match event.name {
            //     Some(string) => println!("User wrote {:?}", string),
            //     None => (),
            // }
            // println!("{event:?}");
            // match event.event_type {
                //     EventType::ButtonPress(p) => println!("{p:?} do"),
                //     EventType::KeyPress(p) => println!("{p:?} do"),
                //     EventType::ButtonRelease(p) => println!("{p:?} up"),
                //     EventType::KeyRelease(p) => println!("{p:?} up"),
                //     _ => (),
                // }
        let tracked = match event.event_type {
            EventType::ButtonPress(b) => Some(TrackableEvent::ButtonPress(InputKey::Mouse(b))),
            EventType::KeyPress(k) => Some(TrackableEvent::KeyPress(InputKey::Keyboard(k))),
            _ => None,
        };
        // if let Some(tr) = tracked {
            // let mut hm = m.lock().unwrap();
            // if let Some(v) = hm.get_mut(&tr) {
                // println!();
                // *v += 1
            // } else {
                // hm.insert(tr, 1);
            // };
        // };
        // if let Some(tr) = tracked {
        //     let mut map = m.lock().unwrap();
        //     *map.entry(tr).or_insert(0) += 1;
        // };
        if let Some(tr) = tracked {
            tx.send(tr);
        };

    }

}

fn signal_consumer(m: Arc<Mutex<HashMap<TrackableEvent, i32>>>, rx: Receiver<TrackableEvent>) {
    while let Ok(tr) = rx.recv() {
        println!("{tr:?}");
        let m_clone = m.clone();
        let mut hm = m_clone.lock().unwrap();
        *hm.entry(tr).or_insert(0) += 1;
        // dbg!(&hm);
    }
}


fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let mut curState = HashMap::new();

    let m = Arc::new(Mutex::new(HashMap::<TrackableEvent, i32>::new()));

    let (tx, rx) = channel();

    let m_clone2 = Arc::clone(&m);
    let handle2 = thread::spawn(keyboard_listener(m_clone2, tx.clone()));
    
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

