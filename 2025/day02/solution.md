# Day 1

## Part 1

## Part 2

We can reuse our framework from part 1 to solve part 2. When solving part 2, we only considered splitting the input number into two parts. In other words, we considered splitting by a factor of two. Now we allow any splitting factor that is greater than or equal to 2.

First of all, it should be clear that the splitting factor cannot be greater than the number of digits D in the input number. Hence, we can restrict ourselves to considering splitting factors from 2 to D. Secondly, if we have already considered splitting by 2, then there is no reason to consider any other splitting factor that is a multiple of 2: if the input number does not split into two equal parts, then surely it cannot split into 4 equal parts, or 8 equal parts, etc. This is in fact true for any multiple of a splitting factor we have already considered: if we have already tried splitting by a factor of 3, then there is no reason to consider 9, 12, 15, or anything else. In other words, we only need to consider prime numbers between 2 and D.

The idea is then to iterate through each of these splitting factors S. For a given splitting factor S, we first check whether it splits the input number evenly by checking whether D mod S equals 0. If not, then we can skip to the next splitting factor. Otherwise, we continue.

At this point, we need to do the actual splitting of N into S parts. We can build on what we did in part 2 for the special case where S was 2.

What we do now is start by computing the last split using N mod 10^(D/S). This result can be temporarily saved in a variable X. We next compute N / 10^(D/S). This removes the last split and leaves the remaining leading digits. We save that in some variable Y.

If Y != 0, we next compute Z = Y mod 10^(D/S) to get the next split coming from the right. If Z != X, then we exit immediately and return false, since we know not all splits are equal. Otherwise, we set X := Z and Y := Y / 10^(D/S). We continue this until Y == 0, in which case we have processed all splits and know they must all be equal, so we can return true.

This solution is obviously less efficient than the solution to part 1. The solution to part 1 processes a given number in constant time. Here, on the other hand, we are doing iterations for all prime numbers in the range 2 to N. For each of these iterations, we are doing k splits and hence potentially k iterations. All in all, we can derive a worst-case upper bound of O(N**2). In practice, the runtime will be much faster than this, since in the outer loop we skip all non-prime numbers and, among the prime numbers, only consider those that divide the input perfectly. Secondly, in the inner loop we exit prematurely once we encounter the first case of two splits not being equal.

### Filtering prime numbers

If we do want to exclude non-prime numbers, then there is a smarter way. Instead of checking whether a number is prime during the outer loop, we can first call a function to get all prime divisors of the number of digits D. This function combines the checks for divisibility and primality. Basically, we return only those numbers that divide D and that are prime, because all the other ones are irrelevant to look at.

The function implements direct factorization using trial division. It finds the distinct prime divisors by repeatedly testing candidate divisors and dividing them out when found. Its runtime is O(sqrt(n)) in the worst case because any composite number must have a factor at most equal to its square root.

Furthermore, the resulting set of distinct prime divisors of D must have a sum that is less than or equal to D itself. This is because the product of the distinct prime divisors divides D, and is therefore at most D. Since all prime divisors are integers greater than or equal to 2, their sum is at most their product.

Remember that each inner loop of the main function executes S iterations corresponding to the current splitting factor S. Also remember that each such iteration is constant time because we are doing simple arithmetic operations (exponentiation, division, remainder, and comparison) on fixed-size integers (u64). Now that we know the sum of the considered splitting factors is less than or equal to D, we can infer that the total number of iterations must be O(D). Hence, the total cost of our solution becomes O(sqrt(D) + D) = O(D), i.e. linear in the number of digits in the input number. This is still worse than the constant-time runtime of our solution to part 1, but significantly better than quadratic runtime.

### A Better Alternative Approach

In fact, there exists an alternative solution that does not rely on converting
the number to a string. However, it does not build on the approach from part 1,
and instead requires a different underlying idea.

We can express a number satisfying the condition as

\[
m \cdot \sum_{k=0}^{r-1} 10^{kn}
\]

where:

- \(r\) is the number of repetitions,
- \(n\) is the number of digits in each repetition,
- \(m\) is the \(n\)-digit block being repeated.

For example, we can express \(123123\) as

\[
123 \cdot \sum_{k=0}^{1} 10^{k \cdot 3}
= (10^3 + 10^0) \cdot 123
= 1001 \cdot 123
= 123123
\]

Hence, we can cycle through all prime divisors \(P\) of the number of digits
\(D\), just as before. Each such \(P\) is treated as a candidate repetition
count \(r\). We then compute the number of digits in each repetition as

\[
n = D / r
\]

