#![feature(test)]
extern crate test;

use std::collections::HashMap;

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TestSt {
    id: String,
}

#[cfg(test)]
//https://doc.rust-lang.org/unstable-book/library-features/test.html
mod tests {
    use super::*;
    use suggestion_trie::suggestion::*;
    use suggestion_trie::trie::trie_structs::*;
    use test::Bencher;

    fn trie_test_data(keyword: String) -> TrieInputData<HashMap<String,String>> {
        TrieInputData {
            keywords: vec![keyword.clone()], // duplicating on passing the same node
            suggestion: Suggestion::new(keyword, None),
        }
    }

    fn load_insert_bench_entries() -> Vec<TrieInputData<HashMap<String,String>>>{
        let entries = vec![      
            trie_test_data("iiiiiiiiii".to_string()),             
            trie_test_data("iiiiiiiiiii".to_string()),            
            trie_test_data("iiiiiiiiiiii".to_string()),           
            trie_test_data("iiiiiiiiiiiii".to_string()),          
            trie_test_data("iiiiiiiiiiii".to_string()),           
            trie_test_data("iiiiiiiiiii".to_string()),            
            trie_test_data("iiiiiiiiii".to_string()),             
            trie_test_data("iiiiiiiii".to_string()),              
            trie_test_data("iiiiiiii".to_string()),               
            trie_test_data("iiiiiii".to_string()),
            trie_test_data("iiiiiiiiiiiiiiiiiiiiiiii".to_string()),          
            trie_test_data("iiiiiiiiiiiiiiiiiiiiiii".to_string()),           
            trie_test_data("iiiiiiiiiiiiiiiiiiiiii".to_string()),            
            trie_test_data("iiiiiiiiiiiiiiiiiiiii".to_string()),             
            trie_test_data("iiiiiiiiiiiiiiiiiiii".to_string()),              
            trie_test_data("iiiiiiiiiiiiiiiiiii".to_string()),               
            trie_test_data("iiiiiiiiiiiiiiiiii".to_string()),      
            trie_test_data("iiiiiiiiiiiiiiiiioiiiiiii".to_string()),          
            trie_test_data("iiiiiiiiiiiiiiiiioiiiiii".to_string()),           
            trie_test_data("iiiiiiiiiiiiiiiiioiiiii".to_string()),            
            trie_test_data("iiiiiiiiiiiiiiiiioiiii".to_string()),             
            trie_test_data("iiiiiiiiiiiiiiiiioiii".to_string()),              
            trie_test_data("iiiiiiiiiiiiiiiiioii".to_string()),               
            trie_test_data("iiiiiiiiiiiiiiiiioi".to_string()),   
            trie_test_data("iiiiiiiiiiiiiiiiiwiiiiiii".to_string()),          
            trie_test_data("iiiiiiiiiiiiiiiiwwiiiiii".to_string()),           
            trie_test_data("iiiiiiiiiiiiiiiiasdioiiiii".to_string()),            
            trie_test_data("iiiiiiiiiiiiiiiiidsoiiii".to_string()),             
            trie_test_data("iiiiiiiiiiiiiiiiiasdoiii".to_string()),              
            trie_test_data("iiiiiiiiiiiiiiiiizxcoii".to_string()),               
            trie_test_data("iiiiiiiiiiiiiiiiisoi".to_string()),               
            trie_test_data("iiiiiiiiio".to_string()),             
            trie_test_data("iiiiiiiiiio".to_string()),            
            trie_test_data("iiiiiiiiiiio".to_string()),           
            trie_test_data("iiiiiiiiiiiio".to_string()),          
            trie_test_data("iiiiiiiiiiio".to_string()),           
            trie_test_data("iiiiiiiiiio".to_string()),            
            trie_test_data("iiiiiiiiio".to_string()),             
            trie_test_data("iiiiiiiio".to_string()),              
            trie_test_data("iiiiiiio".to_string()),               
            trie_test_data("iiiiiio".to_string()),                
            trie_test_data("iiiiiiiiiz".to_string()),             
            trie_test_data("iiiiiiiiiiz".to_string()),            
            trie_test_data("iiiiiiiiiiiz".to_string()),           
            trie_test_data("iiiiiiiiiiiiz".to_string()),          
            trie_test_data("ppppppppppppppp".to_string()),        
            trie_test_data("ppppppppppppppp".to_string()),        
            trie_test_data("iiiiiiiiiz".to_string()),             
            trie_test_data("iiiiiiiiz".to_string()),              
            trie_test_data("iiiiiiiz".to_string()),
            trie_test_data("iiiiiiiiiiiiiiiiiouiiiiiii".to_string()),          
            trie_test_data("iiiiiiiiiiiiiiiiiouiiiiii".to_string()),           
            trie_test_data("iiiiiiiiiiiiiiiiiouiiiii".to_string()),            
            trie_test_data("iiiiiiiiiiiiiiiiiouiiii".to_string()),             
            trie_test_data("iiiiiiiiiiiiiiiiiouiii".to_string()),              
            trie_test_data("iiiiiiiiiiiiiiiiiouii".to_string()),               
            trie_test_data("iiiiiiiiiiiiiiiiioui".to_string()),   
            trie_test_data("iiiiiiiiiiiiiiiiiwuiiiiiii".to_string()),          
            trie_test_data("iiiiiiiiiiiiiiiiwwuiiiiii".to_string()),           
            trie_test_data("iiiiiiiiiiiiiiiiasudioiiiii".to_string()),            
            trie_test_data("iiiiiiiiiiiiiiiiidusoiiii".to_string()),             
            trie_test_data("iiiiiiiiiiiiiiiiiausdoiii".to_string()),              
            trie_test_data("iiiiiiiiiiiiiiiiizuxcoii".to_string()),               
            trie_test_data("i".to_string()),                
            trie_test_data("iii".to_string()),                
            trie_test_data("ii".to_string()),                
            trie_test_data("iiii".to_string()),                
            trie_test_data("iii".to_string()),                
            trie_test_data("iiiii".to_string()),                
            trie_test_data("iiii".to_string()),                
            trie_test_data("iiiiii".to_string()),                
            trie_test_data("iiiii".to_string()),                
            trie_test_data("iiiiiii".to_string()),                
            trie_test_data("iiiiiiii".to_string()),                
            trie_test_data("iiiiiiii".to_string()),                
            trie_test_data("iiiiiiiiiiiiiiiiisuoi".to_string()),
            trie_test_data("iiiiiiiiiiiiiiiiioiiiiiii".to_string()),          
            trie_test_data("iiiiiiiiiiiiiiiiioiiiiii".to_string()),           
            trie_test_data("iiiiiiiiiiiiiiiiioiiiii".to_string()),            
            trie_test_data("iiiiiiiiiiiiiiiiioiiii".to_string()),             
            trie_test_data("iiiiiiiiiiiiiiiiioiii".to_string()),              
            trie_test_data("iiiiiiiiiiiiiiiiioii".to_string()),               
            trie_test_data("iiiiiiiiiiiiiiiiioi".to_string()),   
            trie_test_data("iiiiiiiiiiiiiiiiiwiiiiiii".to_string()),          
            trie_test_data("iiiiiiiiiiiiiiiiwwiiiiii".to_string()),           
            trie_test_data("iiiiiiiiiiiiiiiiasdioiiiii".to_string()),            
            trie_test_data("iiiiiiiiiiiiiiiiidsoiiii".to_string()),             
            trie_test_data("iiiiiiiiiiiiiiiiiasdoiii".to_string()),              
            trie_test_data("iiiiiiiiiiiiiiiiizxcoii".to_string()),               
            trie_test_data("iiiiiiiiiiiiiiiiisoi".to_string()),
            trie_test_data("iiiiiiiiiuiiiiiiiioiiiiiii".to_string()),          
            trie_test_data("iiiiiiiiiuiiiiiiiioiiiiii".to_string()),           
            trie_test_data("iiiiiiiiiuiiiiiiiioiiiii".to_string()),            
            trie_test_data("iiiiiiiiiuiiiiiiiioiiii".to_string()),             
            trie_test_data("iiiiiiiiiuiiiiiiiioiii".to_string()),              
            trie_test_data("iiiiiiiiiuiiiiiiiioii".to_string()),               
            trie_test_data("iiiiiiiiiuiiiiiiiioi".to_string()),   
            trie_test_data("iiiiiiiiiuiiiiiiiiwiiiiiii".to_string()),          
            trie_test_data("iiiiiiiiiuiiiiiiiwwiiiiii".to_string()),           
            trie_test_data("iiiiiiiiiuiiiiiiiasdioiiiii".to_string()),            
            trie_test_data("iiiiiiiiiuiiiiiiiidsoiiii".to_string()),             
            trie_test_data("iiiiiiiiiuiiiiiiiiasdoiii".to_string()),              
            trie_test_data("iiiiiiiiiuiiiiiiiizxcoii".to_string()),               
            trie_test_data("iiiiiiiiiuiiiiiiiisoi".to_string()),                     
        ];
    
        entries
    }

