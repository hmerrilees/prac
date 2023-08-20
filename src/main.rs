//! # The feedback-oriented utility for a practice-oriented life.
//!
//! ## UI demo + TLDR
//! 
//! ```bash
//! prac list
//! ```
//! ```text
//! distributed systems programming ▬▬▬
//!                       daily log ▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬
//!                        exercise ▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬
//!                     kierkegaard ▬▬▬▬▬▬▬
//!                           steno ▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬
//!                     weekly repo ▬▬▬
//!
//! (tip: use `prac list --cumulative` to see cumulative hours tracked, are we 10000 yet?)
//! ```
//! > Looks like I haven't done steno in a while... when I get stuck, I'll switch to that.
//!
//! When I'm done, I'll ```prac log steno 2 hours``` to reset the bar and track time, and ```prac notes steno``` to make some notes w/ $EDITOR on my progress.
//!
//! ### Motivation
//! Developing skill takes time + structure. prac attempts to promote both while being as lightweight as possible.
//!
//!
//! ### Solving the right problems.
//! To remain lightweight, prac sticks only to problems that (to me) most obviously need solving:
//! - "What should I do now?" in instances where pre-planning is inadviseable or impossible,
//! - losing track of practices I haven't done in a while, and
//! - progress and time tracking without excessive overhead or breaking flow.
//!
//! ### What's so special about prac?
//! Not much, and that's on purpose, but there are a few key differences:
//! - Rather than "events" being triggered by the clock/calendar, which are not privileged to your
//!    psychological state, the proc lifecycle starts when the user gets stuck in their current task 
//!    or otherwise decides it's time to do something new. This avoids flow-breaking interruptions 
//!    while encoraging the user to become more in tune with their own needs and psychological rhythms.
//! - Rather than on a scheduled interval, items run on time elapsed since prior log. E.g. a
//! daily task period begins when you log it, and ends within 24 hours (plus a default 2-hr grace period).
//!  Time does not displace your agency, rather time-since-last-log for each practice is displayed
//! as a fraction of the period set for each. This information can be incorporated into the final decision entirely on the users terms. 
//! - Tracking is dead-simple, intentionally adding no functionality that is not possible with pen
//! and paper. Time is tracked is a sum total of self-reported increments. Logging is done in plain-text.
//!
//! ### More benefits of elapsed-time periods
//! - Scheduled/calendar intervals are intolerant to period drift either way. If you finish too
//! late (i.e. need a longer feedback cycle), you find yourself having to work more quickly to
//! catch up on the accumulated iterations. If you finish too early (i.e. need shorter feedback
//! cycle), you have to wait even longer until the next scheduled event.
//! - With elapsed-time periods, an overrun is no big deal, nothing stacks up, just log it when you
//! get to it and you'll start again with a full period.
//! - You also are not "penalized" for overachieving / finishing early... just make sure you are working at a
//! pace sustainable to finish within the next period which you have just moved forward. 
//! - If you find yourself regularly finishing very early/late, no big deal! Just take it as a sign
//! that you need to adjust the period of your feedback cycle!
//!
//!
//!
//! 
use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use skim::prelude::*;
use std::collections::btree_map;
use std::io::Cursor;
use std::{
    collections::BTreeMap,
    env::var,
    fs::File,
    io::{BufReader, BufWriter},
    path::PathBuf,
    time::{Duration, SystemTime},
};

use anyhow::{bail, Context, Result};

mod utils;
use utils::TimeUnit;

// TODO, config edit command
 /// Configuration state.
#[derive(Serialize, Deserialize)]
struct Config {
    grace_period: GracePeriod,
    /// Version of prac that created this state file.
    version: String,
    // potential versioning compat implementation for new fields: use options on import, and let-else w/
    // defaults on save.
}

impl Default for Config {
    fn default() -> Self {
        Self {
            grace_period: GracePeriod::Fractional(1.1),
            version: env!("CARGO_PKG_VERSION").to_owned(),
        }
    }
}

#[derive(Serialize, Deserialize)]
enum GracePeriod {
    Fixed(Duration),
    Fractional(f64),
}

// TODO, solve backwards compatibility issue, see Config
#[derive(Serialize, Deserialize, Default)]
struct State {
    routines: BTreeMap<String, Practice>,
    config: Config,
}

#[derive(Serialize, Deserialize)]
struct Practice {
   created: SystemTime,
    // last time logged
   logged: SystemTime,
    // how often you wish to repeat practice
   period: Duration,
    // unique id of practice, will be used for retrieval
   name: String,
    // take notes
   notes: String,
   cumulative: Duration,
    // TODO maybe a Completion struct? then a body enum {practice, Task} that contains Vec<Comepletion> for practice and raw
    // Completion for task. Trying not to prematurely optimize.
}

impl Practice {
    fn new(name: String, notes: String, period: Duration) -> Self {
        let created = SystemTime::now();
        let logged = created;

        Self {
            created,
            logged,
            period,
            name,
            notes,
            cumulative: Duration::new(0, 0),
        }
    }

    // TODO: find external crate for this
}

impl State {
    fn new() -> Self {
        Self::default()
    }

