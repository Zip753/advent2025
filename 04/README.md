# Day 4

Decided to try out [ Unison ](https://www.unison-lang.org/)!

I exported all of my local server code into the scratch file, maybe later I'll figure out how to actually upload it to a public server and share it idiomatically :)

## Part 1

The idea is very simple - go through all rolls and for each of them count the amount of neighbours.

What tripped me up originally was that I was counting the number of neighbours for non-roll cells as well, and then I had to exclude them as well. On code level I modeled this as `Optional Boolean`, so `None` corresponded to a non-roll cell, `Some true` was a sparse roll (to be removed) and `Some false` was a remaining roll.

## Part 2

I got this almost for free, since I just had to do the same operation in a loop :) Due to the previous representation, I was able to detect both sparse roll cells to count the number of removed rolls and non-sparse roll cells to have an input for the next iteration.

This actually took some time to run locally - I didn't do any complexity analysis, but when I logged the number of removed rolls on each iteration, I could see that it actually took quite a few iterations to go through this process to the end. Just had to be a tiny bit more patient :)

Also, I didn't have proper LSP for part 1 and it made it quite a challenge to figure out how things work, but I really enjoyed the workflow. Being able to look up all of the standard library docs and definitions right in your terminal was super convenient, even without LSP. And once I set it up, it became quite a bit more convenient, and I got into the flow a bit more too. Being able to inspect types on intermediate expressions was very helpful, and having inline errors gave more context.
