use chrono::{Local,Utc};

fn main() {
    loop {
        let local_time = Local::now();
        let utc_time = Utc::now();
        std::thread::sleep(std::time::Duration::from_millis(999));
    }
}