To get \(m\), we simply compute

\[
m = N \bmod 10^n,
\]

where \(N\) is the full number.

We can then apply the formula above to reconstruct a candidate number. If that
number is equal to our input, then the input belongs to the set.

The runtime is the same as the previous alternative approach, but this one is a
bit simpler programmatically.

There is one further optimization though. There in fact exists a closed form of the summation from above. We can derive i by realizing that the formula above actually describes a finite geometric series.
Here the constant term is m, the common ratio of the progression is 10^n and r is the number of terms in the geoemtric series. IN this case we get

\[
m \cdot \sum_{k=0}^{r-1} 10^{kn} = m\cdot \frac{1-{10^n}^r}{1 - 10^n}
\]

WIth this optimization each loop through for a given prime divisor runs in constnat time. Hence the full run time is equal to the number of prime divisors of N. but how many prime divisors of N are there? First recall that the prime divisors of N are all the prime numbers that divide N. When we multiply these prime divisors together with the right exponents recover N. For exmaple we have

12 = 2^3 * 3

This is why we also call the process of finding prime divisors of N the prime factorization of N.

We can write it formally as

\[
D = p_1^{a_1} p_2^{a_2} \cdots p_k^{a_k}.
\]

Because each exponent \(a_i \ge 1\), this means \(D\) is at least

\[
p_1 p_2 \cdots p_k.
\]

And since every prime is at least \(2\),

\[
D =p_1^{a_1} p_2^{a_2} \cdots p_k^{a_k} \ge  p_1 p_2 \cdots p_k \ge 2^k.
\]

It follows that

\[
log_2(D) \ge log_2(2^k) = k
\]

hence the number of prime divisors of N is $k \le log_2(D)$.

It follows that the full runtime must be $O(log(D))$ excluding the factorization itself. We already know the factorization is $O(\sqrt(D))$ so it follows that the full runtime including facorization is $O(\sqrt(D))$. We know that the full number of digits D is floor(log10(N)) + 1 so we can also characterize the runtime as

$O(\sqrt(\log(N)))$

### The Optimal solution

So I ended up asking codex whether my solution was optimal and it turns out it is far from it. While my solution efficiently determines whether a single number consists of 2 or more repeated segments, it does not scale very well when considering the full input for this problem. Recall that the full input is a set of ranges where each range implicitly defines a sequence of numbers, each of which my solution processes in $O(\sqrt(\log(N)))$ time. The problem is that each such sequence potentially contains many numbers. Even a fast per-number check becomes expensive if we apply it to every number in every range.

It turns out that we do not actually need to process each number in a range individually. In particular, codex was able to find another solution to this problem online which inverts the problem: Instead of iterating over every number and asking whether it is repeated, it directly characterizes the repeated numbers that can occur inside each range and sum them in groups. The full method is a bit complicated but high-level it can be described as follows:

1. For a given range `R`, iterate through each digit length `D` for which `R` contains at least one `D`-digit number.
2. For each `D`, iterate over each segment length `n`, where `n` is a proper divisor of `D` and `n` is not `D` itself.
3. For each `(D, n)`, compute the multiplier that repeats an `S`-digit segment enough times to form a `D`-digit number.
4. Use the multiplier to find the lowest and highest `n`-digit segments whose repeated forms fall inside the `D`-digit subrange of `R`; together, these two bounds define the interval of valid segments for the current `(D, S)`.
5. Sum all numbers formed by repeating the segments in this interval using the formula for an arithmetic series, instead of generating and summing each number individually.
6. For each digit length `D`, correct the sum contributed by each segment length `S` using a bottom-up DP approach, so that numbers already accounted for by shorter segment lengths are excluded.

Now let's go through each step in more detail.

## 1. Iterating though all valid digit lengths D for R

The range `R` is defined by a `(start, end)` tuple where `end` >= `start`. We can get the number of digits of each using the same log10 based approach described before. In particular we can derive:

D_start = floor(log10(start)) + 1 and D_end = floor(log10(end)) + 1

so we know to iterate from D_start to D_end to get all valid digits lengths for R.

## 2. Iterating through all segment lengths `n` that are divisors of `D`

For `n` to be a divisor of D we must have `D mod n == 0`. Furthermore, we cannot have `n` equal to `D` as that would result in a number `N` with no repeated segments. Hence, we can restrict ourselves to lengths `n` in the range `1` to floor(`D/2`), as the only divisor of `D` greater than `D/2` is `D` itself.

## 3. Computing the multiplier that repeats a segment into a `D`-length number

