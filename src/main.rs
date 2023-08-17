//! # a utility for a feedback-cycle conducive to immersive practice in a busy world
//! 
//! ## Installation
//! install rust, clone repo, then
//! ```
//! cargo install --path <path to repo>
//! prac help
//! ```
//! If this doesn't work, you may have yet to add `~/.cargo/bin` to your path.
//! 
//! I will try to get up on crates.io soon, want to make a little more headway on state version compat, and possibly move the more incendiary dogma from the crate docs to my website (which I have yet to host).
//! 
//! //! ## UI demo + TLDR
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
//! When I'm done, I'll ```prac track steno 2 hours``` to reset the bar and track time, and ```prac log steno``` to make some notes w/ $EDITOR on my progress.
//!
//!
//! In a state of immersion, time is experienced. In productivity systems, time is controlled--better had than spent, better
//! spent than lost. What appears externally to be "lost" time is in fact the negative space prerquisite to immersive states.
//! We have no business interrupting these states ourselves, but it remains that our immersive drive will eventually outlast 
//! the focus it can find for any particular task--we get stuck. We require a mechanism to smoothly carry our momentum into the next state of practice.
//! We need a subtle cue to reintroduce temporal awareness of our other practices
//! in a manner not so jarring that it will rip us from our direct experience of the present moment.
//!
//! In contrast with other tools, in `prac`, the clock has no authority you do not yourself solicit. 
//! It appears only as an intentionally simple, gentle visual indicator, leaning more on our [Graphical Perception](https://www.jstor.org/stable/2288400) than our numerically-obsessed scheduling proclivities.
//! The progress bars for each item display time elapsed since last participation as a fraction the period in which we intend to practice.
//! How you wish to systematize this is up to you, personally, I wouldn't even think about it. 
//! Just look at it when you get stuck and need something else to do.
//!
//! If life happens, just `prac reset` to wipe the bars, and start again! It's no big deal.
//!
//! ## What this isn't
//! This is not a todo list, a calendar, pomodoro timer, a project manager, a scheduling app, or a habit tracker.
//!
//! Design decisions have been made on ideological grounds to intentionally maximize incompatibility with these types of apps.
//! Neither is this a compliance mechanism, if you wish to Pavlov yourself into a life well lived,
//! that is your prerogative, but I will not be helping you.
//!
//! I have attempted to make it as difficult as possible to use this tool to "make a project of
//! oneself."
//!
//! ## What this is
//! A tool to augment a practice-driven life/workflow,
//! specifically providing feedback to enable those practices
//! which cannot all be done daily, or the efficacy of which is highly sensitive to factors
//! knowable only in the moment.
//!
//! That REM sleep lifts us gracefully through from one phase of deep sleep to the next at an
//! average period of ~90 minutes does not make possible reproduction of the effect by way of 
//! a 90-minute alarm clock. This is the motivating philosophy of `prac`. 
//!
//! If you want, you can get started right now with `prac help`. If you want to see how I
//! attempted to integrate the dream theory of flow state, read on to the minifesto.
//!
//! ## Minifesto and user guide
//! Begin by negatively scheduling, making time with guarantee only that you will not permit yourself to be otherwise scheduled or interrupted (including by phones/notifications).
//!
//! After having `prac add`-ed a few practices, you can `prac list` and choose one to _start_ on (how you choose is none of my
//! business).
//!
//! This feedback loop orients on _starts_ rather than completions, banking on that (canonically) self-promising 
//! to make it through the gym doors will be more successful to motivate a workout than a list of the entailed exercises. 
//! Output, sharing, and interpersonal feedback are no less important than starts but happen to inhabit another segment of the loop, whereas this 
//! tool limits its scope to the "refocusing" stage.
//!
//! In a conventional productivity system, interrupts are triggered externally, by calendar
//! notifications, timers, due dates, etc. There are few ways in which these systems could be less accountable
//! to the psychological state of the user in the relevant moment. The discipline of producing
//! conventional productivity tools might as well be called "distraction engineering."
//!
//! Is genuine feedback even possible under a system where all the decisions are made ahead of
//! time? When we are most efficient, it might not appear to us that we are making a decision at
//! all, whether to continue or stop. Clearly then the problem is not implementation but orientation. 
//!
//! It's not about when you work, not everything is going to get done, you have X hours no matter what (and should probably limit yourself to even fewer), but more about when you switch between tasks.
//!
//! In a mode of practice, control is not exerted by the clock, but follows naturally from the
//! persons instincts of relative flow and stuckness. When you get stuck, rather than banging your head until your pomodoro takes pity on your soul,
//! you simply `prac list` to see a handful of practices with a progress bar showing how long
//! it's been since you last practiced as a fraction of how frequently you wish to practice. This
//! provides a very gentle way prioritize those practices that have been recently neglected.
//!
//! For those who struggle with work-life balance, these tasks can include things like rest, play, socializing, eating, outdoor time, family time, and other practices of self-care.
//!
//! This is a feedback tool, not a compliance tool. It's not a big deal if you let a bar run over, take it as a signal that the relavent period needs extending.
//! If you find yourself regularly finishing early, you've identified that you would benefit from a shorter feedback cycle!
//!
//! I firmly believe that time and quality of practice are responsible for the bulk of actually-existing competence, and so
//! I've implemented only two tracking features, `proc trac` for a bare total of time, and `prac log` for plain text goal-setting and reflection.
//!
//! ### Can I have x feature to track something that I could just as easily track in the plain text notes?
//! no.
//!
//! ### A rant on self-scheduling
//! Ideologically, I despise self-scheduling. Spontaneity is in all things beautiful.
//! Forgetfulness is spontaneity in the negative, no less an exercise of freedom. 
//! Without scheduling we would have much less to forget, and for that I respect scheduling. 
//! However, I have zero respect for self-scheduling. "Sorry, I can't [be a normal fun person]," says the self-scheduler, "I have to do this thing that nobody told me I had to and that I don't even want to do myself." 
//! Neither is there spontaneous beauty in forgetting self-orders--you are back where you started except now also a failure. 
//! When I "succeed" in perfectly following my elaborate self-scheduling, it means that I accomplished something so mundane that I had already totally understood it before I even began. 
//!
//! ### Incedental functionality
//! I used clock periods instead of calendar periods to eliminate the incentive to start at the very beginning of the block (i.e. scheduling).
//! Scheduling is not only is somewhat life-denying, also works against the resiliency of your system, as every moment is an opportunity to fail.
//! It also fails to tolerate either end of a poorly selected period.
//! - If the period is too short, iterations form a backlog, and in a state of overwhelm, it is easier to give up on the system.
//! - If the perriod is too long, Having completed an iteration early in the calendar period, one gets the sense that they should
//! wait until the next period to start again, artificially drawing out the period, making it very
//! easy to fall out of habit and accidentally drop the system.
//!
//! When you have a daily todo list that also includes practices, it becomes basically impossible
//! not to plan. Without scheduling is no mechanism to naturally encourage routine engagement in practices at
//! the same within a calendar period.
//!
//! If all your daily tasks are clocked 24 (+ 2hr grace) periods, you are steered naturally towards doing
//! things more or less at the same time you did them last. When you can keep up, your bar reminds
//! you not to do something too late, and you know that if you do it too early you will be met with
//! an early bar tomorrow too. If you can't keep up, it's no big deal, tomorrow's practice window will be
//! adjusted automatically to when you were able to practice today.
//!
//! As a bottom-up feedback utility, and not a top-down compliance regieme, keeping
//! with it becomes so much easier.
//!
//! ### Inspiration
//!
//! The initial name was "toDoom" as the interface was
//! inspired by [The World's Most Dangerous Writing
//! App](https://www.squibler.io/dangerous-writing-prompt-app/write?limit=5&type=minutes), and I
//! intentionally hadn't handled progress bar overflow, resulting in a crash and arbitrary data loss. 
//!
//!
//! ## State management warning
//!
//! I would HIGHLY recommend backing up your state file (@`prac state-location`).
//! State management is so far from stabilized, backwards compatibility is in no way guaranteed.
//! As far as I'm concerned, your data may be lost at any time.

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
    /// List practices, with a progress bar showing time elapsed through period since last practice. See `help list` for options.
    List {
        /// Show cumulative hours tracked alongside practices.
        #[arg(short, long)]
        cumulative: bool,
        /// Show period of practice alongside practices
        #[arg(short, long)]
        period: bool,
    },
    /// Add a new practice. See `help add` for usage.
    Add {
        /// A (unique) name for the practice.
        name: String,
        /// Some notes to outline goals and log progress. Opens $EDITOR if not provided.
        #[arg(short, long)]
        notes: Option<String>,
        /// Anticipated time period between practice sessions. (There is a 2 hr grace period by default.)
        period: u64,
        #[arg(value_enum)]
        time_unit: TimeUnit,
    },
    /// Mark a practice completion, tracking time spent. See `help track` for usage.
    Track {
        /// Specify, or leave blank to fuzzy search.
        name: Option<String>,
        /// How long you practiced for. This is added to the cumulative time displayed in `prac list --cumulative`.
        time: u64,
        #[arg(value_enum)]
        time_unit: TimeUnit,
        /// An optional shortcut to `prac log` when you're done.
        #[arg(short, long)]
        log: bool,
    },
    /// Edit practice notes w/ your $EDITOR. Each practice has its own log.
    Log {
        /// Specify practice to edit, or leave blank to fuzzy search.
        name: Option<String>,
    },
    /// Edit practice period.
    #[command(alias = "ep")]
    EditPeriod {
        /// Specify, or leave blank to fuzzy search.
        name: Option<String>,
        /// Anticipated time period between practice sessions. (There is a 2 hr grace period by default.)
        period: u64,
        #[arg(value_enum)]
        time_unit: TimeUnit,
    },
    /// Delete a practice.
    Remove {
        /// Specify name of practice to remove, or leave blank to fuzzy search.
        name: Option<String>,
    },
    /// Rename practice.
    Rename {
        current_name: String,
        new_name: String,
    },
    /// Reset progress bars. Equivalent to tracking all practices w/ zero time.
    Reset,
    /// Show state file location.
    ///
    /// State is stored in $PRACTICE_PATH, $PRACTICE_HOME/prac.json, [dirs::data_dir]/prac/prac.json 
    /// or [dirs::home_dir]/.prac.json, searched in that order.
    ///
    /// It's a good idea to vcs your state file.
    StateLocation,
}


