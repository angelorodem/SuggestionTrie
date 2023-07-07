use std::collections::HashMap;

pub(crate) const CONSTRAINED_FUZZY_RATIO: [ConstrainedFuzzyRatio; 4] = [
    ConstrainedFuzzyRatio {
        char_count: 0,
        fuzzy_count: 0,
    },
    ConstrainedFuzzyRatio {
        char_count: 4,
        fuzzy_count: 1,
    },
    ConstrainedFuzzyRatio {
        char_count: 8,
        fuzzy_count: 2,
    },
    ConstrainedFuzzyRatio {
        char_count: 12,
        fuzzy_count: 3,
    }
];

pub(crate) struct ConstrainedFuzzyRatio {
    char_count: i32,
    fuzzy_count: i32,
}

pub(crate) fn get_query_ratio(query_len: &usize) -> i32 {
    let mut result = 0;
    for ratio in CONSTRAINED_FUZZY_RATIO.iter() {
        if *query_len >= ratio.char_count as usize {
            result = ratio.fuzzy_count;
        } else {
            return result;
        }
    }
    CONSTRAINED_FUZZY_RATIO[CONSTRAINED_FUZZY_RATIO.len() - 1].fuzzy_count
}

pub(crate) struct FuzzyCharSwap {
    pub char: char,
    pub score: u32,
}

