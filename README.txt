# ToDoom
The todo list for overachieving procrastinators with an unhealthy fixation on self-development. Inspired by _The most dangerous writing app_. Designed to help discovery of routines/practices that work.

How the world works: recurrent deadlines.
How I work: one thing at a time until something else screams louder for my attention.
Issues: 
	-- when I'm focused, screams don't really get through.
Proposee solutions and why they suck:
	- Minute to minute scheduling: usually doesn't work because (1) life happens and (2) every moment is an opportunity to deviate from your system. When it does work it sucks your mind hollow of your soul.
	- Daily checklists / don't break the chain: probably the sane thing to do. But then I stop doing it once I remember all the items, at which point I forget them. More difficult for those problems which can't be worked on every day.
	- General Calender
		- You plan once when things are due, and (if you want to finish anything) again when they should be done. At best, these systems are feedback loops for cultivating agency; they should support you in transform information to the moments of action which will best serve your values. As contemplation drifts forward from the moment with which it is concerned, relevant knowledge attenuates, and backwards, does not concretely exist. A concrete plan fallen through is wasted effort, so planning is better done probabilistically. This is precomputation for your world model so that when the moment comes your focus will be dedicated entirely to aspects most uncertain and impactful. Humans are really really bad at doing this algorithmically, but are (presently) unparallelled in their heuristic capability. Our logistical/causal reasoning tuned to consider past events, which in having actually occured can contradict and tune its errors. In practice, this is less of a complete mental subjugation of the actually existing world in its totality, but rather taking from our complicated experiences the small chunks we have any chance at grasping, boiling them down, and incorporating what heuristics reman to tune our associative, knee-jerk intuition. Planning is like rejection sampling, it's works great unless you're in a very high dimensional space and missing 99% of the time. Life is exactly such a high-dimensional space.
	- Todo lists with recurrent deadlines:
		- These are great because they know their place, that an algorithm has really no business telling you what to do, and should really only be used to surface the most relevant information. Still, I think this can be done better. 
		- If you finish a task early, you now just have more time until the next iteration. It doesn't lean into your unhealthy work binges. Of course no one is going to stop you, but a great opportunity to exploit your psychological reward system is being thrown away. No room for overachievers. *It's much more difficult to discover that you should be on a shorter feedback cycle*, the pinnacle of efficient growth. 
		- Let's face it, you're going to get into a routine anyways, whether it is doing the tasks when they are marked due, or falling into some other pattern, it's going to happen. Days are the most natural period for human tasks, so you use them. Calendar days are not responsive to when you last completed a task within the day. You end up with a bunch of stuff that all has to be done some time today. A normal person has like 3-5 things to do, decides when to do them and does them. 

		  and  generally you end up with a pile of things to do by the end of the day, at which point you either start ticking them off like a normal person, or like me, you get stuck in a single one, or in planning itself.

		  When this routine get's broken, your system isn't really set to handle it.

	  they model the world very well, but still have the feeling of calendars. 



- Let's say you're the type of person that opts to work 100% of your time. That you find this survivable means primarily that you are insane, but secondarily that you are at least a little into it. You are probably pretty sensitive to how your work is going in any given moment. It's not about when you work, you have X hours no matter what, but more about when you switch between tasks. You need a way to at a glance 

  , and are probably pretty sensitive to how well your work is going in any given moment, and as well you are probably a bit insane.

my brain ---------------------------------------------------------------------------------- the world
                           ^ gamified todo apps                 ^ todo apps        ^ project management

	-- recurrent deadlines are usually too much to keep straight in my head, but the applications that are supposed to help are generally an order of magnitude more complicated. 

Todo apps too much reflect the rigidity of the "real world" to capture how I actually get stuff done, which is by thinking about one thing at a time until something else screams for my attention so loudly that I can't ignore.


Daily checklists are both too simple and too demanding. I stop using it once I feel I can remember everything, and exactly then do I forget. When everything is due at the end of the day, I'll work on some other project until it's too late. Hot streaks work a little better, but I can really only hold onto a few at a time.

