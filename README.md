# prac
## Install
First, get cargo, then install prac.
```
curl https://sh.rustup.rs -sSf | sh
cargo install prac
```


## The feedback-oriented utility for a practice-oriented life.

## TL:DR;
Awww, the minifesto is not *that* insufferable, I assure you! But if you're feeling lazy,
just run `prac help`, and `prac help <subcommand>`. The help is fairly complete. Applicable
subcommands have `-i` flag for interactive mode, highly recommended.

I'd start with `prac add -i` (interactive), then `prac session -i` to start a practice session, then `prac list` to pick what to do next.
Just remember, you are specifying a time period, not duration--this is the difference between running
a 30-minute 5k once a month and running a 5k every 30 minutes. I know which I'd prefer.

Most of the utility of this tool is not in the functionality but in the approach, so I would
recommend reading on.

## Minifesto
#### problem
You have high-level values which should materialize in certain regular practices that
reflect your life-intentions. Modern life is busy, and given that we have well-armed trivial things
with calendars, todo lists, pomodoro timers, etc. to conquer our natural motivations,
it is no surprise that such things regularly displace our practices, and in doing so, our values.

We need a system to set our life-practices on equal footing with our extrinsically-motivated
activity, even if only to give us in the business of life a conventent excuse to abide by our values.
We need a tool to save us from our tools.

#### existing (un)solutions
Existing productivity tools are actively hostile to the uninterruptabile nature of valuable practice.
We have come to accept a certain shallowness in our work. We tolerate the disruptive
mechanisms of our productivity tools as it is not clear exactly what of value they are
disrupting. Our life-practices are not advantaged by unimportance in this way.

Additionally, humans have not always required these tools. There existed ritual kindness long before the calendar.
When supplementing our practices with technology, we should ensure that every aspect is
genuinely additive. Knowing what focus has been lost to notifications, our system should only
look to the clock when it is clear that it has something of value to say. We know far
better ourselves when it is time to move on.

#### method

Before doing anything else, first you need to add practices, with what else but `prac add'. I would
recommend doing this interactively with `prac add -i'

```rust
 ♥  prac add -i
  What would you like to practice?: discrete mathematics (algorithms)
  How often (not how long) would you like to practice "discrete mathematics (algorithms)?": 1week 3days
```

W/r/t practice, your calendar has one useful purpose--to fight fire with fire. To stop yourself
and others from scheduling over your values, you can block our explicit time to practice. You
shut off all your non-emergency notifications. Now what?

Since scheduling technology has done so much to put distance between ourselves and the last
occurences of our practice, the least the clock can do is tell us how long it's been since
each. This is `prac list`.

Numbers are of limited use for time except when scheduling, and humans are really bad with them
anyways, so `prac list` displays time elapsed only as a visual cue--as a fraction of the period we have
specified as how often we wish to participate in the activity.
```bash
♥  prac list
```
```
          communicate gratitude ▬▬▬▬▬▬▬▬▬▬▬
distributed systems programming ▬
                      daily log ▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬
                       exercise ▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬
                    kierkegaard ▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬
                          steno ▬▬▬▬▬
```
Nice, we're on top of distributed systems programming! However, it looks like we haven't done steno in a while, maybe we should start with that.

> Hint: configure your shell config to `prac list` on first prompt to be reminded of your priorities!

To begin a session of a particular task, we use `prac session...`. I recommended `prac session -i` for interactive mode. We also should specify how long we'd like to practice for.
```bash
♥  prac session steno 2h
```
Prac will then keep an eye on how long it's been. Optionally, using a terminal with job-completion
notifications to ping you after the desired participation duration has elapsed.



You keep going either until the end of the specified duration or you get stuck--whichever
comes first, there's no point stressing. Simply `prac list` again
```
          communicate gratitude ▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬
distributed systems programming ▬▬▬▬
                      daily log ▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬
                       exercise ▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬
                    kierkegaard
                          steno ▬▬▬▬▬▬▬▬
```

