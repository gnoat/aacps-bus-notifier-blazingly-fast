use busnote::site;

fn main() {
    let bus_site = reqwest::blocking::get("https://busstops.aacps.org/public/BusRouteIssues.aspx")
        .unwrap()
        .text()
        .unwrap_or("".to_string());
    let bus_info = site::BusInfo::new(bus_site);
    let dataframes = site::ScheduleFrames::new(bus_info);
    println!("{:?}", dataframes.new);
    println!("{:?}", dataframes.old);

}
