# Advent of Code 2022
Yup, Rust again.

## Notes
I overengineered a bit, for learning purposes. Don't overthink about it.

### Day 1
I knew that I would need to reuse something from the P1 on P2, but I didnt know what,
so I solved P1 first. Then I realized what I could reuse and decided to solve P2 before
refactoring.

Reused an iterator. I could have reused something easier (or nothing), but was curious about how to do it.

### Day 2
Parsing as tuples.

I felt very dirty doing it. I was about to do something harder, but focused on getting the answers first
and didn't overcomplicate myself.

### Day 3
Building helping functions to use in iterators.

### Day 4
Finding the right boolean expression.

### Day 5
Welcome to Parsing Hell!

The input was the true puzzle.

### Day 6
I could have used a vector of chars to collect the input, but wanted to keep the original form.

I made a version that returns the marker/message and the first time it appeared, just in case I needed everything later.
Changed it to only return the first appearance.

It felt easy.

### Day 7
Parsing Purgatory.

I had to rewrite the parser: it worked for the example, not for my input. Probably because there were multiple directories with the same
name. Another try also worked with the example, but not with my input.

I misunderstood the text. 🤦‍♂️

The same helping functions worked for both parts.

### Day 8
Basically, view from outside and from inside.

### Day 9
Not in the mood, I decided to leave it for later.

### Day 10
No useful test for Part 2.

After submitting my answers correctly, I realized that I may have a mistake in the code that doesn't render the text properly.

### Day 11
I wanted to try without parsing input files.

Part 2 was about math, and I have to say that I am not superb at it, I needed help.
