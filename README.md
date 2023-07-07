# SuggestionTrie  

A Radix trie for suggestion search, it allows to search for data indexed by a set of keywords fast.

* Compressed trie by default, more memory efficient for sparse tries
* It supports a simple fuzzy to fix typos in the search.
* Customizable suggestions data, you can add anything associated with a word or a set of words.
* Non-fuzzy suggestion retrieval in µs to ns.

## Examples  

Search for a word (in this example a password) in a list with +256M entries in µs.  
<img src="https://github.com/angelorodem/SuggestionTree/blob/master/doc_imgs/alot.gif?raw=true">

Find data associated with a word like file paths.  
<img src="https://github.com/angelorodem/SuggestionTree/blob/master/doc_imgs/files.gif?raw=true">

## Docs  

check the docs [here](https://docs.rs/suggestion_trie/0.1.0/suggestion_trie/)