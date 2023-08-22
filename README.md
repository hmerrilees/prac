# prac
<!-- cargo-rdme start -->

## The feedback-oriented utility for a practice-oriented life.

## UI demo + TLDR
Let's say we'd like to set a new practice of making a weekly repo, well... every week.
```bash
prac add "weekly repo" 1week
```
Now, we can view "weekly repo" alongside all our practices.
```bash
prac list
```
```text
distributed systems programming ▬▬▬
                      daily log ▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬
                       exercise ▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬
                    kierkegaard ▬▬▬▬▬▬▬
                          steno ▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬
                    weekly repo
```
> As time elapses through the weekly repo's period, the bar will fill like the rest.

Looks like I haven't done steno in a while... when I get stuck with whatever I'm doing, I'll switch to that.

When I'm done, I'll ```prac log steno 30minutes``` to reset the bar and track time, and ```prac notes steno``` to make some notes with `$EDITOR` on my progress.

(tip: use `prac list --cumulative` to see cumulative hours logged, are we 10000 yet?)

Be sure to explore `prac help` and `prac help <subcommand>` for more.

### A note on time...
Whenever you see a time argument, you can use a systemd.like time string, ex. as follows:
show you some examples
```text
1day
2days        # plural is fine
3days15hours # combined quantities
1w4d         # abbreviations
4M           # just be careful... M is month, m is minute
```
Intermediate whitespace is permissible if whole string is surrounded in quotes so the cli can capture as single arg.
```text
"1Y 2M 3w 4d 5h 6m 7s"
"1         day"
"1day 30 min"
```
There are many ways to write the same unit, all the following are equivalent.
```text
2seconds
2second
2sec
2s
```
See [src/time/time.pest](https://github.com/henry-merrilees/prac/blob/main/src/time/time.pest) for the complete grammar.
Errors are decent enough to help you if you get stuck.

## Motivation, problems, and a possible solution

### Motivation
Developing skill takes time + structure. prac attempts to promote both while being as lightweight as possible.


### Solving the right problems
To remain lightweight, prac sticks only to problems that (to me) most obviously need solving.

Primarily,
- "What should I do now?" in instances where pre-planning is inadviseable or impossible,
- losing track of practices I haven't done in a while, and
- progress/time tracking without excessive overhead or breaking flow.

### What's so special about prac?
Not much, and that's on purpose, but in service of the above, proc has a few distinguishing
design decisions:
- Rather than "events" being triggered by the clock/calendar, which are not privileged to
user's psychological state, the proc lifecycle starts when the user gets stuck in their current task
   or otherwise decides it's time to do something new. This avoids flow-breaking interruptions
   while promoting mindfulness as an active part of the user's feedback loop.
- Rather than on a scheduled interval, items run on time elapsed since prior log. E.g. a
daily task period begins when you log it, and ends within 24 hours (plus a default 2-hr grace period).
 There is no scheduling to displace user agency, elapsed time since last log is displayed
as a fraction of the period set for each practice. This information can be incorporated into the final decision entirely at the user's discretion.
- Tracking is dead-simple, intentionally adding no functionality that is not possible with pen
and paper. Time is tracked is a sum total of self-reported increments. Logging is done in plain-text.

#### More benefits of elapsed-time periods
- Scheduled/calendar intervals are intolerant to period drift either way. If you finish too
late (i.e. need a longer feedback cycle), you find yourself having to work more quickly to
catch up on the accumulated iterations. If you finish too early (i.e. need shorter feedback
cycle), you have to wait even longer until the next scheduled event.
- With elapsed-time periods, an overrun is no big deal, nothing stacks up, just log it when you
get to it and you'll start again with a full period.
- You also are not "penalized" for overachieving / finishing early... just make sure you are working at a
pace sustainable to finish within the next period which you have just moved forward.
- If you find yourself regularly finishing very early/late, no big deal! Just take it as a sign
that you need to adjust the period of your feedback cycle!

<!-- cargo-rdme end -->
## Installation
First, [install rust/cargo](https://www.rust-lang.org/tools/install).

For crates.io version:
```
cargo install prac
```
For latest version:
```
git clone https://github.com/henry-merrilees/prac.git
cargo install --path prac
```
If `cargo install` doesn't work, you may have yet to add `~/.cargo/bin` to your path.
