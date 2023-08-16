# prac
# a utility for a feedback-cycle conducive to immersive practice in a busy world

## Installation
install rust, clone repo, then
```
cargo install --path <path to repo>
prac help
```

If you don't 
```
cargo run --release -- help
```
or
```
cargo build --release
./target/release/prac help
```

I will try to get up on crates.io soon, want to make a little more headway on state version compat, and possibly move the more incendiary dogma from the crate docs to my website (which I have yet to host).


## UI demo + TLDR
```bash
prac list
```
```
distributed systems programming ▬▬▬
                      daily log ▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬
                       exercise ▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬
                    kierkegaard ▬▬▬▬▬▬▬
                          steno ▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬▬
                    weekly repo ▬▬▬

(tip: use `prac list --cumulative` to see cumulative hours tracked, are we 10000 yet?)
```
> Looks like I haven't done steno in a while... when I get stuck, I'll switch to that.

When I'm done, I'll `prac track steno 1 hrs` to reset the bar and track time, and `prac log steno` to make some notes on my progress.


In a state of immersion, time is experienced. In productivity systems, time is controlled--better had than spent, better
spent than lost. What appears externally to be "lost" time is in fact the negative space prerquisite to immersive states.
We have no business interrupting these states ourselves, but it remains that our immersive drive will eventually outlast
the focus it can find for any particular task--we get stuck. We require a mechanism to smoothly carry our momentum into the next state of practice.
We need a subtle cue to reintroduce temporal awareness of our other practices
in a manner not so jarring that it will rip us from our direct experience of the present moment.

