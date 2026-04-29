= Problems

== Bits equalizer

The problem may be solved by performing first a linear scan of the input sequence, keeping track of
indices, in two separate lists, for both the `1` bytes in the input sequence that are `0` bytes in
the target sequence (the _available_ list,) and of the number of `0` or `?` bytes in the input
sequence that are `1` bytes in the target sequence (the _required_ list.) We make use of the term
_list_ but any container with decent random access, index-based lookup operations will serve our
purposes.

Upon completion of this initial pass, sort the _required_ list by indices denoting `0` bytes, and
then by indices denoting `?` bytes. Then perform a second pass over the _required_ list, and for
each index, attempt to swap its corresponding byte in the input sequence for one of the bytes
denoted by an element of the _available_ list, if any, and remove such index from both the
_available_ and _required_ lists. For each one of these operations, increment the moves counter by
`1`. If iteration has not yet finished by the time the _available_ list is empty, resolve all `?`
and `0` bytes to `1`. For each resolution operation, increment the moves counter by 1.

Compare, in one last linear pass, the (current) input sequence (after all swapping and
toggling/setting operations) with the target sequence. Halt iteration as soon as one byte
mismatches, and output `-1`, for there is no possible solution. If iteration finishes without any
mismatching bytes, then the sequences match, and the number of moves registered thus far should be
output.

The total cost of the algorithm should be $O(n)$ for the initial pass, then $O(n log n)$ to sort the
_required_ list, then $O(n)$ for the second linear pass comparing the _required_ list with the
_available_ list, and finally one $O(n)$ linear pass to compare the input sequence with the target
sequence. For inputs where $n$ ranges between 1 and 100, the asymptotic approximation seems
feasible, as it goes for $O(n + n log n + n) = O(n log n)$. Considering there are only 200 sample
cases per timed program run, ignoring constant factors also seems feasible.

The algorithm falls apart in some test case. Time to figure out what is going on exactly.

Assume the input to be `100?01`, and the target sequence to be `101000`. First, let us establish the
lower bound on the number of moves operations for this specific test case.

+ Going from `100?01` to `100001`, we resolve the only `?` byte, which is a mandatory move for any
  input sequence to even attempt resembling the target sequence.
+ Going from `100001` to `101000`, we swap the two `1` characters and thus compute the final
  solution; Namely, `2` for a number of moves equivalent to the length of this enumerated list.

The current algorithm attemps to perform these steps in reverse, as it assumes the `?` bytes require
resolution anyway, and can be made into any one of `1` or `0` bytes. Thus, it acts first on the
constrained parts of the input byte sequence: The `1` and `0` bytes. It prioritizes finding swapping
operations instead of toggling operations, such that if some byte index $theta$ denotes a `1` byte
in the target sequence where a `0` or `?` byte is found at the same index in the input sequence, and
some byte index $omega$ denotes a `0` byte in the target sequence where a `1` byte is found at the
same index in the input sequence, it will prioritize swapping these byte indices at the input
sequence to avoid performing unnecessary bit toggling operations on existing `0` bytes in the input
sequence, as those could very well ruin the input sequence with a larger number of `1` bytes than
those present in the target sequence.

By this heuristic, the algorithm would gather first the two byte indices matching possible swaps,
namely byte indices 2 and 5, corresponding with the only "required" `1` and the only "available"
`1`. A swap would follow and thus the moves counter would increase by 1. Without any other
"required" `1` bytes left, the input byte sequence would perform a linear scan, comparing each of
its (current) bytes with the target sequence, and incrementing the moves counter by 1 when
encountering a `?` byte in the input sequence.

The missing piece was getting the final linear pass to not only account for the number of `?` bytes,
but for the number of `0` bytes in the input sequence that had to be made `1` bytes. This necessity
arises in cases where the number of `1` bytes in the target sequence is larger than the combined sum
of the number of `?` and `1` bytes in the input sequence, thus requiring toggling operations on some
of the latter's `0` bytes.

#pagebreak()

== Battleship

The problem may be solved by storing, for each player, only the locations where they keep thei
ships. This should take up at most $2(w times h)$ bytes, for a 2-tuple consisting of the coordinates
for the ship. This could potentially overflow system memory if the $w$ were large enough, but the
initial iteration should do. Then, assumming player 1 is always the one to start the game, simply
emulate each of the, at most, 2000 shot order queries.

