# Rigetti Compiler Assignment

## Clarifications / Assumptions
I had asked a few clarification questions a week or two back, but didn't hear anything. Its the holidays though, so no worries on that. These are the questions I had and the assumptions I made:

_1. Does the sample input represent the expected set of valid tokens? i.e. I won't see any variable names or arithmetic expressions in place of numeric literals? RX(0.1 + 0.2), etc. 
(Or at least these expressions should be treated as invalid for now?)_
- I Assumed that this would be true, but just wanted to check.

_2. "Consecutive CZ instructions cancel" means only pairs of two CZ instructions cancel? I.e. 3 CZ's in a row reduces down to 1 rather than 0?_
- From what I could understand of the provided quil spec it made sense to me that only pairs of CZ's would cancel, so thats what I went with.

_3. Are the optimizations valid on the non-native version of the code? i.e. Can optimizations occur prior to native translation?_
- This one I was stuck on for a bit... In the problem description, native translation is provided as the step _before_ optimization. But I realize that there would be a immense benefit
to performing the optimizations first if allowed. I.e. if we can combine RX instructions ahead of time we can avoid the 5x expanding identity. But I went for the easier version of the
problem and only optimized after expanding the non-native instructions.

## Build and Run

This compiler is built using cargo and the rust std library, so build and execution is simple.

### Build:
```
cargo build --release
```
_executable located at target/release/dustinw-qc_

Supply the quil file to be compiled as an argument:
```
dustinw-qc examples/testdata/sample_1.quil
```

### Tests:
```
cargo test
```

## Dependencies / Techniques

I decided not to use any 3rd party dependencies for this project. Mostly because I'm not familiar with many of them, but also as a learning opportunity for myself.
I think I became a lot more comfortable with Rust by writing the parser/lexer myself vs. finding some parser library to do it for me.

Tokenizing / Parsing the program is fairly simple with such a small subset of the language, so I tried to design my solution with the idea that a larger subset would 
need to be introduced later. I wanted it to be easy to expand and add other instructions. I guess you could say this is a top-down parser. There are no scopes or anthing like 
that, so no need for recursive techniques yet. I first just broke the input file up into lines of tokens, then tried to pattern match the tokens to make valid instructions.

Once I had an array of instructions, my optimization passes could just be modeled as array operations. So I pass the instrucion array though a series of functions that each 
attempt to apply a different identity to the program. I keep looping throught the optimizations until the program is no longer getting shorter, or a hard limit is reached.

## Additions / Changes

This is my first project written in Rust, so Im absolutely sure that there are prettier / more idiomatic ways to write a lot of what I did. Ideally I would love to re-write 
the whole thing again but with 1 or 2 years of Rust experience. Realisticly I would look at replacing my parser/lexer implementation with some kind of grammar/parsing library. 
Im pretty happy with what I came up with, but I could see it starting to get messy if I need to add more complex grammar rules. Starting fairly early in development with a set of solid tools 
would help make the project more robust over time.