fn get_state_path() -> Result<PathBuf> {
    if let Ok(practice_path) = var("PRACTICE_PATH") {
        let path = PathBuf::from(practice_path);
        Ok(path)
    } else if let Ok(practice_home) = var("PRACTICE_HOME") {
        let practice_home = PathBuf::from(practice_home);
        std::fs::create_dir_all(&practice_home).context("$PRACTICE_HOME specified but could not be created.")?;
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
fn main() -> Result<()> {
    let state_path = get_state_path()?;

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
                        Some("Welcome! Here you can set some clear goals. You can log your progress and view this page later in your $EDITOR with `prac log`.".to_string())
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
        SubCommand::Track {
            name,
            time,
            time_unit: unit,
            log,
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
                println!("Good job! It's a good idea to make notes on your progress with `prac log`.");
            }
        }
        SubCommand::Log { name } => {
            let mut find = state.find(name.as_deref())?;
            let practice = find.get_mut();
            let body = utils::long_edit(Some(practice.notes.clone()))?;
            practice.notes = body;
        }
        SubCommand::Remove { name } => {
            let practice = state.find(name.as_deref())?;
            practice.remove();
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
                 .max(30); // TODO magic number
             
            let term_width = termsize::get().context("failed to obtain termsize")?.cols;

            println!();
            let now = SystemTime::now();
            for (name, practice) in &state.routines {
                let mut truncated_name = name.clone();
                truncated_name.truncate(max_name_len); // better way to do this?

                let start_message = format!(" {truncated_name:>max_name_len$} ");

                let hours_cumulative = practice.cumulative.as_secs_f64() / 3600.0_f64;
                let hours_period = practice.period.as_secs_f64() / 3600.0;

                let end_message = match (cumulative, period) {
                    (true, true) => format!(
                        " {:>4}h c  {:>4}h p ",
                        hours_cumulative as u64, hours_period as u64
                    ),
                    (true, false) => format!(" {:>4}h cumulative ", hours_cumulative as u64),
                    (false, true) => format!(" {:>4}h period ", hours_period as u64),
                    (false, false) => String::new(),
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
            let mut find = state.find(name.as_deref())?;
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
