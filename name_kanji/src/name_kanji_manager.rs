extern crate rustc_serialize;
use rustc_serialize::*;
use std::result::*;
use std::cmp::Ordering;
use lucky_manager::*;
use kanji_manager::*;

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
           is_new_count: bool,
           kanji_manager: &NameKanjiManager)
           -> Result<NameCounts, Vec<char>> {
        let mut last_name_writing_count_vec = Vec::new();
        let mut first_name_writing_count_vec = Vec::new();
        let mut err_char_vec = Vec::new();
        for c in last_name.chars() {
            match kanji_manager.kanji_manager.get_writing_count_of_kanji(&c, is_new_count) {
                Some(m) => {
                    if m == 0 {
                        println!("kanji not supported.{:?}", &c);
                        err_char_vec.push(c);
                    } else {
                        last_name_writing_count_vec.push(m);
                    }
                },
                None => {
                    println!("kanji not supported.{:?}", &c);
                    err_char_vec.push(c);
                }
            }
        }
        for c in first_name.chars() {
            match kanji_manager.kanji_manager.get_writing_count_of_kanji(&c, is_new_count) {
                Some(m) => {
                    if m == 0 {
                        println!("kanji not supported.{:?}", &c);
                        err_char_vec.push(c);
                    } else {
                        first_name_writing_count_vec.push(m);
                    }
                },
                None => {
                    println!("kanji not supported.{:?}", &c);
                    err_char_vec.push(c);
                }
            }
        }
        if err_char_vec.len() != 0 {
            return Err(err_char_vec);
        }
        Ok(NameCounts::create(first_name, last_name, &first_name_writing_count_vec,
                              &last_name_writing_count_vec, is_male, kanji_manager))
    }

    fn cmp_by_total_point(&self, target: &NameCounts) -> Ordering {
        if self.total_point < target.total_point {
            return Ordering::Less;
        } else if self.total_point > target.total_point {
            return Ordering::Greater;
        } else {
            return Ordering::Equal;
        }
    }
}

#[derive(Debug)]
#[derive(RustcDecodable)]
// 名前に使える漢字の管理
pub struct NameKanjiManager {
    lucky_manager: LuckyManager,
    kanji_manager: KanjiManager
}

impl NameKanjiManager {
    pub fn load_csv(kanji_csv_path: &str,
                    hiragana_csv_path: &str,
                    lucky_csv_path: &str)
                    -> NameKanjiManager {
        NameKanjiManager {
            lucky_manager: LuckyManager::load_csv(lucky_csv_path),
            kanji_manager: KanjiManager::load_csv(kanji_csv_path, hiragana_csv_path),
        }
    }

    pub fn get_name_counts(&self,
                           first_name: &str,
                           last_name: &str,
                           is_male: bool,
                           is_new_count: bool)
                           -> Result<NameCounts, Vec<char>> {
        NameCounts::new(first_name, last_name, is_male, is_new_count, self)
    }

