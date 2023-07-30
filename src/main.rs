#![allow(non_snake_case)]
use clap::{Parser, Subcommand, ValueEnum};
use serde::{Deserialize, Serialize};
use skim::prelude::*;
use std::collections::btree_map;
use std::io::Cursor;
use std::{
    collections::BTreeMap,
    env::{temp_dir, var},
    fs::File,
    io::{BufReader, BufWriter, Read, Write},
    path::PathBuf,
    process::Command,
    time::{Duration, SystemTime},
};

use anyhow::{bail, Context, Result};

fn long_edit(content: Option<String>) -> Result<String> {
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

#[derive(Serialize, Deserialize)]
struct Habit {
    created: SystemTime,
    logged: SystemTime,
    period: Duration,
    name: String,
    body: String,
}

// TODO maybe a Completion struct? then a body enum {Habit, Task} that contains Vec<Comepletion> for habit and raw
// Completion for task. Trying not to prematurely optimize.

impl Habit {
    fn new(name: String, body: String, period: Duration) -> Self {
        let created = SystemTime::now();
        let logged = created.clone();

        Self {
            created,
            logged,
            period,
            name,
            body,
        }
    }
}

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: SubCommand,
}

// TODO if you used a BTreeMap<timebaseduuid, Habit> you could solve a lot of problems
type Key = String;
#[derive(Serialize, Deserialize, Default)]
struct State {
    todo: BTreeMap<Key, Habit>,
}

impl State {
    fn new() -> Self {
        Self::default()
    }

    fn find(&mut self, name: Option<&str>) -> Result<btree_map::OccupiedEntry<Key, Habit>> {
        let name = name
            .context("name not provided")
            .map(String::from)
            .or_else(|e| {
                let options = SkimOptionsBuilder::default()
                    .height(Some("50%"))
                    .build()
                    .unwrap();

                // `SkimItemReader` is a helper to turn any `BufRead` into a stream of `SkimItem`
                // `SkimItem` was implemented for `AsRef<str>` by default
                let items = SkimItemReader::default().of_bufread(Cursor::new(
                    self.todo
                        .iter()
                        .map(|(k, _v)| format!("{}", k))
                        .collect::<Vec<_>>()
                        .join("\n"),
                ));

                // TODO figure out what these errors acutally are
                let selected_items = Skim::run_with(&options, Some(items))
                    .context("Selection error.")?
                    .selected_items;

                // ensure only one item is selected
                let item = match selected_items.len() {
                    0 => bail!("No item selected"),
                    1 => selected_items[0].text(),
                    2.. => bail!("Multiple items selected"),
                    _ => unreachable!(),
                };

                Ok(item.into())
            })
            .context("failure to obtain name")?;

        match self.todo.entry(name) {
            btree_map::Entry::Vacant(_) => bail!("Item not found."),
            btree_map::Entry::Occupied(o) => Ok(o),
        }
    }
}

#[derive(Subcommand)]
enum SubCommand {
    Add {
        name: String,
        #[arg(short, long)]
        body: Option<String>,
        period: u64,
        #[arg(value_enum)]
        unit: Unit,
    },
    Log {
        name: Option<String>,
    },
    Edit {
        name: Option<String>,
        // TODO add time adjustment option
    },
    List,
    Remove {
        name: Option<String>,
    },
    Rename {
        name: Option<String>,
        new_name: Option<String>,
    }
}

#[derive(ValueEnum, Clone, Serialize, Deserialize)]
enum Unit {
    Hours,
    Days,
    Weeks,
    Months,
    Years,
}

fn parse_time(duration: u64, unit: Unit) -> Duration {
    Duration::from_secs(
        duration
            * match unit {
                Unit::Hours => 60 * 60,
                Unit::Days => 60 * 60 * 24,
                Unit::Weeks => 60 * 60 * 24 * 7,
                Unit::Months => 60 * 60 * 24 * 30, //TODO lollll
                Unit::Years => 60 * 60 * 24 * 365,
            },
    )
}

fn main() -> Result<()> {
    let home = dirs::home_dir().context("could not find home directory")?;
    let default_path = home.join(".todoom");
    dbg!(&default_path);
    let state_path: PathBuf = var("TODOOM_PATH")
        .map(PathBuf::from)
        .unwrap_or(default_path)
        .into();

    // return file if exists, if open fails, tansform to create new file.

    // TODO, probably a better way to do this with OpenOptions
    // I did this way because open can error on more than just file non-existence
    // try_exists returns Ok(False) if confirmed not to exist, we need to handle.
    let mut state = if state_path.try_exists().is_ok_and(|b| b) {
        let state_file =
            File::open(&state_path).context("attempted to open existing path, but found error")?;
        serde_json::from_reader(BufReader::new(state_file)).context("failed to parse state")?
    } else {
        State::new()
    };

    let cli = Cli::parse();

    match cli.command {
        SubCommand::Add {
            name,
            body,
            period,
            unit,
        } => {
            let body = body.unwrap_or_else(|| long_edit(None).unwrap());
            let new = Habit::new(name, body, parse_time(period, unit));
            state.todo.insert(new.name.clone(), new);
        }
        SubCommand::Log { name } => {
            let mut find = state.find(name.as_deref())?;
            let habit = find.get_mut();
            habit.logged = SystemTime::now();
        }
        SubCommand::Edit { name } => {
            let mut find = state.find(name.as_deref())?;
            let habit = find.get_mut();
            let body = long_edit(Some(habit.body.clone()))?;
            habit.body = body;
        }
        SubCommand::Remove { name } => {
            let habit = state.find(name.as_deref())?;
            habit.remove();
        }
        SubCommand::List => {
            for (name, habit) in state.todo.iter() {
                println!("{}: {}", name, habit.name);
            }
        }
        SubCommand::Rename { name, new_name } => todo!(),
    }

    let state_file = File::create(state_path).context("failed to create state file")?;
    serde_json::to_writer_pretty(BufWriter::new(state_file), &state)
        .context("failed to write state to file")?;
    Ok(())
}
