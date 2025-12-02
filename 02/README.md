# Day 2

I solved this using Lua build into Neovim. The idea was to learn about Lua, about Neovim's version of it, and how to run Lua code.

## Part 1

For this one, I decided to build a semi-analytical solution. For upper and lower boundary each, I wanted to find the highest previous number which would be an invalid product code.

This means that for every odd-length number the previous one is going to be a bunch of nines, so 12345 -> 9999.

And for even-length number, we need to compare both halves. If the second half is greater or equal than the first one, then the first half repeated twice is the invalid code. So 3438 -> 3434.

Otherwise, we have to go one number lower, so first half minus one repeated. Note that this doesn't depend on number of digits in first half minus one. So 100023 -> 9999.

Finally, once we have both lower boundaries for (from - 1) and to, we can just iterate through the resulting codes and add them all together. There could be a fancier analytical solution here, but I decided to keep it simpler for this part :)

So, 456-5249 turns into:

- 455 -> 99 ("9" twice)
- 5249 -> 5151 ("51" twice)
- now we iterate from 10 to 51 and add up all repeated numbers -> 1010 + 1111 + 1212 + ... + 5050 + 5151

And as for applying the code, I used `:luado` to parse each line, extract from and to, and call the actual lua function. Then I summed up the results in the file by piping it to `paste -sd+ | bc`.

## Part 2

Surprisingly, my solution proved extendable for the second part too, it's just that instead of two parts we now had to deal with 2 up to 10 parts (10 seemed like enough).

What had to change:

- essentially it's mostly the same but `k` instead of `2`
- in the end, we need to make sure we only collect unique results, since `666666` is `"66"` three times, `"666"` two times and `"6"` 6 times

Otherwise, pretty much the same thing.

What tripped me up was the boundaries - originally I didn't check the lower boundary for `from - 1` but for `from` and was getting lower results.
