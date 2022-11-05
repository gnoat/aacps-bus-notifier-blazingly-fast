use regex::Regex;
// use std::fs::OpenOptions;

pub struct BusInfo {
    pub current_schedule: Vec<Vec<String>>,
    pub previous_schedule: Option<Vec<Vec<String>>>,
}

impl BusInfo {
    pub fn new(bus_site_body: String) -> Self {
        // Read the schedule table column order from the returned HTML
        let columns_vec: Vec<String> = Regex::new(r#"\{\s*"title":\s*"([a-zA-Z\s]+)"\s*}"#)
            .unwrap()
            .captures_iter( 
                &Regex::new(r"columns: \[((\n|.)*)\s*\]\s*}\);")
                .unwrap()
                .captures(&bus_site_body)
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
        let schedule_vec: Vec<Vec<String>> = Regex::new(r"var dataArray =\s*\[(.*)\];")
            .unwrap()
            .captures(&bus_site_body)
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
            .collect();

        println!("{:?}", schedule_vec);

        BusInfo {
            current_schedule: schedule_vec,
            previous_schedule: None,
        }
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
