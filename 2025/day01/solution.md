# Day 1

## Part 1

### Handling modular arithmetic

The dial can be turned both ways and hence is a bidirectional ring. We want to reset "counting" when reaching 0 from either direction. Hence we need some modular arithmetic for integers. This is no different than implementing a circular buffer with a pointer that resets.

The important thing to realize is that true modular arithemtic for integers is not implemented by the `%` operator in all languages. In some languages `%` implements remainder, while in other languages it implements modulo in the mathematic sense. The difference is evident when dealing with a dividend that is negative, such as in:

```rust
-1 % 100
```

if `%` is implemented as remainder, the above equals -1. However if `%` is implemented as true modulo then the above equals `99`. The latter definition is the one that makes sense for our problem because when going one step left of `0` on our dial we would expect to see `99`, not `-1`. In rust, the `%` operator implements remainder. However, rust also provides `rem_euclid` which implements true modulo.

#### Understanding the math

Mathematically speaking, both remainder and modulo are defined by the following formula
$$
a = bq + r
$$
where $a$ is dividend, $b$ is divisor, $q$ is quotient and $r$ is remainder or modulus. The difference is how we choose $q$ and therefore what sign $r$ has when $a$ is negative. One possiblity is to choose $q$ such that

$$
q = trunc(a/b)
$$
where $trunc$ removes the fractional part of a/b. This gives us the remainder. Reusing the example from above we see that

$$r = a - bq = a - b * trunc(a/b) = -1 - 100 * trunc(-1/100) = -1 - 100 * 0 = -1 $$

The other possiblity is to choose $q$ such that it satisfies: $0 <= q < b$, i.e is within the ring defined by the divisor n. This is achieved by choosing q as follows:

$$
q = floor(a/b), b > 0
q = ceil(a/b), b < 0
$$
This gives us the true modulus. Reusing the example from above we see that:

$r = a -b *floor(a/b) = -1 -100* floor(-1/100) = -1 -100 * -1  = -1 + 100 = 99$

This is the true modulo operation.

### Handling Sequential input

Now we get to the more implementation specific details. Since we need to count all times that the dial lands on `0` the optimal way is to use a for loop where each iteration:

1. parses current input line to a whole number
2. adds that whole number to a running sum
3. applies true modulo operator to the running sum
4. checks if the result is 0 and if so increments a second counter by 1

#### Parsing input lines

The difficulty here is we need to split a line into a direction part and an amount part, however the two parts are not separated by whitespace or some other symbol, so we cant use the regular split methods on the str primitive. We can use `split_at_checked` with an argument of 0.

Once we have a direction and amount part we can parse the amount part to an integer, then pattern match on the direction part: if direction is L then invert the sign of the integer.

## Part 2

Now we need to count not just when we land at 0 after a rotation but also all the times we land at 0 during a rotation.

### Solution 1: Count full turns + remaining steps

My first idea was to extend the solution to part 1 by computing the number of full turns for a given rotation, and then incrementing by 1 depending on whether the remaining steps would pass 0 from either the left or the right. We can compute the number of full turns as follows: $abs(steps) / 100$, where division truncates. Whether the remaining steps pass 0 can be determined as follows:

```rust
let remainder_end = position + step % 100;
if remainder_end >= 100 || position != 0 && remainder_end <= 0 {
    code += 1
}
```

Note the usage of (truncated) remainder rather than modulo. Because remainder does not wrap, we get the exact remaining steps, rather than a value in the ring from 0 to 99. We can then compute the
the end position after these remaining steps. if that end position is greater than or equal to 100 we must have passed 0 going right. Likewise, if the end position is less than or equal to 0 while start position is not 0, then we must have passed 0 going left.

### Solution 2: Count number of multiples of 100 in interval

I was not happy with my initial solution due to the fact that it used truncated remainder and division in addition to euclidean modulo. I had a suspicion that the truncuated remainder at the very least was not necessary in an optimal solution. Was it possibly that there was some kind of analogue to rem_euclid which i was supposed. I could not figure it out myself so I asked codex for help.

We first need to reframe the problem. We know that the goal is to compute the number of times we land on 0 during a given rotation. Lets assume the start position is p >=0 and the number of steps in the rotation is s. Then an equivalent problem formulation is finding the number of multiples of 100 (including 0) in the interval constituted by p and the (unwrapped) end position e = p + s.

When the rotation direction is R, the interval can defined mathematically as (p,e]. The reason the interval is half-open at p is because we do not want to count p even if it is a multiple of 100: that would be wrong as p is just a starting position and does not count as a crossing of a multiple of 100. Lets define the number of multiples of 100 from 0 up to a point x as M(x). In that case we can compute the number of multiples in an interval (p,e] as M(p,e) = M(e) - M(p). Because e > p >=0, the number of multiples of 100 can be computed simply using truncated division. Hence we get M(p,e) = e/100 - p/100. We can in fact simplify this further. Our solutions so far always end an iteration by setting p = (s +e) mod 100 so that p is always <=99 at the start of an iteration. In that case p/100 will always be 0 and so we get M(p,e) = e/100.