    fn load_entries() -> Vec<TrieInputData<HashMap<String,String>>>{
        let entries = vec![
            trie_test_data("++++++".to_string()),         
            trie_test_data("+++++".to_string()),          
            trie_test_data("+++-+".to_string()),          
            trie_test_data("++++++".to_string()),         
            trie_test_data("+++-----".to_string()),       
            trie_test_data("hello".to_string()),          
            trie_test_data("world".to_string()),          
            trie_test_data("hello world".to_string()),    
            trie_test_data("hello world!".to_string()),   
            trie_test_data("helloo".to_string()),         
            trie_test_data("helloo world!".to_string()),          
            trie_test_data("helloo world".to_string()),           
            trie_test_data("aaaaaaaaaaaa".to_string()),           
            trie_test_data("jjjjjjjjjjjj".to_string()),           
            trie_test_data("aaaa".to_string()),                   
            trie_test_data("jjjj".to_string()),                   
            trie_test_data("a".to_string()),                      
            trie_test_data("j".to_string()),                      
            trie_test_data("aa".to_string()),                     
            trie_test_data("jj".to_string()),                     
            trie_test_data("aaa".to_string()),                    
            trie_test_data("jjj".to_string()),                    
            trie_test_data("as".to_string()),                     
            trie_test_data("aaaaaaas".to_string()),               
            trie_test_data("helloworld".to_string()),             
            trie_test_data("hello friends".to_string()),          
            trie_test_data("help world".to_string()),             
            trie_test_data("abcdefgh".to_string()),               
            trie_test_data("abcdefg".to_string()),                
            trie_test_data("abcdef".to_string()),                 
            trie_test_data("abcdefghijkl".to_string()),           
            trie_test_data("iiiiiiiiii".to_string()),             
            trie_test_data("iiiiiiiiiii".to_string()),            
            trie_test_data("iiiiiiiiiiii".to_string()),           
            trie_test_data("iiiiiiiiiiiii".to_string()),          
            trie_test_data("iiiiiiiiiiii".to_string()),           
            trie_test_data("iiiiiiiiiii".to_string()),            
            trie_test_data("iiiiiiiiii".to_string()),             
            trie_test_data("iiiiiiiii".to_string()),              
            trie_test_data("iiiiiiii".to_string()),               
            trie_test_data("iiiiiii".to_string()),
            trie_test_data("iiiiiiiiiiiiiiiiiiiiiiii".to_string()),          
            trie_test_data("iiiiiiiiiiiiiiiiiiiiiii".to_string()),           
            trie_test_data("iiiiiiiiiiiiiiiiiiiiii".to_string()),            
            trie_test_data("iiiiiiiiiiiiiiiiiiiii".to_string()),             
            trie_test_data("iiiiiiiiiiiiiiiiiiii".to_string()),              
            trie_test_data("iiiiiiiiiiiiiiiiiii".to_string()),               
            trie_test_data("iiiiiiiiiiiiiiiiii".to_string()),      
            trie_test_data("iiiiiiiiiiiiiiiiioiiiiiii".to_string()),          
            trie_test_data("iiiiiiiiiiiiiiiiioiiiiii".to_string()),           
            trie_test_data("iiiiiiiiiiiiiiiiioiiiii".to_string()),            
            trie_test_data("iiiiiiiiiiiiiiiiioiiii".to_string()),             
            trie_test_data("iiiiiiiiiiiiiiiiioiii".to_string()),              
            trie_test_data("iiiiiiiiiiiiiiiiioii".to_string()),               
            trie_test_data("iiiiiiiiiiiiiiiiioi".to_string()),   
            trie_test_data("iiiiiiiiiiiiiiiiiwiiiiiii".to_string()),          
            trie_test_data("iiiiiiiiiiiiiiiiwwiiiiii".to_string()),           
            trie_test_data("iiiiiiiiiiiiiiiiasdioiiiii".to_string()),            
            trie_test_data("iiiiiiiiiiiiiiiiidsoiiii".to_string()),             
            trie_test_data("iiiiiiiiiiiiiiiiiasdoiii".to_string()),              
            trie_test_data("iiiiiiiiiiiiiiiiizxcoii".to_string()),               
            trie_test_data("iiiiiiiiiiiiiiiiisoi".to_string()),               
            trie_test_data("iiiiiiiiio".to_string()),             
            trie_test_data("iiiiiiiiiio".to_string()),            
            trie_test_data("iiiiiiiiiiio".to_string()),           
            trie_test_data("iiiiiiiiiiiio".to_string()),          
            trie_test_data("iiiiiiiiiiio".to_string()),           
            trie_test_data("iiiiiiiiiio".to_string()),            
            trie_test_data("iiiiiiiiio".to_string()),             
            trie_test_data("iiiiiiiio".to_string()),              
            trie_test_data("iiiiiiio".to_string()),               
            trie_test_data("iiiiiio".to_string()),                
            trie_test_data("iiiiiiiiiz".to_string()),             
            trie_test_data("iiiiiiiiiiz".to_string()),            
            trie_test_data("iiiiiiiiiiiz".to_string()),           
            trie_test_data("iiiiiiiiiiiiz".to_string()),          
            trie_test_data("ppppppppppppppp".to_string()),        
            trie_test_data("ppppppppppppppp".to_string()),        
            trie_test_data("iiiiiiiiiz".to_string()),             
            trie_test_data("iiiiiiiiz".to_string()),              
            trie_test_data("iiiiiiiz".to_string()),               
            trie_test_data("iiiiiiz".to_string()),                
            trie_test_data("my nice spaceship".to_string()),      
            trie_test_data("my niece spaceship".to_string()),     
            trie_test_data("my nice niece spaceship".to_string()),
            trie_test_data("my nike".to_string()),                
            trie_test_data("my nice space flight".to_string()),   
            trie_test_data("may see it".to_string()),             
            trie_test_data("may seeing it".to_string()),          
            trie_test_data("matt".to_string()),                   
            trie_test_data("nay see it".to_string()),             
            trie_test_data("may ser it".to_string()),             
            trie_test_data("qxevtnuluntdqxtnig".to_string()),     
            trie_test_data("xqnigebqo".to_string()),              
            trie_test_data("xqnigebwo".to_string()),              
            trie_test_data("xqnigebeo".to_string()),              
            trie_test_data("xqnigebdo".to_string()),              
            trie_test_data("xqnigebco".to_string()),              
            trie_test_data("xqnigebxo".to_string()),              
            trie_test_data("xqnigebzo".to_string()),              
            trie_test_data("xqnigebao".to_string()),              
            trie_test_data("xqnigebso".to_string()),              
            trie_test_data("list of nice mooses".to_string()),    
            trie_test_data("list of nice mices".to_string()),     
            trie_test_data("list of nice macros".to_string()),    
            trie_test_data("list of neices".to_string()),         
            trie_test_data("list off ".to_string()),              
            trie_test_data("list of nicer mices".to_string()),    
            trie_test_data("list of nicer".to_string()),          
            trie_test_data("look at this 𓅂߷æ世 so nice".to_string()) ,
            trie_test_data("look at this ߷ææ世 so nice".to_string()) ,
            trie_test_data("look at this æ߷æ世 so nice".to_string()) ,
            trie_test_data("look at this 世ææ世 so nice".to_string()),
            trie_test_data("世世世世世世".to_string()),
            trie_test_data("世世世世世".to_string()),
            trie_test_data("世世世世".to_string()),
            trie_test_data("世߷世߷世߷世߷世".to_string()),
            trie_test_data("世߷世߷世߷世߷".to_string()),
            trie_test_data("世߷世߷世߷世".to_string()),
            trie_test_data("世߷世߷世߷𓅂".to_string()),
            trie_test_data("世߷世߷世߷世߷世𓅂".to_string()),
            trie_test_data("世߷世߷世߷世߷𓅂".to_string()), 
            trie_test_data("世߷世߷世߷世𓅂".to_string()), 
            trie_test_data("世߷世߷世߷𓅂".to_string()), 
            trie_test_data("世߷世߷世߷世߷𓅂𓅂".to_string()), 
            trie_test_data("世߷世߷世߷世𓅂𓅂".to_string()), 
            trie_test_data("世߷世߷世߷𓅂世".to_string()),      
            trie_test_data("世߷世߷世߷世߷æ𓅂".to_string()), 
            trie_test_data("世߷世߷世߷世𓅂æ".to_string()), 
            trie_test_data("世߷世߷世߷æ世".to_string()), 
            trie_test_data("世߷世߷世߷æ".to_string()), 
            trie_test_data("世߷世߷世æ世".to_string()), 
            trie_test_data("世߷世߷世æ".to_string()), 
            trie_test_data("世߷世߷æ世".to_string()), 
            trie_test_data("السلام عليكم".to_string()), 
            trie_test_data("Dobrý den".to_string()), 
            trie_test_data("שָׁלוֹם".to_string()), 
            trie_test_data("नमस्ते".to_string()), 
            trie_test_data("こんにちは".to_string()), 
            trie_test_data("안녕하세요".to_string()), 
            trie_test_data("你好".to_string()), 
            trie_test_data("Olá".to_string()), 
            trie_test_data("Здравствуйте".to_string()), 
            trie_test_data("Hola".to_string()), 
        ];
    
        entries
    }
    
