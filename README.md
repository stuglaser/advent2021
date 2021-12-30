This is an attempt to finish all problems in Advent of Code 2021 in under 1 second. 

Inspired by:
* https://www.forrestthewoods.com/blog/solving-advent-of-code-in-under-a-second/
* https://old.reddit.com/r/adventofcode/comments/kkq6r3/2020_optimized_solutions_in_c_291_ms_total/


Total runtime (all days):
```
All days
Took 331.083712ms  (31 samples)
```

```
All days
Day  1 |     121 µs  (32948 samples)
Day  2 |      91 µs  (43698 samples)
Day  3 |     201 µs  (19872 samples)
Day  4 |     104 µs  (38143 samples)
Day  5 |  10,388 µs  (386 samples)
Day  6 |      17 µs  (222273 samples)
Day  7 |      43 µs  (92765 samples)
Day  8 |     114 µs  (35014 samples)
Day  9 |     191 µs  (20854 samples)
Day 10 |      56 µs  (71038 samples)
Day 11 |     249 µs  (16036 samples)
Day 12 |   5,806 µs  (689 samples)
Day 13 |     174 µs  (22963 samples)
Day 14 |     275 µs  (14531 samples)
Day 15 |  28,292 µs  (142 samples)
Day 16 |      29 µs  (134986 samples)
Day 17 |   6,343 µs  (631 samples)
Day 18 |  23,099 µs  (174 samples)
Day 19 |   5,472 µs  (731 samples)
Day 20 |  11,752 µs  (341 samples)
Day 21 |   1,231 µs  (3247 samples)
Day 22 |  10,581 µs  (379 samples)
Day 23 | 182,140 µs  (22 samples)
Day 24 |     406 µs  (9842 samples)
Day 25 |  48,713 µs  (83 samples)
Theoretical total: 335.89836600000007 ms
```

The rules I follow are roughly:
* Parsing the input counts as time spent (but it's ok to warm the disk cache).
* The solution should be general (try not to hack for a certain input).
* You can generally guess input sizes, but don't measure directly on the test input (for pre-reserving arrays, for example).