    fn find(&mut self, name: Option<&str>) -> Result<btree_map::OccupiedEntry<String, Practice>> {
        let name = name
            .context("name not provided")
            .map(String::from)
            .or_else(|_e| {
                let options = SkimOptionsBuilder::default().build().unwrap();

                let items = SkimItemReader::default().of_bufread(Cursor::new(
                    self.routines
                        .keys()
                        .map(|k| format!("{}", k))
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
                    1 => selected_items.get(0).expect("we know there is one").text(),
                    2.. => bail!("Multiple items selected"),
                    _ => unreachable!(),
                };

                Ok(item.into())
            })
            .context("failure to obtain name")?;

        match self.routines.entry(name) {
            btree_map::Entry::Vacant(_) => bail!("Practice not found."),
            btree_map::Entry::Occupied(practice) => Ok(practice),
        }
    }

    fn get_path() -> Result<PathBuf> {
        if let Ok(practice_path) = var("PRAC_PATH") {
            let path = PathBuf::from(practice_path);
            Ok(path)
        } else if let Ok(practice_home) = var("PRAC_HOME") {
            let practice_home = PathBuf::from(practice_home);
            std::fs::create_dir_all(&practice_home).context("$PRAC_HOME specified but could not be created.")?;
            let path = practice_home.join("prac.json");
            Ok(path)
        } else if let Some(data_home) = dirs::data_dir() {
            let default_dir = data_home.join("prac");
            std::fs::create_dir_all(&default_dir).with_context(|| format!("could not create {}", default_dir.display()))?;
            let path = default_dir.join("prac.json");
            Ok(path)
        } else {
            let path = dirs::home_dir().context("could not find home directory")?.join(".prac.json");
            Ok(path)
        }
    }
}

/// Prac: a dead-simple practice-cultivating utility.
#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: SubCommand,
}


// TODO: config edit command
#[derive(Subcommand)]
enum SubCommand {
    /// List practices w/ progress bars showing time elapsed through period. `help list` for options
    /// Use this when you get stuck on another task, want to switch to something else, and would be
    /// benefitted by knowing how long it's been since you last .
    List {
        /// Show cumulative hours tracked alongside practices.
        #[arg(short, long)]
        cumulative: bool,
        /// Show period of practice alongside practices
        #[arg(short, long)]
        period: bool,
    },
    /// Add a new practice: `help add <name> <period> <unit>`.
    Add {
        /// A (unique) name for the practice.
        name: String,
        /// Some notes to outline goals and log progress. Opens $EDITOR if not provided.
        #[arg(short, long)]
        notes: Option<String>,
        /// Anticipated time period between practice sessions. (There is a 2 hr grace period by default.)
        period: f64,
        #[arg(value_enum)]
        time_unit: TimeUnit,
    },
    /// After you practice, `prac log <name> <time> <unit>` to reset the bar and track time.
    Log {
        /// Specify, or leave blank to fuzzy search.
        name: Option<String>,
        /// How long you practiced for. This is added to the cumulative time displayed in `prac list --cumulative`.
        time: f64,
        #[arg(value_enum)]
        time_unit: TimeUnit,
        /// An optional shortcut to `prac notes` when you're done.
        #[arg(short, long)]
        notes: bool,
    },
    /// Edit practice notes. Each practice has its own page.
    Notes {
        /// Specify practice to edit, or leave blank to fuzzy search.
        name: Option<String>,
    },
    /// Reset all progress bars if you fall behind.
    /// Equivalent to tracking all practices w/ zero time.
    Reset,
    /// Show state file location.
    ///
    /// State is stored in $PRACTICE_PATH, $PRACTICE_HOME/prac.json, [dirs::data_dir]/prac/prac.json 
    /// or [dirs::home_dir]/.prac.json, searched in that order.
    ///
    /// It's a good idea to vcs your state file.
    StateLocation,
    #[command(alias = "ep")]
    EditPeriod {
        name: String,
        /// Anticipated time period between practice sessions. (There is a 2 hr grace period by default.)
        period: f64,
        #[arg(value_enum)]
        time_unit: TimeUnit,
    },
    Remove {
        /// Specify name of practice to remove, or leave blank to fuzzy search.
        name: Option<String>,
    },
    Rename {
        /// Current (old) name of practice.
        current_name: String,
        /// New name of practice.
        new_name: String,
    },
}