The solution derived above makes intuitive sense: when moving right we would expect the number of times 0 is passed to be exactly the number of full times 100 divides the end position. What is important is that we arrived at this intuitive solution from a reframed problem statement. We will now see that this reframed problem statement also applies when the rotation direction is L. It should be clear that in this case we cannot simply compute the number of crossings of 0 as e/100. First of all e < p, so if anything it we should compute p/100. But that also does not make sense as e and or p can be negative.

To make progress let us start by looking at the reframed problem statement. We are looking at the number of multiples of 100 in the interval constituted by e and p. Given that e < p in this case, the interval can be defined as [e,p). Note that the interval is still half-open at p rather than e (which is now the start point of the interval). This makes sense because the rotation itself starts a p so we dont want to count p, even if its a multiple of 100. The question now remains how we compute M(e,p). First of all, it is important realize that we cannot use truncated division to compute the number of multiples of 100 from 0 to e or from 0 to p when either is negative.

What we should do instead is use the type of division that corresponds to modulo, namely euclidean division. Given that the divisor b = 100 > 0, the formula from section (#### Understanding the math) above defines euclidean division as floor(a/100). If we apply this formula instead of truncated division when computing M(e,p) then we will see that negative values for either e or p are handled correctly. For example. Lets assume the interval starts at e = -50 and ends at p = -25. In this case we compute the number of multiples of 100 in this interval as M(-50,-25) = floor(-25/100) - floor(-50/100) = -1 - (-1) = 0, because floor rounds towards negative infinity. We can also try an interval that does have a crossing of 100, for example [-150, -50). In this case we get M(-150, -50) = floor(-50/100) - floor(-150/100) = -1 - (-2) = 1. Finally, note that this also works when p and or e is positive:

for [-25, 50): M(-25, 50) = floor(50 /100) - floor (-25/100) = 0 - (-1) = 1

for [25,50): M(25, 50) = floor (50/100) - floor(25/100) = 0 - 0 = 0

The current solution is a good starting point but it does not properly handle a certain edge case: namely when either p or q is equal to 0 or a multiple of 100.  When p is equal to 0 or a multiple of 100 we end up overcounting:

for [-25, 0): M(-25, 0) = floor(0/100) - floor (-25/100) = 0 - (-1) = 1, but there are 0 crossings of 0 in this case (as we are starting on 0)
for [-150, -100): M(-150, -100) = floor(-100/100) - floor(-150/100) = -1 - (-2) = -1, but there are 0 crossings of a multiple of 100 as we start on -100

In other words, we are not excluding p and hence computing [e,p] instead of [e,p).

The problem when e is 0 or a mulitple of 100 is the opposite, in this case we are undercounting:

for [0,25) : M(0,25) = floor(25/100) - floor(0/100) = 0 - 0 = 0
for [-100, -50) : M(-100,-50) = floor(-50/100) - floor(-100/100) = -1 - (-1) = 0

hence basically computing (e,p].

The way we can fix both problems is by shifting p and e left by one step. By doing so, we always include e and never include p in the given interval. We end up with the following formula: M(e,p) = floor((p-1)/100) - floor((e-1)/100). To see that it works note:

for [-25, 0): M(-25, 0) = floor(0-1/100) - floor (-25 -1/100) = floor(-1/100) - floor (-26/100) = -1 - (-1) = 0
for [-150, -100): M(-150, -100) = floor(-101 /100) - floor(-151/100) = -2 - (-2) = 0
for [0,25) : M(0,25) = floor(24/100) - floor(-1/100) = 0 - (-1) = 1
for [-100, -50) : M(-100,-50) = floor(-51/100) - floor(-101/100) = -1 - (-2) = 1

As was the case when direction was R, we can also simplify further when direction is L, if we assume that 0<=p<=100. in this case floor(p-1/100) can only take on two values: if p > 0 then the result must be 0, else the result must be -1. First statement is easy to see if we remember that when p > 0 then p -1 >= 0 and hence floor(p-1/100) = 0. Second statement is easy to see if we remember that when p = 0 then p-1 = -1 and floor(-1/100) = -1.

Hence we end up with the following implementation:

```rust
if step.is_positive() {
    code += end / 100;
} else if step.is_negative() {
    let start = if position == 0 { -1 } else { 0 };
    code += start - (end - 1).div_euclid(100);
}
position = end.rem_euclid(100);
```

### Solution 3: Only work with unwrapped numbers

It is in fact possible to solve the given problem by only working with raw positions that are never normalized by wrapping to the 0 to 99 range. We rely on the same problem formulation as in solution 2, i.e. we are considering number of multiples of 100 in intervals (p,e] and [e,p), depending on right or left direction, respectively. Since there is now no restriction on the value of p. We cannot apply the same simplifications as we did in solution 2. Instead what we get is the following:

```rust
if step.is_positive() {
    code += end.div_euclid(100) - position.div_euclid(100);
} else if step.is_negative() {
    code += (position - 1).div_euclid(100) - (end - 1).div_euclid(100);
}
```

Note that we are now also applying the euclidean division for the case when direction is R. This is because either p or e can be negative now. However, we do not to subtract 1 from either position in this case. The reason for this is that the subtraction operation itself naturally excludes the lower bound: position.div_euclid(100) counts all multiples up to and including position, and subtracting it from end.div_euclid(100) leaves only the multiples in (position, end].
