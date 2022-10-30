Notice that for sake of checking if same row/col/slash/backslash, we can utilize a hash set.
Specifically, using backtrack algorithm, and for each row, try col/slash/backslash and record what we've done in a hash set.
How to record slash/backslash efficiently?

```
0 1 2 3 4  |  4 3 2 1 0
1 2 3 4 5  |  5 4 3 2 1
2 3 4 5 6  |  6 5 4 3 2
3 4 5 6 7  |  7 6 5 4 3
4 5 6 7 8  |  8 7 6 5 4
```