fn main() -> Result<()> {
    let state_path = State::get_path()?;

    // return file if exists, if open fails, tansform to create new file.

    // I did this way because open can error on more than just file non-existence
    // try_exists returns Ok(False) if confirmed not to exist, we need to handle.
    // ... bad default semantics, maybe use OpenOptions instead
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
            notes,
            period,
            time_unit: unit,
        } => {
            let notes = notes.unwrap_or_else(|| {
                let placeholder = if state.routines.is_empty() { 
                        Some("\
                            When you complete a practice, you should log it with `prac log`.\n\
                            Come back to these practice notes and view this page later in your $EDITOR with `prac notes`. \n\
                            When you get stuck, you can view all your practices (and how far along they are in their periods) with `prac list`.\n\
                            Delete this message and set some clear goals for your first practice!\
                            ".to_string())
                    } else {None};

                utils::long_edit(placeholder).unwrap()
            });

            let new = Practice::new(name.clone(), notes, TimeUnit::to_duration(period, unit));
            state.routines.insert(new.name.clone(), new);

            println!("Added practice {name}.");

            if state.routines.len() == 1 {
                println!("You can view your practice with `prac list`. It may take a little time elapsed for progress bars to progress to display a character.");
            }
        }
        SubCommand::Log {
            name,
            time,
            time_unit: unit,
            notes: log,
        } => {
            let mut find = state.find(name.as_deref())?;
            let practice = find.get_mut();
            practice.logged = SystemTime::now();

            let duration = TimeUnit::to_duration(time, unit);
            practice.cumulative += duration;

            if log {
                let body = utils::long_edit(Some(practice.notes.clone()))?;
                practice.notes = body;
            } else {
                println!("Good job! It's a good idea to make notes on your progress with `prac notes`.");
            }
        }
        SubCommand::Notes { name } => {
            let mut find = state.find(name.as_deref())?;
            let practice = find.get_mut();
            let body = utils::long_edit(Some(practice.notes.clone()))?;
            practice.notes = body;
        }
        SubCommand::Remove { name } => {
            let practice = state.find(name.as_deref())?;
            let name = practice.get().name.clone();
            let confirm = format!("Confirm remove {}", name);
            // Confirm
            // TODO this is terrible
            let options = SkimOptionsBuilder::default().build().unwrap();
            let items = SkimItemReader::default().of_bufread(Cursor::new(
                [&confirm, "Abort"].join("\n")
            ));

            // TODO figure out what these errors acutally are
            let selected_items = Skim::run_with(&options, Some(items))
                .context("Selection error.")?
                .selected_items;

            // ensure only one item is selected
            let item = match selected_items.len() {
                0 => bail!("No item selected"),
                1 => selected_items.get(0).expect("we know there is one").text(),
                2.. => bail!("Multiple items selected"),
                _ => unreachable!(),
            };

            if item != confirm {
                println!("Aborting.");
                return Ok(());
            } else {
                practice.remove();
                print !("Removed {}", name);
            }

        }
        SubCommand::List { cumulative, period } => {
            if state.routines.is_empty() {
                println!("You don't have any practices yet. Add some with `prac add`.");
                return Ok(());
            }
            let max_name_len = state
                .routines
                .keys()
                .map(|name| name.len())
                .max()
                .unwrap_or(0)
                 .min(30); // TODO magic number
             
            let term_width = termsize::get().context("failed to obtain termsize")?.cols;

            println!();
            let now = SystemTime::now();
            for (name, practice) in &state.routines {
                let mut truncated_name = name.clone();
                truncated_name.truncate(max_name_len); // better way to do this?

                let start_message = format!("  {truncated_name:>max_name_len$} ");

                let hours_cumulative = practice.cumulative.as_secs_f64() / 3600.0_f64;
                let hours_period = practice.period.as_secs_f64() / 3600.0;

                let end_message = match (cumulative, period) {
                    (true, true) => format!(
                        " {:>4}h c  {:>4}h p  ",
                        hours_cumulative as u64, hours_period as u64
                    ),
                    (true, false) => format!(" {:>4}h cumulative  ", hours_cumulative as u64),
                    (false, true) => format!(" {:>4}h period  ", hours_period as u64),
                    (false, false) => String::from("  "),
                };

                let elapsed = now
                    .duration_since(practice.logged)
                    .context("last log is in future")?;

                let grace_adjusted_period = match state.config.grace_period {
                    GracePeriod::Fixed(d) => (d + practice.period).as_secs_f64(),
                    GracePeriod::Fractional(f) => practice.period.as_secs_f64() * f,
                };
                let fraction = elapsed.as_secs_f64() / grace_adjusted_period;

                let bar_width = (term_width as usize)
                    .checked_sub(start_message.len() + end_message.len())
                    .context("terminal too narrow")?;

                let whole_bar = format!(
                    "{}{}{}",
                    start_message,
                    utils::bar(bar_width, fraction),
                    end_message
                );

                println!("{whole_bar}");
            }
            println!();
        }
        SubCommand::Rename {
            current_name: name,
            new_name,
        } => {
            let mut practice = state.routines.remove(&name).context("practice not found")?;
            practice.name = new_name.clone();
            state.routines.insert(new_name, practice);
        }
        SubCommand::Reset => {
            for practice in state.routines.values_mut() {
                practice.logged = SystemTime::now();
            }
        }
        SubCommand::EditPeriod {
            name,
            period,
            time_unit: unit,
        } => {
            let mut find = state.find(Some(&name))?;
            let practice = find.get_mut();
            let period = TimeUnit::to_duration(period, unit);
            practice.period = period;
        }
        SubCommand::StateLocation => {
            println!("state_path: `{}`", state_path.display());
        }
    }

    let state_file = File::create(state_path).context("failed to create state file")?;
    serde_json::to_writer_pretty(BufWriter::new(state_file), &state)
        .context("failed to write state to file")?;
    Ok(())
}
