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
    let year_dir = root.join("inputs").join(year.to_string());
    fs::create_dir_all(&year_dir)?;

    let input_path = year_dir.join(format!("{day:02}.txt"));
    let session_path = root.join(".session");

    if !input_path.exists() {
        get_input(&input_path, &session_path, year, day)?;
    }

    match language {
        Language::Rust => {
            let rust_dir = root.join("rust");
            let bin_dir = rust_dir.join("src").join("bin");

            if !rust_dir.join("Cargo.toml").exists() {
                fs::create_dir_all(&rust_dir)?;
                Command::new("cargo")
                    .args(["init", "--lib", "--name", "aoc", "--vcs", "none"])
                    .current_dir(rust_dir)
                    .status()?;
                fs::create_dir_all(&bin_dir)?;
            }

            let file_path = bin_dir.join(format!("{year}_{day:02}.rs"));
            if file_path.exists() {
                return Ok(());
            }

            let template = include_str!("../templates/rust.rs")
                .replace("{{year}}", &year.to_string())
                .replace("{{day}}", &format!("{day:02}"));

            fs::write(file_path, template)?;
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
