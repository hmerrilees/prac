use crate::time::FlatTime;

use super::utils;
use chrono::{DateTime, Duration, Utc};
use itertools::izip;
use serde::{Deserialize, Serialize};

use serde_with::serde_as;
use std::collections::btree_map;
use std::fmt::{Display, Formatter};
use std::{collections::BTreeMap, env::var, path::PathBuf};

use anyhow::{bail, ensure, Context, Result};

use dialoguer::{Confirm, Editor, FuzzySelect, Input};

/// User exposed (via [``SubCommand::config``](crate::cli::SubCommand::Config)) configuration.
#[serde_as]
#[derive(Serialize, Deserialize)]
struct UserConfig {
    #[serde_as(as = "serde_with::DurationSeconds<i64>")]
    #[serde(rename = "grace_period_in_seconds")]
    /// Grace period adds extra time in progress display. This aids against practices creeping earlier.
    grace_period: Duration,
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
struct Practice {
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
    fn elapsed(&self) -> i64 {
        let now = Utc::now();
        let elapsed = now - self.logged;
        elapsed.num_seconds()
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

    /// Find the name of a practice either validating an name input, or if not provided, prompting the user to select one.
    fn find_name(&self) -> Result<String> {
        let options = &self.practices.keys().collect::<Vec<_>>();

        let selection_index = FuzzySelect::new()
            .with_prompt("Select practice")
            .items(options)
            .interact_opt()
            .context("Selection error.")?;

        match selection_index {
            Some(i) => Ok(options[i].clone()),
            None => bail!("No item selected"),
        }
    }

    /// Retrieve practice object by name, or if not provided, prompt the user to select one.
    fn find_mut(
        &mut self,
        name: Option<&str>,
    ) -> Result<btree_map::OccupiedEntry<String, Practice>> {
        let name = match name {
            Some(name) => name.to_owned(),
            None => self.find_name()?,
        };

        match self.practices.entry(name.clone()) {
            btree_map::Entry::Vacant(_) => {
                bail!("Practice \"{name}\" not found. (Case sensitive)")
            }
            btree_map::Entry::Occupied(practice) => Ok(practice),
        }
    }

    /// Get the path to the state file, either from the environment, or from the default location.
    /// Search order: ``$PRAC_PATH``, ``$PRAC_HOME/prac.json``, [`dirs::data_dir`]/prac/prac.json, [`dirs::home_dir`]/.prac.json
    pub fn get_path() -> Result<PathBuf> {
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

/// See [``Cli::SubCommand``](crate::cli::SubCommand) for documentation.
#[allow(clippy::missing_docs_in_private_items)]
pub trait StateExt {
    fn add(&mut self, name: String, period: Duration, add_notes: bool) -> anyhow::Result<()>;
    fn log(&mut self, add_notes: bool) -> Result<()>;
    fn notes(&mut self, name: Option<String>) -> Result<()>;
    fn remove(&mut self, name: Option<String>) -> Result<()>;
    fn list(&self, cumulative: bool, period: bool) -> Result<()>;
    fn rename(&mut self, current_name: Option<String>) -> Result<()>;
    fn reset(&mut self);
    fn edit_period(&mut self, name: String, new_period: Duration) -> Result<()>;
    fn config(&mut self) -> Result<()>;
}

impl StateExt for State {
    fn add(&mut self, name: String, period: Duration, add_notes: bool) -> anyhow::Result<()> {
        if self.practices.contains_key(&name) {
            bail!("Practice with name `{}` already exists.", &name);
        }

        if self.practices.is_empty() {
            println!("\
                    When you complete a practice, you should log it with `prac log`.\n\
                    When you get stuck, you can view all your practices (and how far along they are in their periods) with `prac list`.\n\
                    It may take a little time for the first tick to show up on the progress bar.\n\
                    ");
        }

        let mut notes = format!("{name} notes:\n");
        if add_notes {
            notes = utils::long_edit(Some(&notes))?;
        }
        let new = Practice::new(name, notes, period);
        println!("Added practice {new}.");
        if !add_notes {
            println!(
                "Before you get started, you should set some goals with `prac notes {new}`.\n"
            );
        }
        self.practices.insert(new.name.clone(), new);

        Ok(())
    }
    fn log(&mut self, add_notes: bool) -> Result<()> {
        let mut find = self.find_mut(None)?;
        let practice = find.get_mut();
        practice.logged = Utc::now();

        let time_input = dialoguer::Input::<String>::new()
            .with_prompt(format!(
                "How long did you practice \"{practice}\" for?",
                practice = practice.name
            ))
            .allow_empty(false)
            .interact()?;

        let time = super::time::parse_time_span(&time_input)?;

        practice.cumulative = practice.cumulative + time;

        if add_notes {
            practice.notes = utils::long_edit(Some(&practice.notes))?;
        } else {
            // if there is whitespace in notes, surround in quotes
            println!(
                "Logged. Be sure to make notes on your progress with `prac notes {practice}`."
            );
        }
        Ok(())
    }
    fn notes(&mut self, name: Option<String>) -> Result<()> {
        let mut find = self.find_mut(name.as_deref())?;
        let practice = find.get_mut();
        practice.notes = Editor::new()
            .edit(&practice.notes)?
            .context("Content not saved")?;
        Ok(())
    }
    fn remove(&mut self, name: Option<String>) -> Result<()> {
        let practice = self.find_mut(name.as_deref())?;
        let name = practice.get().name.clone();

        let delete_message = format!("CONFIRM DELETE {name}");

        let confirm = Input::<String>::new()
            .with_prompt(format!(
                "Type \"{delete_message}\" to remove practice (case sensitive)"
            ))
            .allow_empty(true)
            .interact()
            .is_ok_and(|s| s == delete_message);

        if confirm {
            practice.remove();
            print!("Removed {name}");
        } else {
            println!("Aborting.");
        }
        Ok(())
    }
    fn list(&self, cumulative: bool, period: bool) -> Result<()> {
        if self.practices.is_empty() {
            println!("You don't have any practices yet. Add some with `prac add`.");
            return Ok(());
        }

        let start_messages = self
            .practices
            .keys()
            .map(|name| format!("  {} ", name))
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

        let term_width = termsize::get().context("failed to obtain termsize")?.cols;
        let bar_width = (term_width as usize)
            .checked_sub(max_start_len + max_end_len)
            .context("terminal too narrow")?;

        println!();
        for (practice, start, end) in izip!(
            self.practices.values(),
            padded_start_messages,
            padded_end_messages
        ) {
            let grace_adjusted_period = practice.period + self.config.user_config.grace_period;
            #[allow(clippy::cast_precision_loss)]
            let fraction = practice.elapsed() as f64 / grace_adjusted_period.num_seconds() as f64;

            let whole_bar = format!("{}{}{}", start, utils::bar(bar_width, fraction), end);

            println!("{whole_bar}");
        }
        println!();
        Ok(())
    }
    fn rename(&mut self, current_name: Option<String>) -> Result<()> {
        // Annoying constraints here of having to know name of existing practice prior to rename
        // And then only having one mutable borrow of practices
        let found_name = match current_name {
            Some(name) => name,
            None => self.find_name()?,
        };
        let new_name = Input::<String>::new()
            .with_prompt(format!("New name for {found_name}"))
            .interact()?;

        ensure!(!self.practices.contains_key(&new_name));
        let mut practice = self
            .practices
            .remove(&found_name)
            .with_context(|| format!("Practice `{}` not found.", found_name))?;
        practice.name = new_name.clone();
        self.practices.insert(new_name, practice);
        Ok(())
    }
    fn reset(&mut self) {
        for practice in self.practices.values_mut() {
            practice.logged = Utc::now();
        }
    }
    fn edit_period(&mut self, name: String, new_period: Duration) -> Result<()> {
        let mut find = self.find_mut(Some(&name))?;
        let practice = find.get_mut();

        let old = FlatTime::from(practice.period).format();
        let new = FlatTime::from(new_period).format();

        if Confirm::new()
            .with_prompt(format!("Change period of `{name}` from {old} to {new}?",))
            .interact()?
        {
            practice.period = new_period;
            println!("Changed period of `{name}` to {new:?}.");
        }
        Ok(())
    }
    fn config(&mut self) -> Result<()> {
        // TODO: this should get a nicer interface
        let config_string = serde_json::to_string_pretty(&self.config.user_config).unwrap();
        let new_config_string = utils::long_edit(Some(&config_string))?;
        let new_config: UserConfig = serde_json::from_str(&new_config_string)?;
        self.config.user_config = new_config;
        Ok(())
    }
}
