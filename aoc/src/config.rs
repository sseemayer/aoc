use anyhow::{anyhow, Context, Result};
use chrono::{prelude::*, Duration};
use serde::{Deserialize, Serialize};
use std::{
    fs::File,
    io::{Read, Write},
    path::{Path, PathBuf},
};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    /// Session token to authenticate with Advent of Code
    pub session_token: Option<String>,

    /// Last time the client has invoked the API
    pub last_used_api: Option<DateTime<Utc>>,

    /// Minimum waiting time before API can be called again
    pub rate_limit_wait_seconds: i64,
}

impl Default for Config {
    fn default() -> Self {
        let session_token = std::env::var("AOC_SESSION").ok().and_then(|v| {
            if v.is_empty() {
                None
            } else {
                Some(v.to_string())
            }
        });

        Self {
            session_token,
            last_used_api: None,

            // we should throttle by "a few minutes". One request every five minutes seems
            // reasonable.
            rate_limit_wait_seconds: 300,
        }
    }
}

impl Config {
    fn get_config_path() -> Result<PathBuf> {
        let mut path =
            dirs::config_dir().ok_or_else(|| anyhow!("Could not get config directory!"))?;
        path.push("aoc-seemays.toml");

        Ok(path)
    }

    pub fn load() -> Result<Self> {
        Self::load_from(&Self::get_config_path()?)
    }

    pub fn load_from(path: &Path) -> Result<Self> {
        if let Ok(mut file) = File::open(path) {
            let mut content = String::new();
            file.read_to_string(&mut content)
                .context("Read config TOML")?;

            toml::from_str::<Config>(&content).context("Parse config TOML")
        } else {
            Ok(Self::default())
        }
    }

    pub fn save(&self) -> Result<()> {
        self.save_to(&Self::get_config_path()?)
    }

    pub fn save_to(&self, path: &Path) -> Result<()> {
        let content = toml::to_string_pretty(&self).context("Serialize TOML")?;

        let mut file = File::create(path).context("Create config file")?;
        file.write_all(content.as_bytes())?;

        Ok(())
    }

    pub fn get_wait_time(&self) -> Option<Duration> {
        let previous = &self.last_used_api?;
        let now = Utc::now();

        let wait_time = *previous - now + Duration::seconds(self.rate_limit_wait_seconds);

        if wait_time > Duration::zero() {
            Some(wait_time)
        } else {
            None
        }
    }

    pub fn rate_limit(&mut self) -> Result<()> {
        if let Some(duration) = self.get_wait_time() {
            println!(
                "Waiting for {} seconds (rate limiting)",
                duration.num_seconds()
            );
            std::thread::sleep(duration.to_std()?)
        }

        self.last_used_api = Some(Utc::now());
        self.save()?;

        Ok(())
    }
}
