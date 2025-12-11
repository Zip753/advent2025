# Day 11

Keeping it up in Rust, because I'm really enjoying it and because I don't have that much time for trying out newer things 😫 But also I didn't have anything on my immediate list to try out 😄

## Part 1

Had to undust my graph skills! I knew this was dynamic programming and that we would have to do it in a smart order, which is when I remembered about a thing called topological sorting. I seriously went to Wikipedia for that 😄 After reading the algorithms diagonally across quickly in the morning, I realised that it's "just DFS", and then I had to remember how to implement the latter too 😄

Constructing the graph gave me grief... Doing the string-based version with hashmaps was fine, but converting it into index-based version was annoying as heck, so I was asking for some help from Claude Code (with how to do things or what I could've missed, not with writing code), as this wasn't part of the fun for this challenge.

And then I just went and did it, almost from the first try. Once you know what to do, it's not really that unexpected :)

## Part 2

I thought of being lazy-ish and just trying to collect all of the routes and filter them, but then I thought why not try to do this properly. And I ended up extending the solution with dynamic programming. I even did a generic version which accepts a longer list of nodes we have to pass through and uses bit masks to keep track of which ones we passed through. I had practice with bit masks and DP from yesterday, so this was kinda natural.

And then I spent some time debugging what turned out to be `&` instead of `|` for computing bit masks... And it passed! Results were quite big, but luckily they fit into u64, and I was super happy about that.

Also spent some time trying to figure out how to update options in the best way possible 😄 The solution was to use `get_or_insert` API which returned a mutable borrow which you can then, well, mutate. I really like this pattern, and I think I've seen it somewhere around entry API before, but this ended up being really concise and logical, which is why I love Rust's standard library - it's full of neat solutions like this one, and I can't get enough of it.
