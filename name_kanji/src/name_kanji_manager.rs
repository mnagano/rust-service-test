extern crate csv;
extern crate rustc_serialize;
use std::collections::HashMap;
use csv::*;
use rustc_serialize::*;
use rustc_serialize::json::{Json, Parser};

#[derive(Debug)]
#[derive(RustcDecodable)]
pub struct NameKanjiManager {
    kanji_map: HashMap<u32, char>,
    writing_count_map: HashMap<char, u32>,
    lucky_manager: LuckyManager,
}

#[derive(Debug)]
#[derive(RustcDecodable)]
pub struct LuckyManager {
    male_map: HashMap<u32, i32>,
    female_map: HashMap<u32, i32>,
}

#[derive(Debug)]
#[derive(RustcDecodable, RustcEncodable)]
pub struct NameCounts {
    is_male: bool,
    first_name: String,
    last_name: String,
    last_name_writing_count_vec : Vec<u32>,
    first_name_writing_count_vec : Vec<u32>,
    ten_un: u32,
    jin_un: u32,
    ti_un: u32,
    gai_un : u32,
    sou_un : u32,
    in_myou_point : i32,
    ten_un_point : i32,
    jin_un_point : i32,
    gai_un_point : i32,
    ti_un_point : i32,
    sou_un_point : i32,
    total_point : i32,
}


#[derive(RustcDecodable)]
struct KanjiRecord {
    kanji: char,
    writing_count: u32,
}

#[derive(Debug)]
#[derive(RustcDecodable)]
struct LuckyRecord{
    writing_count: u32,
    male_point: i32,
    female_point : i32
}

impl NameCounts {
    fn new(first_name: &str, last_name: &str, is_male: bool, kanji_manager: &NameKanjiManager) -> NameCounts {
        let mut last_name_writing_count_vec = Vec::new();
        let mut first_name_writing_count_vec = Vec::new();
        for c in last_name.chars(){
            last_name_writing_count_vec.push(kanji_manager.get_writing_count_kanji(&c))
        }
        for c in first_name.chars(){
            first_name_writing_count_vec.push(kanji_manager.get_writing_count_kanji(&c))
        }
        let mut jin_un = last_name_writing_count_vec[last_name_writing_count_vec.len() - 1] +
        first_name_writing_count_vec[0];
        let mut ten_un = last_name_writing_count_vec.iter().fold(0, |sum, x| sum + x);
        let mut ti_un = first_name_writing_count_vec.iter().fold(0, |sum, x| sum + x);
        let mut gai_un:u32 = 0u32;
        let last_name_len = last_name_writing_count_vec.len();
        if (last_name_len > 2) {
            gai_un += last_name_writing_count_vec[1];
        }
        gai_un += last_name_writing_count_vec[0];
        let first_name_len = first_name_writing_count_vec.len();
        if (first_name_len > 2) {
            gai_un += first_name_writing_count_vec[first_name_len - 2];
        }
        gai_un += first_name_writing_count_vec[first_name_len - 1];
        let mut sou_un = ten_un + ti_un;
        let mut in_myou_point:i32 = ((first_name_writing_count_vec[first_name_writing_count_vec.len()-1] + last_name_writing_count_vec[0]) % 2) as i32;
        let mut ten_un_point = kanji_manager.lucky_manager.get_lucky_point(ten_un, is_male);
        let mut ti_un_point = kanji_manager.lucky_manager.get_lucky_point(ti_un, is_male);
        let mut jin_un_point = kanji_manager.lucky_manager.get_lucky_point(jin_un, is_male);
        let mut gai_un_point = kanji_manager.lucky_manager.get_lucky_point(gai_un, is_male);
        let mut sou_un_point = kanji_manager.lucky_manager.get_lucky_point(sou_un, is_male)
        ;
        let mut total_point = ten_un_point + ti_un_point + jin_un_point + gai_un_point +
                              sou_un_point
                              + in_myou_point;

        NameCounts {
                is_male: is_male,
                first_name: first_name.to_string(),
                last_name: last_name.to_string(),
                last_name_writing_count_vec: last_name_writing_count_vec,
                first_name_writing_count_vec: first_name_writing_count_vec,
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
}

impl LuckyManager {
    pub fn load_csv(csv_path: &str) -> LuckyManager {
        let mut lucky_manager = LuckyManager {
            male_map: HashMap::new(),
            female_map: HashMap::new()
        };
        let mut reader = csv::Reader::from_file(csv_path).unwrap();
        for record in reader.decode() {
            let record: LuckyRecord = record.unwrap();
            lucky_manager.male_map.insert(record.writing_count, record.male_point);
            lucky_manager.female_map.insert(record.writing_count, record.female_point);
        }
        lucky_manager
    }

    pub fn get_lucky_point(&self, writing_count: u32, is_male: bool) -> i32 {
        if (is_male){
            return *self.male_map.get(&writing_count).unwrap();
        }else{
            return *self.female_map.get(&writing_count).unwrap();
        }
    }
}

impl NameKanjiManager {
    pub fn load_csv(kanji_csv_path: &str, lucky_csv_path : &str) -> NameKanjiManager {
        let mut kanji_manager = NameKanjiManager {
            kanji_map: HashMap::new(),
            writing_count_map: HashMap::new(),
            lucky_manager : LuckyManager::load_csv(lucky_csv_path)
        };
        let mut reader = csv::Reader::from_file(kanji_csv_path).unwrap();
        for record in reader.decode() {
            let record: KanjiRecord = record.unwrap();
            kanji_manager.kanji_map.insert(record.writing_count, record.kanji);
            kanji_manager.writing_count_map.insert(record.kanji, record.writing_count);
        }
        kanji_manager
    }

    pub fn get_writing_count_kanji(&self, kanji:&char) -> u32 {
        *self.writing_count_map.get(&kanji).unwrap()
    }

    pub fn get_writing_count(&self, last_name:&str) -> u32 {
        let mut count = 0u32;
        for kanji in last_name.chars() {
            count += self.get_writing_count_kanji(&kanji);
        }
        count
    }

    pub fn get_name_counts(&self, first_name:&str, last_name:&str, is_male:bool) -> NameCounts {
        NameCounts::new(first_name, last_name, is_male, self)
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
  println!("{:?}", kanji_manager.get_name_counts("元彦", "長野", true));
  println!("{:?}", kanji_manager.get_name_counts("亜希子", "谷村", false));
  assert_eq!(1, 2);
}