In contrast with other tools, in `prac`, the clock has no authority you do not yourself solicit.
It appears only as an intentionally simple, gentle visual indicator, leaning more on our [Graphical Perception](https://www.jstor.org/stable/2288400) than our numerically-obsessed scheduling proclivities.
The progress bars for each item display time elapsed since last participation as a fraction the period in which we intend to practice.
How you wish to systematize this is up to you, personally, I wouldn't even think about it.
Just look at it when you get stuck and need something else to do.

### What this isn't
This is not a todo list, a calendar, pomodoro timer, a project manager, a scheduling app, or a habit tracker.

Design decisions have been made on ideological grounds to intentionally maximize incompatibility with these types of apps.
Neither is this a compliance mechanism, if you wish to Pavlov yourself into a life well lived,
that is your prerogative, but I will not be helping you.

I have attempted to make it as difficult as possible to use this tool to "make a project of
oneself."

## What this is
A tool to augment a practice-driven life/workflow,
specifically providing feedback to enable those practices
which cannot all be done daily, or the efficacy of which is highly sensitive to factors
knowable only in the moment.

That REM sleep lifts us gracefully through from one phase of deep sleep to the next at an
average period of ~90 minutes does not make possible reproduction of the effect by way of
a 90-minute alarm clock. This is the motivating philosophy of `prac`.

If you want, you can get started right now with `prac help`. If you want to see how I
attempted to integrate the dream theory of flow state, read on to the minifesto.

## Minifesto and user guide
Begin by negatively scheduling, making time with guarantee only that you will not permit yourself to be otherwise scheduled or interrupted (including by phones/notifications).

After having `prac add`-ed a few practices, you can `prac list` and choose one to _start_ on (how you choose is none of my
business).

This feedback loop orients on _starts_ rather than completions, banking on that (canonically) self-promising
to make it through the gym doors will be more successful to motivate a workout than a list of the entailed exercises.
Output, sharing, and interpersonal feedback are no less important than starts but happen to inhabit another segment of the loop, whereas this
tool limits its scope to the "refocusing" stage.

In a conventional productivity system, interrupts are triggered externally, by calendar
notifications, timers, due dates, etc. There are few ways in which these systems could be less accountable
to the psychological state of the user in the relevant moment. The discipline of producing
conventional productivity tools might as well be called "distraction engineering."

Is genuine feedback even possible under a system where all the decisions are made ahead of
time? When we are most efficient, it might not appear to us that we are making a decision at
all, whether to continue or stop. Clearly then the problem is not implementation but orientation.

It's not about when you work, not everything is going to get done, you have X hours no matter what (and should probably limit yourself to even fewer), but more about when you switch between tasks.

In a mode of practice, control is not exerted by the clock, but follows naturally from the
persons instincts of relative flow and stuckness. When you get stuck, rather than banging your head until your pomodoro takes pity on your soul,
you simply `prac list` to see a handful of practices with a progress bar showing how long
it's been since you last practiced as a fraction of how frequently you wish to practice. This
provides a very gentle way prioritize those practices that have been recently neglected.

For those who struggle with work-life balance, these tasks can include things like rest, play, socializing, eating, outdoor time, family time, and other practices of self-care.

This is a feedback tool, not a compliance tool. It's not a big deal if you let a bar run over, take it as a signal that the relavent period needs extending.
If you find yourself regularly finishing early, you've identified that you would benefit from a shorter feedback cycle!

I firmly believe that time and quality of practice are responsible for the bulk of actually-existing competence, and so
I've implemented only two tracking features, `proc trac` for a bare total of time, and `prac log` for plain text goal-setting and reflection.

#### Can I have x feature to track something that I could just as easily track in the plain text notes?
no.

### A rant on self-scheduling
Ideologically, I despise self-scheduling. Spontaneity is in all things beautiful.
Forgetfulness is spontaneity in the negative, no less an exercise of freedom.
Without scheduling we would have much less to forget, and for that I respect scheduling.
However, I have zero respect for self-scheduling. "Sorry, I can't [be a normal fun person]," says the self-scheduler, "I have to do this thing that nobody told me I had to and that I don't even want to do myself."
Neither is there spontaneous beauty in forgetting self-orders--you are back where you started except now also a failure.
When I "succeed" in perfectly following my elaborate self-scheduling, it means that I accomplished something so mundane that I had already totally understood it before I even began.

### Incedental functionality
I used clock periods instead of calendar periods to eliminate the incentive to start at the very beginning of the block (i.e. scheduling).
Scheduling is not only is somewhat life-denying, also works against the resiliency of your system, as every moment is an opportunity to fail.
It also fails to tolerate either end of a poorly selected period.
- If the period is too short, iterations form a backlog, and in a state of overwhelm, it is easier to give up on the system.
- If the perriod is too long, Having completed an iteration early in the calendar period, one gets the sense that they should
wait until the next period to start again, artificially drawing out the period, making it very
easy to fall out of habit and accidentally drop the system.

When you have a daily todo list that also includes practices, it becomes basically impossible
not to plan. Without scheduling is no mechanism to naturally encourage routine engagement in practices at
the same within a calendar period.

If all your daily tasks are clocked 24 (+ 2hr grace) periods, you are steered naturally towards doing
things more or less at the same time you did them last. When you can keep up, your bar reminds
you not to do something too late, and you know that if you do it too early you will be met with
an early bar tomorrow too. If you can't keep up, it's no big deal, tomorrow's practice window will be
adjusted automatically to when you were able to practice today.

As a bottom-up feedback utility, and not a top-down compliance regieme, keeping
with it becomes so much easier.

If life happens, just `prac reset` to wipe the bars, and start again!
## Inspiration

The initial name was "toDoom" as the interface was
inspired by [The World's Most Dangerous Writing
App](https://www.squibler.io/dangerous-writing-prompt-app/write?limit=5&type=minutes), and I
intentionally hadn't handled progress bar overflow, resulting in a crash and arbitrary data loss.


## State management warning

I would HIGHLY recommend backing up your state file (@[[state location](#state-location)])
State management is so far from stabilized, backwards compatibility is in no way guaranteed.
As far as I'm concerned, your data may be lost at any time.
