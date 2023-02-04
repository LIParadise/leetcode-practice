A CLRS way is to do it with heaps; basically an exercise from the corresponding chapter.

Another way to do this is to use the *quick select algorithm*, which selects the k-th element in an array of which size is n in linear time on average.

How?

It's the good-old partition scheme.

Randomly pick a pivot. Partition the array into two, one less, one no less. In the mean time record their length.
If length matches k, we're lucky and we're done.
Else we continue on the side where k should belong.
In this problem, specifically, we return the array rather than the k-th alone.

Why it's linear time though?
Intuitively, any pivot should scan linear time (n), leaving two equal-length array, one of which in which we need to do again.
Hence each time we deal with half of problem size.
$\sum\limits_{i=0}^{\infty}{\frac{n}{2^i}} = 2n$