    fn bench_insert_i(root: &TrieRoot<HashMap<String,String>>) {
        let mut nroot = (*root).clone();
        let entries = load_insert_bench_entries();
        nroot.build(&entries);
        
    }

    fn bench_fuzzy_find_case_a(root: &TrieRoot<HashMap<String,String>>) {
        root.force_fuzzy_search("hello world");
    }

    fn bench_fuzzy_find_case_b(root: &TrieRoot<HashMap<String,String>>) {
        root.force_fuzzy_search("世߷世߷世߷世߷");
    }

    fn bench_fuzzy_find_case_c(root: &TrieRoot<HashMap<String,String>>) {
        root.force_fuzzy_search("世");
    }

    fn bench_fuzzy_find_case_d(root: &TrieRoot<HashMap<String,String>>) {
        root.force_fuzzy_search("世߷");
    }

    fn bench_fuzzy_find_case_e(root: &TrieRoot<HashMap<String,String>>) {
        root.force_fuzzy_search("list");
    }

    fn bench_fuzzy_find_case_f(root: &TrieRoot<HashMap<String,String>>) {
        root.force_fuzzy_search("list of");
    }

    fn bench_fuzzy_find_case_g(root: &TrieRoot<HashMap<String,String>>) {
        root.force_fuzzy_search("list off");
    }

