use crate::suggestion::Suggestion;
use crate::trie::fuzzy_swaps::{get_query_ratio, FUZZY_CHAR_SWAPS_DATA};
use crate::trie::trie_structs::{SearchResults, TrieNode, TrieRoot};
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};

use ahash::RandomState;

const FUZZY_PENALTY_REMOVE: i32 = 200;
const FUZZY_PENALTY_ADD: i32 = 130;
const FUZZY_PENALTY_SWAP_BASE: i32 = 30;

pub(crate) struct FuzzyFunctionData<'a> {
    pub(crate) original_len: usize,
    pub(crate) swap_table: Vec<(char, Option<&'a Vec<char>>)>,
    pub(crate) visited_nodes: HashMap<u32, SearchResults, RandomState>,
    pub(crate) memoize_function: HashSet<(String, u32, i32, i32), RandomState>,
}

impl<T: std::clone::Clone> TrieRoot<T> {
    pub fn should_fuzzy_match(query_len: &usize, fuzzy_count: i32) -> bool {
        let ratio = get_query_ratio(query_len);

        ratio > 0 && ratio > fuzzy_count
    }

    /// We could return the swap score, but its not that important
    fn fuzzy_swap(
        char_find: &Option<char>,
        swap_array: &Option<&(char, Option<&Vec<char>>)>,
    ) -> bool {
        if let Some(swap) = swap_array {
            if let Some(combs) = swap.1 {
                if combs.iter().any(|x| {
                    if let Some(c) = &char_find {
                        *x == *c
                    } else {
                        false
                    }
                }) {
                    return true;
                }
                return false;
            } else {
                return false;
            }
        }
        false
    }

    fn end_of_node_compare(
        query: &str,
        fuzzy_data: &mut FuzzyFunctionData,
        local_fuzzy: &i32,
        nd: &(String, TrieNode),
        score_modifier: i32,
        new_chars: i32,
    ) {
        match nd.0.len().cmp(&query.len()) {
            Ordering::Less => {
                let add_here = TrieRoot::<T>::search_query_fuzzy(
                    &query[query.ceil_char_boundary(nd.0.len() - (new_chars as usize))..],
                    fuzzy_data,
                    &nd.1,
                    *local_fuzzy,
                    false,
                    score_modifier,
                    0,
                );

                if add_here {
                    if let Some(result_node) = fuzzy_data.visited_nodes.get_mut(&nd.1.uid) {
                        let new_score = score_modifier - (nd.0.len() as i32 * FUZZY_PENALTY_ADD);
                        if result_node.score_modifier < new_score {
                            result_node.score_modifier = new_score;
                        }
                    } else {
                        fuzzy_data.visited_nodes.insert(
                            nd.1.uid,
                            SearchResults {
                                node_id: nd.1.uid,
                                indexes: nd.1.indices.clone(),
                                score_modifier: score_modifier
                                    - (nd.0.len() as i32 * FUZZY_PENALTY_ADD),
                            },
                        );
                    }
                }
            }
            _ => {
                if let Some(result_node) = fuzzy_data.visited_nodes.get_mut(&nd.1.uid) {
                    if result_node.score_modifier < score_modifier {
                        result_node.score_modifier = score_modifier;
                    }
                } else {
                    fuzzy_data.visited_nodes.insert(
                        nd.1.uid,
                        SearchResults {
                            node_id: nd.1.uid,
                            indexes: nd.1.indices.clone(),
                            score_modifier,
                        },
                    );
                }
            }
        }
    }

    pub(crate) fn search_query_fuzzy(
        query: &str,
        fuzzy_data: &mut FuzzyFunctionData,
        node_borrow: &TrieNode,
        fuzzy_count: i32,
        ignore_first: bool,
        score_modifier: i32,
        node_skip: usize,
    ) -> bool {
        {
            let set = &mut fuzzy_data.memoize_function;
            if set.contains(&(
                query.to_string(),
                node_borrow.uid,
                fuzzy_count,
                score_modifier,
            )) {
                return false;
            }
            set.insert((
                query.to_string(),
                node_borrow.uid,
                fuzzy_count,
                score_modifier,
            ));
        }

        TrieRoot::<T>::search_query_fuzzy_original(
            query,
            fuzzy_data,
            node_borrow,
            fuzzy_count,
            ignore_first,
            score_modifier,
            node_skip,
        )
    }

