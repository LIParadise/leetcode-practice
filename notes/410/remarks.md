# Synopsys

Given an integer array nums and an integer k, split nums into k non-empty subarrays such that the largest sum of any subarray is minimized.

Return the minimized largest sum of the split.

A subarray is a contiguous part of the array.

## Attempt: Local Flattening

Given `k = n`, one can think of some `(n-1)` dimensional landscape, e.g. for `k = 5`, with interval sum `[9, 6, 7, 10, 5]` depicted using width. The `(n-1)` dimensional comes from the fact this graph is determined only by `4` separation between intervals.

```
*********
======
*******
==========
*****
```

One may be tempted to try the following algorithm:
When possible, for every neighboring pair of intervals (maybe from that with the largest sum and its neighbors to that with the lowest sum) try remove one element from the larger interval to append/prepend it to the smaller interval, s.t. the largest between the pair ain't larger; i.e. try *flatten* the landscape *locally*. If not possible, claim done.

Visually it's like

```
*******                                 *****
====    [[5, 2], [2, 1, 1], [3, 2]] =>  ====== [[5], [2, 2, 1, 1], [3, 2]]
*****       7        4         5        *****    5         6          5
```

I.e. for every pair of neighbors, try make them more "flat" *locally*, and hope that every sequence of local change till not possible would yield global answer.

This algorithm, however, is *wrong*. Consider the following, where `k = 3`:

```
******
=======    [[2, 4], [3, 4], [4, 6]]
**********     6       7       10   => sup = 10
```

No "local flatten" is possible: for `[2, 4], [3, 4]`, attempt to swap gives `[2], [4, 3, 4]` or `[2, 4, 3], [4]`, while for `[3, 4], [4, 6]` it's `[3], [4, 4, 6]` or `[3, 4, 4], [6]`.
But, there's a better config:

```
*********
========  [[2, 4, 3], [4, 4], [6]]
******         9       8       6    => sup = 9
```

## Attempt: Critical Intervals

Consider problem array with sum `s`, interval numbers target `k`, consider real number `r = s/k`, and define intervals *critical* if either of the following is true:

1. The subarray has sum no greater than `r`, but any attempt to lengthen it (e.g. `[start..end] -> [(start-1)..end]` or `[left..=right] -> [left..=(right+1)]`) makes sum greater than `r`
2. The subarray has sum no less than `r` but any removal of its boundary element causes the sum to become less than `r`

And claim that a optimal solution must be when all intervals are critical.

This, however, is again not true: consider the following:

```
[[99, 1], [100], [90], [100], [1, 99]]
   100     100    90    100     100     => sum = 490, k = 5, r = sum/k = 98
```

The `[99, 1]` and `[1, 99]` are not critical, i.e. 2 intervals that are not critical, yet this config has the lowest possible supremum. This illustrates that not all problems presents their optimal solution as set of critical intervals.

The converse is also not true, i.e. all intervals being critical doesn't guarantee optimal solution; consider the example from last attempt: `[[2, 4], [3, 4], [4, 6]]`.