    fn bench_fuzzy_find_case_h(root: &TrieRoot<HashMap<String,String>>) {
        root.force_fuzzy_search("list of nice");
    }

    fn bench_fuzzy_find_case_i(root: &TrieRoot<HashMap<String,String>>) {
        root.force_fuzzy_search("iiiiii");
    }

    fn bench_fuzzy_find_case_j(root: &TrieRoot<HashMap<String,String>>) {
        root.force_fuzzy_search("iiiiiiii");
    }

    fn bench_fuzzy_find_case_k(root: &TrieRoot<HashMap<String,String>>) {
        root.force_fuzzy_search("iiiiiiiii");
    }

    fn bench_fuzzy_find_case_l(root: &TrieRoot<HashMap<String,String>>) {
        root.force_fuzzy_search("iiiiiiiiii");
    }

    fn bench_fuzzy_find_case_m(root: &TrieRoot<HashMap<String,String>>) {
        root.force_fuzzy_search("iiiiiiiiiii");
    }

    fn bench_fuzzy_find_case_n(root: &TrieRoot<HashMap<String,String>>) {
        root.force_fuzzy_search("iiiiiiiiiiii");
    }

    fn bench_fuzzy_find_case_o(root: &TrieRoot<HashMap<String,String>>) {
        root.force_fuzzy_search("iiiiiiiiiiiio");
    }