We now have a `(D, n)` pair. What we want to do conceptually is generate each possible segment of digits `m` of length `n` and then repeat it `r = D/n` times. Each resulting number `N` must then necessarily satisfy the conditition that it consists of `m` repeated 2 or more times.

We already know how to repeat `m` a certain number of times from our previous solution. In particular we know that we can repeat `m` a total of `r` times using the formula:

\[
m \cdot \sum_{k=0}^{r-1} 10^{kn} = m\cdot \frac{1-{10^n}^r}{1 - 10^n}
\]

So the multiplier that repeats `m` a total of `r` times is  $ M = \frac{1-{10^n}^r}{1 - 10^n}$ which we can compute in constant time

## Finding the Interval of `n`-Digit Segments That Repeat Into a `D`-Digit Number in `R`

We now have `start`, `end`, `D`, `n`, and `M`. What we would like to find next is the interval of all `n`-digit segments `m` which, when multiplied by `M`, gives us a number `N` in `R`.

For `N = m \cdot M` to be in the inclusive range `R = [start, end]`, we need:

\[
start \le M \cdot m \le end
\]

Next note that  $M  = \sum_{k=0}^{r-1} 10^{kn} > 0$ because we require r >=2 repeated segments each of which must be non-empty . Hence we can divide by `M` without changing the direction of the inequalities:

\[
\left\lceil \frac{start}{M} \right\rceil
\le m \le
\left\lfloor \frac{end}{M} \right\rfloor
\]

The ceiling is required on the left and the floor is required on the right because `M` may not divide `start` or `end` exactly. For example, if `start = 5`, `end = 10`, and `M = 3`, then:

\[
\frac{start}{M} = \frac{5}{3} = 1.66...
\]

and

\[
\frac{end}{M} = \frac{10}{3} = 3.33...
\]

Hence, the valid integer values of `m` must satisfy:

\[
2 \le m \le 3
\]

The `m` values in the range defined above satisfy that `N = m * M` lies in
the range `R`, which is a necessary but not sufficient condition. In particular,
each `m` must also contain exactly `n` digits in order for `N = m * M` to be a
number with `r` repeated segments of size `n`.

While we derived `M` from `n` and `r = D / n`, this does not force every `m` in
the range above to have `n` digits. It only ensures that copies of `m` are
placed `n` digits apart. For example, if `n = 2` and `r = 2`, then:

\[
M = 10^2 + 1 = 101
\]

Now suppose the range `R` is broad enough to contain `707`, for example
`R = [1, 9999]`. The range-derived inequality gives:

\[
\left\lceil \frac{1}{101} \right\rceil \le m \le
\left\lfloor \frac{9999}{101} \right\rfloor
\]

so:

\[
1 \le m \le 99
\]

Therefore, `m = 7` satisfies the range-derived constraint, and:

\[
7 \cdot 101 = 707
\]

But `7` is not a 2-digit segment. Conceptually this corresponds to repeating
`07` twice, giving `0707`, but as an integer the leading zero disappears and we
get `707`. Hence, the multiplier alone does not guarantee that `m` has length
`n`.
¨
It follows that we must restrict the range further to only include those $m$ which have $n$ digits. Note first that in order for $m$ to have $n$ digits it must be true that

$ 10^(n-1) <= m <= 10^(n) -1 $

therefore the lower bound of the correct range for m must be

m_start = max(\left\lceil \frac{start}{M} \right\rceil, 10^(n-1))

while the upper bound of th correct range for m must be

m_end = min(\left\lfloor \frac{end}{M} \right\rfloor, 10^(n) - 1)

## 5. Summing All Numbers Formed by Repeating the Segments in the Given Interval

Now we have a range of numbers `m` which, when multiplied with `M`, gives us
those numbers `N` that are in the range `R` and are made up of `r` repeated
segments of size `n`. What we want to compute now is the sum of these `N`.

The crucial detail is that we do not need to compute and sum each `N`
iteratively. Let us start by defining the sum mathematically as:

\[
\sum_{m=m_{start}}^{m_{end}} M \cdot m
\]

Since `M` is constant, we can move it outside the sum:

\[
M \cdot \sum_{m=m_{start}}^{m_{end}} m
\]

The values of `m` form a finite sequence where the difference between
consecutive values is exactly `1`. In other words, we are dealing with a finite
arithmetic progression whose sum has a closed-form solution:

\[
S_m = \frac{\text{count}_m}{2} \cdot (m_{start} + m_{end})
\]

where:

\[
\text{count}_m = m_{end} - m_{start} + 1
\]

Hence, we can express the full sum of all `N` as:

