# Homework 3
Parse the `target` text and add the sequence of
valid words contained in it to this `KWIndex`
index.

This is a "builder method": calls can be
conveniently chained to build up an index.

Words are separated by whitespace or punctuation,
and consist of a span of one or more consecutive
letters (any UTF-8 character in the "letter" class)
with no internal punctuation.

For example, the text:

```text
"It ain't over untïl it ain't, over."
```

contains the sequence of words `"It"`, `"over"`,
`"untïl"`, `"it"`, `"over"`.

## Example Usage:
```rust
let index = kwindex::KWIndex::new()
    .extend_from_text("Hello world.");

assert_eq!(2, index.len());
assert_eq!(1, index.count_matches("world"));
```
#### Populate the struct with counted words:
```rust
pub fn extend_from_text(mut self, target: &'a str) -> Self
```

#### Count the number of occurrences of the given `keyword` that are indexed by this `KWIndex`:

```rust
pub fn count_matches(&self, keyword: &str) -> usize
```

#### Count the number of words that are indexed by this `KWIndex`:
```rust
pub fn len(&self) -> usize
```

#### Is this index empty:
```rust
pub fn is_empty(&self) -> bool
```
