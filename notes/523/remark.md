At first, I thought something like the following would work: supp. this is an array:
```
XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX
```

We can first calculate remainder of whole; I below means entry included.
```
XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX
IIIIIIIIIIIIIIIIIIIIIIIIIIIIIII
```
This takes linear time.

While such remainder dosn't fit, we check one less length:
```
XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX
IIIIIIIIIIIIIIIIIIIIIIIIIIIIII
```
and
```
XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX
 IIIIIIIIIIIIIIIIIIIIIIIIIIIIII
```
Since modulo operation of add/subtract single element at the two ends, it's constant time per check, and this time we check 2 times.
Then one less length still:
```
XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX
IIIIIIIIIIIIIIIIIIIIIIIIIIIII
 IIIIIIIIIIIIIIIIIIIIIIIIIIIII
  IIIIIIIIIIIIIIIIIIIIIIIIIIIII
```
Which takes 3 units of time.

As such, in $\Theta{O(n^2)}$ time, we should get the result.
And unsurprisingly this solution TLE.

Turns out it could be done way faster: there's only `k` possible remainders.
We calculate all the prefix sum remainders, i.e. remainder of sum of all previous entries.
Each time we get a new result, we push it into some sort of map, key being the remainder, value being where we found it, i.e. current index.
Any inserted entry would never be updated.
Why? Suppose we got a same key, i.e. we've met two prefix sums of which remainder is the same.
This is exactly when the entries in between sum to some multiple of `k`!
Overall time complexity is hence $O(n \times \log{k})$, where `n` is length of input array, in which we're assigned to find contiguous subarray with sum equal multiple times of `k` and length no less than `2`.
