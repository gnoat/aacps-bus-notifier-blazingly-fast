use busnote::schedule;

fn main() {
    let bus_site = reqwest::blocking::get("https://busstops.aacps.org/public/BusRouteIssues.aspx")
        .unwrap()
        .text()
        .unwrap_or("".to_string());
    let _bus_info = schedule::BusInfo::new(bus_site);
}
