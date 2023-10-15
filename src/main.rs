mod data;
use std::thread;
use std::time::Duration;
fn main() {
    
    let mut battery_status= data::BatteryInfo::new();
    battery_status.print_info();

    let duration = Duration::from_secs(120);
    thread::sleep(duration);
    println!("\n\n\n\n\n\n");
    battery_status.update_info();
    battery_status.print_info();
    
    
}