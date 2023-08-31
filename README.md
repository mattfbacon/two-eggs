# Two Eggs Problem

This program solves the Two Eggs Problem:

> You are given two eggs and you are dropping them from floors of a 100-story building. There is some floor _N_ such that eggs will break when dropped from that floor or higher. If an egg breaks, you can no longer use it. The goal is to reliably find _N_ in all cases while minimizing the number of tries _in the worst case_.

This program implements various strategies:

- Linear: try every floor. This is the strategy you would use if you only have one egg, and indeed, all other strategies degrade to this strategy when they only have one egg left. The worst case is if the egg breaks on the no floors, which requires trying all 100 floors.
- Chunks: evenly dividing the floors into chunks of a given size _M_. Once you find a floor where it breaks, you only have to linearly search the _M - 1_ floors below it. For the optimal value of _M = 10_, the worst case is floor 99, because it requires checking every tenth floor until the first egg breaks at the 100th floor (10 tries) then checking floors 91..=99, for a total of 19 tries.
- Shrinking chunks: As far as I know, this is the optimal algorithm. The problem with even chunks of 10 is that it's biased toward lower floors. For the worst case of floor 99, not only is there the relatively constant 9 floors at the end, but also the 10 floors (in steps of 10) up to that point. This can be compensated for by making lower chunks larger. It turns out that the optimal choice here is to start with a chunk size of 14.

## License

0BSD
