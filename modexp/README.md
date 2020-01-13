# Mod-Exponent

### Usage:

`$` `cargo run <base> <exponent> <modulo>`

or

`$` `modexp <base> <exponent> <modulo>`

### Report:

This module went fairly smoothly. First I built the program as a hello world type application, then I built the IO portion of the program using stdin instead of cmdline args just to play with IO. Then I built the modexp function expected of the assignment which was by far the easiest part, just a matter of translating pseudo code to actual mathematical code, and luckily, Rust seems to follow most of the basic mathematical programming conventions. Then I generalized the collection of user input to one function and added type/error checking in that one place to cut down on duplicate code. The hardest part was probably when I went to convert from typical user IO to actual cmdline arguments via crate or a regular executable. I had to read ahead a little bit to understand the Vector type better. Then of course the annoying headache was reconciling it with my generalized input function, which would take a string and spit out a crash or a valid u64. The annoying part was the fact that there are multiple types of string in rust, and not only that, but strings passed in via `env::args()` are considered string references, which need to be dereferenced then translated to mutable strings. I eventually figured it out, so it works as specified now.
