extern crate redis;

use redis::{Commands, Value};

use json_generator::{
    read_json_from_file,
    print_divider,
    save_to_redis,
    read_from_redis,
    get_connection
};

fn main() {
    println!("STRING");
    print_divider();
    
    let json_data = read_json_from_file();
    print_divider();

    let mut conn = get_connection();

    let data = serde_json::to_string(&json_data).unwrap();

    save_to_redis(
        String::from("Сохранение всего json как строки под ключом 'json' без использования pipeline"),
        &data,
        1,
        |data: String| {
            let _: () = conn.set("json", &data).unwrap();
        }
    );
    print_divider();

    read_from_redis(
        String::from("Читение всего json без использования pipeline"),
        1,
        || {
            conn.get::<&str, Value>("json").unwrap();
        }
    );
    print_divider();

}
