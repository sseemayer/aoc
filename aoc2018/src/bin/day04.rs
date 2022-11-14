use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    str::FromStr,
};

use anyhow::{Context, Result};
use lazy_static::lazy_static;
use regex::Regex;
use thiserror::Error;

#[derive(Debug, Error)]
enum Error {
    #[error("I/O error: {}", _0)]
    Io(#[from] std::io::Error),

    #[error("Int parsing error: {}", _0)]
    ParseInt(#[from] std::num::ParseIntError),

    #[error("Timestamp parsing error: '{}'", _0)]
    ParseTimestamp(String),

    #[error("Unrecognized log line: '{}'", _0)]
    ParseLine(String),
}

lazy_static! {
    static ref RE_BEGIN: Regex = Regex::new(r"Guard #(\d+) begins shift").unwrap();
    static ref RE_SLEEP: Regex = Regex::new(r"falls asleep").unwrap();
    static ref RE_WAKE: Regex = Regex::new(r"wakes up").unwrap();
    static ref RE_LOGLINE: Regex = Regex::new(r"\[(\d{4}-\d{2}-\d{2} \d{2}:\d{2})\] (.*)").unwrap();
}

#[derive(Debug, Clone)]
enum LogType {
    Begin(u16),
    Sleep,
    Wake,
}

impl FromStr for LogType {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(m) = RE_BEGIN.captures(s) {
            let guard_id: u16 = m.get(1).expect("guard id capture").as_str().parse()?;
            Ok(LogType::Begin(guard_id))
        } else if let Some(_m) = RE_SLEEP.captures(s) {
            Ok(LogType::Sleep)
        } else if let Some(_m) = RE_WAKE.captures(s) {
            Ok(LogType::Wake)
        } else {
            Err(Error::ParseLine(s.to_string()))
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct LogTimestamp {
    year: u16,
    month: u16,
    day: u16,

    hour: u16,
    minute: u16,
}

impl FromStr for LogTimestamp {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (date, time) = s
            .split_once(" ")
            .ok_or_else(|| Error::ParseTimestamp(s.to_string()))?;

        let d = date
            .split("-")
            .map(|v| Ok(v.parse()?))
            .collect::<std::result::Result<Vec<u16>, Error>>()?;

        let t = time
            .split(":")
            .map(|v| Ok(v.parse()?))
            .collect::<std::result::Result<Vec<u16>, Error>>()?;

        if d.len() != 3 || t.len() != 2 {
            return Err(Error::ParseTimestamp(s.to_string()));
        }

        let year = d[0];
        let month = d[1];
        let day = d[2];

        let hour = t[0];
        let minute = t[1];

        Ok(LogTimestamp {
            year,
            month,
            day,
            hour,
            minute,
        })
    }
}

impl std::cmp::PartialOrd for LogTimestamp {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.year.partial_cmp(&other.year) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        match self.month.partial_cmp(&other.month) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        match self.day.partial_cmp(&other.day) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        match self.hour.partial_cmp(&other.hour) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        self.minute.partial_cmp(&other.minute)
    }
}

impl std::cmp::Ord for LogTimestamp {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).expect("always ordered")
    }
}

#[derive(Debug, Clone)]
struct LogLine {
    timestamp: LogTimestamp,
    message: LogType,
}

impl FromStr for LogLine {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let m = RE_LOGLINE
            .captures(s)
            .ok_or_else(|| Error::ParseLine(s.to_string()))?;

        let timestamp: LogTimestamp = m
            .get(1)
            .expect("timestamp capture group")
            .as_str()
            .parse()?;
        let message: LogType = m.get(2).expect("message capture group").as_str().parse()?;

        Ok(LogLine { timestamp, message })
    }
}

#[derive(Debug, Default, Clone)]
struct Schedule {
    items: Vec<LogLine>,
}

impl Schedule {
    fn sleep_time(&self) -> u16 {
        let mut last_sleep: Option<LogTimestamp> = None;
        let mut total = 0;

        for item in &self.items {
            match item.message {
                LogType::Begin(_) => {}
                LogType::Sleep => {
                    last_sleep = Some(item.timestamp);
                }
                LogType::Wake => {
                    if let Some(ls) = last_sleep {
                        total += item.timestamp.minute - ls.minute;
                    }

                    last_sleep = None;
                }
            }
        }

        total
    }

    fn sleepiest_minute(&self) -> Option<(u16, usize)> {
        let mut minutes: HashMap<u16, usize> = HashMap::new();

        let mut last_sleep: Option<LogTimestamp> = None;
        for item in &self.items {
            match item.message {
                LogType::Begin(_) => {}
                LogType::Sleep => {
                    last_sleep = Some(item.timestamp);
                }
                LogType::Wake => {
                    if let Some(ls) = last_sleep {
                        for min in ls.minute..item.timestamp.minute {
                            *minutes.entry(min).or_default() += 1
                        }
                    }

                    last_sleep = None;
                }
            }
        }

        minutes
            .iter()
            .max_by_key(|(k, v)| **v)
            .map(|(k, v)| (*k, *v))
    }
}

fn get_schedules(lines: &Vec<LogLine>) -> HashMap<u16, Schedule> {
    let mut out = HashMap::new();
    let mut current = None;

    for line in lines {
        match line.message {
            LogType::Begin(i) => {
                let schedule: &mut Schedule = out.entry(i).or_default();
                schedule.items.push(LogLine {
                    timestamp: line.timestamp,
                    message: LogType::Wake,
                });

                current = Some(i)
            }
            LogType::Sleep | LogType::Wake => {
                if let Some(i) = current {
                    let schedule: &mut Schedule = out.entry(i).or_default();
                    schedule.items.push(line.clone());
                }
            }
        }
    }

    out
}

fn get_reliable_minute(schedules: &HashMap<u16, Schedule>) -> (u16, u16) {
    let mut max_counts = 0;
    let mut max_guard = 0;
    let mut max_minute = 0;

    for (guard_id, schedule) in schedules {
        if let Some((minute, counts)) = schedule.sleepiest_minute() {
            if counts > max_counts {
                max_minute = minute;
                max_guard = *guard_id;
                max_counts = counts;
            }
        }
    }

    (max_guard, max_minute)
}

fn main() -> Result<()> {
    let mut lines: Vec<LogLine> =
        BufReader::new(File::open("data/day04/input").context("Open input")?)
            .lines()
            .map(|line| line.context("Read line")?.parse().context("Parse line"))
            .collect::<Result<Vec<_>>>()?;

    lines.sort_by_key(|l| l.timestamp);

    let schedules = get_schedules(&lines);

    for (guard_id, schedule) in &schedules {
        println!("# {}", guard_id);

        for itm in &schedule.items {
            println!("    {:?}", itm);
        }
    }

    let (sleepiest_id, sleepiest_schedule) = schedules
        .iter()
        .max_by_key(|(_i, s)| s.sleep_time())
        .expect("max sleep");

    if let Some((sleepiest_minute, _sleepiest_count)) = sleepiest_schedule.sleepiest_minute() {
        println!(
            "Part 1: {}",
            (*sleepiest_id as usize) * (sleepiest_minute as usize)
        );
    }

    let (reliable_guard, reliable_minute) = get_reliable_minute(&schedules);
    println!(
        "Part 2: {}",
        (reliable_guard as usize) * (reliable_minute as usize)
    );

    Ok(())
}
