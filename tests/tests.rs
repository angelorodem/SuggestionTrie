#[cfg(test)]

//TODO: Remake this test file, it's a mess (it does not pass because of the CONSTRAINED_FUZZY_RATIO, each time i fine tune it it breaks the tests)
mod tests {
    use core::panic;
    use std::{vec};
    use hashbrown::HashMap;
    use suggestion_trie::suggestion::*;
    use suggestion_trie::trie::trie_structs::*;

    fn trie_test_data(keyword: String) -> TrieInputData<HashMap<String,String>> {
        TrieInputData {
            keywords: vec![keyword.clone()], // duplicating on passing the same node
            suggestion: Suggestion::new(keyword, None),
        }
    }

    // insert weird cases in the trie and test if they are found, indexes are based on position of the SuggestionEntry in the vector
    fn setup() -> TrieRoot<HashMap<String,String>> {
        let mut root = TrieRoot::new(4, Some(100));
        let new_suggestions = vec![
            trie_test_data("".to_string()),               // 0
            trie_test_data("aaaaaaaa".to_string()),       // 1
            trie_test_data("".to_string()),               // 2
            trie_test_data("a".to_string()),              // 3
            trie_test_data("aaaaaa".to_string()),         // 4
            trie_test_data("aaaaaaab".to_string()),       // 5
            trie_test_data("baaaaa".to_string()),         // 6
            trie_test_data("aaaabaaa".to_string()),       // 7
            trie_test_data("aaa".to_string()),            // 8
            trie_test_data("aaa".to_string()),            // 9
            trie_test_data("aab".to_string()),            // 10
            trie_test_data("aaba".to_string()),           // 11
            trie_test_data("aaca".to_string()),           // 12
            trie_test_data("aa a".to_string()),           // 13
            trie_test_data("aa b".to_string()),           // 14
            trie_test_data("aa a".to_string()),           // 15
            trie_test_data("a😀世a aa".to_string()),      // 16
            trie_test_data("xxxx".to_string()),           // 17
            trie_test_data("xxx".to_string()),            // 18
            trie_test_data("xxyx".to_string()),           // 19
            trie_test_data("xx&x".to_string()),           // 20
            trie_test_data("xx(x".to_string()),           // 21
            trie_test_data("xx-x".to_string()),           // 22
            trie_test_data("xx x".to_string()),           // 23
            trie_test_data("x".to_string()),              // 24
            trie_test_data("".to_string()),               // 25
            trie_test_data("a😀世a aa".to_string()),      // 26
            trie_test_data("\n".to_string()),             // 27
            trie_test_data("".to_string()),               // 28
            trie_test_data("😀".to_string()),             // 29
            trie_test_data("😀😀".to_string()),           // 30
            trie_test_data("😀😀😀".to_string()),         // 31
            trie_test_data("😀😀".to_string()),           // 32
            trie_test_data("😀🫠".to_string()),           // 33
            trie_test_data("😀🍆".to_string()),           // 34
            trie_test_data("++++++".to_string()),         // 35
            trie_test_data("+++++".to_string()),          // 36
            trie_test_data("+++-+".to_string()),          // 37
            trie_test_data("++++++".to_string()),         // 38
            trie_test_data("+++-----".to_string()),       // 39
            trie_test_data("test".to_string()),           // 40
            trie_test_data("fest".to_string()),           // 41
            trie_test_data("teste".to_string()),          // 42
            trie_test_data("y".to_string()),              // 43
            trie_test_data("yy".to_string()),             // 44
            trie_test_data("hello friend".to_string()),   // 45
            trie_test_data("hello world".to_string()),    // 46
            trie_test_data("hello friends!".to_string()), // 47
            trie_test_data("hello worl".to_string()),     // 48
            trie_test_data("help world".to_string()),     // 49
            trie_test_data("help o world".to_string()),   // 50
            trie_test_data("an nice cat".to_string()),    // 51
            trie_test_data("an nice dog".to_string()),    // 52
            trie_test_data("an evil cat".to_string()),    // 53
            trie_test_data("an evil".to_string()),        // 54
            trie_test_data("an nice".to_string()),        // 55
            trie_test_data("an nice ".to_string()),       // 56
            TrieInputData {
                // 57
                keywords: vec![
                    "a cow".to_string(),
                    "a cow".to_string(),
                    "a cow".to_string(),
                    "a cow".to_string(),
                ], // duplicating
                suggestion: Suggestion::new("test".to_string(), None),
            },
            trie_test_data("a cow ".to_string()),       // 58
            trie_test_data("Ângelo".to_string()),       // 59
            trie_test_data("ÂÂngelo".to_string()),      // 60
            trie_test_data("ÂÂÂngeloço".to_string()),   // 61
            trie_test_data("ÂÂÂngeloçoo®".to_string()), // 62
            trie_test_data("ÂÂÂngeloçàoÖý°ËŒ©50¢×®™☕✓".to_string()), // 63
            trie_test_data("ÂÂÂngeloçàoÖý°ËŒ©50¢×™☕✓".to_string()), // 64
            trie_test_data("ÂÂÂngeloçàoÖý°ËŒ©50¢×☕✓".to_string()), // 65
            trie_test_data("ÂÂÂngeloçàoÖý°ËŒ©50¢×✓".to_string()), // 66
            trie_test_data("ÂÂÂngeloçàoÖý°ËŒ©50¢×".to_string()), // 67
            trie_test_data("ÂÂÂngeloçàoÖý°ËŒ©50".to_string()), // 68
            trie_test_data("ÂÂÂngeloçàoÖý°CËŒ©50".to_string()), // 69
            trie_test_data("ÂÂÂngeloçàoÖý°ËKŒ©50".to_string()), // 70
            trie_test_data("こんにちは世界.doc".to_string()), // 71
            trie_test_data("こんにちは友人.doc".to_string()), // 72
            trie_test_data("你好伙伴".to_string()),     // 73
            trie_test_data("你好世界".to_string()),     // 74
            trie_test_data("नमस्ते".to_string()),         // 75
        ];

        root.build(new_suggestions.as_slice());

        root
    }