    fn bench_fuzzy_find_case_oo(root: &TrieRoot<HashMap<String,String>>) {
        root.force_fuzzy_search("iiiiiiiiiiiiiiio");
    }

    fn bench_fuzzy_find_case_ooo(root: &TrieRoot<HashMap<String,String>>) {
        root.force_fuzzy_search("iiiiiiiiiiiiiiiiiio");
    }

    fn bench_fuzzy_find_case_oooo(root: &TrieRoot<HashMap<String,String>>) {
        root.force_fuzzy_search("iiiiiiiiiiiiiiuuuuo");
    }

    fn bench_fuzzy_find_case_ooooo(root: &TrieRoot<HashMap<String,String>>) {
        root.force_fuzzy_search("iiiiiiiiiiiiiiiiiiiuuo");
    }
    
    fn bench_fuzzy_find_case_ooooi(root: &TrieRoot<HashMap<String,String>>) {
        root.force_fuzzy_search("iiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooo");
    }

    fn bench_fuzzy_find_case_p(root: &TrieRoot<HashMap<String,String>>) {
        root.non_fuzzy_search("hello");
    }

    fn bench_fuzzy_find_case_q(root: &TrieRoot<HashMap<String,String>>) {
        root.non_fuzzy_search("hello world");
    }

    fn bench_fuzzy_find_case_r(root: &TrieRoot<HashMap<String,String>>) {
        root.non_fuzzy_search("look at this 𓅂߷");
    }

