# Day 3

## Part 1

I decided to solve this one using regexes 😄

The idea is very simple:

- we look for the best possible option which is 99
- this means checking for any two nines spaced out arbitrarily
- we replace all such lines with 99
- we now look for 98 and repeat the process
- we proceed all the way down to 11

Of course, this was a chance for me to practice neovim macros and multiediting - generating numbers from 11 to 99, reversing them, turning each line into a substitution command, figuring out how to concatenate everything into one big comment, and finally running it via `:norm <giant command>` and summing up the results using the already classic `paste -sd+ % | bc`.

## Part 2

Regexes don't quite scale for this one 😄

Luckily, we can just go for the greedy algorithm:

- first, look for the first occurrence of the largest digit in all digits except the last 11
- then, do the same, but start with the previous digit's position + 1 and end at -10
- rinse and repeat

Decided to go easy on this one and to implement it in Lua, since I didn't have much time to try out something new and it was a good chance to reinforce my skills from day 2.
