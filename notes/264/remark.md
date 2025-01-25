In the code, we maintain 3 separate containers, each containing "ugly" numbers that have yet considered their 2, 3, and 5 multiplier ugly derivatives; the algorithm then proceeds by first picking the smallest of the derivative, and move/remove the elements correspondingly: after an element had its 2 mutiplied ugly derivative considered, it might still give new derivatives when we consider 3 or 5, so we remove it from the "2's list" and put'em in "3's list". Similarly for "3's list" and "5's list", except in the latter we just discard it since there's no more multiplier to consider.

In other words, every ugly number has its 2, 3, and 5 multiplier considered, in order; the next multiplier to be considered is determined by the list in which the element itself resides; in particular, the lists are mutually exclusive: one element lives in exactly one list at any given time.

As a side note, doubly ended linked list or `VecDeque` is quite good at this sort of work.

Well, if you think about it, we don't really need 3 separate containers, and one generic `Vec` works just fine: we just keep **3 pointers**, which resp. points to the smallest "ugly" number that yet have resp. multiplier derivative considered.

The former approach (i.e. the one in the Rust code) focuses on each "ugly" number's own PoV, whereas the latter is more or less based on the multipliers' PoV: as a multiplier, it multiplies some "ugly" number to generate other, ideally unforseen, "ugly" numbers, so the fact it traverses over "ugly" numbers should generate decent amount of "ugly" numbers, and via considering all these **traversals** we shall be able to get'em all.

[leetcode official tutorial](https://leetcode.com/problems/ugly-number-ii/editorial)
