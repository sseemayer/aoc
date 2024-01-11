use anyhow::{anyhow, Context, Result};
use clap::{Args, Parser, Subcommand};

use aoc::config::Config;

#[derive(Parser)]
enum Cli {
    /// Login to advent of code or display session information
    Login(LoginCommand),
}

impl Cli {
    fn run(self) -> Result<()> {
        match self {
            Cli::Login(cmd) => cmd.run(),
        }
    }
}

#[derive(Args)]
struct LoginCommand {
    /// If provided, store the session token for future use in the user profile
    session_token: Option<String>,
}

impl LoginCommand {
    fn run(self) -> Result<()> {
        let mut config = Config::load()?;

        if let Some(new_session_token) = self.session_token {
            config.session_token = Some(new_session_token);
            config.save()?;
        }

        println!("{:#?}", config);

        Ok(())
    }
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    cli.run()
}
