# 207. Course Schedule

So this is a problem about dependency, which naturally leads to DAG and topological sort.

## Topological Sort

Note that a topological sort algorithm is simply *prepend* the element in front of some list right when DFS is marking the node as completed.

## Implementation

Instead of utilizing the function call stack to do the work, we could try something more succinct: local stack.
There however are two major issues using this approach, though:

1. Which neighbor to start with?
2. How to write that in Rust?

For 1., this is since using traditional function call stack based approach, we naturally would have *which neighbor* we descend DFS into remembered: it's in the call stack! E.g. this time I discovered that some neighbor `n` is yet walked (fresh), we recursively call DFS on `n`, then after `n` DFS has done, we're back to where we discovered `n`, meaning the *next neighbor* operation is trivial.

This is not true for local stack, though: using adjacency list stored in `std::collections::Vec`, we need to additionally store the indices s.t. when we're back where we were we could directly jump to next neighbor instead of starting from head of the list of neighbors. Without this, the linear time DFS algorithm may become $O(n^2)$: think of a graph that looks like a star.

For 2., one may find the Rust feature in which one could `break`/`continue` arbitrary encompassing loop with labels handy.