    fn setup_fuzzy() -> TrieRoot<HashMap<String,String>> {
        let mut root = TrieRoot::new(4, Some(100));

        let entries = vec![
            trie_test_data("++++++".to_string()),                  // 0
            trie_test_data("+++++".to_string()),                   // 1
            trie_test_data("+++-+".to_string()),                   // 2
            trie_test_data("++++++".to_string()),                  // 3
            trie_test_data("+++-----".to_string()),                // 4
            trie_test_data("hello".to_string()),                   // 5
            trie_test_data("world".to_string()),                   // 6
            trie_test_data("hello world".to_string()),             // 7
            trie_test_data("hello world!".to_string()),            // 8
            trie_test_data("helloo".to_string()),                  // 9
            trie_test_data("helloo world!".to_string()),           // 10
            trie_test_data("helloo world".to_string()),            // 11
            trie_test_data("aaaaaaaaaaaa".to_string()),            // 12
            trie_test_data("jjjjjjjjjjjj".to_string()),            // 13
            trie_test_data("aaaa".to_string()),                    // 14
            trie_test_data("jjjj".to_string()),                    // 15
            trie_test_data("a".to_string()),                       // 16
            trie_test_data("j".to_string()),                       // 17
            trie_test_data("aa".to_string()),                      // 18
            trie_test_data("jj".to_string()),                      // 19
            trie_test_data("aaa".to_string()),                     // 20
            trie_test_data("jjj".to_string()),                     // 21
            trie_test_data("as".to_string()),                      // 22
            trie_test_data("aaaaaaas".to_string()),                // 23
            trie_test_data("helloworld".to_string()),              // 24
            trie_test_data("hello friends".to_string()),           // 25
            trie_test_data("help world".to_string()),              // 26
            trie_test_data("abcdefgh".to_string()),                // 27
            trie_test_data("abcdefg".to_string()),                 // 28
            trie_test_data("abcdef".to_string()),                  // 29
            trie_test_data("abcdefghijkl".to_string()),            // 30
            trie_test_data("iiiiiiiiii".to_string()),              // 31
            trie_test_data("iiiiiiiiiii".to_string()),             // 32
            trie_test_data("iiiiiiiiiiii".to_string()),            // 33
            trie_test_data("iiiiiiiiiiiii".to_string()),           // 34
            trie_test_data("iiiiiiiiiiii".to_string()),            // 35
            trie_test_data("iiiiiiiiiii".to_string()),             // 36
            trie_test_data("iiiiiiiiii".to_string()),              // 37
            trie_test_data("iiiiiiiii".to_string()),               // 38
            trie_test_data("iiiiiiii".to_string()),                // 39
            trie_test_data("iiiiiii".to_string()),                 // 40
            trie_test_data("iiiiiiiiio".to_string()),              // 41
            trie_test_data("iiiiiiiiiio".to_string()),             // 42
            trie_test_data("iiiiiiiiiiio".to_string()),            // 43
            trie_test_data("iiiiiiiiiiiio".to_string()),           // 44
            trie_test_data("iiiiiiiiiiio".to_string()),            // 45
            trie_test_data("iiiiiiiiiio".to_string()),             // 46
            trie_test_data("iiiiiiiiio".to_string()),              // 47
            trie_test_data("iiiiiiiio".to_string()),               // 48
            trie_test_data("iiiiiiio".to_string()),                // 49
            trie_test_data("iiiiiio".to_string()),                 // 50
            trie_test_data("iiiiiiiiiz".to_string()),              // 51
            trie_test_data("iiiiiiiiiiz".to_string()),             // 52
            trie_test_data("iiiiiiiiiiiz".to_string()),            // 53
            trie_test_data("iiiiiiiiiiiiz".to_string()),           // 54
            trie_test_data("ppppppppppppppp".to_string()),         // 55
            trie_test_data("ppppppppppppppp".to_string()),         // 56
            trie_test_data("iiiiiiiiiz".to_string()),              // 57
            trie_test_data("iiiiiiiiz".to_string()),               // 58
            trie_test_data("iiiiiiiz".to_string()),                // 59
            trie_test_data("iiiiiiz".to_string()),                 // 60
            trie_test_data("my nice spaceship".to_string()),       // 61
            trie_test_data("my niece spaceship".to_string()),      // 62
            trie_test_data("my nice niece spaceship".to_string()), // 63
            trie_test_data("my nike".to_string()),                 // 64
            trie_test_data("my nice space flight".to_string()),    // 65
            trie_test_data("may see it".to_string()),              // 66
            trie_test_data("may seeing it".to_string()),           // 67
            trie_test_data("matt".to_string()),                    // 68
            trie_test_data("nay see it".to_string()),              // 69
            trie_test_data("may ser it".to_string()),              // 70
            trie_test_data("qxevtnuluntdqxtnig".to_string()),      // 71
            trie_test_data("xqnigebqo".to_string()),               // 72
            trie_test_data("xqnigebwo".to_string()),               // 73
            trie_test_data("xqnigebeo".to_string()),               // 74
            trie_test_data("xqnigebdo".to_string()),               // 75
            trie_test_data("xqnigebco".to_string()),               // 76
            trie_test_data("xqnigebxo".to_string()),               // 77
            trie_test_data("xqnigebzo".to_string()),               // 78
            trie_test_data("xqnigebao".to_string()),               // 79
            trie_test_data("xqnigebso".to_string()),               // 80
            trie_test_data("list of nice mooses".to_string()),     // 81
            trie_test_data("list of nice mices".to_string()),      // 82
            trie_test_data("list of nice macros".to_string()),     // 83
            trie_test_data("list of neices".to_string()),          // 84
            trie_test_data("list off ".to_string()),               // 85
            trie_test_data("list of nicer mices".to_string()),     // 86
            trie_test_data("list of nicer".to_string()),           // 87
            trie_test_data("look at this 𓅂߷æ世 so nice".to_string()), // 88
            trie_test_data("look at this ߷ææ世 so nice".to_string()), // 89
            trie_test_data("look at this æ߷æ世 so nice".to_string()), // 90
            trie_test_data("look at this 世ææ世 so nice".to_string()), // 91
            trie_test_data("世世世世世世".to_string()), // 92
            trie_test_data("世世世世世".to_string()), // 93
            trie_test_data("世世世世".to_string()), // 94
            trie_test_data("世߷世߷世߷世߷世".to_string()), // 95
            trie_test_data("世߷世߷世߷世߷".to_string()), // 96
            trie_test_data("世߷世߷世߷世".to_string()), // 97
            trie_test_data("世߷世߷世߷𓅂".to_string()), // 98
            trie_test_data("世߷世߷世߷世߷世𓅂".to_string()), // 99
            trie_test_data("世߷世߷世߷世߷𓅂".to_string()), // 100
            trie_test_data("世߷世߷世߷世𓅂".to_string()), // 101
            trie_test_data("世߷世߷世߷𓅂".to_string()), // 102
            trie_test_data("世߷世߷世߷世߷𓅂𓅂".to_string()), // 103
            trie_test_data("世߷世߷世߷世𓅂𓅂".to_string()), // 104
            trie_test_data("世߷世߷世߷𓅂世".to_string()), // 105            
            trie_test_data("世߷世߷世߷世߷æ𓅂".to_string()), // 106
            trie_test_data("世߷世߷世߷世𓅂æ".to_string()), // 107
            trie_test_data("世߷世߷世߷æ世".to_string()), // 108
            trie_test_data("世߷世߷世߷æ".to_string()), // 109
            trie_test_data("世߷世߷世æ世".to_string()), // 110
            trie_test_data("世߷世߷世æ".to_string()), // 111
            trie_test_data("世߷世߷æ世".to_string()), // 112
            trie_test_data("السلام عليكم".to_string()), // 113
            trie_test_data("Dobrý den".to_string()), // 114
            trie_test_data("שָׁלוֹם".to_string()), // 116
            trie_test_data("नमस्ते".to_string()), // 117
            trie_test_data("こんにちは".to_string()), // 118
            trie_test_data("안녕하세요".to_string()), // 119
            trie_test_data("你好".to_string()), // 120
            trie_test_data("Olá".to_string()), // 121
            trie_test_data("Здравствуйте".to_string()), // 122
            trie_test_data("Hola".to_string()), // 123
        ];

        root.build(entries.as_slice());

        root
    }

