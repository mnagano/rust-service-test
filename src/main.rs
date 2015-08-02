#[macro_use]
extern crate lazy_static;

extern crate iron;
extern crate router;
extern crate logger;
extern crate urlencoded;
extern crate name_kanji;
extern crate rustc_serialize;

use name_kanji::name_kanji_manager::*;
use rustc_serialize::json;
use iron::*;
use router::*;
use logger::Logger;
use urlencoded::UrlEncodedQuery;

lazy_static!{
    static ref KANJI_MANAGER: NameKanjiManager = {
        NameKanjiManager::load_csv("./kanji.csv", "./hiragana.csv", "./lucky.csv")
    };
}

fn root_handler(_: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok, "top")))
}

fn api_handler(_: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok, "api")))
}

fn name_handler(req: &mut Request) -> IronResult<Response> {
    let result = req.get_ref::<UrlEncodedQuery>();
    if result.is_err() {
        return Ok(Response::with((status::NotFound, "no query")));
    }
    let query_map = result.unwrap();
    if !query_map.contains_key("last_name") {
        return Ok(Response::with((status::NotFound, "last_name")));
    }
    let last_name = query_map.get("last_name").unwrap().get(0).unwrap();
    if last_name.len() == 0 {
        return Ok(Response::with((status::NotFound, "last_name")));
    }
    if !query_map.contains_key("is_male") {
        return Ok(Response::with((status::NotFound, "is_male")));
    }
    let is_male = match query_map.get("is_male") {
        Some(m) => m.get(0).unwrap() != "false" && m.get(0).unwrap() != "0",
        None => true
    };

    let is_new_count = match query_map.get("is_new_count") {
        Some(m) => m.get(0).unwrap() != "false" && m.get(0).unwrap() != "0",
        None => true
    };

    // 名前を含む場合は姓名診断
    if query_map.contains_key("first_name") {
        let name_count_result =
            KANJI_MANAGER.get_name_counts(query_map.get("first_name").unwrap().get(0).unwrap(),
                                          last_name, is_male, is_new_count);
        if name_count_result.is_err() {
            return Ok(Response::with((status::NotFound,
                                      json::encode(&name_count_result.err().unwrap()).unwrap())));
        }
        return Ok(Response::with((status::Ok,
                                  json::encode(&name_count_result.unwrap()).unwrap())));
        // 名前を含まず画数を含む場合は画数での名前リスト検索
    } else if query_map.contains_key("writing_count") {
        let writing_count_option =
            query_map.get("writing_count").unwrap().get(0).unwrap().parse::<u32>();
        if writing_count_option.is_err() {
            return Ok(Response::with((status::BadRequest, "writing_count")));
        }
        let writing_count = writing_count_option.unwrap();
        if writing_count < 1 || writing_count > 81 {
            return Ok(Response::with((status::BadRequest, "writing_count")));
        }
        let offset = match query_map.get("offset") {
            Some(m) => m.get(0).unwrap().parse::<u32>().unwrap_or(0),
            None => 0
        };
        let limit = match query_map.get("limit") {
            Some(m) => m.get(0).unwrap().parse::<u32>().unwrap_or(200),
            None => 200
        };
        if offset > 100000 {
            return Ok(Response::with((status::BadRequest, "offset")));
        }
        if limit > 2000 {
            return Ok(Response::with((status::BadRequest, "limit")));
        }
        let is_include_kana = match query_map.get("is_include_kana") {
            Some(m) => m.get(0).unwrap() != "false" && m.get(0).unwrap() != "0",
            None => true
        };
        let kanji_num = match query_map.get("kanji_num") {
            Some(m) => m.get(0).unwrap().parse::<u32>().unwrap_or(2),
            None => 2
        };
        return Ok(Response::with((status::Ok, json::encode(&KANJI_MANAGER.get_name_candidates(
                                                 last_name, writing_count, is_male, is_new_count,
                                                 is_include_kana, kanji_num, offset, limit)).unwrap())));
        // 名前を含まず画数を含む場合は画数での名前リスト検索
    } else {
        let count_list_result =
            KANJI_MANAGER.get_lucky_point_candidates(last_name, is_male, is_new_count);
        return Ok(Response::with((status::Ok, json::encode(&count_list_result).unwrap())));
    }
}

// fn api_post_handler(req: &mut Request) -> IronResult<Response> {
//     match req.get_ref::<UrlEncodedBody>() {
//         Ok(ref hashmap) => println!("Parsed POST request body:\n {:?}", hashmap),
//         Err(ref e) => println!("{:?}", e)
//     };
//     Ok(Response::with((status::Ok, "api_post")))
// }

fn main() {
    let logger = Logger::new(None);
    let mut router = Router::new();
    router.get("/", root_handler);
    router.get("/api", api_handler);
    router.get("/names", name_handler);
    // router.post("/api", api_post_handler);
    let mut chain = Chain::new(router);
    chain.link(logger);
    Iron::new(chain).http("127.0.0.1:4000").unwrap();
}