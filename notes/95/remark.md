Supp. tree is as following, of which linear array representation is `[A..=L]` (i.e. 0-indexed, supp. node idx `i`, then left at `2*i+1`, right at `2*i+2`)
```
           A
          / \
         /   \
        /     \
       /       \
      /         \
     B           C
    / \         / \  
   /   \       /   \
  D     E     F     G
 / \   / \   /
H   I J   K L
```
Notice linear scan on `[A..=L]` yields exactly same result as BFS from root.
In general, for complete binary trees, BFS is somehow the same as linear array, in that when traversed they give exactly the same sequence.

Where could this be applied?
E.g. when converting complete binary tree in tree representation into linear array representation, just do a BFS from root and push all values encountered to the array, and the semantics of the tree would be preserved.

Unfortunately this property doesn't hold for trees that are not complete. In which case if one wants to convert tree representation to linear array representation, one should probably allocate corresponding $2^\text{depth}$ size first, after which either DFS or BFS works since in this case we'd be using array index to fill in the values.
