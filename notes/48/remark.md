# Rotate Image

Notice the operation (rotate the entries clockwise) is could be composed of a reflect and a transpose. This is nice since it's mathematically elegant and could potentially be trivial two-line code if one's got some matrix library.

As of this problem, one could try implement the [transpose](https://math.stackexchange.com/q/1299901/595630) or the reflect operators since they are linear and could be represented by a matrix, but this require additional memory, contradicting with in-place requirement.