    pub(crate) fn modify_query_remove(query: &str, position: usize) -> String {
        let upper_cut = query.ceil_char_boundary(position + 1);
        [&query[..position], &query[upper_cut..]].concat()
    }

    pub(crate) fn modify_query_fuzzy(
        node_char: &Option<char>,
        query_char: &Option<char>,
        fuzzy_data: &FuzzyFunctionData,
        query: &str,
        position: usize,
    ) -> Option<String> {
        let f_char = query_char.unwrap();
        let can_swap = TrieRoot::<T>::fuzzy_swap(
            node_char,
            &fuzzy_data.swap_table.iter().find(|x| x.0 == f_char),
        );

        if can_swap {
            let upper_cut = query.ceil_char_boundary(position + 1);

            let mut next_query = query.to_string();
            next_query.replace_range(position..upper_cut, &node_char.unwrap().to_string());
            return Some(next_query);
        }
        None
    }

    // TODO:
    // change from recursive to iterative
    // Speedup by using a mask instead of removing chars from the query (it probably needs to change from recursive to iterative)
    // make ignore first optional
    // use levenshtein distance instead of recursive? lev > max_fuzzy
    pub(crate) fn search_query_fuzzy_original(
        query: &str,
        fuzzy_data: &mut FuzzyFunctionData,
        node_borrow: &TrieNode,
        fuzzy_count: i32,
        ignore_first: bool,
        score_modifier: i32,
        node_skip: usize,
    ) -> bool {
        if query.is_empty() {
            return true;
        }

        let query_iterator = query.chars();
        let mut nodes_iter = node_borrow.nodes.iter().skip(node_skip).enumerate();
        'node_loop: loop {
            let mut local_fuzzy = fuzzy_count;
            let current_node: Option<(usize, &(String, TrieNode))> = nodes_iter.next();

            // The following if, is a check to see if we still can match the previous node if we have fuzzy swaps available,
            // if we can fully erase the number of chars that this node have, we can still match the previous node.
            // But we should only do this at the end of the query and node,
            if current_node.is_none() {
                // Check if we have fuzzy swaps available
                let should_fuzzy_match_last = TrieRoot::<T>::should_fuzzy_match(
                    &fuzzy_data.original_len,
                    // What we fuzzed already + the remaining chars
                    local_fuzzy + (query.len() - 1) as i32,
                );

                return should_fuzzy_match_last;
            }

            let nd = current_node.unwrap();
            let mut block_query_iterator = true; // since we load the first char into the iterator, we need to skip it once at start
            let mut local_score = score_modifier;
            let mut new_chars = 0;

            // Node and query string iterators

            let mut node_chars = nd.1 .0.chars();

            let mut query_chars = query_iterator.clone();
            let mut query_char = query_chars.next();
            let mut char_pos = 0;

            loop {
                let node_char = node_chars.next();

                if block_query_iterator {
                    block_query_iterator = false;
                } else {
                    query_char = query_chars.next();
                }

                // fuzzy happen for query as long as there is chars in the node,
                // i need to find a way to allow fuzzy remove chars that exceed the existence of the node chars
                if node_char.is_none() || query_char.is_none() {
                    break;
                }

                if node_char != query_char {
                    if (char_pos == 0 && ignore_first)
                        || !TrieRoot::<T>::should_fuzzy_match(&fuzzy_data.original_len, local_fuzzy)
                        || char_pos >= query.len()
                    {
                        if node_skip > 0 {
                            // if we fail inside a fuzzy attempt we just go back to the node we were before
                            return false;
                        }
                        continue 'node_loop;
                    }

                    // Fuzzy REMOVE
                    let modified_query = TrieRoot::<T>::modify_query_remove(query, char_pos);
                    TrieRoot::<T>::search_query_fuzzy(
                        &modified_query,
                        fuzzy_data,
                        node_borrow,
                        local_fuzzy + 1,
                        false,
                        local_score - FUZZY_PENALTY_REMOVE,
                        nd.0,
                    );

                    // Fuzzy SWAP
                    if let Some(next_query) = TrieRoot::<T>::modify_query_fuzzy(
                        &node_char,
                        &query_char,
                        fuzzy_data,
                        query,
                        char_pos,
                    ) {
                        TrieRoot::<T>::search_query_fuzzy(
                            &next_query,
                            fuzzy_data,
                            node_borrow,
                            local_fuzzy + 1,
                            false,
                            local_score - FUZZY_PENALTY_SWAP_BASE,
                            nd.0,
                        );
                    }

                    // Fuzzy ADD
                    // We assume the current char that is wrong as right as if we added a char to compensate it,
                    // so we block the query iterator so we advance only the node and assume we matched
                    // this way we don't need to copy query again
                    local_score -= FUZZY_PENALTY_ADD;
                    local_fuzzy += 1;
                    block_query_iterator = true;
                    new_chars += 1;
                }
                if char_pos + 1 < query.len() {
                    char_pos = query.ceil_char_boundary(char_pos + 1);
                } else {
                    break;
                }
            }

            // if iterator finishes without mismatch we have one of the below
            // compare Node length against query length
            TrieRoot::<T>::end_of_node_compare(
                query,
                fuzzy_data,
                &local_fuzzy,
                nd.1,
                local_score,
                new_chars,
            );
        }
    }

    pub(crate) fn setup_swap_table(&self, query: &str) -> Vec<(char, Option<&Vec<char>>)> {
        let mut ret = Vec::with_capacity(query.len());

        for c in query.chars() {
            ret.push((c, (FUZZY_CHAR_SWAPS_DATA.get(&c))));
        }

        ret.sort_by(|a, b| a.0.cmp(&b.0));
        ret.dedup_by(|a, b| a.0 == b.0);
        ret
    }

    // Function used mostly for bench and debugging, use get_suggestions instead
    pub fn force_fuzzy_search(&self, query: &str) -> Option<Vec<SearchResults>> {
        let swap_table = self.setup_swap_table(query);
        let mut fuzzy_data = FuzzyFunctionData {
            swap_table,
            original_len: query.len(),
            visited_nodes: HashMap::default(),
            memoize_function: HashSet::default(),
        };
        TrieRoot::<T>::search_query_fuzzy(query, &mut fuzzy_data, &self.root, 0, false, 0, 0);

        if !fuzzy_data.visited_nodes.is_empty() {
            let res = Vec::from_iter(fuzzy_data.visited_nodes.into_values());
            return Some(res);
        }
        None
    }

    pub(crate) fn resolve_indexes_search_results(
        &self,
        search_results: &mut Vec<&SearchResults>,
    ) -> Option<Vec<Suggestion<T>>> {
        let mut ret_vec = Vec::new();
        for sr in search_results {
            for index in sr.indexes.iter() {
                if *index < self.trie_data_array.len() as u32 {
                    let mut suggs = self.trie_data_array[*index as usize].clone();
                    suggs.rank += sr.score_modifier - (suggs.title.len() as i32);
                    suggs.index = *index;
                    ret_vec.push(suggs);
                }
            }
        }

        if ret_vec.is_empty() {
            None
        } else {
            // sort answers by index and order by score_modifier for dedupe
            ret_vec.sort_by(|a, b| match a.index.cmp(&b.index) {
                Ordering::Equal => b.rank.cmp(&a.rank),
                other => other,
            });
            ret_vec.dedup_by(|a, b| a.index == b.index);

            Some(ret_vec)
        }
    }
}