Then there are todo apps with recurrent tasks, complete with timed calendar integrations.



Don't plan. Deadlines suck, you will write the thing down, then plan, then replan. Just write it down, and then look at the app you need something to do.
You only have the time that you have anyway, and only when you have it.

Only once the any particular moment arrives will you have the information to decide what to do with it. A framework of relative "okay so now what am I doing"'s is so much better than at each time I will be doing X. 
I think a "task stack" cli would actually be super useful.




Scheduling just doesn't work for me. The minute-by-minute calendar is unsurvivable--life gets in the way anyways. Planning takes way too much time, isn't at all flexible, and gives a ridiculously easy failure mechanism. I miss one event and it's over.

Ideologically, I just despise self-scheduling. Spontaneity is in all things beautiful. Forgetfulness is just spontaneity in the negative, no less an exercise of freedom. Without scheduling we would have much less to forget, and that I respect. However, I have zero respect for self-scheduling. "Sorry, I can't [be a normal fun person]," says the self-scheduler, "I have to do this thing that nobody told me I had to and that I don't even want to do." Neither is there spontaneous beauty in forgetting self-orders--you are just back where you started except now also a failure. When I "succeed" in perfectly following my elaborate self-scheduling, it means that I accomplished something so mundane that I had already totally understood it before I even began. 

I'm down with taking time for oneself, be it alone or with others. Scheduling against scheduling itself might be the one thing that scheduling is good for. Negative space is the only room for practice in this world. What I detest is the pretend virtue of forcing oneself to do what one hates. It just drips with self-pity. The same self-pity which perfectly justifies the inevitable failure to follow through.

We treat our poor long-term rational foresight as a deficiency when it does so much to protect us.







Task recurrence features in existing apps (or in those of which I am aware) do not tolerate multiple completions within a given task period.

## design choices
-- this is to help you make sure that none of your habits slip through the cracks, not to help you remember to do things you would otherwise forget.
	-- no priority system -- you should just be able to look at your stuff.
-- period based from last completion.
	-- to provide stability
	-- to proivde 


-- doesn't need to be dead simple, but complexity should should be dead obvious.

-- people are reactive.


## mvp 
-- "doom screen" progress/health bar...

## upcoming features
-- hot-streaks, completions, completion logs
-- complete manifesto
-- Grace period to tune for schedule drift
-- list display needs to be more informative

## delayed abstractions LOG UX THOUGHTS
-- better timing implementation
-- grace period for tasks to preven
-- daily logging integration (I struggle when my todolist and my notes and my journal are all separate, especially when attempting a daily journal.)
	-- I really should put this in though
	-- brainstormed implementation steps:
		-- <NEW FIELD> Habit {log: Vec<Completion { time: T, log: Option<Text> }>, ..}
		-- I don't even know if this is necessary, I might just be able to get away putting things in main log. Drawback is no timing logic, but still...
		-- maybe log + archive flag to start fresh?
	-- templates?
	-- Edit and view logs? A log that you can save before you complete the task?
-- would be a little funny to do live progress bars
-- One-off tasks 
-- git vcs
-- Subtasks? other clocks? subclocks seems like a better idea. I think it's still a bad idea though. If you want a weekly task, but want to work on it every day, just set the shorter clock and make sure you finish things.
-- should probably switch to a folder + index? loading + saving the whole thing seems dangerous.
-- responsive periods, if you miss regularly, maybe time should be extended
-- how do we re-rail
	-- dropped cycles?, need a mechanism here
	-- No cost, "i'm not gonna get to it"


## Horrible Ideas
-- Logs are just zettlekasten notes, building up to a fully featured productivity suite. Bonus points for doing this all without any custom software, all logs being just doubly-linked zettlekasten note list.
-- Agile/scrum integration
-- Project management strategies
-- Daily log, keyword extraction and sentiment analysis into covariance. Maybe also track how the day went? but that sounds like a different app.
