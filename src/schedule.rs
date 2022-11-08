use regex::Regex;
use std::collections::HashSet;

pub struct BusInfo {
    pub schedule_info: RawInfo,
    pub current_schedule: HashSet<Vec<String>>,
    pub previous_schedule: Option<HashSet<Vec<String>>>,
}

#[derive(Clone)]
pub enum RawInfo {
    Url(String),
    Text(String),
}

impl BusInfo {
    pub fn new(schedule_info: RawInfo) -> Self {
        // Read bus schedule from website and extract schedule deficiencies
        let bus_schedule_text = match schedule_info {
            RawInfo::Url(ref url) => Self::request_schedule(&url).to_string(),
            RawInfo::Text(ref text) => text.to_string()
        };
        let schedule_vec = Self::extract_schedule(bus_schedule_text);

        BusInfo {
            schedule_info: schedule_info.clone(),
            current_schedule: schedule_vec,
            previous_schedule: None,
        }
    }

    pub fn update(&self) -> Self {
        // Generate a new BusInfo struct that has updated info and current info
        let current_schedule_text = match &self.schedule_info {
            RawInfo::Text(s) => s.to_string(),
            RawInfo::Url(url) => Self::request_schedule(&url).to_string()
        };
        BusInfo {
            schedule_info: self.schedule_info.clone(),
            current_schedule: Self::extract_schedule(&current_schedule_text),
            previous_schedule: Some(self.current_schedule.clone()),
        }
    }

    fn request_schedule(bus_schedule_url: &String) -> String {
        reqwest::blocking::get(bus_schedule_url)
            .unwrap()
            .text()
            .unwrap_or("".to_string())
    }