Emulation of each of the queries will require an efficient $O(1)$ lookup container to store each
player's ship coordinates. A hashset should do just fine. Then, for each of the queries, starting
with player 1, check if player 2's collection of ship coordinates contains the shot order in the
query, and remove the coordinate from player 2's container if so. Otherwise, switch to assumming the
next shot corresponds to player 2, and thus perform the reverse container operations. Whenever a
shot order has been determined to be a hit, check the length of the container from which removal has
taken place, and halt the game if the container is empty. Check then the length of the attacker's
container, and if non-empty, determine the player to have won. Otherwise, it's a draw.

If the winner hasn't been determined before all queries have been processed, then it's a draw.

One consideration the algorithm isn't accounting for is the possibility for a draw after one player
has had their navy completely sunk. The initial implementation is not going to put any thought into
this, but secrete sample cases likely exploit the fact that the last number of turns that a player
took must be repeated by the other player prior to determining a winner, irrespective of whether the
last shot completely sank all of the other player's ships.

The current implementation considers the above case, but is lacking in some respect. The problem
statement seems ambiguous in its reach; It is said that each player ought have the same number of
turns, but on special consideration is put into whether a game may end abruptly with one player
having fewer turns than another player.

This is made clear by some example test case whereby the sequence of shots is assumed to follow this
correspondence:
+ Player 1 hits player 2
+ Player 1 hits player 2
- Player 2 is left without any other vessels, so following the first point on the number of
  consecutive turns that a player may take, player one is owed another turn, even if no ships remain
  on its navy.
+ Player 1 falters.
+ Player 1 hits player 2
+ Player 1 hits player 2
+ Player 1 hits player 2
- Player 2 follows up with another move on player 1, even if neither of them have any ships left.
+ Player 2 falters.

At this point, if we assume that player 2's ships have all been sinked, and thus control should be
handed back to it, game rules dictate that its turn should be made up of at least three more turns.
This should only really affect the time complexity of the implemented algorithm and not the final
solution, for the problem is unaffected by further turns as no more ships remain on any one side
(even though the time complexity here would be completely ruined.)

If we assume that game rules implicitly dictate that the only possibility for turn-taking extensions
is that of satisfying both initial requirements, then surely there's a point where one player ought
abandon the game without having taken the same set of turns, for otherwise termination would never
be reached.

Resolution of such a situation would be non-trivial, especially considering the fact that if by the
end, player 2 is bound to same number of turns as player 1, then surely player 1 is bound to the
same number of turns as player 2. But that inherently goes against the rules for turn swapping
between players, which dictate that no player ought hold the turn if they haven't hit the opposing
player, *and* the other player has some ship left unsinked in its navy.

If these rules held, then termination would be possible, for indeed only a single turn would be
awarded to a player, a "mercy turn" of sorts; The rest would have to follow from that player having
satisfied both requirements and thus have "won" the right to have another shot.

This certainly implies that each player ought have at most one turn, and quite possibly that turns
are not incremented by the number of shots that player may take at a time, but rather by the act of
swapping from one player to another.

Turn-taking logic turned out to be unrequited, and indeed, all secret sample cases run without
issues except the same failing subset as presented before. The issue must lie somewhere else, for
otherwise one of the implemented strategies would've worked differently, but they all seem to be
aligned in experimental behavior.

The issue may be in a small detail from the second paragraph of the problem statement. It states
that the second player may get another turn even if their entire navy is sunk, but my program
assumed that to mean that any player is getting another turn if they sink the entire navy of the
opposing player.

But it seems like the only player that gets another turn even if its navy is fully sunk is player
two, and not player one, were the latter to have all its ships sunk.

The final solution should thus be to only ever allow switching players without trigerring the `fail`
flag for input-only processing, upon reaching one of the existing states, but now only for player 1.
That should allow reusing the same algorithm, but adding to it a slight change related to
functionality once one of the current states where the `fail` flag is modified actually hits.

This solved the problem.

#pagebreak()

== Tic-tac-toe

The problem seems to be akin to a simulation problem, except the simulation steps are not given.
Instead, one is expected to either precompute all possible scenarios and evalute whether any one of
them matches the end result, or otherwise perform an in-place simulation as data is read in.

Clearly, there are no limits on the memory that may be used at once for the purposes of reading in
some input data, as sample test cases per run have an upper bound of 150. For each of the sample
cases, a single $3 times 3$ grid is layed out, which shouldn't put a constraint on reading in all of
`stdin` at once.

