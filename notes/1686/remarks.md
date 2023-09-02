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
To put it in another way, under such a competitive game settings, thier values, albeit different at first glance, would come to agreement.
To honor such effect, we name elements in W the consensus values.

Proof:
Let sum of U be u, V be v, each step net worth of Alice and Bob be {a\_i} and {b\_i}, and each step Alice and Bob hold {A\_i} and {B\_i} in terms of sum of members in W i.e. the consensus values.
Length being n, notice that A\_n + B\_n = u + v which is a constant.

Lemma: Alice wins iff $A_n + u \geq B_n + v$, draw iff $A_n + u = B_n + v$, and loses iff $A_n + u \leq B_n + v$.
Proof:
Let $\{\beta_i = A_i - a_i\}$ and $\{\alpha_i = B_i - b_i\}$, i.e. the respective component of their opponent in consensus value.
Notice that $u = a_n + \alpha_n$ and $v = b_n + \beta_n$
$$ A_n + u \geq B_n + v \implies \\
a_n + \beta_n + u \geq \alpha_n + b_n + v \implies \\
a_n + \beta_n + a_n + \alpha_n \geq \alpha_n + b_n + b_n + \beta_n \implies \\
2 \times a_n \geq 2 \times b_n $$

Lemma: Both players' optimal strategy are exactly the same, choosing in non-increasing order the consensus values.
Proof: Since in the end their choices of consensus values sum to constant $u + v$, this becomes an obvious greedy game.

To prevent integer overflow to some extent, we don't calculate $\{\{A_i\}, \{B_i\}, u, v\}$, instead we maintain their differences. This is since intuitively, both players should not disagree too much on value of any gem.
