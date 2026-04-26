# Day 1

## Part 1

## Part 2

We can reuse our framework for part 1 to solve part 2. When solving part 2 we only considered a split of the input number into two parts. In other we considered splitting by factor two. Now we any splitting factor that is greater than or equal to 2.

First of all, it should be clear that the splitting factor cannot be greater than the number of digits D in the input number. Hence we can restrict ourelves to consider split from 2 to D. Secondly, if we have already considered splitting by 2, then there is not reason to consider any other splitting factor that is a multiple of 2: If the input number does not split into two parts that are equal, then surely it cannot split into 4 parts that are equal, or 8 parts that are equal, etc. This in fact is true for any multiple of a splitting factor we have already considered: If we have already tried splitting by a factor 3, then there is no reason to consider 9, 12, 15 or anything else. In other words, we only need to consider prime numbers between 2 and D.

The idea is then to iterate through each of these splitting factors S. For a given splitting factor S we first check if it splits the input number evenly by checking whether N mod S equals 0. If not then we can skip to the next splitting factor. Otherwise we continue.

At this point we need to do the actual splitting of N into S parts. We can build on what we did in part 2 for the special case where S was 2.

What we do now is start by computing the last split using N mod 10^S. This result can be temporarily saved in a variable X. We next compute N / 10^S. This gives us whatever is left of the last split. we save that in some varaible Y.  

if Y != 0 we next compute Z = Y mod 10^S to get next split coming from the right. If Z != X then we exit immediately and return false as we know not all splits are equal. Otherwise we set X := Z and Y := Y / 10^S. We continue this until Y == 0, in which case we have processed all splits and know they must all be equal so we can return true.

This solution is obviously less efficient than the solution to part 1. The solution to part 1 processes a given number in constant time. Here on the other hand we are doing iterations for all prime numbers in the range 2 to N. For each of these iteraetions we are doing k splits and hence doing potentially k iterations. All in all we can derive a worst case upper bound as O(N**2). In practice the run time will be a lot faster than this as we in he outer loop skip all non prime numbers and out all prime numbers consider only those that divide the input perfectly. Secondly, in the inner loop we exit prematurely once we hit upon the first case of two splits not being equal.
