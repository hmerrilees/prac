//! # The feedback-oriented utility for a practice-oriented life.
//!
//! # UI demo + TLDR
//! Let's say we'd like to set a new practice of making a weekly repo, well... every week.
//! ```bash
//! prac add "weekly repo" 1week
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
//! When I'm done, I'll ```prac log steno 30minutes``` to reset the bar and track time, and ```prac notes steno``` to make some notes with `$EDITOR` on my progress.
//!
//! (tip: use `prac list --cumulative` to see cumulative time logged, are we 10000h yet?)
//!
//! Be sure to explore `prac help` and `prac help <subcommand>` for more.
//!
//! ## A note on time...
//! In lieu of knowing better, I wrote a little duration parser (an approximate superset of systemd.time).
//! Whenever you see a duration/time argument, you can input time as follows:
//! ```text
//! 1day
//! 2days        # plural is fine
//! 3days15hours # combined quantities
//! 1w4d         # abbreviations
//! 4M           # just be careful... M is month, m is minute
//! ```
//! Intermediate whitespace is permessible, but you still need quotes in the cli so as to be
//! captured as a single argument.
//! ```text
//! "1Y 2M 3w 4d 5h 6m 7s"
//! "1         day"
//! "1day 30 min"
//! ```
//! There are many ways to write the same unit, all the following are equivalent.
//! ```text
//! 2seconds
//! 2second
//! 2sec
//! 2s
//! ```
//! See [src/time/time.pest](https://github.com/henry-merrilees/prac/blob/main/src/time/time.pest) for the complete grammar.
//! Errors are decent enough to help you if you get stuck.
//!
//! # Motivation, problems, and a possible solution
//!
//! ## Motivation
//! Developing skill takes time + structure. `prac` attempts to promote both while being as lightweight as possible.
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
//! Not much, and that's on purpose, but in service of the above, prac has a few distinguishing
//! design decisions:
//! - Rather than "events" being triggered by the clock/calendar, which are not privileged to
//! user's psychological state, the prac lifecycle starts when the user gets stuck in their current task
//!    or otherwise decides it's time to do something new. This avoids flow-breaking interruptions
//!    while promoting mindfulness as an active part of the user's feedback loop.
//! - Rather than on a scheduled (absolute) interval, items run on (relative) time elapsed since prior log. E.g. a
//! daily task period begins when you log it, and ends within 24 hours (plus a grace period as
//! specified in `prac config` to prevent forward creep).
//!  There is no scheduling to displace user agency, elapsed time since last log is displayed
//! as a fraction of the period set for each practice. This information can be incorporated into the final decision entirely at the user's discretion.
//! - Tracking is dead-simple, intentionally adding no functionality that is not possible with pen
//! and paper. Time is tracked is a sum total of self-reported increments. Logging is done in plain-text.
//!
//! - How you wish to use prac in a larger context is up to you. For practices that demand deeper
//! focus, rather than worrying about them all the time, consider blocking off
//! a regular 2-4 hour period in which you get settled, turn off all distractions hook
//! in. Personally, I do four blocks of one hour with 15 minute breaks between, one practice per
//! block. In each blocks I can stop early but can't go back--somewhere between an standardized
//! test and a "reverse pomodoro."
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
//! ### Why a cli?
//! - It's lightweight, harder to implement needless overhead.
//! - It's generally distraction free... no notifications, no default swiping from app to app. You
//! have to be sitting down at a computer to use it, etc. etc.
//! - I spend most of my time in the terminal anyway.
//! - In the long run, it should become easier to integrate into other workflows. I have `prac
//! list` in my shell prompt.
#![warn(
    clippy::all,
    clippy::pedantic,
    clippy::nursery,
    clippy::style,
    clippy::complexity,
    clippy::perf,
    clippy::style
)]

mod application;
mod cli;
mod time;
mod utils;

use anyhow::{Context, Result};
use application::{State, StateExt};
use clap::Parser;
use cli::{Cli, SubCommand};
use std::io::BufWriter;
use std::path::Path;

fn process_subcommand(state: &mut State, subcommand: SubCommand, state_path: &Path) -> Result<()> {
    match subcommand {
        SubCommand::List {
            cumulative,
            period,
            danger,
        } => state.list(cumulative, period, danger)?,
        SubCommand::Add {
            name,
            period,
            notes: add_notes,
        } => state.add(name, period, add_notes)?,
        SubCommand::Log {
            name,
            time,
            notes: add_notes,
            no_reset,
        } => state.log(name, time, add_notes, no_reset)?,
        SubCommand::Notes { name } => state.notes(name)?,
        SubCommand::Reset => state.reset(),
        SubCommand::StateLocation => {
            println!("state path: `{}`", state_path.display());
        }
        SubCommand::EditPeriod { name, period } => state.edit_period(name, period)?,
        SubCommand::Remove { name } => state.remove(name)?,
        SubCommand::Rename { current_name } => state.rename(current_name)?,
        SubCommand::Config => state.config()?,
    };
    Ok(())
}

fn main() -> Result<()> {
    let state_path = State::get_path()?;

    let mut state = if state_path.exists() {
        serde_json::from_str(
            &std::fs::read_to_string(&state_path).context("could not read statefile")?,
        )
        .context("failed to parse state")?
    } else {
        State::new()
    };

    let cli = Cli::parse();

    process_subcommand(&mut state, cli.command, &state_path)?;

    let state_file = std::fs::File::create(state_path).context("failed to create state file")?;
    state.update_version();
    serde_json::to_writer_pretty(BufWriter::new(state_file), &state)
        .context("failed to write state to file")?;
    Ok(())
}
