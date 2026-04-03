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

#pagebreak()

= Battleship

The problem may be solved by storing, for each player, only the locations where
they keep thei ships. This should take up at most $2(w times h)$ bytes, for a
2-tuple consisting of the coordinates for the ship. This could potentially
overflow system memory if the $w$ were large enough, but the initial iteration
should do. Then, assumming player 1 is always the one to start the game, simply
emulate each of the, at most, 2000 shot order queries.

Emulation of each of the queries will require an efficient $O(1)$ lookup
container to store each player's ship coordinates. A hashset should do just
fine. Then, for each of the queries, starting with player 1, check if player 2's
collection of ship coordinates contains the shot order in the query, and remove
the coordinate from player 2's container if so. Otherwise, switch to assumming
the next shot corresponds to player 2, and thus perform the reverse container
operations. Whenever a shot order has been determined to be a hit, check the
length of the container from which removal has taken place, and halt the game if
the container is empty. Check then the length of the attacker's container, and
if non-empty, determine the player to have won. Otherwise, it's a draw.

If the winner hasn't been determined before all queries have been processed,
then it's a draw.

One consideration the algorithm isn't accounting for is the possibility for a
draw after one player has had their navy completely sunk. The initial
implementation is not going to put any thought into this, but secrete sample
cases likely exploit the fact that the last number of turns that a player took
must be repeated by the other player prior to determining a winner, irrespective
of whether the last shot completely sank all of the other player's ships.

The current implementation considers the above case, but is lacking in some
respect. The problem statement seems ambiguous in its reach; It is said that
each player ought have the same number of turns, but on special consideration is
put into whether a game may end abruptly with one player having fewer turns than
another player.

This is made clear by some example test case whereby the sequence of shots is
assumed to follow this correspondence:
+ Player 1 hits player 2
+ Player 1 hits player 2
- Player 2 is left without any other vessels, so following the first point on
  the number of consecutive turns that a player may take, player one is owed
  another turn, even if no ships remain on its navy.
+ Player 1 falters.
+ Player 1 hits player 2
+ Player 1 hits player 2
+ Player 1 hits player 2
- Player 2 follows up with another move on player 1, even if neither of them
  have any ships left.
+ Player 2 falters.

At this point, if we assume that player 2's ships have all been sinked, and thus
control should be handed back to it, game rules dictate that its turn should be
made up of at least three more turns. This should only really affect the time
complexity of the implemented algorithm and not the final solution, for the
problem is unaffected by further turns as no more ships remain on any one side
(even though the time complexity here would be completely ruined.)

If we assume that game rules implicitly dictate that the only possibility for
turn-taking extensions is that of satisfying both initial requirements, then
surely there's a point where one player ought abandon the game without having
taken the same set of turns, for otherwise termination would never be reached.

Resolution of such a situation would be non-trivial, especially considering the
fact that if by the end, player 2 is bound to same number of turns as player 1,
then surely player 1 is bound to the same number of turns as player 2. But that
inherently goes against the rules for turn swapping between players, which
dictate that no player ought hold the turn if #l-enum[they haven't hit the
  opposing player, *and*][the other player has some ship left unsinked in its
  navy.]

If these rules held, then termination would be possible, for indeed only a
single turn would be awarded to a player, a "mercy turn" of sorts; The rest
would have to follow from that player having satisfied both requirements and
thus have "won" the right to have another shot.

This certainly implies that each player ought have at most one turn, and quite
possibly that turns are not incremented by the number of shots that player may
take at a time, but rather by the act of swapping from one player to another.

Turn-taking logic turned out to be unrequited, and indeed, all secret sample
cases run without issues except the same failing subset as presented before. The
issue must lie somewhere else, for otherwise one of the implemented strategies
would've worked differently, but they all seem to be aligned in experimental
behavior.

The issue may be in a small detail from the second paragraph of the problem
statement. It states that the second player may get another turn even if their
entire navy is sunk, but my program assumed that to mean that any player is
getting another turn if they sink the entire navy of the opposing player.

But it seems like the only player that gets another turn even if its navy is
fully sunk is player two, and not player one, were the latter to have all its
ships sunk.

The final solution should thus be to only ever allow switching players without
trigerring the `fail` flag for input-only processing, upon reaching one of the
existing states, but now only for player 1. That should allow reusing the same
algorithm, but adding to it a slight change related to functionality once one of
the current states where the `fail` flag is modified actually hits.

This solved the problem.

#pagebreak()

= Tic-tac-toe

The problem seems to be akin to a simulation problem, except the simulation
steps are not given. Instead, one is expected to either precompute all possible
scenarios and evalute whether any one of them matches the end result, or
otherwise perform an in-place simulation as data is read in.

