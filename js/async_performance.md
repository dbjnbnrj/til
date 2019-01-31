# Kyle Simpsson - https://github.com/getify/You-Dont-Know-JS/blob/master/async%20%26%20performance

Interesting - I/O maybe slow and async so console.log on slow I/O machine may not give you desired result

# Event Loop

JS engine runs inside hosting env (browser). Currently (as of Es6) it will specify how instruction happens. Event loop queue which maintains which chunk of JS pgm should be executing. For sequential chunks this is fine. For traditional async operation eg setTimeout, timer will be started. Once timer is over callback is placed on event loop queue.

JS never shares data across threads. Event loop single thread responsible for sequential execution of chunks of JS pgm.

JS single-threaded so once code inside starts running whole code will finish - this is called run to completion behaviour

Too aync functions that depend on each other cause race condition with non-deterministic output.
Eg:
--- Outcome 1---
var a = 1;
var b = 2;

// foo()
a++;
b = b * a;
a = b + 3;

// bar()
b--;
a = 8 + b;
b = a * 2;

a; // 11
b; // 22

--- Outcome 2 ---
var a = 1;
var b = 2;

// bar()
b--;
a = 8 + b;
b = a * 2;

// foo()
a++;
b = b * a;
a = b + 3;

a; // 183
b; // 180


Concurrency is process level parallelism eg: two processes executing simultaneously over the same period.

Event Loop Queue:

onscroll, request 1   <--- Process 1 starts
onscroll, request 2
response 1            <--- Process 2 starts
onscroll, request 3
response 2
response 3
onscroll, request 4
onscroll, request 5
onscroll, request 6
response 4
onscroll, request 7   <--- Process 1 finishes
response 6
response 5
response 7            <--- Process 2 finishes


Concurrency may be interactive, non-interactive and cooperation.

Async hack -
// async schedule next batch
		setTimeout( function(){
			response( data );
		}, 0 );

# Summary

    A JavaScript program is (practically) always broken up into two or more chunks, where the first chunk runs now and the next chunk runs later, in response to an event. Even though the program is executed chunk-by-chunk, all of them share the same access to the program scope and state, so each modification to state is made on top of the previous state.

  Whenever there are events to run, the event loop runs until the queue is empty. Each iteration of the event loop is a "tick." User interaction, IO, and timers enqueue events on the event queue.

  At any given moment, only one event can be processed from the queue at a time. While an event is executing, it can directly or indirectly cause one or more subsequent events.

  Concurrency is when two or more chains of events interleave over time, such that from a high-level perspective, they appear to be running simultaneously (even though at any given moment only one event is being processed).

  It's often necessary to do some form of interaction coordination between these concurrent "processes" (as distinct from operating system processes), for instance to ensure ordering or to prevent "race conditions." These "processes" can also cooperate by breaking themselves into smaller chunks and to allow other "process" interleaving.


  # Callbacks

  Review
Callbacks are the fundamental unit of asynchrony in JS. But they're not enough for the evolving landscape of async programming as JS matures.

First, our brains plan things out in sequential, blocking, single-threaded semantic ways, but callbacks express asynchronous flow in a rather nonlinear, nonsequential way, which makes reasoning properly about such code much harder. Bad to reason about code is bad code that leads to bad bugs.

We need a way to express asynchrony in a more synchronous, sequential, blocking manner, just like our brains do.

Second, and more importantly, callbacks suffer from inversion of control in that they implicitly give control over to another party (often a third-party utility not in your control!) to invoke the continuation of your program. This control transfer leads us to a troubling list of trust issues, such as whether the callback is called more times than we expect.

Inventing ad hoc logic to solve these trust issues is possible, but it's more difficult than it should be, and it produces clunkier and harder to maintain code, as well as code that is likely insufficiently protected from these hazards until you get visibly bitten by the bugs.

We need a generalized solution to all of the trust issues, one that can be reused for as many callbacks as we create without all the extra boilerplate overhead.

We need something better than callbacks. They've served us well to this point, but the future of JavaScript demands more sophisticated and capable async patterns. The subsequent chapters in this book will dive into those emerging evolutions.
