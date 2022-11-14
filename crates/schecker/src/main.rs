use schecker::configs::Configs;
use schecker::schedule;
use std::{thread, time};

fn main() {
    let configs: Configs = Configs::new("defaults.toml");
    let bus_url = schedule::BusInfoWebsite::Url(configs.schedule_url);
    let bus_info = schedule::BusInfo::new(bus_url);
    loop {
        let bus_info = bus_info.update(None);
        let bus_diff = bus_info.diff();
        println!("New: {:?}\n\nUpdated: {:?}\n\nRemoved: {:?}\n", bus_diff.new, bus_diff.updated, bus_diff.now_running);
        thread::sleep(time::Duration::from_secs(5));
    }
}
