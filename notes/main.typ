#import "@local/typst-template:0.40.0": *

#show: template.with(
  title: [Notes --- Competitive programming],
  authorship: (
    (
      name: "Adam Martinez",
      email: "staying@never.land",
      affiliation: "University of Life",
    ),
  ),
)

= Bits equalizer

The problem may be solved by performing first a linear scan of the input
sequence, keeping track of indices, in two separate lists, for both the `1`
bytes in the input sequence that are `0` bytes in the target sequence (the
_available_ list,) and of the number of `0` or `?` bytes in the input sequence
that are `1` bytes in the target sequence (the _required_ list.) We make use of
the term _list_ but any container with decent random access, index-based lookup
operations will serve our purposes.

Upon completion of this initial pass, sort the _required_ list by indices
denoting `0` bytes, and then by indices denoting `?` bytes. Then perform a
second pass over the _required_ list, and for each index, attempt to swap its
corresponding byte in the input sequence for one of the bytes denoted by an
element of the _available_ list, if any, and remove such index from both the
_available_ and _required_ lists. For each one of these operations, increment
the moves counter by `1`. If iteration has not yet finished by the time the
_available_ list is empty, resolve all `?` and `0` bytes to `1`. For each
resolution operation, increment the moves counter by 1.

Compare, in one last linear pass, the (current) input sequence (after all
swapping and toggling/setting operations) with the target sequence. Halt
iteration as soon as one byte mismatches, and output `-1`, for there is no
possible solution. If iteration finishes without any mismatching bytes, then the
sequences match, and the number of moves registered thus far should be output.

The total cost of the algorithm should be $O(n)$ for the initial pass, then
$O(n log n)$ to sort the _required_ list, then $O(n)$ for the second linear pass
comparing the _required_ list with the _available_ list, and finally one $O(n)$
linear pass to compare the input sequence with the target sequence. For inputs
where $n$ ranges between 1 and 100, the asymptotic approximation seems feasible,
as it goes for $O(n + n log n + n) = O(n log n)$. Considering there are only 200
sample cases per timed program run, ignoring constant factors also seems
feasible.

The algorithm falls apart in some test case. Time to figure out what is going on
exactly.

Assume the input to be `100?01`, and the target sequence to be `101000`. First,
let us establish the lower bound on the number of moves operations for this
specific test case.

+ Going from `100?01` to `100001`, we resolve the only `?` byte, which is a
  mandatory move for any input sequence to even attempt resembling the target
  sequence.
+ Going from `100001` to `101000`, we swap the two `1` characters and thus
  compute the final solution; Namely, `2` for a number of moves equivalent to
  the length of this enumerated list.

The current algorithm attemps to perform these steps in reverse, as it assumes
the `?` bytes require resolution anyway, and can be made into any one of `1` or
`0` bytes. Thus, it acts first on the constrained parts of the input byte
sequence: The `1` and `0` bytes. It prioritizes finding swapping operations
instead of toggling operations, such that if some byte index $theta$ denotes a
`1` byte in the target sequence where a `0` or `?` byte is found at the same
index in the input sequence, and some byte index $omega$ denotes a `0` byte in
the target sequence where a `1` byte is found at the same index in the input
sequence, it will prioritize swapping these byte indices at the input sequence
to avoid performing unnecessary bit toggling operations on existing `0` bytes in
the input sequence, as those could very well ruin the input sequence with a
larger number of `1` bytes than those present in the target sequence.

By this heuristic, the algorithm would gather first the two byte indices
matching possible swaps, namely byte indices 2 and 5, corresponding with the
only "required" `1` and the only "available" `1`. A swap would follow and thus
the moves counter would increase by 1. Without any other "required" `1` bytes
left, the input byte sequence would perform a linear scan, comparing each of its
(current) bytes with the target sequence, and incrementing the moves counter by
1 when encountering a `?` byte in the input sequence.

The missing piece was getting the final linear pass to not only account for the
number of `?` bytes, but for the number of `0` bytes in the input sequence that
had to be made `1` bytes. This necessity arises in cases where the number of `1`
bytes in the target sequence is larger than the combined sum of the number of
`?` and `1` bytes in the input sequence, thus requiring toggling operations on
some of the latter's `0` bytes.
