use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};

use serde_with::serde_as;
use std::collections::btree_map;
use std::fmt::{Display, Formatter};
use std::{collections::BTreeMap, path::PathBuf};

use anyhow::{bail, ensure, Context, Result};

use dialoguer::FuzzySelect;

/// User exposed (via [``SubCommand::config``](crate::cli::SubCommand::Config)) configuration.
#[serde_as]
#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct UserConfig {
    #[serde_as(as = "serde_with::DurationSeconds<i64>")]
    #[serde(rename = "grace_period_in_seconds")]
    /// Grace period adds extra time in progress display. This aids against practices creeping earlier.
    pub grace_period: Duration,
}

impl Default for UserConfig {
    fn default() -> Self {
        Self {
            grace_period: Duration::zero(),
        }
    }
}
/// This is the application config, which includes user-editable [``UserConfig``]
/// , as well as other items like version that the user probably shouldn't touch.
#[derive(Serialize, Deserialize)]
struct Config {
    /// Version of prac that created this state file.
    version: String,
    /// User editable configuration.
    user_config: UserConfig,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            version: env!("CARGO_PKG_VERSION").to_owned(),
            user_config: UserConfig::default(),
        }
    }
}

/// A practice is an activity that you wish to repeat every so often. Not so much a task (completion oriented), not a habit (in absolute time), or scheduling-item.
#[serde_as]
#[derive(Serialize, Deserialize)]
pub struct Practice {
    /// Time practice created by user
    created: DateTime<Utc>,
    /// Last time practice was logged
    logged: DateTime<Utc>,
    /// How often you wish to repeat practice (starting from last log)
    #[serde_as(as = "serde_with::DurationSeconds<i64>")]
    period: Duration,
    /// Unique id of practice, will be used for retrieval
    name: String,
    /// Plain-text notes where user can set goals, track progress, etc.
    notes: String,
    /// Cumulative time spent on this practice
    #[serde_as(as = "serde_with::DurationSeconds<i64>")]
    cumulative: Duration,
    // TODO maybe a Completion struct? then a body enum {practice, Task} that contains Vec<Comepletion> for practice and raw
    // Completion for task. Trying not to prematurely optimize.
}

impl Practice {
    /// Create a new practice with the given name, notes, and period, starting with no logged time.
    fn new(name: String, notes: String, period: Duration) -> Self {
        let created = Utc::now();
        let logged = created;

        Self {
            created,
            logged,
            period,
            name,
            notes,
            cumulative: Duration::seconds(0),
        }
    }

    /// Number of seconds elapsed since last practice
    fn elapsed(&self) -> Duration {
        let now = Utc::now();
        now - self.logged
    }
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

/// Application state containing all data for prac, also w/ serialization/deserialization.
#[derive(Serialize, Deserialize, Default)]
pub struct State {
    config: Config,
    practices: BTreeMap<String, Practice>,
}

impl State {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn update_version(&mut self) {
        self.config.version = env!("CARGO_PKG_VERSION").to_owned();
    }

    pub fn list(&self, cumulative: bool, period: bool, danger: bool) -> Result<()> {
        if self.practices.is_empty() {
            println!("You don't have any practices yet. Add some with `prac add`.");
            return Ok(());
        }

        let start_messages = self
            .practices
            .keys()
            .map(|name| format!("  {name} "))
            .collect::<Vec<_>>();

        let end_messages = &self
            .practices
            .values()
            .map(|practice| {
                let period_time = super::time::FlatTime::from(practice.period);
                let cumulative_time = super::time::FlatTime::from(practice.cumulative);

                match (cumulative, period) {
                    (true, true) => format!(
                        " {} c / {} p  ",
                        cumulative_time.format_abbreviated(),
                        period_time.format_abbreviated(),
                    ),
                    (true, false) => {
                        format!(" {}  ", cumulative_time.format())
                    }
                    (false, true) => format!(" {}  ", period_time.format()),
                    (false, false) => "  ".to_string(),
                }
            })
            .collect::<Vec<_>>();

        let max_start_len = start_messages.iter().map(String::len).max().unwrap();
        let max_end_len = end_messages.iter().map(String::len).max().unwrap();

        let padded_start_messages = start_messages
            .iter()
            .map(|s| format!("{s:>max_start_len$}"))
            .collect::<Vec<_>>();
        let padded_end_messages = end_messages
            .iter()
            .map(|s| format!("{s:<max_end_len$}"))
            .collect::<Vec<_>>();

        let term_width = termion::terminal_size()
            .context("failed to obtain termsize")?
            .0;
        let padding_width = max_start_len + max_end_len;
        let bar_width = (term_width as usize)
            .checked_sub(padding_width)
            .with_context(|| {
                format!("term width {term_width} too small, must be at least {padding_width}")
            })?;

        println!();
        for (practice, start, end) in itertools::izip!(
            self.practices.values(),
            padded_start_messages,
            padded_end_messages
        ) {
            let grace_adjusted_period = practice.period + self.config.user_config.grace_period;
            #[allow(clippy::cast_precision_loss)]
            let fraction = practice.elapsed().num_seconds() as f64
                / grace_adjusted_period.num_seconds() as f64;

            let whole_bar = format!("{}{}{}", start, crate::utils::bar(bar_width, fraction), end);

            println!("{whole_bar}");
        }
        println!();

        if danger {
            let sum_progress: i64 = self
                .practices
                .values()
                .map(|p| p.elapsed().num_seconds())
                .sum();
            let sum_period: i64 = self
                .practices
                .values()
                .map(|p| (p.period + self.config.user_config.grace_period).num_seconds())
                .sum();

            #[allow(clippy::cast_precision_loss)]
            let sum_fraction = sum_progress as f64 / sum_period as f64;

            // TODO make red
            let sum_bar = crate::utils::bar(bar_width, sum_fraction);
            let start = format!("  {} ", "danger");
            let end = String::new();

            println!("{start:>max_start_len$}{sum_bar}{end:<max_end_len$}");
        }

        Ok(())
    }