    fn bench_fuzzy_find_case_s(root: &TrieRoot<HashMap<String,String>>) {
        root.non_fuzzy_search("look at this ߷");
    }

    fn bench_fuzzy_find_case_t(root: &TrieRoot<HashMap<String,String>>) {
        root.non_fuzzy_search("abcdef");
    }

    fn bench_fuzzy_find_case_u(root: &TrieRoot<HashMap<String,String>>) {
        root.non_fuzzy_search("Saint Peter Drink coffee at the corner of the striet");
    }

    // fn load_real_entries() -> Vec<TrieInputData>{

    //     let bkm =
    //         fs::read_to_string("./test_data/bkm90.json").expect("Should have been able to read the file");

    //     let entries_bkm: Vec<TrieInputData> = serde_json::from_str(&bkm).unwrap();

    //     let mut entries = entries_bkm;

    //     entries
    // }

    #[bench]
    fn bench_insert(b: &mut Bencher) {
        let root = TrieRoot::new(4,Some(100));
        b.iter(|| bench_insert_i(&root));
    }

    #[bench]
    fn bench_fuzzy_case_b(b: &mut Bencher) {
        let entries = load_entries();

        let mut root = TrieRoot::new(4,Some(100));
        root.build(entries.as_slice());

        b.iter(|| bench_fuzzy_find_case_b(&root));
    }

