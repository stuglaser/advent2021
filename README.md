This is an attempt to finish all problems in Advent of Code 2021 in under 1 second. 

Inspired by:
* https://www.forrestthewoods.com/blog/solving-advent-of-code-in-under-a-second/
* https://old.reddit.com/r/adventofcode/comments/kkq6r3/2020_optimized_solutions_in_c_291_ms_total/


Total runtime (through day 21):
```
Hello, world!
All days
Took 93.596752ms  (107 samples)
```

```
All days
Day  1 |     116 µs  (8553 samples)
Day  2 |      86 µs  (11570 samples)
Day  3 |     197 µs  (5064 samples)
Day  4 |     101 µs  (9819 samples)
Day  5 |   10283 µs  (98 samples)
Day  6 |      15 µs  (63332 samples)
Day  7 |      42 µs  (23491 samples)
Day  8 |     102 µs  (9716 samples)
Day  9 |     178 µs  (5593 samples)
Day 10 |      55 µs  (17882 samples)
Day 11 |     236 µs  (4223 samples)
Day 12 |    5574 µs  (180 samples)
Day 13 |     168 µs  (5936 samples)
Day 14 |     257 µs  (3883 samples)
Day 15 |   28490 µs  (36 samples)
Day 16 |      28 µs  (35328 samples)
Day 17 |    6359 µs  (158 samples)
Day 18 |   22644 µs  (45 samples)
Day 19 |    5678 µs  (177 samples)
Day 20 |   11152 µs  (90 samples)
Day 21 |    1223 µs  (818 samples)
Theoretical total: 92.99829700000001 ms
```

The rules I follow are roughly:
* Parsing the input counts as time spent (but it's ok to warm the disk cache).
* The solution should be general (try not to hack for a certain input).
* You can generally guess input sizes, but don't measure directly on the test input (for pre-reserving arrays, for example).