    fn extract_schedule<T: AsRef<str>>(site_text: T) -> HashSet<Vec<String>> {
        // Read the schedule table column order from the returned HTML
        let columns_vec: Vec<String> = Regex::new(r#"\{\s*"title":\s*"([a-zA-Z\s]+)"\s*}"#)
            .unwrap()
            .captures_iter( 
                &Regex::new(r"columns: \[((\n|.)*)\s*\]\s*}\);")
                .unwrap()
                .captures(&site_text.as_ref())
                .unwrap()
                .get(1)
                .unwrap()
                .as_str()
                .trim()
            )
            .map(|s| s.get(1).unwrap().as_str().to_string())
            .collect();

        // Find the index of each expected column
        let bus_idx = columns_vec
            .iter()
            .position(|s| s == "Bus")
            .expect("Missing column name");
        let sub_idx = columns_vec
            .iter()
            .position(|s| s == "Sub Bus")
            .expect("Missing column name");
        let schedules_idx = columns_vec
            .iter()
            .position(|s| s == "Schedules")
            .expect("Missing column name");
        let schools_idx = columns_vec
            .iter()
            .position(|s| s == "Schools")
            .expect("Missing column name");
        let impact_idx = columns_vec
            .iter()
            .position(|s| s == "Impact")
            .expect("Missing column name");
        let impacto_idx = columns_vec
            .iter()
            .position(|s| s == "Impacto")
            .expect("Missing column name");

        // Capture rows from schedule table and put them in the expected order
        // Columns should always be ordered as:
        //     1) Bus
        //     2) Sub Bus
        //     3) Schedules
        //     4) Schools
        //     5) Impact
        //     6) Impacto
        let schedule_vec: HashSet<Vec<String>> = Regex::new(r"var dataArray =\s*\[(.*)\];")
            .unwrap()
            .captures(&site_text.as_ref())
            .unwrap()
            .get(1)
            .unwrap()
            .as_str()
            .split("], [")
            .map(|entry| {
                entry
                    .replace("[", "")
                    .replace("]", "")
                    .split("', '")
                    .map(|entry_col| entry_col.replace("'", ""))
                    .collect::<Vec<String>>()
            })
            .map(|col| {
                vec![
                    col.get(bus_idx).unwrap_or(&"".to_string()).to_owned(),
                    col.get(sub_idx).unwrap_or(&"".to_string()).to_owned(),
                    col.get(schedules_idx).unwrap_or(&"".to_string()).to_owned(),
                    col.get(schools_idx).unwrap_or(&"".to_string()).to_owned(),
                    col.get(impact_idx).unwrap_or(&"".to_string()).to_owned(),
                    col.get(impacto_idx).unwrap_or(&"".to_string()).to_owned(),
                ]
            })
            .filter(|v| !v[0].is_empty() && !v[3].is_empty())
            .collect();

        println!("{:?}", schedule_vec);
        schedule_vec
    }

    pub fn diff(&self) -> BusInfoDiff {
        match &self.previous_schedule {
            None => if self.current_schedule.is_empty() {
                BusInfoDiff { new: None, updated: None, now_running: None }
            } else {
                BusInfoDiff { new: Some(self.current_schedule.clone()), updated: None, now_running: None }
            },
            Some(prev) => if self.current_schedule.is_empty() {
                BusInfoDiff { new: None, updated: None, now_running: Some(prev.clone()) }
            } else {
                BusInfoDiff::new(self.current_schedule.clone(), self.previous_schedule.clone().unwrap())
            }
        }
    }

    
}

pub struct BusInfoDiff {
    pub new: Option<HashSet<Vec<String>>>,
    pub updated: Option<HashSet<Vec<String>>>,
    pub now_running: Option<HashSet<Vec<String>>>,
}

impl BusInfoDiff {
    pub fn new(left: HashSet<Vec<String>>, right: HashSet<Vec<String>>) -> Self {
            let mut new: HashSet<Vec<String>> = HashSet::new();
            let mut updated: HashSet<Vec<String>> = HashSet::new();
            let mut now_running: HashSet<Vec<String>> = HashSet::new();

            for l_row in left.iter() {
                let r_filtered = right.iter().filter(|v| (l_row[0] == v[0]) && (l_row[3] == v[3])).next();
                match r_filtered {
                    Some(v) => {
                        if (l_row[1] != v[1]) || (l_row[2] != v[2]) || (l_row[4] != v[4]) {
                            updated.insert(v.to_vec());
                        }
                    },
                    None => { new.insert(l_row.to_vec()); }
                }
            }

            for r_row in right.iter() {
                let l_filtered = left.iter().filter(|v| (r_row[0] == v[0]) && (r_row[3] == v[3])).next();
                match l_filtered {
                    Some(_) => {},
                    None => { now_running.insert(r_row.to_vec()); }
                }
            }
            BusInfoDiff { new: Some(new), updated: Some(updated), now_running: Some(now_running) }
    }
}

// pub struct ScheduleFrames {
//     new: DataFrame,
//     old: DataFrame,
// }
//
// impl ScheduleFrames {
//     pub fn new(bus_info: BusInfo, archive_location: Option<String>, archive: Option<bool>) -> Self {
//         let mut df = Self::extract_dataframe(bus_info).unwrap();
//
//         let archive_location = match archive_location {
//             Some(f) => f,
//             None => "archive.json".to_string()
//         };
//         let archive = match archive {
//             Some(f) => f,
//             None => false
//         };
//
//         let archived_file = OpenOptions::new()
//             .read(true)
//             .write(true)
//             .create(true)
//             .open(archive_location);
//         let df_archived = match archived_file {
//             Ok(f) => CsvReader::new(f)
//                 .infer_schema(None)
//                 .has_header(true)
//                 .finish(),
//             Err(f) => Self::extract_dataframe(BusInfo{ schedule: Vec::new(), columns: vec!["Bus".to_string(), "Sub Bus".to_string(), "Schools".to_string(), "Schedules".to_string(), "Impact".to_string(), "Impacto".to_string()] }),
//         }.unwrap();
//
//         if archive {
//             CsvWriter::new(archived_file)
//                 .has_header(true)
//                 .with_delimiter(b'|')
//                 .finish(&mut df);
//         };
//         ScheduleFrames{ new: df, old: df_archived }
//     }
//
//     pub fn extract_dataframe(bus_info: BusInfo) -> Result<DataFrame, PolarsError> {
//         let mut series_vec: Vec<Series> = Vec::new();
//
//         for (idx, column) in bus_info.columns.iter().enumerate() {
//             let column_rows_vec: Vec<String> = bus_info.schedule.iter().map(|i| i[idx].to_string()).collect();
//             series_vec.push(Series::new(column, column_rows_vec));
//         }
//
//         DataFrame::new(series_vec)
//    }
// }
