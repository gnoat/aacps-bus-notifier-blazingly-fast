use busnote::configs::Configs;
use busnote::schedule;

fn main() {
    let configs: Configs = Configs::new("src/defaults.toml");
    let bus_url = schedule::RawInfo::Url(configs.schedule_url);
    let _bus_info = schedule::BusInfo::new(bus_url);
}

