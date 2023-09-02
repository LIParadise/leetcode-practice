[Problem Description]

Given bunch of gems, to whom value differs, s.t. Alice and Bob take turns picking exactly one.
If Alice were to have more value than Bob, by their respective own standards, then Alice wins.
The values are given in two arrays; same index refers to same gem; one corresponds to Alice, one Bob.
The order Alice/Bob picks gems is arbitrary.

Suppose that Alice and Bob both plays the game optimally and knows the other's value, i.e. suppose one gem is 42 to Alice and 69 to Bob then Bob knows it's 42 to Alice and Alice knows it's 69 to Bob, given the values, who would win?

[My Solution]

Name the two arrays U and V.
Claim: let sum of individual elements be new array W, also mark every value its originating index.
Sort W into non-increasing order by the sums, then corresponding indices is exactly the order Alice/Bob should take the gems.
In some sense their optimal strategies are exactly the same.
To put it in another way, under such a competitive game settings, thier values, albeit different at first glance, would come to agreement, after all.
Proof: let sum of U be u, V be v, and each step net worth of Alice and Bob be {a\_i} and {b\_i}.
First, suppose all elements in U and V are non-negative.



How much if Alice were to grab all the gems home? Sum of U. Similarly for Bob it's sum of V.

The point i
