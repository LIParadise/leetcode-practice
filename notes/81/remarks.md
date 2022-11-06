For distinct values like that in [LC 33](https://leetcode.com/problems/search-in-rotated-sorted-array/description/), a binary search could easily lead to where rotation/cyclic group op. is applied.
For possibly same values like in this problem, binary search could still work, albeit worst case linear time: for duplicates skip them and shrink the interval.
This should be faster than linear scan than what I did in the code if value collisions are expected to be scarce since that would be almost log time like that in [LC 33](https://leetcode.com/problems/search-in-rotated-sorted-array/description/).
As stated, here I just utilized linear scan to find where the rotation happened and then utilize binary search.
