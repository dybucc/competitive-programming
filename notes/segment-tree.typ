== Segment tree implementation notes

The segment tree itself should hold references to the reference collection's elements. The key idea,
though, is that the segment tree holds the answer to the RMQ and friends. This could, in theory, be
implemented in terms of an index-based approach. But that is only feasible when the reference
collection is known to be static. For that type of queries, other data structures like a sparse
table would be simpler to implement. We must also consider the fact that the segment tree has point
and range update operations. These must reflect value updates in the reference collection. An
index-based approach is still possible but inconvenient in this particular scenario.

The segment tree must thus hold some form of indirection to items from the reference collection.
This should be achievable through any one of a custom smart pointer or a built-in reference pointer.
Considering the overhead of building a custom smart pointer, it's best if we go with the latter.

The de facto constructor should likely allow taking any collection of items. This should be further
constrained to items that are part of a collection with a well-defined order. This is because the
tree is meant to exist as a helping hand in solving RMQ or RSQ for +1 queries. Considering there's
other DSs for one-off uses, iteration order should be strictly deterministic. This is not enforcable
through any type class in the standard library, though. The only type class that gets near achieving
this is `ExactSizeIterator`. This type class ensures there exists some known end of iteration. But
it doesn't force implementors to always keep iteration order deterministic. A chief example of this
is the standard library's `HashMap`. Its iterators implement it, but hashing is inherently
non-deterministic in its ordering of items. Thus, each call to build a new iterator is likely to
yield elements in differing orders. This does not have an easy solution, but maybe a solution is not
really necessary.

A segment tree does not require any such invariants to be upheld for it to be constructed. Neither
does it expect such trait bound as discussed above to be held for it to solve RMQ or RSQ. The DS can
simply advise of such iteration order guarantees, and leave the rest up to the user. After all, the
invariant is not required to even guarantee soundness. The segment tree will still work because it
holds on to element references. It organizes the layout of its internal tree in terms of the
iteration order. This can and will work with any iterator, so long as its length is known. Consider
the case of a `HashMap`'s `Iter` iterator. This iterator knows its length, and can very well build a
segment tree. The tree would not yield answers with respect to the `HashMap`, but with respect to
the iterator. Thus, the only guarantee that the tree makes is that it yields answers with respect to
the iterator. This could align with the underlying source of items for the iterator or not. If it
does, like for `Vec`'s `Iter`, then iteration order will align with the collection's order. Thus,
the segment tree would yield answers that would also be valid for the collection.

There is one (possibly rare) exception to this. When we spoke of determinism, we referred purely to
the transition between source and iterator. Provided two condition states $A$ and $B$, assume that
each yields a transition source-iterator. Let the resulting iterators be $alpha$ and $beta$,
respectively. Assume these transitions, $A -> alpha$ and $B -> beta$, happen with the same iteration
source. The above reasoning assumed that if $A equiv B$ holds, then $alpha equiv beta$ also held.
This may or may not be true. Each iterator is imbued with the capability of determining which next
element to be yield. The only information that is fixed and thus equivalent is the source of such
iterators. Let the mapping to yield elements for some iterator $gamma$ be $f_gamma: S -> D$. Define
determinism when, provided the same source $S$, the same destination $D$ is yield every time. Iff
$f_alpha$ and $f_beta$ are deterministic, does it hold that $alpha equiv beta$. On a practical note,
assume the existence of an iterator `NewIter` for `HashMap`. We will assume this iterator is
constructed with a sequence $SS$ that yields each $S$ in some order. Assume that for two states
during iterator construction, $A$ and $B$, it holds $A equiv B$. Then the sequence $SS$ in the
iterator resulting from $A$ is equivalent to that of $B$.

// TODO: finish above discussion.

The above should be carefully implemented, dependingo on the mutability of the reference iterator.
The reason is that a segment tree can only implement update operations if the iterator is mutable.
This is not something type classes can be helpful with. The reason is that the only mutability
marker in a linear type is that of ownwership or indirection. If each of the items in the iterator
