use anyhow::{Result, bail};
use chrono::{Datelike, Local};
use clap::{Parser, Subcommand, ValueEnum, command};
use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    New {
        year: u16,
        day: u8,
        language: Language,
    },
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
enum Language {
    Rust,
    Python,
}

fn find_project_root() -> Result<PathBuf> {
    let current_dir = std::env::current_dir()?;

    let mut current = current_dir.as_path();

    loop {
        if current.join("aoc.toml").exists() {
            return Ok(current.to_path_buf());
        }

        match current.parent() {
            Some(parent) => current = parent,
            None => {
                bail!(
                    "Could not find 'aoc.toml' root marker.\n\
                     Are you inside your AOC repository?\n\
                     (Run 'touch aoc.toml' at your repo root to mark it)"
                );
            }
        }
    }
}

fn validate_date(year: u16, day: u8) -> Result<()> {
    let now = Local::now();

    if year < 2015 {
        bail!("AOC started in 2015.");
    }

    let max_days = if year >= 2025 { 12 } else { 25 };

    if day < 1 || day > max_days {
        bail!("Year {year} has {max_days} days (You requested Day {day})");
    }

    if year > now.year() as u16 {
        bail!("Year {year} is in the future.");
    }

    if year == now.year() as u16 {
        if now.month() < 12 {
            bail!("AOC {year} hasn't started yet!");
        }

        if day > now.day() as u8 {
            bail!("Day {day} has't unlocked yet! (Today is Dec {})", now.day());
        }
    }

    Ok(())
}

fn get_input(input_path: &Path, session_path: &Path, year: u16, day: u8) -> Result<()> {
    if !session_path.exists() {
        bail!(
            "Could not find '.session' at root.\n\
             {session_path:?}\n\
             (Run 'touch .session' and copy your session token into the file)"
        )
    }

    let cookie = fs::read_to_string(session_path)?.trim().to_string();
    let url = format!("https://adventofcode.com/{year}/day/{day}/input");

    let text = reqwest::blocking::Client::new()
        .get(url)
        .header("Cookie", format!("session={cookie}"))
        .header(reqwest::header::USER_AGENT, "github.com/Foxicution/aoc")
        .send()?
        .error_for_status()?
        .text()?;

    fs::write(input_path, text)?;

    Ok(())
}

fn cmd_new(root: &Path, year: u16, day: u8, language: Language) -> Result<()> {
    let day_padded = format!("{day:02}");
    let day_dir = root.join(year.to_string()).join(&day_padded);
    fs::create_dir_all(&day_dir)?;

    let input_path = day_dir.join("input.txt");
    let session_path = root.join(".session");

    if !input_path.exists() {
        get_input(&input_path, &session_path, year, day)?;
    }

    match language {
        Language::Rust => {
            if day_dir.join("rust").exists() {
                return Ok(());
            }

            Command::new("cargo")
                .args([
                    "new",
                    "rust",
                    "--name",
                    &format!("aoc_{year}_{day_padded}"),
                    "--vcs",
                    "none",
                ])
                .current_dir(day_dir)
                .status()?;
        }
        Language::Python => {
            todo!("Python not implemented")
        }
    }

    Ok(())
}

fn main() -> Result<()> {
    let root = find_project_root()?;
    let cli = Cli::parse();

    match cli.command {
        Commands::New {
            year,
            day,
            language,
        } => {
            validate_date(year, day)?;
            cmd_new(&root, year, day, language)?;
        }
    }
    Ok(())
}
