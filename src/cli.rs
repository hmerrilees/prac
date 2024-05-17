use std::path::PathBuf;

use super::time::parse_time_span;
use chrono::Duration;
use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author = "Henry Merrilees")]
#[command(version = env!("CARGO_PKG_VERSION"))]
#[command(about = env!("CARGO_PKG_DESCRIPTION"))]
pub struct Cli {
    /// Absolute path to state file. If not provided, will search for state file in common locations.
    ///
    /// This is useful if you want to track multiple practice sets. I.e. lets say you want to keep
    /// up with your friends.
    /// alias friends='prac --path=/path/to/friends_state'
    #[arg(long, env = "PRAC_PATH")]
    pub(super) path: Option<PathBuf>,
    #[command(subcommand)]
    pub(super) command: SubCommand,
}

#[derive(Subcommand, Debug)]
pub enum SubCommand {
    /// List practices w/ progress bars showing time elapsed through period. `help list` for options
    #[command(
        after_long_help = "-p -c together will display both, but each truncated to the largest unit",
        alias = "ls"
    )]
    List {
        /// Show cumulative hours tracked alongside practices.
        #[arg(short, long)]
        cumulative: bool,
        /// Show period of practice alongside practices
        #[arg(short, long)]
        period: bool,
        /// Show "danger bar" that dissplays sum progression through periods.
        #[arg(short, long, default_value = "false")]
        danger: bool,
    },
    /// Add a new practice.
    Add {
        /// A (unique) name for the practice.
        #[arg(required_unless_present = "interactive")]
        name: Option<String>,
        /// Anticipated time period between practice sessions (as systemd.time-like time span).
        #[arg(value_parser = parse_time_span, required_unless_present = "interactive")]
        period: Option<Duration>,
        /// Interactive
        #[arg(short, long, default_value = "false")]
        interactive: bool,
    },
    /// Begin a session of participation in a practice (ctrl-c to end early). Will log time participated automatically.
    Session {
        /// Specify practice to log, or leave blank to fuzzy search.
        #[arg(required_unless_present = "interactive")]
        name: Option<String>,
        /// Time practiced, as systemd.time-like time span.
        #[arg(value_parser = parse_time_span, requires = "name", required_unless_present = "interactive")]
        max_time: Option<Duration>,
        /// Interactive
        #[arg(short, long, default_value = "false")]
        interactive: bool,
    },
    /// Log time participated in a practice outside a session
    Log {
        /// Specify practice to log, or leave blank to fuzzy search.
        #[arg(required_unless_present = "interactive")]
        name: Option<String>,
        /// Time practiced, as systemd.time-like time span.
        #[arg(value_parser = parse_time_span, requires = "name", required_unless_present = "interactive")]
        time: Option<Duration>,
        /// Interactive
        #[arg(short, long, default_value = "false")]
        interactive: bool,
    },
    /// Edit practice notes in your $EDITOR.
    /// If you don't know vi or have your editor set otherwise, it's probably wise to leave this alone.
    Notes {
        /// Specify practice to edit, or leave blank to fuzzy search.
        #[arg(required_unless_present = "interactive")]
        name: Option<String>,
        #[arg(required_unless_present = "interactive")]
        new_notes: Option<String>,
        /// Interactive
        #[arg(short, long, default_value = "false")]
        interactive: bool,
    },
    /// Reset all progress bars if you fall behind.
    /// Equivalent to tracking all practices w/ zero time.
    Reset,
    /// Show state file location. `help state-location` for more info.
    ///
    /// State is stored in $PRAC_PATH, [dirs::data_dir]/prac/prac.json
    /// or [dirs::home_dir]/.prac.json, searched in that order.
    ///
    /// It's a good idea to vcs your state file.
    StateLocation,
    /// Edit period of practice.
    #[command(alias = "ep")]
    EditPeriod {
        /// Specify name of practice whose period to edit
        #[arg(required_unless_present = "interactive")]
        name: Option<String>,
        /// Anticipated time period between practice sessions.
        #[arg(value_parser = parse_time_span, required_unless_present = "interactive")]
        period: Option<Duration>,
        /// Interactive
        #[arg(short, long, default_value = "false")]
        interactive: bool,
    },
    Remove {
        /// Specify name of practice to remove, or leave blank to fuzzy search.
        #[arg(required_unless_present = "interactive")]
        name: Option<String>,
        /// Interactive
        #[arg(short, long, default_value = "false")]
        interactive: bool,
    },
    /// Rename a practice.
    Rename {
        /// Current (old) name of practice.
        #[arg(required_unless_present = "interactive")]
        current_name: Option<String>,
        /// New name of practice.
        #[arg(required_unless_present = "interactive")]
        new_name: Option<String>,
        /// Interactive
        #[arg(short, long, default_value = "false")]
        interactive: bool,
    },
    /// Edit configuration. `help config` for info on fields.
    #[command(after_long_help = "\
        Grace period pads the end of the bars of `prac list` with some extra time to give you a little \
        flexibility and prevent tasks from creeping earlier on each iteration.\n\n\
        ")]
    Config {
        /// Grace period
        #[arg(short, long, value_parser = parse_time_span, required_unless_present = "interactive")]
        grace_period: Option<Duration>,
        /// Interactive
        #[arg(short, long, default_value = "false")]
        interactive: bool,
    },
}