    fn search_resolve(root: &TrieRoot<HashMap<String,String>>, query: &str) {
        let result = root.non_fuzzy_search(query);
        let suggestions = root.resolve_indexes(&result.unwrap());

        if let Some(s) = suggestions {
            for suggs in s {
                if suggs.title == query {
                    dbg!(suggs.title);
                    dbg!(query);
                    return;
                }
            }
        }
        panic!();
    }

    fn search_true(root: &TrieRoot<HashMap<String,String>>, query: &str, expect: Vec<u32>) {
        let result = root.non_fuzzy_search(query);
        if let Some(r) = result {
            debug_assert_eq!(r, expect);
        } else if !expect.is_empty() {
            panic!();
        }
    }

    fn search_false(root: &TrieRoot<HashMap<String,String>>, query: &str) {
        let result = root.non_fuzzy_search(query);
        if let Some(r) = result {
            assert!(r.is_empty());
        }
    }   

    fn search_fuzzy_true(root: &TrieRoot<HashMap<String,String>>, query: &str, mut expect: Vec<u32>) {
        let result = root.force_fuzzy_search(query);
        if let Some(mut r) = result {
            let mut results: Vec<u32> = vec![];
            for i in r.iter_mut() {
                results.extend(i.indexes.iter());
            }
                results.sort_unstable();
                results.dedup();
                expect.sort_unstable();
                if results != expect {                    
                    panic!(
                        "{} -> {} {:?} expected -> {:?}",
                        query, "Unmatched", results, expect
                    )
                }
            
        } else if !expect.is_empty() {
            panic!("{} -> {}", &query, "Unmatched (empty)");
        }
    }