The issue then lies in determining whether the final scenario can be reached in a game of
tic-tac-toe. The first thing that comes to mind is the possibility of using dynamic programming, and
more specifically using a bottom-up approach where not all states are pre-computed. This should
allow considering the first of the pieces in the board, and compute the next set of possible states
at that point. If the next movement that we read in from the input data set turns out to be one of
the states we just computed, then the algorithm may *not* halt, and we can repeat the same
operation. If the algorithm cannot determine which of the next-computed states is the one being
considered at present, then it should terminate.

If the algorithm terminates prior to having processed the complete input for some sample test case,
then it hasn't been capable of determining a possible scenario in tic-tac-toe that could be reached.

Thinking it some more, maybe the solution is more trivial than it may seem at first glance. If we
consider instead that any one given situation is possible, so long as the number of `X`-marked cells
is only one unit larger than the number of `O`-marked cells, then we can more simply determine
whether the state can be reached or not by checking for the number of `X`- and `O`-marked cells in
the grid under consideration.

There are multiple scenarios to take into consideration, though, as situations like start game have
a different layout than throughout the rest of the play.

Let us thus consider the different set of possible states.
- Upon starting the game, the grid is empty and thus corresponds with a valid state.
- Upon the first player, namely the player marking cells with `X`, taking their turn, the grid is
  left with a single, `X`-marked cell.
- Beyond this, the game can only be in one of two possible states; One where the total number of
  `O`-marked cells is equal to the number of `X`-marked cells, or one where it is strictly one unit
  smaller than the number of `X`-marked cells.

  The first of these situations would correspond with having finished player `O`'s turn, and having
  player `X`'s turn come up next (i.e. the sample test case corresponds with that of a snapshot
  right before player `X` is about to play.)

  The latter would take place if player `X` were to have just finished its turn, and thus the
  snapshot showcased by the test sample would correspond with the state of the grid right before
  player `O` were to take its turn.

Upon further inspection, there's a possibility we haven't considered. Suppose an end game state is
reached, where the number of `X`-marked cells is larger than the number of `O`-marked cells. At this
point, no more states are valid, but the grid may or may have not been marked in its entirety.
Assume the latter scenario. A possible but invalid state would be reached by adding another
`O`-marked cell, which would correspond with one of the valid states, namely the one where player
`O` has just performed its move. If we assume the current state to be the one showcased in the test
sample, then by the above-enumerated rules, the grid would be deemed to be in an valid state, even
when clearly it has gone past end game.

Solving this should be fairly simple; Keeping track of the marked locations for each player, and
adding another check while processing the input data set for one of eight possible end game
arrangements should do just fine. Note we speak of eight possible end-game arrangements because a
$3 times 3$ grid only has 8 different straight line sequences that could be filled by one of the two
players.

An ideal data structure to peform those lookups would be a hashset where a set of locations can be
checked in $O(1)$. Because the locations to check are comprised of only $8 times 3 = 24$ elements,
and it is to be made for upwards of 150 games, there should be no issues with performing
$150 times 24 times 2 = 7,200$ lookups at worst.

The solution seems to be missing something. Either the algorithm is being too stringent in the
allowed states, or it's missing some erroneous case. The problem is that the problem seems too
simple; the only allowed states are those in which player `X` has the same number of occupied grid
cells as player `O`, or otherwise has one more marked position. This is then stacked with the fact
that if any one of `X` or `O` contain winning sets of positions larger than 1, the game has reached
end game state.

This latter condition is checked by first computing the cardinality of such sets, and then ensuring
that they're either both 0, or that only _one_ of them is equal to one. This would align with the
states corresponding with having no winner in the input grid, or with having a single winner, which
would represent an end game state.

It's quite possible that the problem is actually expecting the algorithm to perform extensive input
validation. This is likely because the initial problem statement mentions the set of conditions that
make a game valid, among which we find the grid size. This is not further remarked in the input
description, which may mean that the one case that is not being handled is the possibility for the
grid to be smaller than the expected $3 times 3$ grid. This is the only real case that the current
solution is not taking into consideration.

But it turns out that is not the right approach either. The current implementation performs complete
input validation, except for row number validation because this is actually always accurate based on
the input description. This issue must lie somewhere else.

The problem has finally been solved and it was not an _Eureka_ nor a _Hmm, that's funny_ moment
(meaning it did not come from a one-off idea nor a long session of work, but rather of a (possibly
idiotic) refactoring.) The solution is odd, as it surged merely from an attempt to further
"functionalize" my code (make it beautifully functional and code-golfy.) The solution seemed to be
in performing further input validation, such that by the time the program was performing parsing of
possible winning positions, the algorithm should've exitted early as soon as more than a single
winning position was reached (regardless of the player that had reached such a position.)

