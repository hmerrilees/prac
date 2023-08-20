//! # The feedback-oriented utility for a practice-oriented life.
//!
//! # UI demo + TLDR
//! Let's say we'd like to set a new practice of making a weekly repo, well... every week.
//! ```bash
//! prac add "weekly repo" 1 week
//! ```
//! Now, we can view "weekly repo" alongside all our practices.
//! ```bash
//! prac list
//! ```
//! ```text
//! distributed systems programming ▬▬▬
//!                       daily log ▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬
//!                        exercise ▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬
//!                     kierkegaard ▬▬▬▬▬▬▬
//!                           steno ▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬
//!                     weekly repo
//! ```
//! > As time elapses through the weekly repo's period, the bar will fill like the rest.
//!
//! Looks like I haven't done steno in a while... when I get stuck with whatever I'm doing, I'll switch to that.
//!
//! When I'm done, I'll ```prac log steno 30 minutes``` to reset the bar and track time, and ```prac notes steno``` to make some notes with `$EDITOR` on my progress.
//!
//! (tip: use `prac list --cumulative` to see cumulative hours logged, are we 10000 yet?)
//!
//! Be sure to explore `prac help` and `prac help <subcommand>` for more.
//!
//! # Motivation, problem, and solution(?)
//!
//! ## Motivation
//! Developing skill takes time + structure. prac attempts to promote both while being as lightweight as possible.
//!
//!
//! ## Solving the right problems
//! To remain lightweight, prac sticks only to problems that (to me) most obviously need solving.
//!
//! Primarily,
//! - "What should I do now?" in instances where pre-planning is inadviseable or impossible,
//! - losing track of practices I haven't done in a while, and
//! - progress/time tracking without excessive overhead or breaking flow.
//!
//! ## What's so special about prac?
//! Not much, and that's on purpose, but in service of the above, proc has a few distinguishing
//! design decisions:
//! - Rather than "events" being triggered by the clock/calendar, which are not privileged to
//! user's psychological state, the proc lifecycle starts when the user gets stuck in their current task
//!    or otherwise decides it's time to do something new. This avoids flow-breaking interruptions
//!    while promoting mindfulness as an active part of the user's feedback loop.
//! - Rather than on a scheduled interval, items run on time elapsed since prior log. E.g. a
//! daily task period begins when you log it, and ends within 24 hours (plus a default 2-hr grace period).
//!  There is no scheduling to displace user agency, elapsed time since last log is displayed
//! as a fraction of the period set for each practice. This information can be incorporated into the final decision entirely at the user's discretion.
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
use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::collections::btree_map;
use std::fmt::{Display, Formatter};
use std::{
    collections::BTreeMap,
    env::var,
    fs::File,
    io::{BufReader, BufWriter},
    path::PathBuf,
    time::{Duration, SystemTime},
};

use anyhow::{bail, Context, Result};

use dialoguer::{FuzzySelect, Input};

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
    practices: BTreeMap<String, Practice>,
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

