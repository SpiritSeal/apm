


use evdev::*;
use std::thread;
use std::time::Duration;
use std::collections::HashMap;


fn monitor_loop() -> Result<(), Box<dyn std::error::Error>> {

    // let handle = thread::spawn(|| {
    //     println!("test");
    // });
    
    let mut device = Device::open("/dev/input/event5")?;
    let mut total_cnt = 0;
    let mut uniq = 0.0;
    let mut key_stats = HashMap::new();
    
    loop {
        for event in device.fetch_events().unwrap(){
            if let EventSummary::Key(ev, kc, state) = event.destructure() {
                // println!("Count {}\t{:?}", cnt, &kc);
                // check if released
                if state != 1 {
                    // println!("Skip {ev:?}");
                    continue;
                }
                // println!("{ev:?}");
                if let Some(item_count) = key_stats.get_mut(&kc) {
                    *item_count += 1;
                    total_cnt += 1;
                } else {
                    key_stats.insert(kc, 1);
                }
            }
        }

        thread::sleep(Duration::from_millis(1000));

        let len = key_stats.len();
        if len == 0 {
            println!("No keys pressed");
            continue;
        }

        println!("{key_stats:?}");
        println!("Count {}", total_cnt);
        uniq = total_cnt as f32 / key_stats.len() as f32;
        println!("Uniq {}, len {}", uniq, len);

        // Clear key stats
        key_stats.clear();
    }
    // handle.join().unwrap();
}
fn main() {
    monitor_loop();
}

