use regex::Regex;
use polars::prelude::*;
use std::fs::OpenOptions;

pub struct BusInfo {
   pub schedule: Vec<Vec<String>>,
   pub columns: Vec<String>,
}

impl BusInfo {
    pub fn new(bus_site_body: String) -> Self{
        let schedule_vec: Vec<Vec<String>>= Regex::new(r"var dataArray =\s*\[(.*)\];")
            .unwrap()
            .captures(&bus_site_body)
            .unwrap()
            .get(1)
            .unwrap()
            .as_str()
            .split("], [")
            .map(|entry| entry.replace("[", "")
                 .replace("]", "")
                 .split("', '")
                 .map(|entry_col| entry_col.replace("'", ""))
                 .collect::<Vec<String>>())
            .collect();

        let columns_raw = Regex::new(r"columns: \[((\n|.)*)\s*\]\s*}\);")
            .unwrap()
            .captures(&bus_site_body)
            .unwrap()
            .get(1)
            .unwrap()
            .as_str()
            .trim();

        let columns_vec: Vec<String> = Regex::new(r#"\{\s*"title":\s*"([a-zA-Z\s]+)"\s*}"#)
            .unwrap()
            .captures_iter(&columns_raw)
            .map(|s| s
                 .get(1)
                 .unwrap()
                 .as_str()
                 .to_string())
            .collect();
        
        BusInfo { schedule: schedule_vec, columns: columns_vec }
    }
}

pub struct ScheduleFrames {
    new: DataFrame,
    old: DataFrame,
}

impl ScheduleFrames {
    pub fn new(bus_info: BusInfo, archive_location: Option<String>, archive: Option<bool>) -> Self {
        let mut df = Self::extract_dataframe(bus_info).unwrap();

        let archive_location = match archive_location {
            Some(f) => f,
            None => "archive.json".to_string()
        };
        let archive = match archive {
            Some(f) => f,
            None => false
        };

        let archived_file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(archive_location);
        let df_archived = match archived_file {
            Ok(f) => CsvReader::new(f)
                .infer_schema(None)
                .has_header(true)
                .finish(),
            Err(f) => Self::extract_dataframe(BusInfo{ schedule: Vec::new(), columns: vec!["Bus".to_string(), "Sub Bus".to_string(), "Schools".to_string(), "Schedules".to_string(), "Impact".to_string(), "Impacto".to_string()] }),
        }.unwrap();

        if archive {
            CsvWriter::new(archived_file)
                .has_header(true)
                .with_delimiter(b'|')
                .finish(&mut df);
        };
        ScheduleFrames{ new: df, old: df_archived }
    }

    pub fn extract_dataframe(bus_info: BusInfo) -> Result<DataFrame, PolarsError> {
        let mut series_vec: Vec<Series> = Vec::new();
        
        for (idx, column) in bus_info.columns.iter().enumerate() {
            let column_rows_vec: Vec<String> = bus_info.schedule.iter().map(|i| i[idx].to_string()).collect();
            series_vec.push(Series::new(column, column_rows_vec));
        }
        
        DataFrame::new(series_vec)
   }
}