    #[test]
    fn search_fuzzy_swap_test() {
        let root = setup_fuzzy();
        search_fuzzy_true(&root, "Hellp", vec![]);
        search_fuzzy_true(&root, "Worlf", vec![]);
        search_fuzzy_true(&root, "hellp world", vec![7, 8, 10, 11, 24, 26]);
        search_fuzzy_true(&root, "hello eorld", vec![7, 8, 10, 11, 24, 26]);
        search_fuzzy_true(&root, "hello eprld", vec![7, 8, 10, 11, 24]);
        search_fuzzy_true(&root, "helloo", vec![5, 7, 8, 9, 10, 11, 24, 25]);
        search_fuzzy_true(&root, "hellop", vec![5, 7, 8, 9, 10, 11, 24, 25]);
        search_fuzzy_true(&root, "hellpp", vec![]);
        search_fuzzy_true(&root, "helkpp", vec![]);
        search_fuzzy_true(&root, "hellpo", vec![5, 7, 8, 9, 10, 11, 24, 25]);
        search_fuzzy_true(&root, "helkoo", vec![9, 10, 11]);
        search_fuzzy_true(&root, "heoloo", vec![9, 10, 11]);
        search_fuzzy_true(&root, "hrlloo", vec![9, 10, 11]);

        search_fuzzy_true(&root, "saaaaaaaaaaa", vec![12]);
        search_fuzzy_true(&root, "asaaaaaaaaaa", vec![12]);
        search_fuzzy_true(&root, "aasaaaaaaaaa", vec![12]);
        search_fuzzy_true(&root, "aaasaaaaaaaa", vec![12]);
        search_fuzzy_true(&root, "aaaasaaaaaaa", vec![12]);
        search_fuzzy_true(&root, "aaaaasaaaaaa", vec![12]);
        search_fuzzy_true(&root, "aaaaaasaaaaa", vec![12]);
        search_fuzzy_true(&root, "aaaaaaasaaaa", vec![12]);
        search_fuzzy_true(&root, "aaaaaaaasaaa", vec![12]);
        search_fuzzy_true(&root, "aaaaaaaaasaa", vec![12]);
        search_fuzzy_true(&root, "aaaaaaaaaasa", vec![12]);
        search_fuzzy_true(&root, "aaaaaaaaaaas", vec![12]);
        search_fuzzy_true(&root, "asaaaaaaaaas", vec![12]);
        search_fuzzy_true(&root, "aasaaaaaaaas", vec![12]);

        search_fuzzy_true(&root, "aaaaaaaaaas", vec![12,23]);
        search_fuzzy_true(&root, "aaaaaaaaas", vec![12, 23]);
        search_fuzzy_true(&root, "aaaaaaaas", vec![12, 23]);
        search_fuzzy_true(&root, "aaaaaaas", vec![12, 23]);
        search_fuzzy_true(&root, "aaaaaas", vec![12, 23]);
        search_fuzzy_true(&root, "aaaas", vec![12,14,23]);
        search_fuzzy_true(&root, "aaas", vec![12, 14, 20, 23]);
        search_fuzzy_true(&root, "aas", vec![]);
        search_fuzzy_true(&root, "as", vec![22]);
        search_fuzzy_true(&root, "s", vec![]);
        search_fuzzy_true(&root, "aad", vec![]);

        search_fuzzy_true(&root, "abcdefghk", vec![27, 28, 30]);
        search_fuzzy_true(&root, "abcdefgx", vec![27, 28, 29, 30]);
        search_fuzzy_true(&root, "abcdefghijklxx", vec![30]);
        search_fuzzy_true(&root, "abcdefgh", vec![27, 28, 29, 30]);
        search_fuzzy_true(&root, "abcdefgj", vec![27, 28, 29, 30]);
        search_fuzzy_true(&root, "abcdefg", vec![27, 28, 29, 30]);
        search_fuzzy_true(&root, "abcdeff", vec![27, 28, 29, 30]);
        search_fuzzy_true(&root, "abcdef", vec![27, 28, 29, 30]);
        search_fuzzy_true(&root, "abcded", vec![27, 28, 29, 30]);
        search_fuzzy_true(&root, "abcded", vec![27, 28, 29, 30]);
    }

