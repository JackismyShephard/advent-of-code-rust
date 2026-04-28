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