lazy_static! {
    /// This is a table that contains some swaps for common qwerty keyboard typos and some extra.
    /// The score is based of probability of the typo.
    /// The score can be inaccurate, in the future the scores can be adjusted using some data analysis.
    /// Suggestions that fuzzy swap using a char with high score will be ranked higher when returned by fuzzy search.
    pub(crate) static ref FUZZY_CHAR_SWAPS_DATA: HashMap<char, Vec<FuzzyCharSwap>> = {
        let mut m = HashMap::new();
        m.insert(
            '-',
            vec![
                FuzzyCharSwap {
                    char: ' ',
                    score: 100,
                },
            ],
        );
        m.insert(
            '.',
            vec![

            FuzzyCharSwap {
                char: 'l', // add
                score: 100,
            },
            ],
        );
        m.insert(
            ',',
            vec![

            FuzzyCharSwap {
                char: 'm', // add
                score: 100,
            },
            FuzzyCharSwap {
                char: 'k', // add
                score: 100,
            },
            FuzzyCharSwap {
                char: 'l', // add
                score: 100,
            },
            FuzzyCharSwap {
                char: '.', // add
                score: 100,
            },
            ],
        );
        m.insert(
            'a',
            vec![

                FuzzyCharSwap {
                    char: 'q',
                    score: 100,
                },
                FuzzyCharSwap {
                    char: 's',
                    score: 50,
                },
                FuzzyCharSwap {
                    char: 'w',
                    score: 100,
                },
                FuzzyCharSwap {
                    char: 'x',
                    score: 50,
                },
                FuzzyCharSwap {
                    char: 'i',
                    score: 50,
                },
                FuzzyCharSwap {
                    char: 'z',
                    score: 100,
                },
                FuzzyCharSwap {
                    char: 'à',
                    score: 85,
                },
                FuzzyCharSwap {
                    char: 'á',
                    score: 85,
                },
                FuzzyCharSwap {
                    char: 'â',
                    score: 85,
                },
                FuzzyCharSwap {
                    char: 'ã',
                    score: 85,
                },
                FuzzyCharSwap {
                    char: 'ä',
                    score: 85,
                },
                FuzzyCharSwap {
                    char: 'å',
                    score: 85,
                },
            ],
        );
        m.insert(
            'b',
            vec![

                FuzzyCharSwap {
                    char: 'f',
                    score: 50,
                },
                FuzzyCharSwap {
                    char: 'g',
                    score: 100,
                },
                FuzzyCharSwap {
                    char: 'h',
                    score: 100,
                },
                FuzzyCharSwap {
                    char: 'n',
                    score: 50,
                },
                FuzzyCharSwap {
                    char: 'v',
                    score: 50,
                },
            ],
        );
        m.insert(
            'c',
            vec![
                FuzzyCharSwap {
                    char: 's',
                    score: 100,
                },
                FuzzyCharSwap {
                    char: 'd',
                    score: 100,
                },
                FuzzyCharSwap {
                    char: 'f',
                    score: 100,
                },
                FuzzyCharSwap {
                    char: 'v',
                    score: 50,
                },
                FuzzyCharSwap {
                    char: 'x',
                    score: 50,
                },
                FuzzyCharSwap {
                    char: 'ç',
                    score: 85,
                },
            ],
        );
        m.insert(
            'd',
            vec![

                FuzzyCharSwap {
                    char: 'c',
                    score: 100,
                },
                FuzzyCharSwap {
                    char: 'e',
                    score: 100,
                },
                FuzzyCharSwap {
                    char: 'f',
                    score: 50,
                },
                FuzzyCharSwap {
                    char: 'r',
                    score: 100,
                },
                FuzzyCharSwap {
                    char: 's',
                    score: 50,
                },
                FuzzyCharSwap {
                    char: 'w',
                    score: 50,
                },
                FuzzyCharSwap {
                    char: 'x',
                    score: 100,
                },
            ],
        );
        m.insert(
            'e',
            vec![
                FuzzyCharSwap {
                    char: 'd',
                    score: 100,
                },
                FuzzyCharSwap {
                    char: 'f',
                    score: 50,
                },
                FuzzyCharSwap {
                    char: 'r',
                    score: 50,
                },
                FuzzyCharSwap {
                    char: 's',
                    score: 100,
                },
                FuzzyCharSwap {
                    char: 'w',
                    score: 50,
                },
                FuzzyCharSwap {
                    char: '3',
                    score: 50,
                },
                FuzzyCharSwap {
                    char: '4',
                    score: 50,
                },
                FuzzyCharSwap {
                    char: 'è',
                    score: 85,
                },
                FuzzyCharSwap {
                    char: 'é',
                    score: 85,
                },
                FuzzyCharSwap {
                    char: 'ê',
                    score: 85,
                },
                FuzzyCharSwap {
                    char: 'ë',
                    score: 85,
                },
                FuzzyCharSwap {
                    char: 'i',
                    score: 30,
                },
            ],
        );
        m.insert(
            'f',
            vec![

                FuzzyCharSwap {
                    char: 'b',
                    score: 50,
                },
                FuzzyCharSwap {
                    char: 'c',
                    score: 100,
                },
                FuzzyCharSwap {
                    char: 'd',
                    score: 50,
                },
                FuzzyCharSwap {
                    char: 'e',
                    score: 50,
                },
                FuzzyCharSwap {
                    char: 'g',
                    score: 50,
                },
                FuzzyCharSwap {
                    char: 'r',
                    score: 100,
                },
                FuzzyCharSwap {
                    char: 't',
                    score: 100,
                },
                FuzzyCharSwap {
                    char: 'v',
                    score: 100,
                },
            ],
        );
        m.insert(
            'g',
            vec![

                FuzzyCharSwap {
                    char: 'b',
                    score: 100,
                },
                FuzzyCharSwap {
                    char: 'f',
                    score: 50,
                },
                FuzzyCharSwap {
                    char: 'h',
                    score: 50,
                },
                FuzzyCharSwap {
                    char: 'n',
                    score: 50,
                },
                FuzzyCharSwap {
                    char: 'r',
                    score: 50,
                },
                FuzzyCharSwap {
                    char: 't',
                    score: 100,
                },
                FuzzyCharSwap {
                    char: 'v',
                    score: 100,
                },
                FuzzyCharSwap {
                    char: 'y',
                    score: 100,
                },
            ],
        );
        m.insert(
            'h',
            vec![

                FuzzyCharSwap {
                    char: 'b',
                    score: 100,
                },
                FuzzyCharSwap {
                    char: 'g',
                    score: 50,
                },
                FuzzyCharSwap {
                    char: 'j',
                    score: 50,
                },
                FuzzyCharSwap {
                    char: 'n',
                    score: 100,
                },
                FuzzyCharSwap {
                    char: 't',
                    score: 50,
                },
                FuzzyCharSwap {
                    char: 'u',
                    score: 100,
                },
                FuzzyCharSwap {
                    char: 'y',
                    score: 100,
                },
            ],
        );
        m.insert(
            'i',
            vec![

                FuzzyCharSwap {
                    char: 'j',
                    score: 100,
                },
                FuzzyCharSwap {
                    char: 'k',
                    score: 100,
                },
                FuzzyCharSwap {
                    char: 'l',
                    score: 50,
                },
                FuzzyCharSwap {
                    char: 'a',
                    score: 50,
                },
                FuzzyCharSwap {
                    char: 'o',
                    score: 50,
                },
                FuzzyCharSwap {
                    char: 'u',
                    score: 50,
                },
                FuzzyCharSwap {
                    char: '8',
                    score: 50,
                },
                FuzzyCharSwap {
                    char: '9',
                    score: 50,
                },
                FuzzyCharSwap {
                    char: 'ì',
                    score: 85,
                },
                FuzzyCharSwap {
                    char: 'í',
                    score: 85,
                },
                FuzzyCharSwap {
                    char: 'î',
                    score: 85,
                },
                FuzzyCharSwap {
                    char: 'ï',
                    score: 85,
                },
                FuzzyCharSwap {
                    char: 'e',
                    score: 85,
                },
            ],
        );
        m.insert(
            'j',
            vec![

                FuzzyCharSwap {
                    char: 'h',
                    score: 50,
                },
                FuzzyCharSwap {
                    char: 'i',
                    score: 100,
                },
                FuzzyCharSwap {
                    char: 'k',
                    score: 50,
                },
                FuzzyCharSwap {
                    char: 'm',
                    score: 100,
                },
                FuzzyCharSwap {
                    char: 'n',
                    score: 100,
                },
                FuzzyCharSwap {
                    char: 'u',
                    score: 100,
                },
                FuzzyCharSwap {
                    char: 'y',
                    score: 50,
                },
            ],
        );
        m.insert(
            'k',
            vec![

                FuzzyCharSwap {
                    char: 'i',
                    score: 100,
                },
                FuzzyCharSwap {
                    char: 'j',
                    score: 50,
                },
                FuzzyCharSwap {
                    char: 'l',
                    score: 50,
                },
                FuzzyCharSwap {
                    char: 'm',
                    score: 100,
                },
                FuzzyCharSwap {
                    char: 'o',
                    score: 100,
                },
                FuzzyCharSwap {
                    char: 'u',
                    score: 50,
                },
                FuzzyCharSwap {
                    char: 'ø',
                    score: 50,
                },
            ],
        );
        m.insert(
            'l',
            vec![

                FuzzyCharSwap {
                    char: 'i',
                    score: 50,
                },
                FuzzyCharSwap {
                    char: 'k',
                    score: 50,
                },
                FuzzyCharSwap {
                    char: 'm',
                    score: 50,
                },
                FuzzyCharSwap {
                    char: 'o',
                    score: 100,
                },
                FuzzyCharSwap {
                    char: 'p',
                    score: 100,
                },
                FuzzyCharSwap {
                    char: 'æ',
                    score: 50,
                },
                FuzzyCharSwap {
                    char: 'ø',
                    score: 50,
                },
                FuzzyCharSwap {
                    char: 'å',
                    score: 50,
                },
            ],
        );
        m.insert(
            'm',
            vec![

                FuzzyCharSwap {
                    char: 'j',
                    score: 100,
                },
                FuzzyCharSwap {
                    char: 'k',
                    score: 100,
                },
                FuzzyCharSwap {
                    char: 'n',
                    score: 50,
                },
            ],
        );
        m.insert(
            'n',
            vec![

                FuzzyCharSwap {
                    char: 'b',
                    score: 50,
                },
                FuzzyCharSwap {
                    char: 'g',
                    score: 50,
                },
                FuzzyCharSwap {
                    char: 'h',
                    score: 100,
                },
                FuzzyCharSwap {
                    char: 'j',
                    score: 100,
                },
                FuzzyCharSwap {
                    char: 'm',
                    score: 50,
                },
                FuzzyCharSwap {
                    char: 'ñ',
                    score: 85,
                },
            ],
        );
        m.insert(
            'o',
            vec![

                FuzzyCharSwap {
                    char: 'i',
                    score: 50,
                },
                FuzzyCharSwap {
                    char: 'k',
                    score: 100,
                },
                FuzzyCharSwap {
                    char: 'l',
                    score: 100,
                },
                FuzzyCharSwap {
                    char: 'p',
                    score: 50,
                },
                FuzzyCharSwap {
                    char: '9',
                    score: 50,
                },
                FuzzyCharSwap {
                    char: '0',
                    score: 50,
                },
                FuzzyCharSwap {
                    char: 'ò',
                    score: 85,
                },
                FuzzyCharSwap {
                    char: 'ó',
                    score: 85,
                },
                FuzzyCharSwap {
                    char: 'ô',
                    score: 85,
                },
                FuzzyCharSwap {
                    char: 'õ',
                    score: 85,
                },
                FuzzyCharSwap {
                    char: 'ö',
                    score: 85,
                },
                FuzzyCharSwap {
                    char: 'ø',
                    score: 85,
                },
            ],
        );
        m.insert(
            'p',
            vec![

                FuzzyCharSwap {
                    char: 'l',
                    score: 100,
                },
                FuzzyCharSwap {
                    char: 'o',
                    score: 50,
                },
                FuzzyCharSwap {
                    char: 'æ',
                    score: 50,
                },
                FuzzyCharSwap {
                    char: 'ø',
                    score: 50,
                },
                FuzzyCharSwap {
                    char: 'å',
                    score: 50,
                },
                FuzzyCharSwap {
                    char: '0',
                    score: 50,
                },
            ],
        );
        m.insert(
            'q',
            vec![

                FuzzyCharSwap {
                    char: 'a',
                    score: 100,
                },
                FuzzyCharSwap {
                    char: 's',
                    score: 50,
                },
                FuzzyCharSwap {
                    char: 'w',
                    score: 50,
                },
                FuzzyCharSwap {
                    char: '1',
                    score: 50,
                },
                FuzzyCharSwap {
                    char: '2',
                    score: 50,
                },
            ],
        );
        m.insert(
            'r',
            vec![

                FuzzyCharSwap {
                    char: 'd',
                    score: 100,
                },
                FuzzyCharSwap {
                    char: 'e',
                    score: 50,
                },
                FuzzyCharSwap {
                    char: 'f',
                    score: 100,
                },
                FuzzyCharSwap {
                    char: 'g',
                    score: 50,
                },
                FuzzyCharSwap {
                    char: 't',
                    score: 50,
                },
                FuzzyCharSwap {
                    char: '4',
                    score: 50,
                },
                FuzzyCharSwap {
                    char: '5',
                    score: 50,
                },
            ],
        );
        m.insert(
            's',
            vec![

                FuzzyCharSwap {
                    char: 'a',
                    score: 50,
                },
                FuzzyCharSwap {
                    char: 'c',
                    score: 80,
                },
                FuzzyCharSwap {
                    char: 'd',
                    score: 50,
                },
                FuzzyCharSwap {
                    char: 'e',
                    score: 100,
                },
                FuzzyCharSwap {
                    char: 'q',
                    score: 50,
                },
                FuzzyCharSwap {
                    char: 'w',
                    score: 100,
                },
                FuzzyCharSwap {
                    char: 'x',
                    score: 100,
                },
                FuzzyCharSwap {
                    char: 'z',
                    score: 100,
                },
                FuzzyCharSwap {
                    char: 'ß',
                    score: 85,
                },
            ],
        );
        m.insert(
            't',
            vec![

                FuzzyCharSwap {
                    char: 'f',
                    score: 100,
                },
                FuzzyCharSwap {
                    char: 'g',
                    score: 100,
                },
                FuzzyCharSwap {
                    char: 'h',
                    score: 50,
                },
                FuzzyCharSwap {
                    char: 'r',
                    score: 50,
                },
                FuzzyCharSwap {
                    char: 'y',
                    score: 50,
                },
                FuzzyCharSwap {
                    char: '5',
                    score: 50,
                },
                FuzzyCharSwap {
                    char: '6',
                    score: 50,
                },
            ],
        );
        m.insert(
            'u',
            vec![

                FuzzyCharSwap {
                    char: 'h',
                    score: 100,
                },
                FuzzyCharSwap {
                    char: 'i',
                    score: 50,
                },
                FuzzyCharSwap {
                    char: 'j',
                    score: 100,
                },
                FuzzyCharSwap {
                    char: 'k',
                    score: 50,
                },
                FuzzyCharSwap {
                    char: 'y',
                    score: 50,
                },
                FuzzyCharSwap {
                    char: '7',
                    score: 50,
                },
                FuzzyCharSwap {
                    char: '8',
                    score: 50,
                },
                FuzzyCharSwap {
                    char: 'ù',
                    score: 85,
                },
                FuzzyCharSwap {
                    char: 'ú',
                    score: 85,
                },
                FuzzyCharSwap {
                    char: 'û',
                    score: 85,
                },
                FuzzyCharSwap {
                    char: 'ü',
                    score: 85,
                },
                FuzzyCharSwap {
                    char: 'v',
                    score: 20,
                },
            ],
        );
        m.insert(
            'v',
            vec![

                FuzzyCharSwap {
                    char: 'b',
                    score: 50,
                },
                FuzzyCharSwap {
                    char: 'c',
                    score: 50,
                },
                FuzzyCharSwap {
                    char: 'd',
                    score: 50,
                },
                FuzzyCharSwap {
                    char: 'f',
                    score: 100,
                },
                FuzzyCharSwap {
                    char: 'g',
                    score: 100,
                },
                FuzzyCharSwap {
                    char: 'u',
                    score: 20,
                },
            ],
        );
        m.insert(
            'w',
            vec![

                FuzzyCharSwap {
                    char: 'a',
                    score: 100,
                },
                FuzzyCharSwap {
                    char: 'd',
                    score: 50,
                },
                FuzzyCharSwap {
                    char: 'e',
                    score: 50,
                },
                FuzzyCharSwap {
                    char: 'q',
                    score: 50,
                },
                FuzzyCharSwap {
                    char: 's',
                    score: 100,
                },
                FuzzyCharSwap {
                    char: '2',
                    score: 50,
                },
                FuzzyCharSwap {
                    char: '3',
                    score: 50,
                },
                FuzzyCharSwap {
                    char: 'v',
                    score: 100,
                },
            ],
        );
        m.insert(
            'x',
            vec![

                FuzzyCharSwap {
                    char: 'c',
                    score: 50,
                },
                FuzzyCharSwap {
                    char: 'd',
                    score: 100,
                },
                FuzzyCharSwap {
                    char: 's',
                    score: 100,
                },
                FuzzyCharSwap {
                    char: 'z',
                    score: 50,
                },
                FuzzyCharSwap {
                    char: 'k',
                    score: 6500,
                },
            ],
        );
        m.insert(
            'y',
            vec![

                FuzzyCharSwap {
                    char: 'g',
                    score: 100,
                },
                FuzzyCharSwap {
                    char: 'h',
                    score: 100,
                },
                FuzzyCharSwap {
                    char: 'j',
                    score: 50,
                },
                FuzzyCharSwap {
                    char: 't',
                    score: 50,
                },
                FuzzyCharSwap {
                    char: 'u',
                    score: 50,
                },
                FuzzyCharSwap {
                    char: '6',
                    score: 50,
                },
                FuzzyCharSwap {
                    char: '7',
                    score: 50,
                },
                FuzzyCharSwap {
                    char: 'ý',
                    score: 85,
                },
                FuzzyCharSwap {
                    char: 'ÿ',
                    score: 85,
                },
            ],
        );
        m.insert(
            'z',
            vec![

                FuzzyCharSwap {
                    char: 'a',
                    score: 100,
                },
                FuzzyCharSwap {
                    char: 's',
                    score: 100,
                },
                FuzzyCharSwap {
                    char: 'x',
                    score: 50,
                },
            ],
        );
        m.insert(
            '0',
            vec![

                FuzzyCharSwap {
                    char: '9',
                    score: 50,
                },
                FuzzyCharSwap {
                    char: 'o',
                    score: 50,
                },
                FuzzyCharSwap {
                    char: 'p',
                    score: 50,
                },
                FuzzyCharSwap {
                    char: '-',
                    score: 50,
                },
                FuzzyCharSwap {
                    char: 'ø',
                    score: 20,
                },
                FuzzyCharSwap {
                    char: ')',
                    score: 20,
                },
            ],
        );
        m.insert(
            '1',
            vec![

                FuzzyCharSwap {
                    char: '2',
                    score: 50,
                },
                FuzzyCharSwap {
                    char: 'q',
                    score: 50,
                },
                FuzzyCharSwap {
                    char: 'l',
                    score: 50,
                },
                FuzzyCharSwap {
                    char: 'i',
                    score: 20,
                },
                FuzzyCharSwap {
                    char: '\'',
                    score: 20,
                },
                FuzzyCharSwap {
                    char: '!',
                    score: 20,
                },
                FuzzyCharSwap {
                    char: '¡',
                    score: 20,
                },
                FuzzyCharSwap {
                    char: '¹',
                    score: 20,
                },
            ],
        );
        m.insert(
            '2',
            vec![

                FuzzyCharSwap {
                    char: '1',
                    score: 50,
                },
                FuzzyCharSwap {
                    char: '3',
                    score: 50,
                },
                FuzzyCharSwap {
                    char: 'q',
                    score: 50,
                },
                FuzzyCharSwap {
                    char: 'w',
                    score: 50,
                },
                FuzzyCharSwap {
                    char: '@',
                    score: 20,
                },
                FuzzyCharSwap {
                    char: '²',
                    score: 20,
                },
            ],
        );
        m.insert(
            '3',
            vec![

                FuzzyCharSwap {
                    char: '2',
                    score: 50,
                },
                FuzzyCharSwap {
                    char: '4',
                    score: 50,
                },
                FuzzyCharSwap {
                    char: 'w',
                    score: 50,
                },
                FuzzyCharSwap {
                    char: 'e',
                    score: 50,
                },
                FuzzyCharSwap {
                    char: '#',
                    score: 20,
                },
                FuzzyCharSwap {
                    char: '³',
                    score: 20,
                },
            ],
        );
        m.insert(
            '4',
            vec![

                FuzzyCharSwap {
                    char: '3',
                    score: 50,
                },
                FuzzyCharSwap {
                    char: '5',
                    score: 50,
                },
                FuzzyCharSwap {
                    char: 'e',
                    score: 50,
                },
                FuzzyCharSwap {
                    char: 'r',
                    score: 50,
                },
                FuzzyCharSwap {
                    char: '$',
                    score: 20,
                },
                FuzzyCharSwap {
                    char: '£',
                    score: 20,
                },
            ],
        );
        m.insert(
            '5',
            vec![

                FuzzyCharSwap {
                    char: '4',
                    score: 50,
                },
                FuzzyCharSwap {
                    char: '6',
                    score: 50,
                },
                FuzzyCharSwap {
                    char: 'r',
                    score: 50,
                },
                FuzzyCharSwap {
                    char: 't',
                    score: 50,
                },
                FuzzyCharSwap {
                    char: 's',
                    score: 50,
                },
                FuzzyCharSwap {
                    char: '%',
                    score: 20,
                },
                FuzzyCharSwap {
                    char: '§',
                    score: 20,
                },
                FuzzyCharSwap {
                    char: '¢',
                    score: 20,
                },
            ],
        );
        m.insert(
            '6',
            vec![

                FuzzyCharSwap {
                    char: '5',
                    score: 50,
                },
                FuzzyCharSwap {
                    char: '7',
                    score: 50,
                },
                FuzzyCharSwap {
                    char: 't',
                    score: 50,
                },
                FuzzyCharSwap {
                    char: 'y',
                    score: 50,
                },
                FuzzyCharSwap {
                    char: '¨',
                    score: 10,
                },
                FuzzyCharSwap {
                    char: '¬',
                    score: 10,
                },
                FuzzyCharSwap {
                    char: '^',
                    score: 20,
                },
            ],
        );
        m.insert(
            '7',
            vec![

                FuzzyCharSwap {
                    char: '6',
                    score: 50,
                },
                FuzzyCharSwap {
                    char: '8',
                    score: 50,
                },
                FuzzyCharSwap {
                    char: 'y',
                    score: 50,
                },
                FuzzyCharSwap {
                    char: 'u',
                    score: 50,
                },
                FuzzyCharSwap {
                    char: 't',
                    score: 50,
                },
                FuzzyCharSwap {
                    char: '&',
                    score: 20,
                },
            ],
        );
        m.insert(
            '8',
            vec![

                FuzzyCharSwap {
                    char: '7',
                    score: 50,
                },
                FuzzyCharSwap {
                    char: '9',
                    score: 50,
                },
                FuzzyCharSwap {
                    char: 'u',
                    score: 50,
                },
                FuzzyCharSwap {
                    char: 'i',
                    score: 50,
                },
                FuzzyCharSwap {
                    char: '*',
                    score: 20,
                },
            ],
        );
        m.insert(
            '9',
            vec![

                FuzzyCharSwap {
                    char: '8',
                    score: 50,
                },
                FuzzyCharSwap {
                    char: '0',
                    score: 50,
                },
                FuzzyCharSwap {
                    char: 'i',
                    score: 50,
                },
                FuzzyCharSwap {
                    char: 'o',
                    score: 50,
                },
                FuzzyCharSwap {
                    char: '(',
                    score: 20,
                },
            ],
        );
        m.insert(
            'æ',
            vec![

                FuzzyCharSwap {
                    char: 'l',
                    score: 50,
                },
                FuzzyCharSwap {
                    char: 'p',
                    score: 50,
                },
                FuzzyCharSwap {
                    char: 'ø',
                    score: 50,
                },
                FuzzyCharSwap {
                    char: 'å',
                    score: 50,
                },
            ],
        );
        m.insert(
            'ç',
            vec![
                FuzzyCharSwap {
                    char: 'c',
                    score: 100,
                },
                FuzzyCharSwap {
                    char: 'l',
                    score: 80,
                },
                FuzzyCharSwap {
                    char: 'p',
                    score: 80,
                },
                FuzzyCharSwap {
                    char: ';',
                    score: 50,
                },
                FuzzyCharSwap {
                    char: 'o',
                    score: 50,
                },
                FuzzyCharSwap {
                    char: '.',
                    score: 50,
                },
                FuzzyCharSwap {
                    char: '~',
                    score: 20,
                },
                FuzzyCharSwap {
                    char: '´',
                    score: 20,
                },
            ],
        );
        m.insert(
            'ø',
            vec![

                FuzzyCharSwap {
                    char: 'k',
                    score: 50,
                },
                FuzzyCharSwap {
                    char: 'l',
                    score: 50,
                },
                FuzzyCharSwap {
                    char: 'o',
                    score: 85,
                },
                FuzzyCharSwap {
                    char: 'p',
                    score: 50,
                },
                FuzzyCharSwap {
                    char: 'æ',
                    score: 50,
                },
                FuzzyCharSwap {
                    char: 'å',
                    score: 50,
                },
                FuzzyCharSwap {
                    char: '0',
                    score: 50,
                },
            ],
        );
        m.insert(
            'å',
            vec![

                FuzzyCharSwap {
                    char: 'l',
                    score: 50,
                },
                FuzzyCharSwap {
                    char: 'o',
                    score: 50,
                },
                FuzzyCharSwap {
                    char: 'p',
                    score: 50,
                },
                FuzzyCharSwap {
                    char: 'æ',
                    score: 50,
                },
                FuzzyCharSwap {
                    char: 'ø',
                    score: 50,
                },
                FuzzyCharSwap {
                    char: 'a',
                    score: 50,
                },
            ],
        );
        m.insert(
            ' ',
            vec![
                FuzzyCharSwap {
                char: '-',
                score: 100,
            },
            FuzzyCharSwap {
                char: '_',
                score: 100,
            },
            FuzzyCharSwap {
                char: '.',
                score: 50,
            }
            ],
        );
        m.insert(
            '-',
            vec![
                FuzzyCharSwap {
                char: ' ',
                score: 100,
            },
            FuzzyCharSwap {
                char: '_',
                score: 100,
            },
            FuzzyCharSwap {
                char: '0',
                score: 50,
            },
            FuzzyCharSwap {
                char: '=',
                score: 50,
            }
            ],
        );
        m.insert(
            '_',
            vec![
                FuzzyCharSwap {
                char: ' ',
                score: 100,
            },
            FuzzyCharSwap {
                char: '-',
                score: 100,
            },
            FuzzyCharSwap {
                char: '0',
                score: 50,
            },
            FuzzyCharSwap {
                char: '=',
                score: 50,
            }
            ],
        );
        m.insert(
            '=',
            vec![
                FuzzyCharSwap {
                char: '-',
                score: 50,
            },
            FuzzyCharSwap {
                char: '_',
                score: 50,
            },
            FuzzyCharSwap {
                char: '+',
                score: 50,
            }
            ],
        );
        m.insert(
            '.',
            vec![
                FuzzyCharSwap {
                char: ' ',
                score: 50,
            },
            FuzzyCharSwap {
                char: ',',
                score: 50,
            }
            ],
        );
        m.insert(
            ',',
            vec![
                FuzzyCharSwap {
                char: ' ',
                score: 50,
            },
            FuzzyCharSwap {
                char: '.',
                score: 50,
            }
            ],
        );
        m.insert(
            '!',
            vec![
                FuzzyCharSwap {
                char: '1',
                score: 50,
            },
                FuzzyCharSwap {
                char: 'i',
                score: 50,
            },
                FuzzyCharSwap {
                char: 'l',
                score: 50,
            }
            ],
        );
        m.insert(
            '?',
            vec![
                FuzzyCharSwap {
                char: '7',
                score: 50,
            },
            FuzzyCharSwap {
                char: '!',
                score: 50,
            }
            ],
        );
        m.insert(
            '(',
            vec![
                FuzzyCharSwap {
                char: '9',
                score: 50,
            }
            ],
        );
        m.insert(
            ')',
            vec![
                FuzzyCharSwap {
                char: '0',
                score: 50,
            }
            ],
        );
        m.insert(
            '/',
            vec![
                FuzzyCharSwap {
                char: '\\',
                score: 50,
            }
            ],
        );
        m.insert(
            '\\',
            vec![
                FuzzyCharSwap {
                char: '/',
                score: 50,
            }
            ],
        );
        m.insert(
            ':',
            vec![
                FuzzyCharSwap {
                char: ';',
                score: 50,
            }
            ],
        );
        m.insert(
            ';',
            vec![
                FuzzyCharSwap {
                char: ':',
                score: 50,
            }
            ],
        );


        m
    };
}
