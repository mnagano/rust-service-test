extern crate csv;
extern crate rustc_serialize;
use std::collections::HashMap;
use rustc_serialize::*;
use std::result::*;
use std::cmp::Ordering;

#[derive(Debug)]
#[derive(RustcDecodable)]
// 名前に使える漢字の管理
pub struct NameKanjiManager {
    kanji_map: HashMap<u32, Vec<char>>, //画数=>漢字のマップ
    writing_count_map: HashMap<char, u32>, //漢字 => 新字体の画数へのマップ
    lucky_manager: LuckyManager,
}

#[derive(Debug)]
#[derive(RustcDecodable)]
// 画数に対する吉タイプの管理
pub struct LuckyManager {
    male_map: HashMap<u32, i32>, //画数 => 吉タイプへのマップ
    female_map: HashMap<u32, i32>, // 画数 => 吉タイプのマップ
}

#[derive(Debug)]
#[derive(RustcDecodable, RustcEncodable)]
// 各名前ごとの画数, 吉タイプのデータ
pub struct NameCounts {
    is_male: bool,
    first_name: String,
    last_name: String,
    last_name_writing_count_vec: Vec<u32>,
    first_name_writing_count_vec: Vec<u32>,
    ten_un: u32,
    jin_un: u32,
    ti_un: u32,
    gai_un: u32,
    sou_un: u32,
    in_myou_point: i32,
    ten_un_point: i32,
    jin_un_point: i32,
    gai_un_point: i32,
    ti_un_point: i32,
    sou_un_point: i32,
    total_point: i32,
}

#[derive(RustcDecodable)]
struct KanjiRecord {
    kanji: char,
    writing_count: u32,
}

#[derive(Debug)]
#[derive(RustcDecodable)]
struct LuckyRecord {
    writing_count: u32,
    male_point: i32,
    female_point: i32
}

impl NameCounts {

    fn create(first_name: &str,
              last_name: &str,
              first_name_writing_count_vec: &Vec<u32>,
              last_name_writing_count_vec: &Vec<u32>,
              is_male: bool,
              kanji_manager: &NameKanjiManager)
              -> NameCounts {
        let jin_un = last_name_writing_count_vec[last_name_writing_count_vec.len() - 1] +
                     first_name_writing_count_vec[0];
        let ten_un = last_name_writing_count_vec.iter().fold(0, |sum, x| sum + x);
        let ti_un = first_name_writing_count_vec.iter().fold(0, |sum, x| sum + x);
        let mut gai_un: u32 = 0u32;
        let last_name_len = last_name_writing_count_vec.len();
        if last_name_len > 2 {
            gai_un += last_name_writing_count_vec[1];
        }
        gai_un += last_name_writing_count_vec[0];
        let first_name_len = first_name_writing_count_vec.len();
        if first_name_len > 2 {
            gai_un += first_name_writing_count_vec[first_name_len - 2];
        }
        gai_un += first_name_writing_count_vec[first_name_len - 1];
        let sou_un = ten_un + ti_un;
        let in_myou_point: i32 =
            ((first_name_writing_count_vec[first_name_writing_count_vec.len() - 1] +
              last_name_writing_count_vec[0]) % 2) as i32;
        let ten_un_point = kanji_manager.lucky_manager.get_lucky_point(ten_un, is_male);
        let ti_un_point = kanji_manager.lucky_manager.get_lucky_point(ti_un, is_male);
        let jin_un_point = kanji_manager.lucky_manager.get_lucky_point(jin_un, is_male);
        let gai_un_point = kanji_manager.lucky_manager.get_lucky_point(gai_un, is_male);
        let sou_un_point = kanji_manager.lucky_manager.get_lucky_point(sou_un, is_male);
        let total_point = ten_un_point + ti_un_point + jin_un_point + gai_un_point + sou_un_point +
                          in_myou_point;

        NameCounts {
            is_male: is_male,
            first_name: first_name.to_string(),
            last_name: last_name.to_string(),
            last_name_writing_count_vec: last_name_writing_count_vec.to_vec(),
            first_name_writing_count_vec: first_name_writing_count_vec.to_vec(),
            ten_un: ten_un,
            jin_un: jin_un,
            ti_un: ti_un,
            gai_un: gai_un,
            sou_un: sou_un,
            in_myou_point: in_myou_point,
            ten_un_point: ten_un_point,
            ti_un_point: ti_un_point,
            jin_un_point: jin_un_point,
            gai_un_point: gai_un_point,
            sou_un_point: sou_un_point,
            total_point: total_point
        }
    }

