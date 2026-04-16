# Part 1

## Handling modular arithmtic

The dial can be turned both ways and hence is a bidirectional ring. We want to reset "counting" when reaching 0 from either direction. Hence we need the some modular arithmetic. This is no different than implementing a circular buffer with a pointer that resets, or dealing with overflow in signed and unsigned integers.

The important thing to realize is that true modular arithemtic is not implemented by the `%` operator in all languages. In some languages `%` implements remainder, while in other languages it implements modulo in the mathematic sense. The difference is evident when dealing with a dividend that is negative, such as in:

```rust
-21 % 4
```

if `%` is implemented as remainder, the above equals -1. However if `%` is implemented as true modulo then the above equals `3`.  The latter definition is the one that makes sense for our problem because when going one step left of `0` we would expect to see `99`, not `-1`.

In rust, the `%` operator imeplements remainder. However, rust also provides `rem_euclid` which implements true modulo.

## Handling Sequential input

Now we get to the more implementation specific details. Since we need to count all times that the dial lands on `0` the naive way is to use a for loop where each iteration:

1. parses current input line to a whole number
2. adds that whole number to a running sum
3. applies true modulo operator to the running sum
4. checks if the result is 0 and if so increments a second counter by 1

Alternative, one can also implement the above in a more declarative, i.e. functional, way by chaining iterator methods. More concretely what we could do is

1. apply `map` to the input with parser function
2. apply `scan` to the mapped input with the `+` operator
3. apply `filter` to the scanned input with 0-identity function
4. apply `count` to the filtered input

As is often the case with declarative programming, we are here sacrificing speed over beauty. More concretely, We are here doing 4 * theta(n) work. This is of course the same as for the naive case asympotically speaking, but the constants factor are much higher. Moreover, more memory will need to be allocated, i.e. Omega(n).