    #[test]
    fn search_fuzzy_delete_test() {
        let root = setup_fuzzy();

        search_fuzzy_true(&root, "aaaaaaaaaaaaa", vec![12]);
        search_fuzzy_true(&root, "hello wworld", vec![7, 8, 10, 11, 24, 26]);
        search_fuzzy_true(&root, "hello  world", vec![7, 8, 10, 11, 24, 26]);
        search_fuzzy_true(&root, "helloo world", vec![7, 8, 10, 11, 24, 26]);
        search_fuzzy_true(&root, "helloo worrld", vec![7, 8, 10, 11, 24]);
        search_fuzzy_true(&root, "hellloo world", vec![7, 8, 10, 11, 24]);

        search_fuzzy_true(&root, "hello world", vec![7, 8, 10, 11, 24, 26]);
        search_fuzzy_true(&root, "helhlo wworld", vec![7, 8, 10, 11, 24]);
        search_fuzzy_true(&root, "hehllo world", vec![7, 8, 10, 11, 24, 26]);
        search_fuzzy_true(&root, "hellko world", vec![7, 8, 10, 11, 24, 26]);
        search_fuzzy_true(&root, "help world", vec![7, 8, 10, 11, 24, 26]);

        search_fuzzy_true(
            &root,
            "iiiiiiiiii",
            vec![
                31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 51, 52, 53, 54, 57, 58, 59
            ],
        );
        search_fuzzy_true(&root, "iiuiiiz", vec![60]);
        search_fuzzy_true(&root, "iiuoiiz", vec![]);

        search_fuzzy_true(&root, "iiiioiiiuiiiz", vec![31, 32, 33, 34, 35, 36, 37, 42, 43, 44, 45, 46, 51, 52, 53, 54, 57]);
        search_fuzzy_true(&root, "iiiibiiiuiiiz", vec![31, 32, 33, 34, 35, 36, 37, 42, 43, 44, 45, 46, 51, 52, 53, 54, 57]);

        search_fuzzy_true(&root, "qxevtnuluntdqxtnit", vec![71]);
        search_fuzzy_true(&root, "qxevtnuluntdqxtniy", vec![71]);
        search_fuzzy_true(&root, "qxevtnuluntdqxtnih", vec![71]);
        search_fuzzy_true(&root, "qxevtnuluntdqxtnin", vec![71]);
        search_fuzzy_true(&root, "qxevtnuluntdqxtnib", vec![71]);
        search_fuzzy_true(&root, "qxevtnuluntdqxtniv", vec![71]);
        search_fuzzy_true(&root, "qxevtnuluntdqxtnif", vec![71]);

        search_fuzzy_true(&root, "xqnigebso", vec![72, 73, 74, 75, 76, 77, 78, 79, 80]);
    }