This aligns with the initial algorithm design that performed a similar input validation at a later
stage to check that a state beyond end game was not being considered possible. The reason why (I, as
of writing this, believe) this refactoring shouldn't have yield the correct solution is that such a
check was already being performed right after parsing all winning positions (ensuring only that both
players couldn't have gotten winning positions.) This should've been caught at the stage where
pattern matching makes sure the ratio of winning positions for player `X` is 1 against 0 from player
`O`, or the other way around. But it didn't seem to catch on, and as it turns out, the solution was
in performing a full lazy evaluation of the input sample, by breaking early out of the winning
position parsing stage as soon as a win event was determined to have taken place (irrespective of
player) after another win was determined to have already taken place in the pattern-matching state
machine.

#pagebreak()

== Divide by 100

The problem is fairly simple, but making it time-efficient is a bit tricky. Initially, the algorithm
attempted to perform a linear scan over both inputs, namely $N$ and $M$. This makes sense for the
former, as we always require knowing which number we're working with, and indeed, there is no
non-trivial way of determining the number of trailing zeroes in $N$ without performing a linear scan
over its digits. However, the same cannot be said for $M$. In this instance, we can perform a
"parallel" read between the input bytes of this number and the bytes of $N$ such that string
processing happens as input is read. This should also allow, in some worst-case scenarios, to avoid
a buffer overflow due to the extreme upper bound set in the problem statement for $M$.

Currently, implementation efforts are focused on having the internal buffer for $N$ be a
double-ended queue, such that we can perform efficient push and pop operations on both ends of the
buffer, as we may need to both add leading zeroes and add a leading dot to the number, depending on
the number of zeroes in $M$. Combining the mandatory linear scan over $M$ with parallel insertions
at the front and back of the buffer for $N$ should allow us to perform the necessary string
processing efficiently, and thus solve the problem within the time limits that the initial
submittion failed to meet.

The problem has been solved. The running time is still subpar (0.02 seconds CPU time on average),
but it works. The approach previously mentioned worked just fine. Initially, regular input
processing was performed to read in bytes for $N$ to be stored in a double-ended queue, which in and
of itself required byte-by-byte input processing. This is likely one possible source of performance
degradation, as each one of the `read_exact()` calls on Rust readers that manage an I/O-bound
resource actually perform a syscall every time, so no good on that end. Beyond that, though, it's
all just basic string processing. Clearing out the buffer, making sure we store the initial length
of $N$ for the final steps, and then reading in, byte-by-byte, the input of $M$. This last part,
though, required (as mentioned in prior paragraphs) that the input not be stored, but rather
processed in-place. This basically amounted to keeping a counter for the floating point index
position, and both removing trailing zeroes from $N$ (every time we processed a `0` byte from $M$,)
and padding it with leading zeroes (whenever the counter for the index of the floating point got
larger than the total length of $N$.)

After this, the only thing left was to check whether the floating point index counter had surpassed
the initial length of $N$ (before possibly removing trailing zeroes) and if that was the case, then
inserting padding a `0.` at the start. Otherwise, the floating point index counter was used to
insert at some point in the middle of the deque a `.`. This is also (relatively) small performance
issue, as all operations thus far on the data structure of choice for this problem work in $O(1)$,
but this one specificaly is $O(n)$ where $|N| = n$. A possible improvement could maybe be to replace
the deque with an ordered set implemented as a tree, such that all seeking operations are worst-case
$O(log n)$. The input processing routines can likely be improved as well.

#pagebreak()

== A classy problem

This problem will require some discussion prior to implementing an initial solution. The first idea
that comes to mind is a ternary tree with height 10. This tree would have the root node represent
the container in which everything is stored, and from that point on, its children would be the three
class levels, each of which would form another subtree with the same three-children structure. There
would be up to 10 levels, corresponding with the worst-case scenario for classes on a given input
item.

This could also be modeled in terms of a container of binary heaps, each of these itself holding
binary heaps. But some details of the problem statement are still fuzzy to me, so this could very
well be a bad idea. If we went for the ternary tree, we would then have to perform DFS, taking note
of the items we come across as we see them, for this order dictates the non-decreasing order that is
expected in the problem statement. This should be feasible, considering tree height is static and
worst-case 10.

The reasoning behind should be that for some ternary tree where the highest class is given by the
leftmost, deepest node, all classes moving to the right, from deepest to shallowest, should
correspond with a decreasing relevance in the class level. This, though, is faced with the fact that
for some class level $A$ and some other class level $B$, where the binary relation $B < A$ holds,
some third class level $B"-"A$ would hold $B < A < B"-"A$. Such relation would have to hold for
strict equality, such that $B < A equiv B"-"A$, for the ternary tree idea to be feasible. This
implies graph traversal is going to require having a notion of ancestors as it goes through the
nodes.

For some sample test case with 500 cases, each with 100 input items, and each itself reaching height
10 in the tree, the time complexity would be dominated by insertion operations. Traversal would
always have the same fixed size cost; $approx O(60,000)$ for a fixed amount of tree nodes. This
would make traversal $O(1)$ for all intents and purposes, while making insertion costs be $O(log n)$
for $n <= 100$. Because nothing stops multiple input items from standing at the same tree node, each
node should hold the following pieces of information.
- The main tree DS info, consisting of the three children nodes.

- Satellite data, consisting of a buffer of elements of up to 100 input items worst-case, as that
  would correspond with some sample case where all input items are at the exact same class level.

Alternatively, an implementation could be approached by means of a linked data structure for each
one of the input items, with a known max size for the list. This, in fact, is the current solution
for the problem, as it allows for fairly convenient total order implementations for the type
representing classes. This solution consists of two components: a recursive-descent parser for the
input class, and an implementation that allows the built-in sorting capabilities of the standard
library to reorder the input items.

The parser is fairly straightforward, and has been implemented in terms of the `FromStr` trait for
the `Class` type, which represents the problem's hierarchy. For a given input string, described by
the regex `(upper|middle|lower)(-(upper|middle|lower))*`, the parser performs a charater-based
string splitting operation on a reverse pattern that looks for the `-` byte in the input haystack.
If this succeeds, then the recurrence relation has not yet hit the base case, and a new class can be
made from the extracted right-most class, while the rest of the string is left to be evaluated on
another call to `from_str()`, such that its return value is wrapped in a pointer that the aggregate
type of the `Class` enum variant instance holds. If string splitting does not succeed, then the base
case has been reached, and a `Class` is constructed directly from the input string, such that the
stack may start unwinding and creating pointers to each element of the class hierarchy.

This ends up forming a linked data structure that could be compared to cons lists, as the pointers
are wrapped in `Option`. An invariant thus holds that for some element $a$ of the list, if the
element has no further level of detail in the class hierarchy, its single compound type is always
`None`. If we take into consideration the time and memory limits on the problem statement, this
approach seems feasible, even against the non-friendly cache space locality of linked lists. A
possible improvement would be for the class type to be a single aggregate type that collects into a
10-element (pre-allocated) buffer the different levels of detail of a given input item's class
hierarchy (taking advantage of the fact we always know the maximum number of class chains.) But so
far nothing has proven the current approch to be insufficient in its performance.

Note that beyond the class hierarchy for each input item, its name is also stored on a collector
type, such that a total order is always found even if the class hierarchy turns out equivalent for
some two instances (taking into account the additional guarantees given in the problem statement on
no two input names being equal.) This also allows performing unstable sorting, as no initial order
ought be preserved, for all elements will eventually compare unequivalent (_possibly_ through the
class or _surely_ through their names.)

Sorting runs in $O(n log n)$ and uses the `Ord` implementation of the afore mentioned collector
type. This type derives automatically the required traits, as it relies on the shortlex ordering of
the fields in the record to follow the required name disambiguation. This implies the stored `Class`
type is always the first field when comparing lexicographically, while the name of the input item is
left second. Something to note is that the name is not simply stored as a string, as the shortlex
order of letters of the Roman alphabet under UTF-8 evaluates letters appearing later in the alphabet
as being larger than earlier ones. This has been solved through a thin wrapper over `std`'s
`&'_ str` where the implementation of `Ord`'s `cmp()` for `&'_ str` is `reverse()`d right before
returning from the routine.

All items are gathered in a contiguous allocation, such that upon sorting, the elements ought be
printed in reverse order to reflect the problem statement's requirements. This has no real
performance bottlenecks as the iterator over this allocation also implements `DoubleEndedIterator`
for efficient reverse traversal.

Currently, efforts are focused on solving a mysterious runtime error in the current implementation.
This is quite surely due to some `unwrap()` call on a `Result` or `Option` that the implemenattion
is not taking into consideration correctly. Once this is solved, the only thing left will be to
refine the `Ord` implementation of `Class`, to completely align with the behavior explained in the
third paragraph of the problem statement.

The runtime error is still present, and no solution has been found thus far. Whatever details were
left to iron out of the `Ord` impl are now done. These consisted of a special case when
disambiguating with two equivalent clases, where one of them had a larger class chain, while the
other didn't. The problem statement, even though underspecified, mentions that in such cases one
ouoght take into consideration the next class of whichever of the two has the largest class chain,
and compare them equivalent if such class is 'middle.' I assume this implies if the next class is
any one of 'lower' or 'upper', the comparison should follow the same specification as outlined
initially. A similar test case to the one presented in the problem statement for this particular
context has been run, and it seems successful.

The only thing that remains unsolved is the runtime error. The runtime error is still a mistery. The
only fallible functions are those that perform I/O-bound operations, and none of those should fail.
The only other operation that performs a panic-bound operation is building a `Class`, which hits an
`unreachable` in the case of the input string not matching any one of the expected clases. This
should not be possible, as the parsing routine within the `FromStr` for the class type cannot
produce that type of fauly input string.

After having manually gone through all possible points of failure with a `panic::catch_unwind()`,
the error seems to be in the sorting routine.

The entirety of the solution has been refactored into using a more iterative approach, in the hopes
that the `Ord` implementation was faulty previously due to differing possible results for a single
given `Class` type (depending on the pointed to elements chained to it.)

The current implementation is also about twice as short (based off of byte counts in the source file
including whitespace,) and centralizes the entire logic for the `Ord` implementation under the
`Item` type, which now includes both fields required for ordering. It also includes a counter used
at construction time to keep track of which of the elements in the array backing the class chain are
initialized. This implementation completely avoids heap allocations, but the issue with the `Ord`
implementation seems to be ever present.

I am lead to believe that the source of it lies in the fact the `Ord` implementation requires
considering both the class and the name, which may make for a non-total order definition. Suppose
some item $a$ has a class $alpha$ and name $A$, and some other item $b$ has class $beta$ and name
$B$. Assume then that there exists an equivalence relation that holds true for classes $alpha, beta$
such that $alpha equiv beta$. Then, $a$ can only compare with respect to $b$ in name, which would
mean that a relation of _larger than_ or _less than_ would have to exist between $A$ and $B$.
Suppose that relation $A > B$ holds, and thus $a > b$. If some third item $c$ with class $gamma$ and
name $C$ has that relation $beta > gamma$ holds, then it would also hold that $a > b > c$. Because
the only way for $a$ and $b$ to be compared in name and not in class is for $alpha$ to be equivalent
to $beta$, it holds by transitivity that $alpha > gamma$. This does not seem to reveal any issues
with the relation of total order that is modeled in the `Ord` implementation, but there may exist
some nuance in the way classes $alpha, beta$ get compared when determining equivalence for some
chains $|alpha| = Alpha, |beta| = Beta$ where $Alpha equiv.not Beta$ and all classes following the
largest of the two do not compare equivalent to the last class in the smallest of the two. Maybe the
issue in the current implementation is in handling that particular case.

The key is possibly in a sentence of the problem statement where the authors mention that for some
two classes $a, b$, if the class chain for $a$, namely $alpha$, and the class chain for $b$, namely
$beta$, are known to be equivalent, then all classes following the one where the length of $alpha$
is equivalent to the length of $beta$, for the largest of the chains, are to be equivalent to the
middle class of the class coming before the "lowest" level of detail, for the shortest of the
chains. The current implementation assumes this to mean that for some two items where the class
chain of one of them is a prefix of the class chain of the other, disambiguation between the two
ought happen with respect to the first class (from right to left in the input samples) that is not
shared by both chains, such that if it is determined to be a middle class, it will compare
equivalent to the class at the previous level of detail in the shorter chain, or otherwise be
considered _less than_ if the former is of lower class, and _greater than_ if of higher class.

What the problem authors may have meant here is for all subsequent classes to the prefix shared by
both chains (i.e. the subset of the chain that is only present in the largest of the class chains)
to have to be middle classes for them to compare equal. This could be implemented by checking for
the length of both iterators we currently traverse through to evaluate class chains, and "fill" the
shortest of them with middle classes. This would then allow to keep the same logic as implemented
thus far. This could even allow the item type to be even lighter, as we wouldn't have the need to
keep track of the number of initialized elements in the buffer backing the class chain; Whatever
elements remained to reach the target length could be filled in with another chained iterator
consisting only of middle classes.

This solved the problem. A subsequent submission should now follow with a slightly more optimized
approach to filling that does not rely on type erasure and iterator chaining. The problem is now
done. The runtime is slow, as it goes for .08 seconds with the above optimization, but beyond that,
it seems to fare well. Judging by the problem time limit, it seems as if the problem is indeed one
to take a fairly long running time for each sample, as it's extended beyond the usual 1 second limit
to the 4 second limit. Still, the top solutions seem to have a .01 seconds solution, so surely
there's room for improvement.

One clear area for performance optimizations is the fact that no matter the input, comparisons are
made for all 10 maximum number of classes in a given class chain. This can't be noticeably improved,
but it can be made to only perform traversal through whatever remains of the longest of the class
chains' iterator, in the case one class chain is found to be a prefix of the other. This would then
mean that the array with classes (including those "filler" middle classes) ought be reverted to
smaller-sized slices, such that an iterator adapter like `all()` can determine if such longest chain
is full of middle classes or should otherwise compare unequal. It is in the latter case where we
speak of comparing unequal that this alternative algorithm is incorrect; There is no place to handle
the case where the longest of the chains does not have its non-prefixed part be made out anything
but middle classes. To fix this, the iterator adapter over the remains of the longest chain should
evaluate to a type more rich in traversal information than a mere boolean. This could be achieved by
means of a `try_fold()`, which would halt on the first non-middle class found in the remaining
classes from the longest of the chains. This class would then be returned to compare it anew (this
time through `Ord`'s `cmp()`) with the middle class. This does not seem like it is any different
than what the prior solution did, only this time moving the logic to a purely iterator-based
approach rather than building up the iterators from the arrays containing middle class "fillers".
The only difference is in the code region where the middle class generator logic lies.

Indeed, the solution seems to yield the same CPU running time as our prior (and simpler) solution.
Maybe a better performance optimization goes through reducing allocation time by reserving in
advance space on each of the buffers we use for both raw input and item processing. That does not
seem to change the CPU time. Finally, one of the optimizations has yield better results. The current
implementation has gone down to .03 seconds, as a result of removing a bunch of flushing writes to
`stdout` and bulk reads to `stdin`. This has meant removing the bulk `read_to_string()` that was
being used to fill an initial in-memory buffer with the information to parse, and instead have the
input be read in-place. The lock over `stdout` that was held then got wrapped in a `BufWriter` that
ensured the writer did not perform any midway-through calls to `flush()`, to instead be manually
flushed right before calling `drop()` on the writer/lock. Other improvements that have come as a
consequence of the above changes include having a smaller allocation for the raw input buffer, as it
is only expected to hold the contents of a single line, and thus the preallocated capacity went down
to little over 100 bytes, corresponding with the maximum byte length of the largest test sample
input item. This, though, has meant that the type for input items no longer holds references into
that raw buffer but rather owns the string that is passed to it by cloning from the substring
produced in the corresponding iterator for the name of each item, within that type's constructor
routine.

== Bread

The problem expects to check if the input collection can be sorted with three element rotations.
This likely has some behavior akin to a data structure that can be exploited, but it's not
immediately obvious to me. The key is likely in performing a check that does not truly sort the
sequence but rather performs the same steps as required to sort it, only stopping at some state in
which it is made clear that further rotations would not result in a sorted sequence. The first idea
that comes to mind is formulating a DP recurrence relation whereby all possible states are
considered for rotation. Considering the rotation operation is fundamentally a trigger of the next
permutation in a 3-element subset of the overall ordered input set (really a sequence, though,) this
problem is fundamentally asking whether there is one possible combination of three element
permutations over three-element subsets of the input set that can get the initial state of the set
to its output state. Considering input collections can get up to 100,000 elements long, a complete
search is not feasible, so the search space ought be pruned, and possibly early-terminated if some
condition holds for construction impossibility before a certain (substantial) number of operations
are performed.

The recurrence for a complete search here would consider upwards of $2^n$ possible permutations,
which for the largest $n$ is infeasible. Still, out of those set permutations there are some that
can't possibly be reached with a three-element rotation, as otherwise the problem wouldn't be asking
for the feasibility of the final state. Which chunk of the search space ought be pruned is, indeed,
the question. This is likely not a mere distraction in the problem statement, as the whole goal of
the problem is to determine whether such final permutation is reachable with the provided rotation
operation.

A little pen and paper sketch has yield some answers. This is likely a problem where a DP recurrence
will come in handy, as there are overlapping states. These, in fact, are also the states that
determine whether some search path ought be pruned, as repetition of some such search path would not
yield program termination (due to infinite recursion.) The logic is fairly simple; For some input
set, there are always the same number of three-element _position_ subsets that are prone to
reordering. These can be easily (but possibly not efficently) obtained by running a sliding window
algorithm from `std` on the input collection's iterator. A reduction on the elements of that
iterator would yield the offsets into the collection where each three-element subset is found.

The current implementation considers a DP recurrence where the function considers three pieces of
state and one comparator collection. The state consists of a three-tuple with a referent
three-element window made out of the original three elements of the subset being permuted at
present, then a range denoting the current three-element subset under consideration at present, and
finally the complete collection, a subset of which we are permuting, and passsing across function
calls. The base case is reached when the sequence being permuted compares equivalent to the static
piece of information. This last part of the recurrence is not stored in static storage because it is
not feasible to do so in Rust, and the overhead is admissible, as we only pay in the size of a
pointer to the `'static` collection in `main`'s stack. All other cases consist of selecting a
three-element subset and start running the routine again each time we check the new rotated sequence
is deemed unequal to the referent. If this latter check evaluates `false`, recursion unwinds as a
limit case (but not a base case) has been reached. The algorithm works correctly, except that the
stack can apparently overflow with certain recursion limits where $2^6$ permutations are being
"potentially" computed. This does not even solve the public test samples. There's likely a pattern
to be exploited here.

Maybe the problem actually needs to store all permutations reachable with the rotation operation
that we get from the statement. The fact the stack overflows likely means the search space is larger
than the allowed stack space on the process' virtual AS, but likely not large enough to exhaust the
heap. This should likely mean using the same algorithm but instead building a graph with vertices
corresponding to the state of the collection post-permutation, and edges representing whether the
currently recursive algorithm would lead to one permutation from another in two consecutive stack
frames. Then, if running SSSP on the resulting graph yields an answer, then surely it is possible to
reach the target permutation. This approach has a few issues, though. The stack recursion is still
going to have to be emulated on the heap which means twice as much memory consumption; With a 1 MiB
limit and permutation limits of $O(n)$ for $n = 100,000$, this is hard. Then there's the SSSP, which
would have to use DFS or BFS for an additional linear cost in traversing the graph. That adds up
fast with the above limits. Even without an upper bound on the possible permutation with the
three-element subset rotation operation, it still isn't enough to fit the AC limits. Or not.
Inspecting the output of the current algorithm implementation on the failing sample case, it seems
as if there is some case the whole thing is converging to, and looping infinitely in. There may
still be hope for the current recursive approach.

Considering the sample test case for the non-decreasingly sorted sequence $1, 2, 3, 4, 5, 6$, there
seems to be a point in the recusion where the following sequence of rotations recuses infinitely:

$
  { 3, 4, 5, 1, 2, 6 }
  { 3, 4, 5, 6, 1, 2 }
  { 3, 4, 5, 2, 6, 1 }
$

This case is, indeed, the one where we ought determine the impossibility of reaching the final state
from the initial state. It may just be that such infinite recursion is the trigger for such
impossibility. There is one fairly simple solution to the entire problem if that is the case; upon
the thread unwinding from the panic, we can simply install a panic handler and a panic hook. The
panic hook simply prints `"Impossible"`, while the panic handler catches the unwind in the DP
recursive function. The latter stops the program from exitting with non-zero exit status, and the
former ensures we get the right answer to `stdout`. But this is not really possible, because the way
the Rust runtime reacts to a stack overflow is by aborting the process and not by cleanly unwinding,
which are the only panics we can catch with `std`. The solution should either way avoid this type of
low-level nifty tricks.

The issue could actually lie not in the sample cases where no solution is found, but in the way the
algorithm is handling the last three-element subset in the collection. The first public sample case
yields an answer before reaching the last three-element subsequence, which of course, did not
trigger the infinite recursion above. Indeed, the issue lies in all recursion happenning as soon as
the first subset furthest to the right of the input collection starts rotating. The problem is in
the way repeated cases are handled. To solve it, we require keeping a memoization table of size 3,
that provides the same value as the referent subset across recursive calls, but instead keeps track
of the three possible element orderings in the current rotating subset under consideration.

= Data structure implementations

#include "segment-tree.typ"
