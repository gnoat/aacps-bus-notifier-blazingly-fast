use regex::Regex;
use std::collections::HashSet;

pub struct BusInfo {
    pub schedule_info: BusInfoWebsite,
    pub current_schedule: HashSet<Vec<String>>,
    pub previous_schedule: Option<HashSet<Vec<String>>>,
}

#[derive(Clone, Debug)]
pub enum BusInfoWebsite {
    Url(String),
    Text(String),
}

impl BusInfo {
    pub fn new(schedule_info: BusInfoWebsite) -> Self {
        // Read bus schedule from website and extract schedule deficiencies
        let bus_schedule_text = match schedule_info {
            BusInfoWebsite::Url(ref url) => Self::request_schedule(&url).to_string(),
            BusInfoWebsite::Text(ref text) => text.to_string(),
        };
        let schedule_vec = Self::extract_schedule(bus_schedule_text);

        BusInfo {
            schedule_info: schedule_info.clone(),
            current_schedule: schedule_vec,
            previous_schedule: None,
        }
    }

    pub fn update(&self, schedule_info: Option<BusInfoWebsite>) -> Self {
        // Generate a new BusInfo struct that has updated info and current info
        let current_schedule_info = match &schedule_info {
            Some(s) => s,
            None => &self.schedule_info
        };
        let current_schedule_text = match current_schedule_info {
            BusInfoWebsite::Text(ref s) => s.to_string(),
            BusInfoWebsite::Url(ref url) => Self::request_schedule(&url).to_string(),
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
                    .trim(),
            )
            .map(|s| s.get(1).unwrap().as_str().to_string())
            .collect();

        // Find the index of each expected column
        let columns = vec![
            "Bus",
            "Sub Bus",
            "Schedules",
            "Schools",
            "Impact",
            "Impacto",
        ]
        .into_iter()
        .map(|c| {
            columns_vec
                .iter()
                .position(|s| s == c)
                .expect("Missing column")
        })
        .collect::<Vec<usize>>();

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
                permute(entry
                    .replace("[", "")
                    .replace("]", "")
                    .split("', '")
                    .map(|entry_col| entry_col.replace("'", ""))
                    .collect::<Vec<String>>(), &columns)
            })
            .filter(|v| !v[0].is_empty() && !v[3].is_empty())
            .collect();

        println!("Read bus shortages: {:?}\n", schedule_vec);
        schedule_vec
    }

    pub fn diff(&self) -> BusInfoDiff {
        // Calculates the BusInfoDiff that details which type of message should be sent for each bus/school combo
        match &self.previous_schedule {
            None => {
                if self.current_schedule.is_empty() {
                    BusInfoDiff {
                        new: None,
                        updated: None,
                        now_running: None,
                    }
                } else {
                    BusInfoDiff {
                        new: Some(self.current_schedule.clone()),
                        updated: None,
                        now_running: None,
                    }
                }
            }
            Some(prev) => {
                if self.current_schedule.is_empty() {
                    BusInfoDiff {
                        new: None,
                        updated: None,
                        now_running: Some(prev.clone()),
                    }
                } else if prev.is_empty() {
                    BusInfoDiff {
                        new: Some(self.current_schedule.clone()),
                        updated: None,
                        now_running: None,
                    }
                } else {
                    BusInfoDiff::new(
                        self.current_schedule.clone(),
                        self.previous_schedule.clone().unwrap(),
                    )
                }
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
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
            let r_filtered = right
                .iter()
                .filter(|v| (l_row[0] == v[0]) && (l_row[3] == v[3]) && (l_row[2] == v[2]))
                .next();
            match r_filtered {
                Some(v) => {
                    if (l_row[1] != v[1]) || (l_row[2] != v[2]) || (l_row[4] != v[4]) {
                        updated.insert(v.to_vec());
                    }
                }
                None => {
                    new.insert(l_row.to_vec());
                }
            }
        }

        for r_row in right.iter() {
            let l_filtered = left
                .iter()
                .filter(|v| (r_row[0] == v[0]) && (r_row[3] == v[3]) && (r_row[2] == v[2]))
                .next();
            match l_filtered {
                Some(_) => {}
                None => {
                    now_running.insert(r_row.to_vec());
                }
            }

        }

        BusInfoDiff {
            new: if new.is_empty() {
                None
            } else {
                Some(new)
            },
            updated: if updated.is_empty() {
                None
            } else {
                Some(updated)
            },
            now_running: if now_running.is_empty() {
                None
            } else {
                Some(now_running)
            },
        }
    }
}

fn permute(v: Vec<String>, idx_map: &Vec<usize>) -> Vec<String> {
    let mut permuted: Vec<String> = Vec::new();
    for idx in idx_map.into_iter() {
        // permuted.push(v[*idx].to_string());
        permuted.push(v.get(*idx).unwrap_or(&"".to_string()).to_owned());
    }

    permuted
}