    #[test]
    fn search_fuzzy_add_test() {
        let root = setup_fuzzy();

        search_fuzzy_true(&root, "helo world", vec![7, 8, 10, 11, 24, 26]);
        search_fuzzy_true(&root, "hell world", vec![7, 8, 10, 11, 24, 26]);
        search_fuzzy_true(&root, "hllo world", vec![7, 8, 10, 11, 24, 26]);
        search_fuzzy_true(&root, "hlo world!", vec![7, 8, 10, 26]);
        search_fuzzy_true(&root, "hello orld", vec![7, 8, 10, 11, 24, 25, 26]);
        search_fuzzy_true(&root, "hello wrld", vec![7, 8, 10, 11, 24, 25, 26]);
        search_fuzzy_true(&root, "hello wold", vec![7, 8, 10, 11, 24, 26]);
        search_fuzzy_true(&root, "hello word", vec![7, 8, 10, 11, 24, 25, 26]);
    }

    #[test]
    fn search_fuzzy_moose_test() {
        let root = setup_fuzzy();

        // Deletes (needs adds)
        search_fuzzy_true(&root, "list of nice mooses", vec![81, 82, 83, 86]);
        search_fuzzy_true(&root, "list of nice moses", vec![81, 82, 83, 86]);
        search_fuzzy_true(&root, "list of nice moes", vec![81, 82, 83, 86]);
        search_fuzzy_true(&root, "list of nice moos", vec![81, 82, 83, 86]);
        search_fuzzy_true(&root, "list of nice oses", vec![81, 82, 83]);
        search_fuzzy_true(&root, "list of ice ooses", vec![81]);
        search_fuzzy_true(&root, "list of nic ooses", vec![81]);
        search_fuzzy_true(&root, "list of niceooses", vec![81]);
        search_fuzzy_true(&root, "lis of nicemooses", vec![81]);
        search_fuzzy_true(&root, "list of nicmooses", vec![81]);

        // Adds (needs deletes) (not swappable)
        search_fuzzy_true(&root, "list of niiice mooses", vec![81]);
        search_fuzzy_true(&root, "list of nicee mooses", vec![81, 82, 86]);
        search_fuzzy_true(&root, "list of nice  mgooses", vec![81]);
        search_fuzzy_true(&root, "list of nice gmooses", vec![81, 82]);
        search_fuzzy_true(&root, "list of nicex mooses", vec![81, 82]);
        search_fuzzy_true(&root, "list of nicex wmooses", vec![81]);

        //list of nice mooses 𓅂߷æ世

        search_fuzzy_true(&root, "list of nice ߷mooses", vec![81, 82]); // 1-byte unicode
        search_fuzzy_true(&root, "list of nice ߷߷mooses", vec![81]); // 1-byte unicode
        search_fuzzy_true(&root, "list of nice߷ ߷mooses", vec![81]); // 1-byte unicode
        search_fuzzy_true(&root, "list of nice߷ mo߷oses", vec![81]); // 1-byte unicode

        search_fuzzy_true(&root, "list of nice mæooses", vec![81, 82]); // 2-byte unicode
        search_fuzzy_true(&root, "list of nice ևևmooses", vec![81]); // 2-byte unicode
        search_fuzzy_true(&root, "list of nice ևmooses", vec![81, 82]); // 2-byte unicode
        search_fuzzy_true(&root, "list of nice æևmooses", vec![81]); // 2-byte unicode
        search_fuzzy_true(&root, "list of nice mæoևoses", vec![81]); // 2-byte unicode

        search_fuzzy_true(&root, "list of nice mæ世ooses", vec![81]); // 3-byte unicode
        search_fuzzy_true(&root, "list of nice m世ooses", vec![81, 82]); // 3-byte unicode
        search_fuzzy_true(&root, "list of nice 世mæooses", vec![81]); // 3-byte unicode
        search_fuzzy_true(&root, "list of nice և世mooses", vec![81]); // 3-byte unicode

        search_fuzzy_true(&root, "list of nice 𓅂𓅂mooses", vec![81]); // 4-byte unicode
        search_fuzzy_true(&root, "list of nice 𓅂m世ooses", vec![81]); // 4-byte unicode
        search_fuzzy_true(&root, "list of nice߷ 𓅂mooses", vec![81]); // 4-byte unicode
        search_fuzzy_true(&root, "list of nice 𓅂߷mooses", vec![81]); // 4-byte unicode
        search_fuzzy_true(&root, "list of nice𓅂 ߷mooses", vec![81]); // 4-byte unicode
        search_fuzzy_true(&root, "list of niceև𓅂 mooses", vec![81]); // 4-byte unicode

        // Adds (needs deletes) (swappable)
        search_fuzzy_true(&root, "list of niuce mooses", vec![81, 82]);
        search_fuzzy_true(&root, "list of nuicew mooses", vec![81]);
        search_fuzzy_true(&root, "list of nicer nmooses", vec![81, 86]);
        search_fuzzy_true(&root, "list of nicer mnooses", vec![81, 86]);
        search_fuzzy_true(&root, "list of nice mooooses", vec![81]);
        search_fuzzy_true(&root, "list of nice mooseees", vec![81]);

        // Swaps (needs swaps)
        search_fuzzy_true(&root, "list of nicr nooses", vec![81]);
        search_fuzzy_true(&root, "list of nlce mioses", vec![81, 82, 83, 86]);
        search_fuzzy_true(&root, "list of nice moozes", vec![81, 82, 83]);
        search_fuzzy_true(&root, "list of nice miises", vec![81, 82, 86]);
        search_fuzzy_true(&root, "list of nice møøses", vec![81, 82]);
        search_fuzzy_true(&root, "list of nice moøses", vec![81, 82, 83, 86]);
    }

