pub static WORD_LIST: &'static str = include_str!("../data/eff_word_list.txt");

pub struct EffWordList {
    vec_word_list: Vec<&'static str>,
}

impl EffWordList {
    pub fn new() -> EffWordList {
        let _vec_word_list: Vec<&str> = 
            WORD_LIST
            .lines()
            .collect();
        //assert_eq!(_vec_word_list[_vec_word_list.len()-1], "66666\tzoom");
        //println!(concat!("_vec_word_list.len() = {}\n",
        //    "_vec_word_list[_vec_word_list.len()-1] = '{}'\n",
        //    "_vec_word_list[_vec_word_list.len()-2] = '{}'"),
        //    _vec_word_list.len(),
        //    _vec_word_list[_vec_word_list.len()-1],
        //    _vec_word_list[_vec_word_list.len()-2]
        //);
        EffWordList{ vec_word_list: _vec_word_list }
    }

    // 0-indexed
    pub fn nth(&self, n: i32) -> &str {
        assert!(n >= 0);
        assert!((n as usize) < self.vec_word_list.len()); 
        let s:&str = self.vec_word_list[n as usize];
        s
    }

    pub fn len(&self) -> usize {
        return self.vec_word_list.len();
    }
}

/*
fn test_main() {
    let ewords:EffWordList = EffWordList::new();
    let s:&str = ewords.nth(7775);
    println!("s = {}", s);
}
*/
