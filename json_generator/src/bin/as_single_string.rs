extern crate redis;

use redis::Commands;

use json_generator::{
    read_json_from_file,
    print_divider,
    save_to_redis,
    read_from_redis
};

fn main() {
    let json_data = read_json_from_file();
    print_divider();

    let client = redis::Client::open("redis://localhost:6379").unwrap();
    let mut conn = client.get_connection().unwrap();
    let data = serde_json::to_string(&json_data).unwrap();

    save_to_redis(
        String::from("Сохраняем весь json как строку под ключом 'json'"),
        &data,
        1,
        |data: String| {
            let _: () = conn.set("json", &data).unwrap();
        }
    );
    read_from_redis(
        String::from("Читаем json из redis"),
        1,
        || -> String {
            conn.get("json").unwrap()
        }
    );
    print_divider();
}
