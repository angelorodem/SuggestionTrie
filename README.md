# SuggestionTrie  

A Radix trie for suggestion search, it allows searching for data indexed by a set of keywords fast.

* Compressed trie by default, more memory efficient for sparse tries
* It supports a simple fuzzy to fix typos in the search.
* Customizable suggestions data, you can add anything associated with a word or a set of words.
* Non-fuzzy suggestion retrieval in µs to ns for even millions of entries.

## Note

This crate uses a nightly feature [#![feature(round_char_boundary)]](https://github.com/rust-lang/rust/issues/93743)
it only works on nightly.

## Examples  

A common usage for suggestion tries is to find data like words, documents, executables and any other data that is associated with a set of words;

Search for a word (in this example, an password) in a list with +256M entries in µs.  
<img src="https://github.com/angelorodem/SuggestionTrie/blob/master/doc_imgs/alot.gif?raw=true">

You can add custom data to the indexed suggestions, like file paths, dates, hashes, etc.  
<img src="https://github.com/angelorodem/SuggestionTrie/blob/master/doc_imgs/files.gif?raw=true">

## Usage

Cargo.toml

```toml
suggestion_trie = "^0.1"
```

Code example

```rust
use suggestion_trie::{TrieRoot, TrieInputData};
use suggestion_trie::Suggestion;
let mut trie_root: TrieRoot<i32> = TrieRoot::<i32>::new(5, Some(100));
// Get the data you want to insert into the trie
let entries = vec![
// Use lowercase keywords and query if you want to do case insensitive searches
TrieInputData {
    suggestion: Suggestion::new("Rat".to_string(), Some(4)),
    keywords: vec!["Mice".to_string(), "Rat".to_string(), "Mouse".to_string(), "Rodent".to_string()],
},
TrieInputData {
    suggestion: Suggestion::new("Cat".to_string(), Some(8000)),
    keywords: vec!["Cat".to_string(), "Kitten".to_string(), "Kitty".to_string()],
}
];
// Build the trie
trie_root.build(&entries);

// Search and get results
let results = trie_root.get_suggestions("Rodent");
assert_eq!(results.unwrap()[0].title, "Rat");
```

## Docs  

check the docs [here](https://docs.rs/suggestion_trie/latest/suggestion_trie/)
