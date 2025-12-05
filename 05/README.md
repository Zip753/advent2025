# Day 5

Decided to continue with Unison today.

Spent most of the time on parsing the input and learning to debug 😄

The workflow started becoming quite interesting, and it would look like this:

- define type signature for your new function
- hardcode some result to make it compile asap, before any implementation
- implement and put placeholders in places where logic becomes more complicated
- define signatures for those placeholders and similarly implement stubs
- as soon as it compiles, update the source
- after this, if the implemented higher level function is clear enough, **delete it from the scratch file**

This way we can get the code out of the way and focus on implementing the core logic. We can also go back to testing the higher-level methods later of course, but having the higher level logic laid out and types checked was enough for me this time.

I also did some TDD for leaf functions, which was quite nice - I didn't really have to design the APIs in a way that they would be easily testable, this kinda happened on its own when I was leaving placeholders in the top-down flow, and at some point some of those placeholders were granular enough for tests.

I'm not saying anything new or revolutionary here, but I'm guessing that having specific constraints of the technology forces you into a specific workflow, and this way allowed me to have a relatively tight iteration and feedback loop: typechecking, tests, and then finally running the whole solve together. It's just interesting how it reinforces being more explicit with the development workflow which I quite enjoyed once I got into the rhythm. And approaching it in top-down manner allowed me to extrapolate from the constraints and to stay grounded - first I had to parse the file, so I had to design the input data model, so I had to design the solver API based on that model, and then I just had a well-defined API which I just had to implement. So from the top side I closed the loop by testing the integration continuously, and from the bottom side I closed the loop via TDD by fleshing out the implementation requirements. And then it just worked :)

## Part 1

For each query, find a range which contains a query, then count all matches.

## Part 2

Unify all the ranges first and then add up their lengths.

How to unify ranges:

- sort by start and then by end (conveniently, this was the default sorting for Unison tuples already)
- iterate through all sorted pairs and keep a resulting unified list of ranges
- if the result list is empty, simply add next range
- otherwise, check if the current range overlaps with the last range (new start <= last end)
  - if yes, update last range's end to max(last end, new end)
  - if no, add current range to the result

At the end we get the list of sorted disjoint unified ranges.
