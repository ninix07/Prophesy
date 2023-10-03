mod data;

fn main() {
    
    let battery_status= data::BatteryInfo::new();
    battery_status.print_info();
    
    
}