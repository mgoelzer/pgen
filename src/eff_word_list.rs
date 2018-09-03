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
 * Example usage of EffWordList
 *
 * {
 *   let ewords:EffWordList = EffWordList::new();
 *   let s:&str = ewords.nth(7775);
 *   println!("s = {}", s);
 * }
*/
