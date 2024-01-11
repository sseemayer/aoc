use std::{fs::File, io::Write};

use anyhow::{anyhow, Context, Result};
use reqwest::header::{COOKIE, USER_AGENT};

use crate::config::Config;

pub trait InputSource {
    fn get_input(&self) -> Result<File>;
}

/// Provide a file path as an InputSource
impl InputSource for &str {
    fn get_input(&self) -> Result<File> {
        File::open(self).context("Load input from disk")
    }
}

/// Provide a (year, day) tuple as an InputSource
impl InputSource for (usize, usize) {
    fn get_input(&self) -> Result<File> {
        let &(year, day) = self;

        // try to obtain the input from the filesystem
        let cache_folder = format!("data/day{:02}", day);
        std::fs::create_dir_all(cache_folder)?;

        let cache_path = format!("data/day{:02}/input", day);
        if let Ok(file) = (cache_path.as_str()).get_input() {
            return Ok(file);
        }

        let mut config = Config::load()?;
        config.rate_limit()?;

        let session_token = &config.session_token.ok_or_else(|| {
            anyhow!("No session token! Need to log in using `aoc login` CLI command")
        })?;

        let client = reqwest::blocking::Client::new();

        let res = client
            .get(format!(
                "https://adventofcode.com/{}/day/{}/input",
                year, day
            ))
            .header(
                USER_AGENT,
                "https://github.com/sseemayer/aoc by mail@semicolonsoftware.de",
            )
            .header(COOKIE, format!("session={}", session_token))
            .send()?
            .error_for_status()?;

        let content = res.text()?;

        let mut file = File::create(&cache_path).context("Create file to store API result")?;
        file.write_all(content.as_bytes())
            .context("Write input to cache")?;

        std::mem::forget(file);

        let file = File::open(cache_path)?;

        Ok(file)
    }
}
