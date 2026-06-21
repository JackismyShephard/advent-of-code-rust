# Day 3

we want to generate some sequence S consisting of (first, second) based on input D of digits.

first we asign first = D_0 and second = D_1.

now lets assume al remaining digits are less than or equal to first. In this case the remaining operation is trivial. we just compare second against each new digit and
if the new digit is greater than second update second to be it.

now lets assume we stumble upon a new number thats graeter than first. actually in this case we should update first to be taht number unless that number is the last.

so the algorithm is more like

## Part 2

The goal is to basically to find a subseuqnec of the given sequence of nubmers that is monitically decreasing and more has the largest sum. its a bit like if we look at a graph this is not monotically decreasing but then we remove all points that drop from what was before