    #[bench]
    fn bench_fuzzy_case_c(b: &mut Bencher) {
        let entries = load_entries();

        let mut root = TrieRoot::new(4,Some(100));
        root.build(entries.as_slice());

        b.iter(|| bench_fuzzy_find_case_c(&root));
    }

    #[bench]
    fn bench_fuzzy_case_d(b: &mut Bencher) {
        let entries = load_entries();

        let mut root = TrieRoot::new(4,Some(100));
        root.build(entries.as_slice());

        b.iter(|| bench_fuzzy_find_case_d(&root));
    }

    #[bench]
    fn bench_fuzzy_case_e(b: &mut Bencher) {
        let entries = load_entries();

        let mut root = TrieRoot::new(4,Some(100));
        root.build(entries.as_slice());

        b.iter(|| bench_fuzzy_find_case_e(&root));
    }

    #[bench]
    fn bench_fuzzy_case_f(b: &mut Bencher) {
        let entries = load_entries();

        let mut root = TrieRoot::new(4,Some(100));
        root.build(entries.as_slice());

        b.iter(|| bench_fuzzy_find_case_f(&root));
    }

    #[bench]
    fn bench_fuzzy_case_a(b: &mut Bencher) {
        let entries = load_entries();

        let mut root = TrieRoot::new(4,Some(100));
        root.build(entries.as_slice());

        b.iter(|| bench_fuzzy_find_case_a(&root));
    }

    #[bench]
    fn bench_fuzzy_case_g(b: &mut Bencher) {
        let entries = load_entries();

        let mut root = TrieRoot::new(4,Some(100));
        root.build(entries.as_slice());

        b.iter(|| bench_fuzzy_find_case_g(&root));
    }

    #[bench]
    fn bench_fuzzy_case_h(b: &mut Bencher) {
        let entries = load_entries();

        let mut root = TrieRoot::new(4,Some(100));
        root.build(entries.as_slice());

        b.iter(|| bench_fuzzy_find_case_h(&root));
    }


    #[bench]
    fn bench_fuzzy_case_i(b: &mut Bencher) {
        let entries = load_entries();

        let mut root = TrieRoot::new(4,Some(100));
        root.build(entries.as_slice());

        b.iter(|| bench_fuzzy_find_case_i(&root));
    }

    
    #[bench]
    fn bench_fuzzy_case_j(b: &mut Bencher) {
        let entries = load_entries();

        let mut root = TrieRoot::new(4,Some(100));
        root.build(entries.as_slice());

        b.iter(|| bench_fuzzy_find_case_j(&root));
    }

    #[bench]
    fn bench_fuzzy_case_k(b: &mut Bencher) {
        let entries = load_entries();

        let mut root = TrieRoot::new(4,Some(100));
        root.build(entries.as_slice());

        b.iter(|| bench_fuzzy_find_case_k(&root));
    }


    #[bench]
    fn bench_fuzzy_case_l(b: &mut Bencher) {
        let entries = load_entries();

        let mut root = TrieRoot::new(4,Some(100));
        root.build(entries.as_slice());

        b.iter(|| bench_fuzzy_find_case_l(&root));
    }