    fn new(first_name: &str,
           last_name: &str,
           is_male: bool,
           kanji_manager: &NameKanjiManager)
           -> Result<NameCounts, Vec<char>> {
        let mut last_name_writing_count_vec = Vec::new();
        let mut first_name_writing_count_vec = Vec::new();
        let mut err_char_vec = Vec::new();
        for c in last_name.chars() {
            match kanji_manager.get_writing_count_kanji(&c) {
                Some(m) => last_name_writing_count_vec.push(m),
                None => {
                    println!("kanji not supported.{:?}", &c);
                    err_char_vec.push(c);
                }
            }
        }
        for c in first_name.chars() {
            match kanji_manager.get_writing_count_kanji(&c) {
                Some(m) => first_name_writing_count_vec.push(m),
                None => {
                    println!("kanji not supported.{:?}", &c);
                    err_char_vec.push(c);
                }
            }
        }
        if err_char_vec.len() != 0{
            return Err(err_char_vec);
        }
        Ok(NameCounts::create(first_name, last_name, &first_name_writing_count_vec,
                           &last_name_writing_count_vec, is_male, kanji_manager))
    }

    fn cmp_by_total_point(&self, target : &NameCounts) -> Ordering{
        if self.total_point < target.total_point {
            return Ordering::Less;
        } else if self.total_point > target.total_point {
            return Ordering::Greater;
        } else {
            return Ordering::Equal;
        }
    }
}

