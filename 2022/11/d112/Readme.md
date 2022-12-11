# Keeping the numbers large

Key insight: the input test values for each monkey are co-prime to each other (they are all primes).
This allows us to multiply them togheter and use the product as a modular space.

So with the given test values $t_0,\dots,t_n$ we can keep the item value $v$ small using the following relation:

$$
v \mod t_i \quad\equiv\quad v \bmod \prod_{k=0}^nt_k \mod t_i
$$

$v$'s upper bound is then $\prod_{k=0}^nt_k - 1$
