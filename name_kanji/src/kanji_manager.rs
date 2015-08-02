extern crate csv;
use std::collections::HashMap;

#[derive(Debug)]
#[derive(RustcDecodable)]
// 名前に使える漢字の管理
pub struct KanjiManager {
    all_map: HashMap<u32, Vec<char>>, //画数=>漢字のマップ
    all_old_map: HashMap<u32, Vec<char>>, //画数=>漢字のマップ
    kanji_map: HashMap<u32, Vec<char>>, //画数=>漢字のマップ
    kanji_old_map: HashMap<u32, Vec<char>>, //画数=>漢字のマップ
    hiragana_map: HashMap<u32, Vec<char>>, //画数=>ひらがなのマップ
    hiragana_old_map: HashMap<u32, Vec<char>>, //画数=>漢字のマップ
    writing_count_map: HashMap<char, u32>, //漢字 => 新字体の画数へのマップ
    writing_count_old_map: HashMap<char, u32>, //漢字 => 新字体の画数へのマップ
    max_writing_count: u32,
    max_writing_count_old: u32,
}

#[derive(RustcDecodable)]
struct KanjiRecord {
    kanji: char,
    writing_count: u32,
    writing_count_old: u32,
}

impl KanjiManager {
    pub fn load_csv(kanji_csv_path: &str, hiragana_csv_path: &str) -> KanjiManager {
        let mut all_map: HashMap<u32, Vec<char>> = HashMap::new();
        let mut all_old_map: HashMap<u32, Vec<char>> = HashMap::new();
        let mut kanji_map: HashMap<u32, Vec<char>> = HashMap::new();
        let mut kanji_old_map: HashMap<u32, Vec<char>> = HashMap::new();
        let mut hiragana_map: HashMap<u32, Vec<char>> = HashMap::new();
        let mut hiragana_old_map: HashMap<u32, Vec<char>> = HashMap::new();
        let mut writing_count_map: HashMap<char, u32> = HashMap::new();
        let mut writing_count_old_map: HashMap<char, u32> = HashMap::new();
        let mut max_writing_count = 0u32;
        let mut max_writing_count_old = 0u32;

        let mut reader = csv::Reader::from_file(kanji_csv_path).unwrap().has_headers(false);
        for record in reader.decode() {
            let record: KanjiRecord = record.unwrap();
            if all_map.get(&record.writing_count).is_none() {
                all_map.insert(record.writing_count, Vec::new());
            }
            if kanji_map.get(&record.writing_count).is_none() {
                kanji_map.insert(record.writing_count, Vec::new());
            }
            all_map.get_mut(&record.writing_count).unwrap().push(record.kanji);
            kanji_map.get_mut(&record.writing_count).unwrap().push(record.kanji);
            writing_count_map.insert(record.kanji, record.writing_count);
            if all_old_map.get(&record.writing_count_old).is_none() {
                all_old_map.insert(record.writing_count_old, Vec::new());
            }
            if kanji_old_map.get(&record.writing_count_old).is_none() {
                kanji_old_map.insert(record.writing_count_old, Vec::new());
            }
            all_old_map.get_mut(&record.writing_count_old).unwrap().push(record.kanji);
            kanji_old_map.get_mut(&record.writing_count_old).unwrap().push(record.kanji);
            writing_count_old_map.insert(record.kanji, record.writing_count_old);
            if record.writing_count > max_writing_count {
                max_writing_count = record.writing_count;
            }
            if record.writing_count_old > max_writing_count_old {
                max_writing_count_old = record.writing_count_old;
            }
        }
        reader = csv::Reader::from_file(hiragana_csv_path).unwrap().has_headers(false);
        for record in reader.decode() {
            let record: KanjiRecord = record.unwrap();
            if all_map.get(&record.writing_count).is_none() {
                all_map.insert(record.writing_count, Vec::new());
            }
            if hiragana_map.get(&record.writing_count).is_none() {
                hiragana_map.insert(record.writing_count, Vec::new());
            }
            all_map.get_mut(&record.writing_count).unwrap().push(record.kanji);
            hiragana_map.get_mut(&record.writing_count).unwrap().push(record.kanji);
            writing_count_map.insert(record.kanji, record.writing_count);
            if all_old_map.get(&record.writing_count_old).is_none() {
                all_old_map.insert(record.writing_count_old, Vec::new());
            }
            if hiragana_old_map.get(&record.writing_count_old).is_none() {
                hiragana_old_map.insert(record.writing_count_old, Vec::new());
            }
            all_old_map.get_mut(&record.writing_count_old).unwrap().push(record.kanji);
            hiragana_old_map.get_mut(&record.writing_count_old).unwrap().push(record.kanji);
            writing_count_old_map.insert(record.kanji, record.writing_count_old);
            if record.writing_count > max_writing_count {
                max_writing_count = record.writing_count;
            }
            if record.writing_count_old > max_writing_count_old {
                max_writing_count_old = record.writing_count_old;
            }
        }
        KanjiManager {
            all_map: all_map,
            kanji_map: kanji_map,
            hiragana_map: hiragana_map,
            writing_count_map: writing_count_map,
            max_writing_count: max_writing_count,
            all_old_map: all_old_map,
            kanji_old_map: kanji_old_map,
            hiragana_old_map: hiragana_old_map,
            writing_count_old_map: writing_count_old_map,
            max_writing_count_old: max_writing_count_old
        }
    }

    pub fn get_max_writing_count(&self) -> u32 {
        self.max_writing_count
    }

    pub fn get_writing_count_of_kanji(&self, kanji: &char, is_new_count: bool) -> Option<u32> {
        if is_new_count {
            match self.writing_count_map.get(&kanji) {
                Some(m) => Some(*m),
                None => None
            }
        } else {
            match self.writing_count_old_map.get(&kanji) {
                Some(m) => Some(*m),
                None => None
            }
        }
    }

    pub fn get_writing_count_of_name(&self, last_name: &str, is_new_count: bool) -> u32 {
        let mut count = 0u32;
        for kanji in last_name.chars() {
            match self.get_writing_count_of_kanji(&kanji, is_new_count) {
                Some(m) => count += m,
                None => println!("kanji not supported.{:?}", &kanji),
            }
        }
        count
    }

    pub fn get_kanji_vec(&self,
                         writing_count: u32,
                         is_new_count: bool,
                         is_include_cana: bool)
                         -> Option<&Vec<char>> {
        if is_include_cana {
            if is_new_count {
                self.all_map.get(&writing_count)
            } else {
                self.all_old_map.get(&writing_count)
            }
        } else {
            if is_new_count {
                self.kanji_map.get(&writing_count)
            } else {
                self.kanji_old_map.get(&writing_count)
            }
        }
    }
}
