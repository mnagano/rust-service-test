extern crate csv;
use std::collections::HashMap;

#[derive(Debug)]
#[derive(RustcDecodable)]
// 名前に使える漢字の管理
pub struct KanjiManager {
    kanji_map: HashMap<u32, Vec<char>>, //画数=>漢字のマップ
    writing_count_map: HashMap<char, u32>, //漢字 => 新字体の画数へのマップ
    max_writing_count:u32,
}

#[derive(RustcDecodable)]
struct KanjiRecord {
    kanji: char,
    writing_count: u32,
}

impl KanjiManager {
    pub fn load_csv(kanji_csv_path: &str) -> KanjiManager {
        let mut kanji_map: HashMap<u32, Vec<char>> = HashMap::new();
        let mut writing_count_map: HashMap<char, u32> = HashMap::new();
        let mut reader = csv::Reader::from_file(kanji_csv_path).unwrap().has_headers(false);
        let mut max_writing_count = 0u32;
        for record in reader.decode() {
            let record: KanjiRecord = record.unwrap();
            if kanji_map.get(&record.writing_count).is_none() {
                kanji_map.insert(record.writing_count, Vec::new());
            }
            kanji_map.get_mut(&record.writing_count).unwrap().push(record.kanji);
            writing_count_map.insert(record.kanji, record.writing_count);
            if record.writing_count > max_writing_count{
                max_writing_count = record.writing_count;
            }
        }
        KanjiManager {
            kanji_map: kanji_map,
            writing_count_map: writing_count_map,
            max_writing_count: max_writing_count
        }
    }

    pub fn get_max_writing_count(&self) -> u32 {
        self.max_writing_count
    }

    pub fn get_writing_count_of_kanji(&self, kanji: &char) -> Option<u32> {
        match self.writing_count_map.get(&kanji) {
            Some(m) => Some(*m),
            None => None
        }
    }

    pub fn get_writing_count_of_name(&self, last_name: &str) -> u32 {
        let mut count = 0u32;
        for kanji in last_name.chars() {
            match self.get_writing_count_of_kanji(&kanji) {
                Some(m) => count += m,
                None => println!("kanji not supported.{:?}", &kanji),
            }
        }
        count
    }

    pub fn get_kanji_vec(&self, writing_count:u32) -> Option<&Vec<char>> {
        self.kanji_map.get(&writing_count)
    }
}