Clearly, there are no limits on the memory that may be used at once for the
purposes of reading in some input data, as sample test cases per run have an
upper bound of 150. For each of the sample cases, a single $3 times 3$ grid is
layed out, which shouldn't put a constraint on reading in all of `stdin` at
once.

The issue then lies in determining whether the final scenario can be reached in
a game of tic-tac-toe. The first thing that comes to mind is the possibility of
using dynamic programming, and more specifically using a bottom-up approach
where not all states are pre-computed. This should allow considering the first
of the pieces in the board, and compute the next set of possible states at that
point. If the next movement that we read in from the input data set turns out to
be one of the states we just computed, then the algorithm may *not* halt, and we
can repeat the same operation. If the algorithm cannot determine which of the
next-computed states is the one being considered at present, then it should
terminate.

If the algorithm terminates prior to having processed the complete input for
some sample test case, then it hasn't been capable of determining a possible
scenario in tic-tac-toe that could be reached.

Thinking it some more, maybe the solution is more trivial than it may seem at
first glance. If we consider instead that any one given situation is possible,
so long as the number of `X`-marked cells is only one unit larger than the
number of `O`-marked cells, then we can more simply determine whether the state
can be reached or not by checking for the number of `X`- and `O`-marked cells in
the grid under consideration.

There are multiple scenarios to take into consideration, though, as situations
like start game have a different layout than throughout the rest of the play.

Let us thus consider the different set of possible states.
- Upon starting the game, the grid is empty and thus corresponds with a valid
  state.
- Upon the first player, namely the player marking cells with `X`, taking their
  turn, the grid is left with a single, `X`-marked cell.
- Beyond this, the game can only be in one of two possible states; One where the
  total number of `O`-marked cells is equal to the number of `X`-marked cells,
  or one where it is strictly one unit smaller than the number of `X`-marked
  cells.

  The first of these situations would correspond with having finished player
  `O`'s turn, and having player `X`'s turn come up next (i.e. the sample test
  case corresponds with that of a snapshot right before player `X` is about to
  play.)

  The latter would take place if player `X` were to have just finished its turn,
  and thus the snapshot showcased by the test sample would correspond with the
  state of the grid right before player `O` were to take its turn.

Upon further inspection, there's a possibility we haven't considered. Suppose an
end game state is reached, where the number of `X`-marked cells is larger than
the number of `O`-marked cells. At this point, no more states are valid, but the
grid may or may have not been marked in its entirety. Assume the latter
scenario. A possible but invalid state would be reached by adding another
`O`-marked cell, which would correspond with one of the valid states, namely the
one where player `O` has just performed its move. If we assume the current state
to be the one showcased in the test sample, then by the above-enumerated rules,
the grid would be deemed to be in an valid state, even when clearly it has gone
past end game.

Solving this should be fairly simple; Keeping track of the marked locations for
each player, and adding another check while processing the input data set for
one of eight possible end game arrangements should do just fine. Note we speak
of eight possible end-game arrangements because a $3 times 3$ grid only has 8
different straight line sequences that could be filled by one of the two
players.

An ideal data structure to peform those lookups would be a hashset where a set
of locations can be checked in $O(1)$. Because the locations to check are
comprised of only $8 times 3 = 24$ elements, and it is to be made for upwards of
150 games, there should be no issues with performing
$150 times 24 times 2 = 7,200$ lookups at worst.

The solution seems to be missing something. Either the algorithm is being too
stringent in the allowed states, or it's missing some erroneous case. The
problem is that the problem seems too simple; the only allowed states are those
in which player `X` has the same number of occupied grid cells as player `O`, or
otherwise has one more marked position. This is then stacked with the fact that
if any one of `X` or `O` contain winning sets of positions larger than 1, the
game has reached end game state.

This latter condition is checked by first computing the cardinality of such
sets, and then ensuring that they're either both 0, or that only _one_ of them
is equal to one. This would align with the states corresponding with having no
winner in the input grid, or with having a single winner, which would represent
an end game state.

It's quite possible that the problem is actually expecting the algorithm to
perform extensive input validation. This is likely because the initial problem
statement mentions the set of conditions that make a game valid, among which we
find the grid size. This is not further remarked in the input description, which
may mean that the one case that is not being handled is the possibility for the
grid to be smaller than the expected $3 times 3$ grid. This is the only real
case that the current solution is not taking into consideration.

But it turns out that is not the right approach either. The current
implementation performs complete input validation, except for row number
validation because this is actually always accurate based on the input
description. This issue must lie somewhere else.
