use schecker::configs::Configs;
use schecker::schedule;

fn main() {
    let configs: Configs = Configs::new("defaults.toml");
    let bus_url = schedule::BusInfoWebsite::Url(configs.schedule_url);
    let _bus_info = schedule::BusInfo::new(bus_url);
}
