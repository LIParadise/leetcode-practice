# My Solution

DP store in array, and use a hash to make elements unique.

## Thoughts

At times one may find that all algorithms seems fine but the answer seems to had skipped something, which seem not possible.

It may will be the case that you indeed didn't skip anything.
It's just you overrode it without noticing.

Check how you update your cache!

# Interesting Solution

[DFS](https://leetcode.com/problems/non-decreasing-subsequences/solutions/3074825/rust-hashset/)

Here's the idea: view every element as an entry point.
One could choose to ignore it, and just goes to the next entry point.
Or, one could choose to "try to" go into that entry point, depending on what one has already; it's a non-decreasing sequence.
Suppose it doesn't fit, then one has no choice but to go to the next entry point.
Suppose it does fit, then one "enters" that entry point, by which we mean that the subsequence one holds is lengthened by 1 by appending that element at the end. And, here's the catch, after entering, it's just the follwoing elements/entries ahead.

This describes how we DFS on an array.

And each time we hit end of array, one checks what one has on hand.
If it's a well-formed non-decreasing array, put it in hash s.t. they would be unique.

This is kinda the preferred way to do this: we don't need to store any more sequence than those absolutely required.
