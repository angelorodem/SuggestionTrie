use crate::suggestion::Suggestion;
use std::hash::{Hash, Hasher};

/// Struct used to feed the trie with data
/// 
/// It takes a vector of keywords and a suggestion, the keywords will be used to index the suggestion in the trie,
/// if any of the Keywords match, the suggestion is returned
/// 
/// # Example
/// ```
/// use suggestion_trie::Suggestion;
/// use suggestion_trie::TrieInputData;
/// let input_data = TrieInputData {
///     keywords: vec!["Hello".to_string(), "World".to_string()],
///     suggestion: Suggestion::new("Hello World!".to_string(), Some(1)),
/// };
/// 
/// // Use it with TrieRoot build method
/// ```
pub struct TrieInputData<T>  where T: std::clone::Clone {
    pub keywords: Vec<String>,
    pub suggestion: Suggestion<T>
}

#[doc(hidden)]
#[derive(Eq, Debug, Clone)]
pub(crate) struct TrieNode {
    pub nodes: Vec<(String, TrieNode)>,
    pub indices: Vec<u32>,
    pub uid: u32,
}

impl PartialEq for TrieNode {
    fn eq(&self, other: &Self) -> bool {
        self.uid == other.uid
    }    
}

impl Hash for TrieNode {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.uid.hash(state);
    }
}
    
/// Base struct for the Trie 
#[derive(Debug, Clone)]
pub struct TrieRoot<T>  where T: std::clone::Clone {
    pub(crate) min_results_before_fuzzy: usize,
    pub(crate) max_node_results: usize,
    pub(crate) root: TrieNode,
    pub(crate) trie_data_array: Vec<Suggestion<T>>,
}

#[doc(hidden)]
pub struct SearchResults {
    pub node_id: u32,
    pub indexes: Vec<u32>,
    pub score_modifier: i32,
}