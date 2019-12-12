# Advent of Code 2019
Have a go yourself [here](https://adventofcode.com/2019).

Did last year's one in python but I was lazy with my coding and the performance was pretty bad.

This year I'm trying to learn Rust by doing these exercises. Performance will be much better and hopefully Rust will force me to think more carefully about what I am doing.

## Thoughts
### Day 1
Easy - simple maths.
### Day 2
Write a simple interpretter for some basic machine language called intcode.
### Day 3
Crossing wires. Trying to find intersection points between wires encoded as a sequence of moves & directions. 
Used a HashSet for part 1 and a HashMap for part 2.
### Day 4
String matching. Surprisingly hard to match groups of repeating digits which are exactly 2 in size.
### Day 5
Expanding the intcode interpretter with immediate arguments, jumps and conditionals.
### Day 6
Planet orbits. Easy in theory using a tree structure. Found it very hard to implement the tree in Rust.
### Day 7
Expanding the intcode interpretter to be able to save and resume it's state in order to have many programs running concurrently.
### Day 8
Easy one. Have to process the input as a series of image layers to reveal a secret message.
### Day 9
Adding relative addressing to the intcode interpretter. Weirdly part 2 required no extra work at all.
### Day 10
Asteroid belt. Have to work out which asteroids are obstructing the line of sight of which others from their coordinates. For part 2 have to order those which are in sight by their location relative to us.

