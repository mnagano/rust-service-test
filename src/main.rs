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
use urlencoded::UrlEncodedBody;
use urlencoded::UrlEncodedQuery;

fn root_handler(_: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok, "top")))
}

fn api_handler(_: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok, "api")))
}

fn name_handler(req: &mut Request) -> IronResult<Response> {
    let kanji_manager =
        NameKanjiManager::load_csv(&"./characters.csv".to_string(), &"./lucky.csv".to_string());
    let query_map = req.get_ref::<UrlEncodedQuery>().unwrap();
    println!("{:?}", query_map);
    if (query_map.contains_key("last_name")
        && query_map.contains_key("first_name")){
        return Ok(Response::with((status::Ok,
            json::encode(&kanji_manager.get_name_counts(
                query_map.get("last_name").unwrap().get(0).unwrap(),
                query_map.get("first_name").unwrap().get(0).unwrap(),
                match query_map.get("is_male") {
                    Some(m) => m.get(0).unwrap() != "false" && m.get(0).unwrap() != "0",
                    None => true
                }
            )).unwrap())));
    }else{
        let count: u32 =
        kanji_manager.get_writing_count(query_map.get(&"last_name".to_string()).unwrap().get(0).unwrap());
        return Ok(Response::with((status::Ok, format!("count:{}", count))));
    }
}

fn api_post_handler(req: &mut Request) -> IronResult<Response> {
    match req.get_ref::<UrlEncodedBody>() {
        Ok(ref hashmap) => println!("Parsed POST request body:\n {:?}", hashmap),
        Err(ref e) => println!("{:?}", e)
    };
    Ok(Response::with((status::Ok, "api_post")))
}

fn query_handler(req: &mut Request) -> IronResult<Response> {
    println!("{:?}", &req);
    match req.get_ref::<UrlEncodedQuery>() {
        Ok(ref hashmap) => println!("Parsed GET request query:\n {:?}", hashmap),
        Err(ref e) => println!("{:?}", e)
    };
    let query_params = req.get_ref::<UrlEncodedQuery>().unwrap();
    println!("{:?}", query_params);
    // let mut query = req.extensions.get::<Router>().unwrap().find("query").unwrap_or("/");
    Ok(Response::with((status::Ok, format!("query"))))
}

fn main() {
    let logger = Logger::new(None);
    let mut router = Router::new();

    router.get("/", root_handler);
    router.get("/api", api_handler);
    router.get("/names", name_handler);
    router.post("/api", api_post_handler);
    router.get("/:query", query_handler);
    let mut chain = Chain::new(router);
    chain.link(logger);
    Iron::new(chain).http("127.0.0.1:4000").unwrap();
}