    #[bench]
    fn bench_fuzzy_case_m(b: &mut Bencher) {
        let entries = load_entries();

        let mut root = TrieRoot::new(4,Some(100));
        root.build(entries.as_slice());

        b.iter(|| bench_fuzzy_find_case_m(&root));
    }

    #[bench]
    fn bench_fuzzy_case_n(b: &mut Bencher) {
        let entries = load_entries();

        let mut root = TrieRoot::new(4,Some(100));
        root.build(entries.as_slice());

        b.iter(|| bench_fuzzy_find_case_n(&root));
    }


    #[bench]
    fn bench_fuzzy_case_o(b: &mut Bencher) {
        let entries = load_entries();

        let mut root = TrieRoot::new(4,Some(100));
        root.build(entries.as_slice());

        b.iter(|| bench_fuzzy_find_case_o(&root));
    }

    #[bench]
    fn bench_fuzzy_case_oo(b: &mut Bencher) {
        let entries = load_entries();

        let mut root = TrieRoot::new(4,Some(100));
        root.build(entries.as_slice());

        b.iter(|| bench_fuzzy_find_case_oo(&root));
    }

    #[bench]
    fn bench_fuzzy_case_ooo(b: &mut Bencher) {
        let entries = load_entries();

        let mut root = TrieRoot::new(4,Some(100));
        root.build(entries.as_slice());

        b.iter(|| bench_fuzzy_find_case_ooo(&root));
    }

    #[bench]
    fn bench_fuzzy_case_oooo(b: &mut Bencher) {
        let entries = load_entries();

        let mut root = TrieRoot::new(4,Some(100));
        root.build(entries.as_slice());

        b.iter(|| bench_fuzzy_find_case_oooo(&root));
    }

    #[bench]
    fn bench_fuzzy_case_ooooo(b: &mut Bencher) {
        let entries = load_entries();

        let mut root = TrieRoot::new(4,Some(100));
        root.build(entries.as_slice());

        b.iter(|| bench_fuzzy_find_case_ooooo(&root));
    }

    #[bench]
    fn bench_fuzzy_case_ooooi(b: &mut Bencher) {
        let entries = load_entries();

        let mut root = TrieRoot::new(4,Some(100));
        root.build(entries.as_slice());

        b.iter(|| bench_fuzzy_find_case_ooooi(&root));
    }

    #[bench]
    fn bench_fuzzy_case_p(b: &mut Bencher) {
        let entries = load_entries();

        let mut root = TrieRoot::new(4,Some(100));
        root.build(entries.as_slice());

        b.iter(|| bench_fuzzy_find_case_p(&root));
    }

    #[bench]
    fn bench_fuzzy_case_q(b: &mut Bencher) {
        let entries = load_entries();

        let mut root = TrieRoot::new(4,Some(100));
        root.build(entries.as_slice());
        b.iter(|| bench_fuzzy_find_case_q(&root));
    }

    #[bench]
    fn bench_fuzzy_case_r(b: &mut Bencher) {
        let entries = load_entries();

        let mut root = TrieRoot::new(4,Some(100));
        root.build(entries.as_slice());
        b.iter(|| bench_fuzzy_find_case_r(&root));
    }

    #[bench]
    fn bench_fuzzy_case_s(b: &mut Bencher) {
        let entries = load_entries();

        let mut root = TrieRoot::new(4,Some(100));
        root.build(entries.as_slice());
        b.iter(|| bench_fuzzy_find_case_s(&root));
    }

    #[bench]
    fn bench_fuzzy_case_u(b: &mut Bencher) {
        let entries = load_entries();

        let mut root = TrieRoot::new(4,Some(100));
        root.build(entries.as_slice());
        b.iter(|| bench_fuzzy_find_case_u(&root));
    }

    #[bench]
    fn bench_fuzzy_case_t(b: &mut Bencher) {
        let entries = load_entries();

        let mut root = TrieRoot::new(4,Some(100));
        root.build(entries.as_slice());
        b.iter(|| bench_fuzzy_find_case_t(&root));
    }
}
