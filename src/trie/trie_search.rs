use crate::suggestion::Suggestion;
use crate::trie::trie_structs::{TrieNode, TrieRoot};
use std::cmp::Ordering;

impl<T: std::clone::Clone> TrieRoot<T> {
    pub fn non_fuzzy_search(&self, query: &str) -> Option<Vec<u32>> {
        self.search_query(query, &self.root)
    }

    pub(crate) fn search_query(&self, mut query: &str, mut node_borrow: &TrieNode) -> Option<Vec<u32>> {
        let mut i = 0;
        'main_loop: while i < node_borrow.nodes.len() {
            let current_node = &node_borrow.nodes[i];
            let mut node_chars = current_node.0.chars();
            let mut query_chars = query.chars();

            // if the node and query do not match
            if node_chars.next() != query_chars.next() {
                i += 1;
                continue;
            }

            // 1. check for partial match
            for chars in node_chars.zip(query_chars) {
                if chars.0 != chars.1 {
                    return None;
                }
            }

            // if iterator finishes without mismatch we have one of the below
            match current_node.0.len().cmp(&query.len()) {
                Ordering::Less => {
                    query = &query[current_node.0.len()..];
                    node_borrow = &current_node.1;
                    i = 0;
                    continue 'main_loop;
                },
                // if node is greater than query, we have partial match, if equal we have a match
                
                _ => {
                    let len = usize::min(current_node.1.indices.len(), self.max_node_results);
                    if current_node.1.indices.len() > self.max_node_results {
                        return Some(Vec::from(&current_node.1.indices[..len]));
                    } else {
                        return Some(current_node.1.indices.clone())
                    }
                },
            }
        }

        None
    }

    pub fn resolve_indexes(&self, indexes: &[u32]) -> Option<Vec<Suggestion<T>>> {
        let mut ret_vec = Vec::new();
        for index in indexes.iter() {
            if *index < self.trie_data_array.len() as u32 {
                ret_vec.push(self.trie_data_array[*index as usize].clone());
            }
        }

        if ret_vec.is_empty() {
            None
        } else {
            Some(ret_vec)
        }
    }
}
