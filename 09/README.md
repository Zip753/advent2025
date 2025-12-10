# Day 9

Decided to go for Rust - things are getting serious and I didn't have that much time, so going into the comfort zone now 😄

Despite me having not that much experience actually writing Rust, I felt the most at home this time, it was just so natural and refreshing. Later, I had some shower thoughts about how I enjoy Rust's expressiveness and how I think I value this quite a bit in languages, which is something that also holds true for the languages I've used for this AoC so far.

> [!NOTE] Disclaimer
> I should probably explain that I've used Claude Code very sparingly and mostly in very conservative discussion capacity. I also used it as a supercharged code editing tool - it helped me scaffold a thing or two, and also add cool things like ascii art for tests, which I find immensely cool, and helpful too :) So maybe it helped me out just a tiny bit with the solution, but like 5% of it. I also didn't want to get stuck because of not knowing some geometrical algorithms, but it didn't even come to that, as I came up with the solution idea before Claude managed to suggest other options (raycasting, which I didn't really fully understand and decided to proceed with my own solution).

## Part 1

This was pure brute force and setting up the stage. Just check all pairs of points and get the max by area, done.

## Part 2

This was finally the most significant bump in difficulty so far.

I thought this is gonna end up as some crazy geometry stuff, but it actually didn't. I tried to focus on the fact that this was all vertical and horizontal paths and this led me to an idea of treating the whole thing like a grid, but not in real coordinates but in compressed ones (this is the term that Claude Code suggested).

The idea is that we can take all x and y coordinates independently and just map them to 0, 1, 2, etc. This way we end up with a grid which is still of reasonable size (same as input), and this means that we can stop thinking about geometric lines and start thinking about the grid itself. The simple idea was to filter out rectangles which were contained in the main shape and then get the max by area, like before. And now the contained rectangle meant that we would just find all of the "cells" that it was covering on the grid and just check that absolutely all of them are covered. So the whole shape turned into a 2d matrix of covered/not covered booleans!

Next challenge was figuring out how to initialise the grid and understand what's exactly covered. But since we're in a finite grid now, we can just try to do some BFS thingy somewhere and try to flood the area inside of the area. I couldn't figure out how to do it, but instead Claude Code suggested to do it from the outside, and so I've added a fake (0, 0) coordinate which would always be a good start for flooding from the outside.

So now all we have to do is to construct the border, set all the grid to covered by default and flood it from (0, 0) with false until the border. Done!

There were however some caveats:

- I had to think for a bit how to represent the grid exactly: would it mean covering the spaces/"cells" between the grid lines? Or would it be covering the vertices themselves? I went for vertices eventually, because in this problem statement points were not actual geometric points, but they were tiles, and so it was possible to have a rectangle of width 1 with the same x or y coordinates. This nicely modeled the case where a 1-width rectangle was intersecting with the area at multiple edges, but with some gaps in the middle.
- once I started writing tests, I noticed that my compressed representation was too compressed sometimes 😄 Since we were now putting distant points next to each other potentially, it was no longer possible to detect gaps between them. And so I've inserted more fake "gridlines" wherever adjacent x or y coordinates differed by more than 1.
- it wasn't obvious from the beginning that I needed to add some padding on the max side to flood from max x and max y sides, so I added another fake point with (max x + 1, max y + 1) coordinates, and this finally made it pass 🎉

I loved this problem, and the feeling at the end was incredible. Also, super happy that I chose Rust for this one, as I enjoyed the workflow immensely.