impl LuckyManager {
    pub fn load_csv(csv_path: &str) -> LuckyManager {
        let mut lucky_manager = LuckyManager {
            male_map: HashMap::new(),
            female_map: HashMap::new()
        };
        let mut reader = csv::Reader::from_file(csv_path).unwrap().has_headers(false);
        for record in reader.decode() {
            let record: LuckyRecord = record.unwrap();
            lucky_manager.male_map.insert(record.writing_count, record.male_point);
            lucky_manager.female_map.insert(record.writing_count, record.female_point);
        }
        lucky_manager
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

impl NameKanjiManager {
    pub fn load_csv(kanji_csv_path: &str, lucky_csv_path: &str) -> NameKanjiManager {
        let mut kanji_map: HashMap<u32, Vec<char>> = HashMap::new();
        let mut writing_count_map: HashMap<char, u32> = HashMap::new();
        let mut reader = csv::Reader::from_file(kanji_csv_path).unwrap().has_headers(false);
        for record in reader.decode() {
            let record: KanjiRecord = record.unwrap();
            if kanji_map.get(&record.writing_count).is_none() {
                kanji_map.insert(record.writing_count, Vec::new());
            }
            kanji_map.get_mut(&record.writing_count).unwrap().push(record.kanji);
            writing_count_map.insert(record.kanji, record.writing_count);
        }
        NameKanjiManager {
            kanji_map: kanji_map,
            writing_count_map: writing_count_map,
            lucky_manager: LuckyManager::load_csv(lucky_csv_path)
        }
    }

    pub fn get_writing_count_kanji(&self, kanji: &char) -> Option<u32> {
        match self.writing_count_map.get(&kanji) {
            Some(m) => Some(*m),
            None => None
        }
    }

    pub fn get_writing_count(&self, last_name: &str) -> u32 {
        let mut count = 0u32;
        for kanji in last_name.chars() {
            match self.get_writing_count_kanji(&kanji) {
                Some(m) => count += m,
                None => println!("kanji not supported.{:?}", &kanji),
            }
        }
        count
    }

    pub fn get_name_counts(&self, first_name: &str, last_name: &str, is_male: bool) -> Result<NameCounts, Vec<char>> {
        NameCounts::new(first_name, last_name, is_male, self)
    }

    pub fn get_lucky_point_candidates(&self, last_name: &str, is_male: bool) -> Vec<NameCounts> {
        // println!("last_name:{:?}", last_name);
        // println!("is_male:{:?}", is_male);
        let mut last_name_writing_count_vec = Vec::new();
        for c in last_name.chars() {
            match self.get_writing_count_kanji(&c) {
                Some(m) => last_name_writing_count_vec.push(m),
                None => println!("Not supported:{:?}", &c)
            }
        }
        let mut lucky_vec: Vec<u32> = Vec::new();
        for writing_count in 1..82 {
            if self.lucky_manager.get_lucky_point(writing_count, is_male) > 0 {
                lucky_vec.push(writing_count);
            }
        }
        // println!("lucky_ve:{:?}", &lucky_vec);
        let ten_un = last_name_writing_count_vec.iter().fold(0, |sum, x| sum + x);
        let last_kanji_count = last_name_writing_count_vec[last_name_writing_count_vec.len() - 1];
        let mut last_name_gai_un: u32 = 0u32;
        let last_name_len = last_name_writing_count_vec.len();
        if last_name_len > 2 {
            last_name_gai_un += last_name_writing_count_vec[1];
        }
        last_name_gai_un += last_name_writing_count_vec[0];
        let mut first_name_candidate_vec = Vec::new();
        for jin_un in lucky_vec.to_vec() {
            if jin_un <= last_kanji_count {
                continue;
            }
            let kanji_count = jin_un - last_kanji_count;
            if kanji_count > 29 { continue; }
            if self.kanji_map.get(&kanji_count).is_none() { continue; }
            // println!("jin_un:{}", jin_un);
            let mut first_name_writing_count_vec = Vec::new();
            first_name_writing_count_vec.push(kanji_count);
            // println!("kanji_count:{:?}", kanji_count);
            for ti_un in lucky_vec.to_vec() {
                if ti_un < kanji_count { continue };
                if self.lucky_manager.get_lucky_point(ti_un, is_male) < 1 { continue; }
                let sou_un = ten_un + ti_un;
                if sou_un > 81 { continue };
                if self.lucky_manager.get_lucky_point(sou_un, is_male) < 1 { continue; }
                // println!("ti_un:{}", ti_un);
                // println!("sou_un:{}", sou_un);
                let mut gai_un = last_name_gai_un;
                if ti_un == kanji_count {
                    //1文字
                    gai_un += ti_un;
                    if self.lucky_manager.get_lucky_point(gai_un, is_male) < 1 { continue; }
                    first_name_candidate_vec.push(first_name_writing_count_vec.to_vec());
                } else {
                    //3文字まで
                    gai_un += ti_un - kanji_count;
                    if self.lucky_manager.get_lucky_point(gai_un, is_male) < 1 { continue; }
                    for next_kanji_count in (1)..(ti_un - kanji_count + 1) {
                        if next_kanji_count > 29 { continue; }
                        if self.kanji_map.get(&next_kanji_count).is_none() { continue; }
                        first_name_writing_count_vec.truncate(1);
                        first_name_writing_count_vec.push(next_kanji_count);
                        let next2_kanji_count = ti_un - kanji_count - next_kanji_count;
                        if next2_kanji_count > 0 {
                            if next2_kanji_count > 29 ||
                               self.kanji_map.get(&next2_kanji_count).is_none() { continue; }
                            first_name_writing_count_vec.push(next2_kanji_count);
                        }
                        // println!("next_kanji_count:{:?}, {:?}", next_kanji_count,next2_kanji_count);
                        first_name_candidate_vec.push(first_name_writing_count_vec.to_vec());
                    }
                }
            }
        }
        let mut candidate_vec = Vec::new();
        for first_name_candidate in first_name_candidate_vec {
            candidate_vec.push(NameCounts::create("", last_name, &first_name_candidate,
                                                  &last_name_writing_count_vec, is_male, self));
        }
        candidate_vec.sort_by(|a, b| a.cmp_by_total_point(b).reverse());
        candidate_vec
    }
    pub fn get_name_candidates(&self,
                               last_name: &str,
                               ti_un: u32,
                               is_male: bool,
                               offset: u32,
                               limit: u32)
                               -> Vec<NameCounts> {
        let point_vec: Vec<NameCounts> = self.get_lucky_point_candidates("長野", true);
        let mut candidate_vec = Vec::new();
        let mut name_index = 0u32;
        // println!("ti_un:{}", ti_un);
        for point_candidate in point_vec {
            // println!("name_index:{}, offset{}, limit{}", name_index, offset, limit);
            if name_index >= (offset + limit) { break; }
            if point_candidate.ti_un != ti_un { continue; }
            let kanji_vec =
                match self.kanji_map.get(&point_candidate.first_name_writing_count_vec[0]) {
                Some(m) => m,
                None => {
                    println!("Not supported count {}",
                             &point_candidate.first_name_writing_count_vec[0]);
                    continue;
                }
            };
            // println!("kanji_vec size:{}", kanji_vec.len());
            for first_kanji in kanji_vec {
                if name_index >= (offset + limit) { break; }
                if point_candidate.first_name_writing_count_vec.len() > 1 {
                    let second_kanji_vec = match self.kanji_map.get(
                        &point_candidate.first_name_writing_count_vec[1]) {
                        Some(m) => m,
                        None => {
                            println!("Not supported count {}",
                                     &point_candidate.first_name_writing_count_vec[1]);
                            continue;
                        }
                    };
                    // println!("second_kanji_vec size:{}", second_kanji_vec.len());
                    for second_kanji in second_kanji_vec {
                        if name_index >= (offset + limit) { break; }
                        if point_candidate.first_name_writing_count_vec.len() > 2 {
                            let third_kanji_vec = match self.kanji_map.get(
                                &point_candidate.first_name_writing_count_vec[2]) {
                                Some(m) => m,
                                None => {
                                    println!("Not supported count {}",
                                             &point_candidate.first_name_writing_count_vec[2]);
                                    continue;
                                }
                            };
                            for third_kanji in third_kanji_vec {
                                if name_index >= (offset + limit) { break; }
                                name_index += 1;
                                if name_index < offset {
                                    continue;
                                }
                                let first_name =
                                    format!("{}{}{}", first_kanji, second_kanji, third_kanji);
                                candidate_vec.push(NameCounts::new(&first_name, last_name, is_male,
                                                                   self).unwrap())
                            }
                        } else {
                            name_index += 1;
                            if name_index < offset {
                                continue;
                            }
                            let first_name = format!("{}{}", first_kanji, second_kanji);
                            candidate_vec.push(NameCounts::new(&first_name, last_name, is_male,
                                                               self).unwrap())
                        }
                    }
                } else {
                    name_index += 1;
                    if name_index < offset {
                        continue;
                    }
                    let first_name = format!("{}", first_kanji);
                    candidate_vec.push(NameCounts::new(&first_name, last_name, is_male, self).unwrap())
                }
            }
        }
        candidate_vec
    }
}

#[test]
fn it_works() {
    let kanji_path = "c:/work/rust01/rust-service-test/characters.csv".to_string();
    let lucky_path = "c:/work/rust01/rust-service-test/lucky.csv".to_string();
    let kanji_manager = NameKanjiManager::load_csv(&kanji_path, &lucky_path);
    println!("{:?}", kanji_manager.writing_count_map.get(&'林'));
    assert_eq!(*kanji_manager.writing_count_map.get(&'林').unwrap(), 8);
    assert_eq!(kanji_manager.get_writing_count("小林"), 11);
    let name_counts = NameCounts::new("元彦", "長野", true, &kanji_manager);
    println!("{:?}", name_counts);
    println!("{:?}", kanji_manager.get_name_counts("力也", "長野", true));
    println!("{:?}", kanji_manager.get_name_counts("亜希子", "谷村", false));
    println!("{:?}", kanji_manager.get_lucky_point_candidates("長野", true).len());
    println!("{:?}", kanji_manager.get_name_candidates("長野", 5, true, 0, 1).len());
    // エラーにすると println が表示される
    assert_eq!(1, 2);
}