    /// Find the name of a practice either validating an name input, or if not provided, prompting the user to select one.
    pub fn find_name(&self) -> Result<&str> {
        let options = &self.practices.keys().collect::<Vec<_>>();

        let selection_index = FuzzySelect::new()
            .with_prompt("Select practice")
            .items(options)
            .interact_opt()
            .context("Selection error.")?;

        match selection_index {
            Some(i) => Ok(options[i].as_str()),
            None => bail!("No item selected"),
        }
    }

    pub fn get_notes(&self, name: &str) -> Result<&str> {
        let notes = self
            .practices
            .get(name)
            .with_context(|| format!("\"{name}\" not found."))?
            .notes
            .as_str();
        Ok(notes)
    }

    pub fn get_user_config(&self) -> &UserConfig {
        &self.config.user_config
    }

    /// Get the path to the default location state file.
    /// Search order: [`dirs::data_dir`]/prac/prac.json, [`dirs::home_dir`]/.prac.json
    /// This may be overridden elsewhere, in either the `PRAC_PATH` env var, or with the --file arg.
    pub fn get_path() -> Result<PathBuf> {
        if let Some(data_home) = dirs::data_dir() {
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

/// See [``Cli::SubCommand``](crate::cli::SubCommand) for documentation.
#[warn(clippy::print_stdout, clippy::print_stderr)]
#[allow(clippy::missing_docs_in_private_items)]
// TODO move option hanlding, completely abstract interface from CLI
#[serde_as]
#[derive(Serialize, Deserialize)]
pub enum StateTransition {
    Add {
        name: String,
        #[serde_as(as = "serde_with::DurationSeconds<i64>")]
        period: Duration,
    },
    Log {
        name: String,
        #[serde_as(as = "serde_with::DurationSeconds<i64>")]
        /// Maximum amount of time intended to spend. Both useful as a time-boxing strategy and as
        /// a backstop to neglected termination.
        time: Duration,
    },
    Notes {
        name: String,
        notes: String,
    },
    Remove {
        name: String,
    },
    Rename {
        current_name: String,
        new_name: String,
    },
    Reset,
    EditPeriod {
        name: String,
        #[serde_as(as = "serde_with::DurationSeconds<i64>")]
        new_period: Duration,
    },
    Config {
        new_config: UserConfig,
    },
}

#[warn(clippy::print_stdout, clippy::print_stderr)] // This shouldn't use cli at all (warning doesn't catch stdin, but if we're printing, something is wrong)
pub fn handle_transition(state: &mut State, transition: StateTransition) -> Result<()> {
    match transition {
        StateTransition::Add { name, period } => {
            let practice = Practice::new(name.clone(), String::new(), period);
            match state.practices.entry(practice.name.clone()) {
                btree_map::Entry::Occupied(_) => {
                    bail!("Practice with name \"{name}\" already exists.")
                }
                btree_map::Entry::Vacant(entry) => entry.insert(practice),
            };
            Ok(())
        }
        StateTransition::Log { name, time } => {
            let practice = state
                .practices
                .get_mut(&name)
                .context("Practice not found.")?;
            practice.logged = Utc::now();
            practice.cumulative = practice.cumulative + time;
            Ok(())
        }
        StateTransition::Notes { name, notes } => {
            let practice = state
                .practices
                .get_mut(&name)
                .context("Practice not found.")?;
            practice.notes = notes;
            Ok(())
        }
        StateTransition::Remove { name } => {
            let practice = state.practices.entry(name.clone());
            match practice {
                btree_map::Entry::Vacant(_) => {
                    bail!("Practice with name \"{name}\" not found. (Case sensitive)")
                }
                btree_map::Entry::Occupied(entry) => entry.remove(),
            };
            Ok(())
        }
        StateTransition::Rename {
            current_name,
            new_name,
        } => {
            ensure!(
                state.practices.contains_key(&new_name),
                "Practice with name \"{new_name}\" already exists."
            );
            ensure!(
                !state.practices.contains_key(&new_name),
                "Practice with name \"{new_name}\" already exists."
            );

            let practice = state
                .practices
                .remove(&current_name)
                .expect("we already checked for key membership");
            state.practices.insert(new_name, practice);
            Ok(())
        }
        StateTransition::Reset => {
            let now = Utc::now();
            for practice in state.practices.values_mut() {
                practice.logged = now;
            }
            Ok(())
        }
        StateTransition::EditPeriod { name, new_period } => {
            let practice = state
                .practices
                .get_mut(&name)
                .context("Practice not found.")?;
            practice.period = new_period;
            Ok(())
        }
        StateTransition::Config { new_config } => {
            state.config.user_config = new_config;
            Ok(())
        }
    }
}
