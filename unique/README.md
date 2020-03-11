# Unique

An implementation of the unique trait

## Demo
	* `$` `cargo run --example demo`

## Report

This project didn't end up being too hard overall. This is especially true since Bart gave us much of the structure to the trait itself as well as the implementation. Logically, I had already figured out in my head how I wanted to deal with checking uniques as well as short circuiting. I decided that traversing the array and appending to a list of values that meet the predicate would be the simplest way of tracking values. Then at the end I would simply check the array size and if it was anything but 1, then there were clearly no unique values given the predicate. Then for the short circuiting bit, stopping after the second non-unique was just a matter of checking the array every cycle, and if it happened to be 2 then we know we are on our second predicate match, i.e. no uniques. Development-wise, the only major trouble I was having for a short while was an odd compilation error where cargo would state "Expected function, found P" when I was attempting to call the predicate. Fortunately, it was a simple error and an easy fix, I just had to redefine what P was for both the trait AND the unique function itself.
