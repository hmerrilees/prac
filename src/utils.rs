use std::{
    env::{temp_dir, var},
    fs::File,
    io::{BufWriter, Read, Write},
    process::Command,
    time::Duration,
};

use clap::ValueEnum;
use serde::{Deserialize, Serialize};

use anyhow::{Context, Result};

/// A unit of time.
#[derive(ValueEnum, Clone, Serialize, Deserialize)]
pub(crate) enum TimeUnit {
    Hours,
    Days,
    Weeks,
    Months,
    Years,
}

/// Conversion from scalar TimeUnits to Duration.
pub(crate) fn parse_time(duration: u64, unit: TimeUnit) -> Duration {
    Duration::from_secs(
        duration
            * match unit {
                TimeUnit::Hours => 60 * 60,
                TimeUnit::Days => 60 * 60 * 24,
                TimeUnit::Weeks => 60 * 60 * 24 * 7,
                TimeUnit::Months => 60 * 60 * 24 * 30, // I'm terribly sorry
                TimeUnit::Years => 60 * 60 * 24 * 365,
            },
    )
}

/// Write content to file
pub(crate) fn long_edit(content: Option<String>) -> Result<String> {
    let editor = var("EDITOR").context("EDITOR environment variable not set")?;

    let mut file_path = temp_dir();
    file_path.push("editable");
    let file = File::create(&file_path).context("Could not create file")?;

    if let Some(content) = content {
        BufWriter::new(file)
            .write_all(content.as_bytes())
            .context("could not write to tempfile")?;
    }

    Command::new(editor)
        .arg(&file_path)
        .status()
        .context("could not open editor")?;

    let mut editable = String::new();
    File::open(file_path)
        .context("could not open tempfile")?
        .read_to_string(&mut editable)?;
    Ok(editable)
}

/// generate a bar for a practice
pub(crate) fn bar(bar_width: usize, mut fraction: f64) -> String {
    fraction = fraction.max(0.0).min(1.0);
    //.with_context(|| "fraction must be between 0 and 1")?;
    let filled = (fraction * bar_width as f64) as usize;
    let empty = bar_width - filled;
    assert!(filled + empty == bar_width);
    format!("{}{}", "\u{025AC}".repeat(filled), " ".repeat(empty))
}
