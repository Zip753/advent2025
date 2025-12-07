# Day 7

I decided to continue in Zig and it was noticeably better than yesterday :)

The main thing that tripped me up was probably reading the file correctly. Eventually, Claude Code helped me debug this and found out that `takeDelimiterExclusive` meant that the delimiter wasn't read and that I had to toss it explicitly. The whole IO library was quite hard to figure out because of the breaking changes in 0.15 and Claude not being so great with Zig evidently. But now I feel much more confident in writing this type of lower level code, even if I just scratched the surface. Spelling out the steps and then doing them one by one was very helpful here.

## Part 1

It's interesting how Zig made me think in terms of allocations here, and I immediately thought of what I really need to process this on the fly.

I was reading the file line by line, skipping every second line. I read the first line and initialised the starting position.

And then for every next line, I was keeping previous and new state of all outgoing beam positions. So I just needed two buffers - previous and new one.

Initially, I was just checking for carrets and previous beams and splitting those into two, and this was passing on the initial example, but gave the wrong answer on the target input. And then after some debugging I realised that I wasn't propagating the beams which were going straight, and as soon as I added that I got the correct answer.

## Part 2

This one was about adding up the number of occurrences, but the algorithm remained absolutely the same. At the end, I just had to sum up all of the numbers in the final state.
