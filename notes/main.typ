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
as it goes for $O(n log n)$. Considering there are only 200 sample cases per
timed program run, ignoring constant factors seems feasible.

The algorithm falls apart in some test case.

```rust
let test = "1??000";
let target = "110110";
```
