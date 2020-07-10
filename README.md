### Supermux

This project reduces the
[superpermutation problem](https://en.wikipedia.org/wiki/Superpermutation)
to Quantified Boolean Formula and runs a state-of-the-art QBF solver on the
result.

### The reduction

The main part of the reduction is a large multiplexor that spans the length of
the string, hence the name of this project. The string is encoded using the
'one hot' encoding and uses the 'commander variable' encoding to enforce the
'exactly one' constraint.

The multiplexor takes an 'address' and produces an 'output' which is the
character in the string at that index. For example, when the multiplexor's
address is set to '110010', its output will be the character at index 50 in the
string.

Each permutation (e.g. 1234) is given an address and it is enforced that the
string must contain the characters in that permutation at that address. This
encodes the superpermutation problem because if every permutation has an address
and the string is of limited size, then it must fit all permutations into that
size.

This reduction is to QBF rather than SAT. We need this extra power to avoid a
combinatorial blow up in encoding. We need to be able to associate every
permutation with every possible position on the string, which results in
approximately N! x (N! + wasted characters) comparisons. For N=6, that would be
720 x 872 = 627,840 comparisons.

Instead, we universally quantify the 'address' to make it so we find solutions
for the contents of the input string that work for any possible address an
adversary might set. The idea being, "I come up with an input string as a straw
man" and "You try to knock it down by finding a permutation that isn't in the
string". If you are unable to knock it down then my input string is indeed a
superpermutation.

This avoids a combinatorial explosion and the emitted formula for N=6 contains
approximately 100k literals and 350k clauses which is far less than the millions
that would be required for a SAT encoding (assuming there's no way to avoid the
combinatorial explosion). However, by using QBF we lift the problem to PSPACE in
the complexity hierarchy making it harder to solve. Whatsmore, QBF solvers are
not currently as mature as SAT solvers.

After preprocessing the formula using a variety of tools, the number of clauses
reduces to approximately 200k for N=6 which still appears to be too many for a
QBF solver to work through. However, I will leave one running for a few
days/weeks to see if it is successful.

This was a fun experiment in reducing a problem to QBF. It's the first time I've
done that and I particularly like thinking through the back-and-forth between
the existential and universal quantifiers as though they are two adversaries
playing a game.
