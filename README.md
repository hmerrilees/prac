# prac
## Installation
Install rust...

For crates.io version...
```
cargo install prac
```
For latest version...
```
git clone https://github.com/henry-merrilees/prac.git
cargo install --path prac
```
If this doesn't work, you may have yet to add `~/.cargo/bin` to your path.
<!-- cargo-rdme start -->

### The feedback-oriented utility for a practice-oriented life.

### UI demo + TLDR

```bash
prac list
```
```text
distributed systems programming ▬▬▬
                      daily log ▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬
                       exercise ▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬
                    kierkegaard ▬▬▬▬▬▬▬
                          steno ▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬
                    weekly repo ▬▬▬

(tip: use `prac list --cumulative` to see cumulative hours tracked, are we 10000 yet?)
```
> Looks like I haven't done steno in a while... when I get stuck, I'll switch to that.

When I'm done, I'll ```prac log steno 2 hours``` to reset the bar and track time, and ```prac notes steno``` to make some notes w/ $EDITOR on my progress.

### Motivation, problem, and solution(?)

#### Motivation
Developing skill takes time + structure. prac attempts to promote both while being as lightweight as possible.


#### Solving the right problems.
To remain lightweight, prac sticks only to problems that (to me) most obviously need solving:
- "What should I do now?" in instances where pre-planning is inadviseable or impossible,
- losing track of practices I haven't done in a while, and
- progress and time tracking without excessive overhead or breaking flow.

#### What's so special about prac?
Not much, and that's on purpose, but there are a few key differences:
- Rather than "events" being triggered by the clock/calendar, which are not privileged to your
   psychological state, the proc lifecycle starts when the user gets stuck in their current task 
   or otherwise decides it's time to do something new. This avoids flow-breaking interruptions 
   while encoraging the user to become more in tune with their own needs and psychological rhythms.
- Rather than on a scheduled interval, items run on time elapsed since prior log. E.g. a
daily task period begins when you log it, and ends within 24 hours (plus a default 2-hr grace period).
 Time does not displace your agency, rather time-since-last-log for each practice is displayed
as a fraction of the period set for each. This information can be incorporated into the final decision entirely on the users terms. 
- Tracking is dead-simple, intentionally adding no functionality that is not possible with pen
and paper. Time is tracked is a sum total of self-reported increments. Logging is done in plain-text.

##### More benefits of elapsed-time periods
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