    #[test]
    fn search_utf8_nodes() {
        let root = setup_fuzzy();

        search_fuzzy_true(&root, "look at this 𓅂߷æ so nice", vec![88, 89, 90]);
        search_fuzzy_true(&root, "look at this ߷ææ so nice", vec![89, 90, 91]);
        search_fuzzy_true(&root, "look at this æ߷æ so nice", vec![88, 89, 90, 91]);
        search_fuzzy_true(&root, "look at this 世ææ so nice", vec![89, 90, 91]);
        search_fuzzy_true(&root, "世߷世߷世߷世߷𓅂𓅂", vec![95, 96, 97, 98, 99, 100, 101, 102, 103, 104, 105, 106, 107, 108, 109]);
        search_fuzzy_true(
            &root,
            "世߷世߷",
            vec![
                92, 93, 94, 95, 96, 97, 98, 99, 100, 101, 102, 103, 104, 105, 106, 107, 108, 109, 110,
                111, 112,
            ],
        );
       

        //
    }

    #[test]
    fn search_resolve_test() {
        let root = setup();

        search_resolve(&root, "hello friend");
        search_resolve(&root, "hello friends!");
        search_resolve(&root, "hello worl");
        search_resolve(&root, "hello world");
        search_resolve(&root, "help world");
        search_resolve(&root, "help o world");
        search_resolve(&root, "an nice cat");
        search_resolve(&root, "an nice dog");
        search_resolve(&root, "an evil cat");
        search_resolve(&root, "an evil");
        search_resolve(&root, "an nice");
        search_resolve(&root, "an nice ");
        search_resolve(&root, "+++-----");
    }

    #[test]
    fn search_1() {
        let root = setup();

        search_true(&root, "aaa", vec![1, 4, 5, 7, 8, 9]);
        search_true(&root, "aa", vec![1, 4, 5, 7, 8, 9, 10, 11, 12, 13, 14, 15]);
        search_true(
            &root,
            "a",
            vec![
                1, 3, 4, 5, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 26, 51, 52, 53, 54, 55, 56, 57, 58,
            ],
        );
    }

    #[test]
    fn search_2() {
        let root = setup();

        search_true(&root, "aab", vec![10, 11]);
        search_true(&root, "y", vec![43, 44]);
        search_true(&root, "yy", vec![44]);
        search_true(&root, "a😀", vec![16, 26]);
    }

    #[test]
    fn search_3() {
        let root = setup();

        search_true(&root, "x", vec![17, 18, 19, 20, 21, 22, 23, 24]);
        search_true(&root, "xx", vec![17, 18, 19, 20, 21, 22, 23]);
        search_true(&root, "xxx", vec![17, 18]);
        search_true(&root, "xx&", vec![20]);
        search_true(&root, "xx ", vec![23]);
    }