\[
M \cdot \frac{(m_{end} - m_{start} + 1)(m_{start} + m_{end})}{2}
\]

## 6. Correct the computed sum for each given number size D and segment size n

At this point, for each possible combination of total digit length `D` and
segment size `n`, we have computed the sum of all numbers `N` of length `D`
that can be formed by repeating a segment of size `n`.

One might therefore assume that we can simply add all of these sums together to
get the full sum of all numbers in the given range that consist of two or more
equal segments. However, this is not correct. The reason is that, for a fixed
`D`, the same number `N` can sometimes be formed using more than one segment
size `n`. Hence, for that `D`, the sums for two different values of `n` may
include the same number `N`. When summing these local sums together, we need to
account for this overlap.

We can do so using a bottom up dynamic programming approach. In order to derive the bottom up approach let us first state the underlying problem more clearly:

For a fixed total digit length `D` and a given segment length `n`, we can
generate a set of numbers `{N}` by repeating the corresponding set of
`n`-digit segments `{m}`. Assume we also have another segment length `n'`,
which generates a set of numbers `{N'}` by repeating the corresponding set of
`n'`-digit segments `{m'}`.

If `n'` divides `n` perfectly, then every number in `{N'}` is also in `{N}`.
For example, if `D = 4` and `n' = 1`, then we can choose `m' = 1`. By
concatenating it 4 times, we get:

\[
N' = 1111
\]

If we instead choose `n = 2`, then we can choose `m = 11`. By concatenating it
2 times, we get:

\[
N = N' = 1111
\]

Another example: let `D = 8`. If `n' = 2`, we can choose `m' = 12`. By
concatenating it 4 times, we get:

\[
N' = 12121212
\]

If we instead choose `n = 4`, then we can choose `m = 1212`. By concatenating
it 2 times, we get:

\[
N = N' = 12121212
\]

Hence, in order to correct the sum for n, what we really want to do is subtract the (corrected) sum for any n' < n where n mod n' = 0. This removes the part of the sum for n which comes from numbers already accounted for by shorter segment lengths. Mathematically speaking, we can define the corrected sum for n via the following recurrence:

\[
exact_n = sum_n - \sum_{\substack{i < n \\ n \bmod i = 0}} exact_i
\]

Since we are already iterating through segment sizes `n` starting at `1`,
solving the above recurrence using a bottom-up approach is easy: after
computing `sum_n`, we iterate through all `i` with `1 <= i < n`, and for each
`i` satisfying `n mod i == 0`, we subtract `exact_i`.

The downside is that this adds another inner loop. If `N` is the largest
segment length considered for the current digit length `D`, then the naive
bottom-up correction takes `O(N^2)` time for that `D`. We can reduce this to
`O(N log N)` by using a slightly different approach.

For a given digit length `D`, we maintain a `to_subtract` array of size
`floor(D / 2)`, where `to_subtract[i]` stores the total amount that must be
subtracted from `sum_i`. Initially, all entries are set to `0`.

Then, for each segment size `i`, we compute:

\[
exact_i = sum_i - to\_subtract_i
\]

This `exact_i` will need to be subtracted from every later segment size `j`
where `j > i` and `j mod i = 0`. Therefore, we iterate through all multiples
of `i`, namely `2i, 3i, 4i, ...`, up to the largest segment size
`floor(D / 2)`, and add `exact_i` to their pending correction:

\[
to\_subtract_k \mathrel{+}= exact_i
\]

Note that the innermost loop iterates through all multiples `i * k`, starting
with `k = 2`, up to `N = floor(D / 2)`. In other words, each iteration
increases the current value by `i`. It follows that for a given segment length
`i`, the total runtime is `O(N / i)`.

Over all segment lengths, we get:

\[
O(N/1) + O(N/2) + \cdots + O(N/N)
\]

This is a classical sieve-like complexity pattern, which appears when an
algorithm iterates over multiples or divisors. We can rewrite it as:

\[
O\left(N \cdot \left(1 + \frac{1}{2} + \cdots + \frac{1}{N}\right)\right)
\]

The expression in parentheses is the `N`th harmonic number `H_N`, and it grows
as:

\[
H_N = \ln(N) + \gamma + o(1)
\]

where `\gamma` is the Euler-Mascheroni constant. Hence:

\[
O(N/1) + O(N/2) + \cdots + O(N/N) = O(N \log N)
\]

Once we have corrected the contribution for each segment size `n`, we can add
the corrected contributions together to get one sum for the current digit length
`D`. This `D`-specific sum is then added to a single running accumulator shared
across all ranges.
