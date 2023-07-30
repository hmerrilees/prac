# ToDoom
The todo list for people with hyper-focus issues. Inspired by _The most dangerous writing app_.

How the world works: recurrent deadlines.
How I work: one thing at a time until something else screams louder for my attention.

Issues: 

	-- when I'm focused, screams don't really get through.

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

## delayed abstractions
-- better timing implementation
-- grace period for tasks to preven
-- daily logging integration (I struggle when my todolist and my notes and my journal are all separate, especially when attempting a daily journal.)
	-- I really should put this in though
	-- brainstormed implementation steps:
		-- <NEW FIELD> Habit {log: Vec<Completion { time: T, log: Option<Text> }>, ..}
	-- templates?
	-- Edit and view logs? A log that you can save before you complete the task?
-- would be a little funny to do live progress bars
-- One-off tasks 
-- git vcs
-- Subtasks? other clocks? subclocks seems like a better idea. I think it's still a bad idea though. If you want a weekly task, but want to work on it every day, just set the shorter clock and make sure you finish things.
-- should probably switch to a folder + index? loading + saving the whole thing seems dangerous.


## Horrible Ideas
-- Logs are just zettlekasten notes, building up to a fully featured productivity suite. Bonus points for doing this all without any custom software, all logs being just doubly-linked zettlekasten note list.
-- Agile/scrum integration
-- Project management strategies