impl Display for Practice {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.name.contains(' ') {
            write!(f, "\"{}\"", self.name)
        } else {
            write!(f, "{}", self.name)
        }
    }
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

    fn find_name(&self) -> Result<String> {
        let options = &self.practices.keys().collect::<Vec<_>>();

        let selection_index = FuzzySelect::new()
            .with_prompt("Select practice")
            .items(options)
            .interact_opt()
            .context("Selection error.")?;

        match selection_index {
            Some(i) => Ok(options[i].to_owned()),
            None => bail!("No item selected"),
        }
    }

    fn find_mut(
        &mut self,
        name: Option<&str>,
    ) -> Result<btree_map::OccupiedEntry<String, Practice>> {
        let name = match name {
            Some(name) => name.to_owned(),
            None => self.find_name()?,
        };

        match self.practices.entry(name) {
            btree_map::Entry::Vacant(_) => bail!("Practice not found. (Case sensitive)"),
            btree_map::Entry::Occupied(practice) => Ok(practice),
        }
    }

    fn get_path() -> Result<PathBuf> {
        if let Ok(practice_path) = var("PRAC_PATH") {
            let path = PathBuf::from(practice_path);
            Ok(path)
        } else if let Ok(practice_home) = var("PRAC_HOME") {
            let practice_home = PathBuf::from(practice_home);
            std::fs::create_dir_all(&practice_home)
                .context("$PRAC_HOME specified but could not be created.")?;
            let path = practice_home.join("prac.json");
            Ok(path)
        } else if let Some(data_home) = dirs::data_dir() {
            let default_dir = data_home.join("prac");
            std::fs::create_dir_all(&default_dir)
                .with_context(|| format!("could not create {}", default_dir.display()))?;
            let path = default_dir.join("prac.json");
            Ok(path)
        } else {
            let path = dirs::home_dir()
                .context("could not find home directory")?
                .join(".prac.json");
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
        /// Some notes to outline goals and log progress. Opens `$EDITOR` if not provided.
        #[arg(short, long)]
        notes: Option<String>,
        /// Anticipated time period between practice sessions (There is a 2 hr grace period by default)
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
        current_name: Option<String>,
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
            if state.practices.contains_key(&name) {
                bail!("Practice with name `{}` already exists.", &name);
            }
            let notes = notes.unwrap_or_else(|| {
                let placeholder = if state.practices.is_empty() {
                        Some("\
                            When you complete a practice, you should log it with `prac log`.\n\
                            Come back to these practice notes and view this page later in your `$EDITOR` with `prac notes`. \n\
                            When you get stuck, you can view all your practices (and how far along they are in their periods) with `prac list`.\n\
                            Delete this message and set some clear goals for your first practice!\
                            ".to_string())
                    } else {None};

                utils::long_edit(placeholder).unwrap()
            });

            let new = Practice::new(name.clone(), notes, TimeUnit::to_duration(period, unit));
            state.practices.insert(new.name.clone(), new);

            println!("Added practice `{name}`.");

            if state.practices.len() == 1 {
                println!("You can view your practice with `prac list`. It may take a little time elapsed for progress bars to progress to display a character.");
            }
        }
        SubCommand::Log {
            name,
            time,
            time_unit: unit,
            notes: log,
        } => {
            let mut find = state.find_mut(name.as_deref())?;
            let practice = find.get_mut();
            practice.logged = SystemTime::now();

            let duration = TimeUnit::to_duration(time, unit);
            practice.cumulative += duration;

            if log {
                let body = utils::long_edit(Some(practice.notes.clone()))?;
                practice.notes = body;
            } else {
                // if there is whitespace in notes, surround in quotes
                println!(
                    "Logged. Be sure to make notes on your progress with `prac notes {practice}`."
                );
            }
        }
        SubCommand::Notes { name } => {
            let mut find = state.find_mut(name.as_deref())?;
            let practice = find.get_mut();
            let body = utils::long_edit(Some(practice.notes.clone()))?;
            practice.notes = body;
        }
        SubCommand::Remove { name } => {
            let practice = state.find_mut(name.as_deref())?;
            let name = practice.get().name.clone();

            let delete_message = format!("CONFIRM DELETE {}", name);

            let confirm = Input::<String>::new()
                .with_prompt(format!(
                    "Type \"{delete_message}\" to remove practice (case sensitive)"
                ))
                .allow_empty(true)
                .interact()
                .is_ok_and(|s| s == delete_message);

            if confirm {
                practice.remove();
                print!("Removed {}", name);
            } else {
                println!("Aborting.");
                return Ok(());
            }
        }
        SubCommand::List { cumulative, period } => {
            if state.practices.is_empty() {
                println!("You don't have any practices yet. Add some with `prac add`.");
                return Ok(());
            }
            let max_name_len = state
                .practices
                .keys()
                .map(|name| name.len())
                .max()
                .unwrap_or(0)
                .min(30); // TODO magic number

            let term_width = termsize::get().context("failed to obtain termsize")?.cols;

            println!();
            let now = SystemTime::now();
            for (name, practice) in &state.practices {
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
        SubCommand::Rename { current_name } => {
            // Annoying constraints here of having to know name of existing practice prior to rename
            // And then only having one mutable borrow of practices
            let found_name = match current_name {
                Some(name) => name.to_owned(),
                None => state.find_name()?,
            };
            let new_name = Input::<String>::new()
                .with_prompt(format!("New name for {found_name}"))
                .interact()?;

            if state.practices.contains_key(&new_name) {
                bail!("Practice with name `{}` already exists.", new_name);
            }

            let mut practice = state
                .practices
                .remove(&found_name)
                .with_context(|| format!("Practice `{}` not found.", found_name))?;
            practice.name = new_name.clone();
            state.practices.insert(new_name, practice);
        }
        SubCommand::Reset => {
            for practice in state.practices.values_mut() {
                practice.logged = SystemTime::now();
            }
        }
        SubCommand::EditPeriod {
            name,
            period,
            time_unit: unit,
        } => {
            let mut find = state.find_mut(Some(&name))?;
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