    pub fn get_lucky_point_candidates(&self,
                                      last_name: &str,
                                      is_male: bool,
                                      is_new_count: bool)
                                      -> Vec<NameCounts> {
        // println!("last_name:{:?}", last_name);
        // println!("is_male:{:?}", is_male);
        // 名字の各漢字の画数取得
        let mut last_name_writing_count_vec = Vec::new();
        for c in last_name.chars() {
            match self.kanji_manager.get_writing_count_of_kanji(&c, is_new_count) {
                Some(m) => last_name_writing_count_vec.push(m),
                None => println!("Not supported:{:?}", &c)
            }
        }
        // 吉運になる数を取得
        let lucky_vec: Vec<u32> = self.lucky_manager.get_lucky_writing_count_vec(is_male);

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
            //人運画数は名字の最後の文字の画数より大きい
            if jin_un <= last_kanji_count {
                continue;
            }
            let kanji_count = jin_un - last_kanji_count;
            // 名前の最初の文字の画数が人運で決まる。
            if self.kanji_manager.get_kanji_vec(kanji_count, is_new_count, true)
                   .is_none() { continue; }
            // println!("jin_un:{}", jin_un);
            let mut first_name_writing_count_vec = Vec::new();
            first_name_writing_count_vec.push(kanji_count);
            // println!("kanji_count:{:?}", kanji_count);
            for ti_un in lucky_vec.to_vec() {
                // 最初の文字より小さい地運はスキップ
                if ti_un < kanji_count { continue };
                // if self.lucky_manager.get_lucky_point(ti_un, is_male) < 1 { continue; }
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
                        if self.kanji_manager.get_kanji_vec(next_kanji_count, is_new_count, true)
                               .is_none() { continue; }
                        first_name_writing_count_vec.truncate(1);
                        first_name_writing_count_vec.push(next_kanji_count);
                        let next2_kanji_count = ti_un - kanji_count - next_kanji_count;
                        if next2_kanji_count > 0 {
                            if self.kanji_manager.get_kanji_vec(next2_kanji_count, is_new_count,
                                                                true).is_none() { continue; }
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
                               is_new_count: bool,
                               is_include_kana: bool,
                               kanji_num: u32,
                               offset: u32,
                               limit: u32)
                               -> Vec<NameCounts> {
        let point_vec: Vec<NameCounts> =
            self.get_lucky_point_candidates(last_name, is_male, is_new_count);
        let mut candidate_vec = Vec::new();
        let mut name_index = 0u32;
        // println!("ti_un:{}", ti_un);
        for point_candidate in point_vec {
            if kanji_num != 0 &&
               point_candidate.first_name_writing_count_vec.len() as u32 != kanji_num { continue; }
            // println!("name_index:{}, offset{}, limit{}", name_index, offset, limit);
            if name_index >= (offset + limit) { break; }
            if point_candidate.ti_un != ti_un { continue; }
            let kanji_vec = match self.kanji_manager.get_kanji_vec(
                point_candidate.first_name_writing_count_vec[0], is_new_count, is_include_kana) {
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
                    let second_kanji_vec = match self.kanji_manager.get_kanji_vec(
                        point_candidate.first_name_writing_count_vec[1], is_new_count,
                        is_include_kana) {
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
                            let third_kanji_vec = match self.kanji_manager.get_kanji_vec(
                                point_candidate.first_name_writing_count_vec[2], is_new_count,
                                is_include_kana) {
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
                                                                   is_new_count, self).unwrap())
                            }
                        } else {
                            name_index += 1;
                            if name_index < offset {
                                continue;
                            }
                            let first_name = format!("{}{}", first_kanji, second_kanji);
                            candidate_vec.push(NameCounts::new(&first_name, last_name, is_male,
                                                               is_new_count, self).unwrap())
                        }
                    }
                } else {
                    name_index += 1;
                    if name_index < offset {
                        continue;
                    }
                    let first_name = format!("{}", first_kanji);
                    candidate_vec.push(NameCounts::new(&first_name, last_name, is_male,
                                                       is_new_count, self).unwrap())
                }
            }
        }
        candidate_vec
    }
}

#[test]
fn it_works() {
    let kanji_path = "c:/work/rust01/rust-service-test/kanji.csv".to_string();
    let hiragana_path = "c:/work/rust01/rust-service-test/hiragana.csv".to_string();
    let lucky_path = "c:/work/rust01/rust-service-test/lucky.csv".to_string();
    let kanji_manager = NameKanjiManager::load_csv(&kanji_path, &hiragana_path, &lucky_path);
    println!("{:?}", kanji_manager.kanji_manager.get_writing_count_of_kanji(&'林'));
    assert_eq!(*kanji_manager.kanji_manager.get_writing_count_of_kanji(&'林').unwrap(), 8);
    assert_eq!(kanji_manager.kanji_manager.get_writing_count_of_name("小林"), 11);
    let name_counts = NameCounts::new("元彦", "長野", true, &kanji_manager);
    println!("{:?}", name_counts);
    println!("{:?}", kanji_manager.get_name_counts("力也", "長野", true));
    println!("{:?}", kanji_manager.get_name_counts("亜希子", "谷村", false));
    println!("{:?}", kanji_manager.get_lucky_point_candidates("長野", true).len());
    println!("{:?}", kanji_manager.get_name_candidates("長野", 5, true, 0, 1).len());
    // エラーにすると println が表示される
    assert_eq!(1, 2);
}