We see that the bars progress with time, and also that `kierkegaard` has conveniently reset, marking
our participation. We see that exercise is the furthest away, but maybe Keirkegaard has
inspired us with gratitude, so we will communicate gratitude next. The point is that `prac`
presents *you* with information that you wouldn't otherwise have, so that *you* may make the
best decision on your own accord. If I wanted to tell you simply to do the most "overdue" task, I
would have done it. That interface would have been a lot easier.

> Hint: consider, in addition to longer blocks, scheduling a dedicated "prac storm" of 1-2
hours, in which practices are attempted only if they may be reasonably kept under 10-15 minutes.
Then hold yourself to it.

If you are a reflective person, `prac list --cumulative` also shows you how much time you have
given total to each item, enabling you to take pride in your work, and to adjust your
priorities to best suit your values.

If you find the bar of a task regularly running to the end of the window or that it never makes it
close, you can adjust the period with `prac edit-period -i` (interactive).

The `-d` flag on `prac list -d` adds a "danger bar" which is a weighted sum display of all
practice periods. You should consider set your periods to where it is achievable to keep the
danger bar under halfway-full.

If you are comfortable using a terminal editor, you should record goals, progress, and whatever
else with `prac notes`. This opens ``$EDITOR``, which often defaults to vi. If this is
all unfamiliar to you, it's probably best to leave this command alone.

## Design
### Why time periods? Why not absolute calendar windows within which the activity could be freely participated?

Absolute windows of time are ideal for establishing a set mean completion interval (as one
would hope for when administering medication). Rather than independently specifying a schedule and
forcing our life to conform to the gaps, we are more concerned with *doing* what it is we set
out to do (with minimally-invasive guardrails), and merely observing what timing falls into place.

With absolute calendar periods, the next participation doesn't become any easier because the
last was done later, despite that we will have less time to do it. For that which we are particularly
excited, we shouldn't have to wait for the next window either. If we missed last period, should we make it up?
All these considerations make it very easy to break with and give up on a system based on
absolute calendar periods.

A period beginning from last participation is natural for our purpose, as the last participation is the event to which the next will be most sensitive.
It's okay to regularly be ahead of "schedule," and even to be occasionally behind.
*That we continually participate early or late in the period is only a indication that we should edit the period accordingly.*

If you get way behind on everything, no need to give up, just `prac reset` to start again with
a clean slate. Prac is intentionally designed to avoid any derailing events.

Nominally an alternative to routine-scheduling systems, *prac is secretly a routine-discovery system.*
Daily practices will near the end of their period at a similar time as they were completed the
previous day, motivating users naturally to fall into a rhythm. The same applies on any other
calendar-aligned period. Additionally, prac has a configurable grace period to enable emergent routines
not to creep earlier and earlier.

Finally, having windows at all entails scheduling. At a macro level, this is not necessarily
harmful so long as you never make any impossible schedule. I am of the belief that daily,
weekly, quarterly, etc. organizational allotments are better spent reflecting than planning,
but the overhead seems to be manageable (if not enjoyable) for some. However, with the "aid" of software,
you can subject yourself daily to an procedurally-generated list of tasks with complexity far in
excess of that which you could ever manage to produce in your head, or even with pen and paper.
To fit them all in, your only option is micro-scheduling the day, which both requires interrupts and
is completely insensitive to what activity one feels in the moment most suited to practice intently.
An impossible schedule is virtually inevitable. Enabling this would defeat the entire purpose of prac.

### Period syntax
In lieu of knowing better, I wrote a little duration parser (an approximate superset of systemd.time).
Whenever you see a duration/time argument, you can input time as follows:
```
1day
2days        # plural is fine
3days15hours # combined quantities
1w4d         # abbreviations
4M           # just be careful... M is month, m is minute
```
Intermediate whitespace is permessible, but you still need quotes in the cli so as to be
captured as a single argument.
```
"1Y 2M 3w 4d 5h 6m 7s"
"1         day"
"1day 30 min"
```
There are many ways to write the same unit, all the following are equivalent.
```
2seconds
2second
2sec
2s
```
See [src/time/time.pest](https://github.com/henry-merrilees/prac/blob/main/src/time/time.pest) for the complete grammar.
Errors are decent enough to help you if you get stuck.

License: MIT
