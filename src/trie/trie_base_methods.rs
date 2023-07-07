use crate::suggestion::Suggestion;
use crate::trie::trie_fuzzy_search::FuzzyFunctionData;
use crate::trie::trie_structs::{TrieInputData, TrieNode, TrieRoot};
use ahash::{HashMap, HashSet};
use rand::Rng;
use std::cmp::Ordering;
use std::vec;

impl TrieNode {
    fn new(indices: Vec<u32>, nodes: Option<Vec<(String, TrieNode)>>) -> TrieNode {
        let mut rng = rand::thread_rng();
        if let Some(nds) = nodes {
            TrieNode {
                nodes: nds,
                indices,
                uid: rng.gen::<u32>(),
            }
        } else {
            TrieNode {
                nodes: Vec::new(),
                indices,
                uid: rng.gen::<u32>(),
            }
        }
    }
}

impl<T: std::clone::Clone> TrieRoot<T> {
    /// Creates a new `TrieRoot`, this is the main struct that you will use to interact with the Trie
    /// - `min_results_before_fuzzy` is used to control when the fuzzy search is used,
    /// if the search results in less than `min_results_before_fuzzy`, the fuzzy search will be used,
    /// if you want to use fuzzy only if no results are found use `min_results_before_fuzzy` as 1, zero will always fuzzy and its not recommended.
    /// - `max_node_results` is the maximum number of indexes returned by a single node. None will return all values,
    /// `max_node_results` is specially useful in shallow nodes that might return too much data, like searching for just "a" in big
    /// tries, might yield too much data for the first nodes, this depends on you application, tune to find the best results and performance.
    /// ```
    /// use suggestion_trie::{TrieRoot, TrieInputData};
    /// use suggestion_trie::Suggestion;
    /// let mut trie_root: TrieRoot<i32> = TrieRoot::<i32>::new(5, Some(100));
    /// // Get the data you want to insert into the trie
    /// let entries = vec![
    /// // Use lowercase keywords and query if you want to do case insensitive searches
    ///    TrieInputData {
    ///     suggestion: Suggestion::new("Rat".to_string(), Some(0)),
    ///     keywords: vec!["Mice".to_string(), "Rat".to_string(), "Mouse".to_string(), "Rodent".to_string()],
    ///   },
    ///  TrieInputData {
    ///     suggestion: Suggestion::new("Cat".to_string(), Some(1)),
    ///     keywords: vec!["Cat".to_string(), "Kitten".to_string(), "Kitty".to_string()],
    ///  }
    /// ];
    /// // Build the trie
    /// trie_root.build(&entries);
    ///
    /// // Search and get results
    /// let results = trie_root.get_suggestions("Rodent");
    /// assert_eq!(results.unwrap()[0].title, "Rat");
    /// ```
    /// Todo: custom CONSTRAINED_FUZZY_RATIO
    pub fn new(min_results_before_fuzzy: usize, max_node_results: Option<usize>) -> TrieRoot<T> {
        if let Some(max_node_results) = max_node_results {
            TrieRoot {
                min_results_before_fuzzy,
                root: TrieNode::new(vec![], None),
                trie_data_array: Vec::new(),
                max_node_results,
            }
        } else {
            TrieRoot {
                min_results_before_fuzzy,
                root: TrieNode::new(vec![], None),
                trie_data_array: Vec::new(),
                max_node_results: usize::MAX,
            }
        }
    }

    /// Searches for a query in the trie, if the query is found, it returns a vector of suggestions
    /// if the query is not found, it returns `None`
    ///
    /// if the returned results by the direct search are less then `min_results_before_fuzzy`
    /// the results are discarded and the fuzzy search is used instead.
    /// ```
    /// use suggestion_trie::TrieRoot;
    /// let mut trie_root: TrieRoot<i32> = TrieRoot::<i32>::new(5,Some(100));
    /// // Get your vector of suggestions and keywords on a TrieInputData struct
    /// // Build the trie using the build method with tge TrieInputData structs
    /// let results = trie_root.get_suggestions("Hello world!");
    /// if let Some(suggestions) = results {
    ///    for suggestion in suggestions {
    ///       println!("{}", suggestion.title);
    ///       // Other operations with the results...
    ///     }
    /// }
    /// ```
    pub fn get_suggestions(&self, query: &str) -> Option<Vec<Suggestion<T>>> {
        if self.min_results_before_fuzzy > 0 {
            let direct_search_results = self.search_query(query, &self.root);

            if let Some(v) = &direct_search_results {
                if v.len() >= self.min_results_before_fuzzy {
                    let ret_vec = self.resolve_indexes(v);
                    if let Some(mut result_suggestions) = ret_vec {
                        result_suggestions.sort_by(|a, b| a.title.len().cmp(&b.title.len()));
                        return Some(result_suggestions);
                    }

                    return None;
                }
            }
        }

        let swap_table = self.setup_swap_table(query);
        let mut fuzzy_data = FuzzyFunctionData {
            swap_table,
            original_len: query.len(),
            visited_nodes: HashMap::default(),
            memoize_function: HashSet::default(),
        };
        TrieRoot::<T>::search_query_fuzzy(query, &mut fuzzy_data, &self.root, 0, true, 0, 0);

        if !fuzzy_data.visited_nodes.is_empty() {
            let mut results = Vec::from_iter(fuzzy_data.visited_nodes.values());
            let ret_vec = self.resolve_indexes_search_results(&mut results);
            if let Some(mut result_suggestions) = ret_vec {
                result_suggestions.sort_by(|a, b| b.rank.cmp(&a.rank));
                return Some(result_suggestions);
            }
            return None;
        }
        None
    }

