/// This struct is the data that is yielded after a search match, it takes a generic type,
/// this is so that you can store any data you want in the suggestion.
/// 
/// You will mainly use this in conjunction with the TrieInputData, where you will crate a suggestion with
/// a set of keywords that will be used to index this suggestion in the Trie.
/// # Example
/// ```
/// use suggestion_trie::suggestion::Suggestion;
/// let suggestion = Suggestion::new("Hello".to_string(), Some(1));
/// 
/// // Add to TrieInputData
/// ```
#[derive(Debug, Clone, Default)]
pub struct Suggestion<T> where T: std::clone::Clone {
    /// Suggestion title, this is what is usually displayed to the user (if needed)
    pub title: String,
    /// Random data that might be associated with the suggestion
    pub data: Option<T>,
    /// Used to order the display of the suggestions
    pub(crate) rank: i32,
    /// What is the position of this suggestion in the suggestion vector
    pub(crate) index: u32,
}

impl<T: std::clone::Clone> Suggestion<T> {
    /// Creates a new suggestion, has the option to add the a data hashmap that can be used to store any data in the suggestion
    pub fn new(title: String, data: Option<T>) -> Suggestion<T> {
        Suggestion {
            title,
            data,
            rank: 0,
            index: 0,
        }
    }
}






