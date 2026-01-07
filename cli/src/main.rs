use anyhow::{Context, Result, bail};
use chrono::{Datelike, Local};
use clap::{Parser, Subcommand, ValueEnum, command};
use regex::Regex;
use scraper::{Html, Selector};
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

    Desc {
        year: u16,
        day: u8,
    },
    Input {
        year: u16,
        day: u8,
        slice: Option<String>,
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

fn clean_md(md: &str) -> String {
    // 1. Fix Headers: Remove the backslash escape
    // html2md turns <h2> into "\--- Title ---" followed by a line of dashes.
    // We want just "--- Title ---"
    let md = md.replace(r"\---", "---");

    // 2. Remove the ugly header separator line (e.g. "----------")
    // We look for a line that contains only dashes (3 or more)
    let re_header_line = Regex::new(r"(?m)^-{3,}\s*$").unwrap();
    let md = re_header_line.replace_all(&md, "");

    // 3. Convert List items from '*' to '-'
    // (?m) enables multi-line mode so ^ matches start of line
    let re_list = Regex::new(r"(?m)^\* ").unwrap();
    let md = re_list.replace_all(&md, "- ");

    // 4. Cleanup excessive newlines left behind by removals
    let re_newlines = Regex::new(r"\n{3,}").unwrap();
    let md = re_newlines.replace_all(&md, "\n\n");

    md.trim().to_string()
}

fn cmd_desc(root: &Path, year: u16, day: u8) -> Result<()> {
    let desc_dir = root.join("descriptions").join(year.to_string());
    let desc_path = desc_dir.join(format!("{day:02}.md"));

    if desc_path.exists() {
        let md = fs::read_to_string(&desc_path)?;

        if md.contains("--- Part Two ---") {
            println!("{md}");
            return Ok(());
        }
    }

    let session_path = root.join(".session");
    if !session_path.exists() {
        bail!(
            "Could not find '.session' at root.\n\
             {session_path:?}\n\
             (Run 'touch .session' and copy your session token into the file)"
        )
    }

    let cookie = fs::read_to_string(session_path)?.trim().to_string();
    let url = format!("https://adventofcode.com/{year}/day/{day}");

    let client = reqwest::blocking::Client::new();
    let res = client
        .get(url)
        .header("Cookie", format!("session={cookie}"))
        .header(reqwest::header::USER_AGENT, "github.com/Foxicution/aoc")
        .send()?
        .error_for_status()?;

    let html = res.text()?;
    let document = Html::parse_document(&html);
    let selector = Selector::parse("article.day-desc").unwrap();

    let mut md = String::new();

    // Loop through articles (Part 1 and Part 2 are separate <article> tags)
    for (i, element) in document.select(&selector).enumerate() {
        if i > 0 {
            md.push_str("\n\n");
        }
        let html_part = element.inner_html();
        let md_part = html2md::parse_html(&html_part);
        md.push_str(&md_part);
    }

    md = clean_md(&md);
    if md.is_empty() {
        bail!("Parsed markdown is empty. Something went wrong.")
    }

    fs::create_dir_all(desc_dir)?;
    fs::write(&desc_path, &md)?;

    println!("{md}");
    Ok(())
}

fn cmd_input(root: &Path, year: u16, day: u8, slice: Option<String>) -> Result<()> {
    let year_dir = root.join("inputs").join(year.to_string());
    // Ensure dir exists so get_input doesn't fail on fs::write
    fs::create_dir_all(&year_dir)?;

    let input_path = year_dir.join(format!("{day:02}.txt"));
    let session_path = root.join(".session");

    // Reuse your existing logic to fetch if missing
    if !input_path.exists() {
        get_input(&input_path, &session_path, year, day)?;
    }

    let content = fs::read_to_string(&input_path)?;

    // If no slice is provided, print the raw content (preserves trailing newlines)
    if slice.is_none() {
        print!("{}", content);
        return Ok(());
    }

    // Handle slicing logic
    let lines: Vec<&str> = content.lines().collect();
    let total_lines = lines.len();
    let range_str = slice.unwrap();

    // Default: full range
    let (mut start, mut end) = (0, total_lines);

    if let Some((s, e)) = range_str.split_once(':') {
        // Handle "10:" -> start=10
        if !s.is_empty() {
            start = s.parse::<usize>().context("Invalid start index")?;
        }
        // Handle ":5" -> end=5
        if !e.is_empty() {
            end = e.parse::<usize>().context("Invalid end index")?;
        }
    } else {
        // Handle single number "5" -> print just line 5?
        // Or assume they meant "0:5"? Let's treat single number as "print line N"
        if let Ok(line_num) = range_str.parse::<usize>() {
            start = line_num;
            end = line_num + 1;
        } else {
            bail!("Invalid slice format. Use 'start:end', 'start:', ':end', or 'line_num'");
        }
    }

    // Bounds checking to prevent panics
    if start > total_lines {
        start = total_lines;
    }
    if end > total_lines {
        end = total_lines;
    }
    if start > end {
        start = end;
    }

    for line in &lines[start..end] {
        println!("{}", line);
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
        Commands::Desc { year, day } => {
            validate_date(year, day)?;
            cmd_desc(&root, year, day)?;
        }
        Commands::Input { year, day, slice } => {
            validate_date(year, day)?;
            cmd_input(&root, year, day, slice)?;
        }
    }
    Ok(())
}
