# Keyword Index *(Assignment 3)*

### General Usage

#### Help Menu:

 * `$` `cargo run --example demo`

#### Command-Line Demo:

 * `$` `cargo run --example demo "<message>" <match word [optional]>`

#### Unit Tests:

 * `$` `cargo test`

### Report:

For this assignment, I spent a good deal of time just working on a dumb way to do the string parsing once inside the `KWIndex`. Which previously looked like a nested for-loop of 5 splits, one split per delimiter. This worked though was fairly inefficient, so I made it a goal to find a better way to split. However, second only to getting the primary interface working was creating all the necessary unit tests. I figured since the interface must stay the same, I could go ahead and do quasi-TDD and write tests that ought to pass now and change implementation later. Unit testing was fairly easy for the most part, I just did simple expected values of each function separately. After this was completed, I went back to `extend_from_text()` and its multi-for-loop state. After some scouring of the online rust textbook, I found out you can simply pass a reference of a vector of chars to `split()` and in this way, it will split by all of the delimiters listed. This made the code more sane as soon as I switched to using this because now I only needed one call to `split()` and adding more than just five punctuation marks was as simple as appending the array of chars. The only for loop I needed at this point was one to iterate over the list of split words and check for internal punctuation, e.g. anything with `'` in it or empty string. There's probably even a way to do this last part in a more functional way, excluding all uses of loops, though I did not pursue this. After the refactoring was done, I went ahead and made a demo program with arguments and spent some time looking at the Rust:tm: way of handling command line arguments. After looking at `iter()`, `match`, and a few other options, I settled on `match` seeing as it was the most sane to program without overdoing it. It's not entirely unix-style arguments, because I have yet to find a sane way of looking ahead one in the args array in case of an option like: `--match <match word>`. After this, I ran the demo, ran the unit tests one more time and everything checked out, bringing this assignment to a close.