    #[test]
    fn search_4() {
        let root = setup();

        search_true(&root, "+", vec![35, 36, 37, 38, 39]);
        search_true(&root, "++", vec![35, 36, 37, 38, 39]);
        search_true(&root, "+++", vec![35, 36, 37, 38, 39]);
        search_true(&root, "++++", vec![35, 36, 38]);
        search_true(&root, "+++-", vec![37, 39]);
    }

    #[test]
    fn search_5() {
        let root = setup();

        search_true(&root, "😀", vec![29, 30, 31, 32, 33, 34]);
        search_true(&root, "😀😀", vec![30, 31, 32]);
        search_true(&root, "😀😀😀", vec![31]);
        search_true(&root, "😀🫠", vec![33]);
        search_true(&root, "😀🍆", vec![34]);
    }

    #[test]
    fn search_6() {
        let root = setup();

        search_true(
            &root,
            "Â",
            vec![59, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 70],
        );
        search_true(
            &root,
            "ÂÂ",
            vec![60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 70],
        );
        search_true(&root, "ÂÂÂ", vec![61, 62, 63, 64, 65, 66, 67, 68, 69, 70]);
        search_true(
            &root,
            "ÂÂÂngeloç",
            vec![61, 62, 63, 64, 65, 66, 67, 68, 69, 70],
        );
        search_true(&root, "ÂÂÂngeloçà", vec![63, 64, 65, 66, 67, 68, 69, 70]);
        search_true(&root, "ÂÂÂngeloçàoÖ", vec![63, 64, 65, 66, 67, 68, 69, 70]);
        search_true(&root, "ÂÂÂngeloçàoÖý", vec![63, 64, 65, 66, 67, 68, 69, 70]);
        search_true(&root, "ÂÂÂngeloçàoÖý°Ë", vec![63, 64, 65, 66, 67, 68, 70]);
        search_true(&root, "ÂÂÂngeloçàoÖý°ËŒ©50", vec![63, 64, 65, 66, 67, 68]);
        search_true(&root, "ÂÂÂngeloçàoÖý°ËŒ©50¢×✓", vec![66]);
        search_true(&root, "ÂÂÂngeloçàoÖý°ËŒ©50¢×☕✓", vec![65]);
        search_true(&root, "ÂÂÂngeloçàoÖý°ËŒ©50¢×®™☕✓", vec![63]);
        search_true(&root, "ÂÂÂngeloçàoÖý°CËŒ©50", vec![69]);
        search_true(&root, "ÂÂÂngeloçàoÖý°ËKŒ©50", vec![70]);
    }

    #[test]
    fn search_7() {
        let root = setup();

        search_true(&root, "こんにちは", vec![71, 72]);
        search_true(&root, "こんにちは世界.doc", vec![71]);
        search_true(&root, "こんにちは世界", vec![71]);
        search_true(&root, "こんにちは友人", vec![72]);
        search_true(&root, "こんにちは友人.doc", vec![72]);
        search_true(&root, "你好", vec![73, 74]);
        search_true(&root, "你好伙伴", vec![73]);
        search_true(&root, "你好世界", vec![74]);
        search_true(&root, "नमस्ते", vec![75]);
        search_true(&root, "æaæaæsdasdasdasda", vec![]);
    }
    #[test]
    fn search_no_result_0() {
        let root = setup();

        search_false(&root, "++-");
        search_false(&root, "++++-");
        search_false(&root, "+++--+");
    }

    #[test]
    fn search_no_result_1() {
        let root = setup();

        search_false(&root, "");
    }

    #[test]
    fn search_no_result_2() {
        let root = setup();

        search_false(&root, "A");
        search_false(&root, "*");
        search_false(&root, "\n\n");
        search_false(&root, "🫠");
        search_false(&root, "🍆");
        search_false(&root, " ");
    }

    // #[test]
    // fn search_no_result_single() {
    //     search_false("F");
    //     search_false("H");
    //     search_false("W");
    // }

    // #[test]
    // fn search_no_result_empty() {
    //     search_false("");
    // }

    // #[test]
    // fn search_hello_world_unicode() {
    //     search_b_true("👋🏻 Hello World!", vec![1, 2]);
    // }

    // #[test]
    // fn search_hello_world_unicode_hindi() {
    //     search_b_true("", vec![5]);
    // }
}
