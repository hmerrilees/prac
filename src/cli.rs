//! How to use value hints and generate shell completions.
//!
//! Usage with zsh:
//! ```console
//! $ cargo run --example completion-derive -- --generate=zsh > /usr/local/share/zsh/site-functions/_completion_derive
//! $ compinit
//! $ ./target/debug/examples/completion_derive --<TAB>
//! ```
//! fish:
//! ```console
//! $ cargo run --example completion-derive -- --generate=fish > completion_derive.fish
//! $ . ./completion_derive.fish
//! $ ./target/debug/examples/completion_derive --<TAB>
//! ```
use super::time::parse_time_span;
use chrono::Duration;
use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author = "Henry Merrilees")]
#[command(version = env!("CARGO_PKG_VERSION"))]
#[command(about = env!("CARGO_PKG_DESCRIPTION"))]
pub struct Cli {
    #[command(subcommand)]
    pub(super) command: SubCommand,
}

// TODO: config edit command
#[derive(Subcommand, Debug)]
pub enum SubCommand {
    /// List practices w/ progress bars showing time elapsed through period. `help list` for options
    #[command(
        after_long_help = "-p -c together will display both, but each truncated to the largest unit"
    )]
    List {
        /// Show cumulative hours tracked alongside practices.
        #[arg(short, long)]
        cumulative: bool,
        /// Show period of practice alongside practices
        #[arg(short, long)]
        period: bool,
    },
    /// Add a new practice: `help add <name> <period>`.
    Add {
        /// A (unique) name for the practice.
        name: String,
        /// Anticipated time period between practice sessions
        #[arg(value_parser = parse_time_span)]
        period: Duration,
        /// Shortcut to `prac notes` to set goals
        #[arg(short, long)]
        notes: bool,
    },
    /// After you practice, `prac log <name> <time>` to reset the bar and track time.
    Log {
        /// Specify, or leave blank to fuzzy search.
        name: Option<String>,
        /// How long you practiced for. This is added to the cumulative time displayed in `prac list --cumulative`.
        #[arg(value_parser = parse_time_span)]
        time: Duration,
        /// An optional shortcut to `prac notes` when you're done.
        #[arg(short, long)]
        notes: bool,
    },
    /// Edit practice notes. Each practice has its own notes file.
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
        /// Anticipated time period between practice sessions.
        #[arg(value_parser = parse_time_span)]
        period: Duration,
    },
    Remove {
        /// Specify name of practice to remove, or leave blank to fuzzy search.
        name: Option<String>,
    },
    Rename {
        /// Current (old) name of practice.
        current_name: Option<String>,
    },
    /// Edit configuration. `help config` for info on fields.
    #[command(after_help = "\
        Grace period pads the end of the bars of `prac list` with some extra time to give you a little \
        flexibility and prevent tasks from creeping earlier on each iteration.")]
    Config,
}