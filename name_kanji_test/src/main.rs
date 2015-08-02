extern crate name_kanji;
use name_kanji::name_kanji_manager::*;

fn main() {
    let kanji_path = "c:/work/rust01/rust-service-test/name_kanji/characters.csv".to_string();
    let lucky_path = "c:/work/rust01/rust-service-test/lucky.csv".to_string();
    let kanji_manager = NameKanjiManager::load_csv(&kanji_path, &lucky_path);
    // println!("{:?}", kanji_manager.get_writing_count(&"小林".to_string()));
    // assert_eq!(kanji_manager.get_writing_count(&"小林".to_string()), 11);
    // println!("{:?}", kanji_manager.get_name_counts("元彦", "長野", true));
    println!("{:?}", kanji_manager.get_lucky_point_candidates("長野", true).len());
}