    fn add_index_to_node(node: &mut TrieNode, index: u32) {
        let nn_index = &mut node.indices;
        // check if its not empty and if the last entry is for the same index
        if (!nn_index.is_empty() && nn_index[nn_index.len() - 1] != index) || nn_index.is_empty() {
            nn_index.push(index);
        }
    }

    fn query_match_insert(
        keyword: &str,
        index: u32,
        node_pair: &(String, TrieNode),
    ) -> (String, TrieNode) {
        let (new_node_id, old_node_id) = node_pair.0.split_at(keyword.len());

        let new_old_node = (old_node_id.to_string(), node_pair.1.clone()); // rename old node as the rest
        let mut new_node = (
            new_node_id.to_string(),
            TrieNode::new(node_pair.1.indices.clone(), Some(vec![new_old_node])),
        );

        TrieRoot::<T>::add_index_to_node(&mut new_node.1, index);

        new_node
    }

    fn partial_match_insert(
        keyword: &str,
        index: u32,
        node_pair: &(String, TrieNode),
        split_point: usize,
    ) -> (String, TrieNode) {
        let (new_node_id, old_node_id) = node_pair.0.split_at(split_point);
        let (_, new_unmatched_id) = keyword.split_at(split_point);

        let new_unmatched_node = (
            new_unmatched_id.to_string(),
            TrieNode::new(vec![index], None),
        );
        let new_old_node = (old_node_id.to_string(), node_pair.1.clone()); // rename old node as the rest
        let mut new_node = (
            new_node_id.to_string(),
            TrieNode::new(
                node_pair.1.indices.clone(),
                Some(vec![new_old_node, new_unmatched_node]),
            ),
        );

        TrieRoot::<T>::add_index_to_node(&mut new_node.1, index);

        new_node
    }

    fn insert_keyword(keyword: &str, nodes_borrow: &mut Vec<(String, TrieNode)>, index: u32) {
        for (i, nd) in nodes_borrow.iter_mut().enumerate() {
            let mut node_chars = nd.0.char_indices();
            let mut query_chars = keyword.char_indices();

            if query_chars.next() != node_chars.next() {
                continue;
            }

            for chars in node_chars.zip(query_chars) {
                if chars.0 .1 != chars.1 .1 {
                    let new_node =
                        TrieRoot::<T>::partial_match_insert(keyword, index, nd, chars.0 .0);
                    nodes_borrow.swap_remove(i);
                    nodes_borrow.push(new_node);
                    return;
                }
            }

            match nd.0.len().cmp(&keyword.len()) {
                Ordering::Greater => {
                    // if the node is bigger than the query, query was fully matched
                    let new_node = TrieRoot::<T>::query_match_insert(keyword, index, nd);
                    nodes_borrow.swap_remove(i);
                    nodes_borrow.push(new_node);
                    return;
                }
                Ordering::Less => {
                    // if the node is smaller than the query, node was fully matched
                    TrieRoot::<T>::insert_keyword(&keyword[nd.0.len()..], &mut nd.1.nodes, index);
                    TrieRoot::<T>::add_index_to_node(&mut nd.1, index);
                    return;
                }
                Ordering::Equal => {
                    // if the node is the same size as the query, full match end
                    TrieRoot::<T>::add_index_to_node(&mut nd.1, index);
                    return;
                }
            }
        }
        let new_node = (keyword.to_string(), TrieNode::new(vec![index], None)); // create new node with the start Bana
        nodes_borrow.push(new_node);
    }

    /// Function that adds all suggestions to the trie
    /// Suggestions are indexed by all keywords inside the `TrieInputData`
    /// # Example
    /// For a Suggestion `Rat` you can index it by `["Mice", "Rat", "Mouse", "Rodent"]`
    /// When searching if you match any of the keywords the suggestion "Rat" will be returned
    ///
    /// ```
    /// use suggestion_trie::{TrieRoot, TrieInputData};
    /// use suggestion_trie::Suggestion;
    /// let entries = vec![
    /// // Use lowercase keywords and query if you want to do case insensitive searches
    ///    TrieInputData {
    ///     suggestion: Suggestion::new("Rat".to_string(), Some(0)),
    ///     keywords: vec!["Mice".to_string(), "Rat".to_string(), "Mouse".to_string(), "Rodent".to_string()],
    ///   },
    ///  TrieInputData {
    ///     suggestion: Suggestion::new("Cat".to_string(), Some(1)),
    ///     keywords: vec!["Cat".to_string(), "Kitten".to_string(), "Kitty".to_string()],
    ///  }
    /// ];
    ///
    /// let mut trie = TrieRoot::new(4, Some(100));
    /// trie.build(&entries);
    ///
    /// // Do your searches here
    /// ```
    /// Todo: insert_keyword with self, and change to iterative
    pub fn build(&mut self, suggestions: &[TrieInputData<T>]) {
        self.trie_data_array.reserve(suggestions.len());
        for entries in suggestions {
            self.trie_data_array.push(entries.suggestion.clone());
            for keyword in &entries.keywords {
                if keyword.is_empty() {
                    continue;
                }
                TrieRoot::<T>::insert_keyword(
                    keyword,
                    &mut (self.root.nodes),
                    (self.trie_data_array.len() - 1) as u32,
                );
            }
        }
        self.trie_data_array.shrink_to_fit();
    }
}
