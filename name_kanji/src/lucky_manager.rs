extern crate csv;
use std::collections::HashMap;

#[derive(Debug)]
#[derive(RustcDecodable)]
// 画数に対する吉タイプの管理
pub struct LuckyManager {
    male_map: HashMap<u32, i32>, //画数 => 吉タイプへのマップ
    female_map: HashMap<u32, i32>, // 画数 => 吉タイプのマップ
    max_writing_count:u32 // 最大画数
}

#[derive(Debug)]
#[derive(RustcDecodable)]
struct LuckyRecord {
    writing_count: u32,
    male_point: i32,
    female_point: i32
}

impl LuckyManager {
    pub fn load_csv(csv_path: &str) -> LuckyManager {
        let mut male_map = HashMap::new();
        let mut female_map = HashMap::new();
        let mut max_writing_count = 0u32;
        let mut reader = csv::Reader::from_file(csv_path).unwrap().has_headers(false);
        for record in reader.decode() {
            let record: LuckyRecord = record.unwrap();
            male_map.insert(record.writing_count, record.male_point);
            female_map.insert(record.writing_count, record.female_point);
            if record.writing_count > max_writing_count {
                max_writing_count = record.writing_count;
            }
        }

        LuckyManager {
            male_map: male_map,
            female_map: female_map,
            max_writing_count: max_writing_count
        }
        // let mut reader = csv::Reader::from_file(csv_path).unwrap().has_headers(false);
        // for record in reader.decode() {
        //     let record: LuckyRecord = record.unwrap();
        //     lucky_manager.male_map.insert(record.writing_count, record.male_point);
        //     lucky_manager.female_map.insert(record.writing_count, record.female_point);
        // }
        // lucky_manager
    }

    pub fn get_max_writing_count(&self) -> u32{
        self.max_writing_count
    }

        // 吉運になる数を取得
    pub fn get_lucky_writing_count_vec(&self, is_male : bool)-> Vec<u32>{
        let mut lucky_vec: Vec<u32> = Vec::new();
        for writing_count in 1..82 {
            if self.get_lucky_point(writing_count, is_male) > 0 {
                lucky_vec.push(writing_count);
            }
        }
        lucky_vec
    }

    pub fn get_lucky_point(&self, writing_count: u32, is_male: bool) -> i32 {
        if is_male {
            return match self.male_map.get(&writing_count) {
                Some(m) => *m,
                None => {
                    println!("Not Supported Count{}", writing_count);
                    0
                }
            }
        } else {
            return match self.female_map.get(&writing_count) {
                Some(m) => *m,
                None => {
                    println!("Not Supported Count{}", writing_count);
                    0
                }
            }
        }
    }
}
