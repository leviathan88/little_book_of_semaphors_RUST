# Puzzle 3.6:

```
Semaphores can also be used to represent a queue. In this case, the initial value
is 0, and usually the code is written so that it is not possible to signal unless
there is a thread waiting, so the value of the semaphore is never positive.
For example, imagine that threads represent ballroom dancers and that two
kinds of dancers, leaders and followers, wait in two queues before entering the
dance floor. When a leader arrives, it checks to see if there is a follower waiting.
If so, they can both proceed. Otherwise it waits.
Similarly, when a follower arrives, it checks for a leader and either proceeds
or waits, accordingly.
Puzzle: write code for leaders and followers that enforces these constraints.
```