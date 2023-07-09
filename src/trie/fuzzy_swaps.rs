use hashbrown::HashMap;

/// This struct controls the fuzzy ratio for the query length
/// the idea is that the longer the query, more issues it can have
/// so we can give a series of `ConstrainedFuzzyRatio` that control how many
/// fuzzy characters are allowed for a given query length
/// 
/// # Example
/// ```rust
/// use suggestion_trie::ConstrainedFuzzyRatio;
/// let CONSTRAINED_FUZZY_RATIO = vec![
///   ConstrainedFuzzyRatio {
///       char_count: 0,
///       fuzzy_count: 0,
///   },
///   ConstrainedFuzzyRatio {
///       char_count: 4,
///       fuzzy_count: 1,
///   },
///   ConstrainedFuzzyRatio {
///       char_count: 8,
///       fuzzy_count: 2,
///   },
///   ConstrainedFuzzyRatio {
///       char_count: 10,
///       fuzzy_count: 3,
///   },
///   ConstrainedFuzzyRatio {
///       char_count: 14,
///       fuzzy_count: 4,
///   },
/// ];
/// 
/// // Use change_fuzzy_ratio() function to set this ratios on the trie search algorithm
/// ```
#[derive(Debug, Clone)]
pub struct ConstrainedFuzzyRatio {
    /// Numbers of chars in the query before the `fuzzy_count` is applied
    pub char_count: i32,
    /// Number 
    pub fuzzy_count: i32,
}

pub(crate) fn get_query_ratio(query_len: &usize, ratio_list: &Vec<ConstrainedFuzzyRatio>) -> i32 {
    let mut result = 0;
    for ratio in ratio_list.iter() {
        if *query_len >= ratio.char_count as usize {
            result = ratio.fuzzy_count;
        } else {
            return result;
        }
    }
    ratio_list[ratio_list.len() - 1].fuzzy_count
}

lazy_static! {
    /// This is a table that contains some swaps for common qwerty keyboard typos and some extra.
    pub(crate) static ref FUZZY_CHAR_SWAPS_DATA: HashMap<char, Vec<char>> = {
        let mut m = HashMap::new();
        m.insert('-', vec![' ']);
        m.insert('.', vec!['l']);
        m.insert(',', vec!['m','k','l','.']);
        m.insert('a', vec!['q','s','w','x','i','z','à','á','â','ã','ä','å']);
        m.insert('b', vec!['f','g','h','n','v']);
        m.insert('c', vec!['s','d','f','v','x','ç']);
        m.insert('d', vec!['c','e','f','r','s','w','x']);
        m.insert('e', vec!['d','f','r','s','w','3','4','è','é','ê','ë','i']);
        m.insert('f', vec!['b','c','d','e','g','r','t','v']);
        m.insert('g', vec!['b','f','h','n','r','t','v','y']);
        m.insert('h', vec!['b','g','j','n','t','u','y']);
        m.insert('i', vec!['j','k','l','a','o','u','8','9','ì','í','î','ï','e']);
        m.insert('j', vec!['h','i','k','m','n','u','y']);
        m.insert('k', vec!['i','j','l','m','o','u','ø']);
        m.insert('l', vec!['i','k','m','o','p','æ','ø','å']);
        m.insert('m', vec!['j','k','n']);
        m.insert('n', vec!['b','g','h','j','m','ñ']);
        m.insert('o', vec!['i','k','l','p','9','0','ò','ó','ô','õ','ö','ø']);
        m.insert('p', vec!['l','o','æ','ø','å','0']);
        m.insert('q', vec!['a','s','w','1','2']);
        m.insert('r', vec!['d','e','f','g','t','4','5']);
        m.insert('s', vec!['a','c','d','e','q','w','x','z','ß']);
        m.insert('t', vec!['f','g','h','r','y','5','6']);
        m.insert('u', vec!['h','i','j','k','y','7','8','ù','ú','û','ü','v']);
        m.insert('v', vec!['b','c','d','f','g','u']);
        m.insert('w', vec!['a','d','e','q','s','2','3','v']);
        m.insert('x', vec!['c','d','s','z','k']);
        m.insert('y', vec!['g','h','j','t','u','6','7','ý','ÿ']);
        m.insert('z', vec!['a','s','x']);
        m.insert('0', vec!['9','o','p','-','ø',')']);
        m.insert('1', vec!['2','q','l','i','\'','!','¡','¹']);
        m.insert('2', vec!['1','3','q','w','@','²']);
        m.insert('4', vec!['3','5','e','r','$','£']);
        m.insert('5', vec!['4','6','r','t','s','%','§','¢']);
        m.insert('6', vec!['5','7','t','y','¨','¬','^']);
        m.insert('7', vec!['6','8','y','u','t','&']);
        m.insert('8', vec!['7','9','u','i','*']);
        m.insert('9', vec!['8','0','i','o','(']);
        m.insert('æ', vec!['l','p','ø','å']);
        m.insert('ç', vec!['c','l','p',';','o','.','~','´']);
        m.insert('ø', vec!['k','l','o','p','æ','å','0']);
        m.insert('å', vec!['l','o','p','æ','ø','a']);
        m.insert(' ', vec!['-','_','.']);
        m.insert('-', vec![' ','_','0','=']);
        m.insert('_', vec![' ','-','0','=']);
        m.insert('=', vec!['-','_','+']);
        m.insert('.', vec![' ',',']);
        m.insert(',', vec![' ','.']);
        m.insert('!', vec!['1','i','l']);
        m.insert('?', vec!['7','!']);
        m.insert('(', vec!['9']);
        m.insert(')', vec!['0']);
        m.insert('/', vec!['\\']);
        m.insert('\\', vec!['/']);
        m.insert(':', vec![';']);
        m.insert(';', vec![':']);

        m
    };
}
