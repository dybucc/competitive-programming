#import "@local/typst-template:0.38.0": *

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
sequence, keeping track, in two separate lists, of the index of `1` bytes in the
input sequence that are `0` bytes in the target sequence (the _available_ list,)
and of the number of `0` or `?` bytes in the input sequence that are `1` bytes
in the target sequence (the _required_ list.) We make use of the term _list_ but
any container with decent random access, index-based lookup operations will
serve our purposes.

Upon completion of this initial pass, perform a second pass over the _required_
list, and for each index, attempt to swap its corresponding byte in the input
sequence for one of the bytes denoted by an element of the _available_ list, if
any, and remove such index from both the _available_ and _required_ lists. For
each one of these operations, increment the moves counter by `1`. Halt the
traversal upon hitting an element of the _required_ list where querying the
_available_ list yields no elements.

By the end of the traversal, and irrespective of whether this end was reached
prior to the end of iteration, resolve all `?` and `0` bytes in the input
sequence that do not yet match `1` bytes in the target sequence by setting and
toggling bits, respectively. For each setting/toggling operation, increment the
moves counter by `1`. Perform one last linear pass over both the input sequence
and the target sequence to search for dissimilarities. As soon as iteration hits
a mismatched byte, halt the traversal and output the corresponding solution
boilerplate and the byte corresponding to the unsolvable case: `-1`. Otherwise,
if the end of iteration is reached and no bytes mismatch, output the solution
boilerplate and the number of moves performed.
