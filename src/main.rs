use std::fs::File;
use std::io::prelude::*;
use serde::{Deserialize, Serialize};

mod swc_test;

fn main() {
    let mut config_file: File = File::open("Routes.toml").expect("File not found");
    let mut config_string: String = String::new();
    config_file.read_to_string(&mut config_string).expect("Failed to read file");

    let config: Config = toml::from_str(&config_string).unwrap();

    let mut json_file = File::create("Routes.json").expect("Failed to create file");
    let json_string = serde_json::to_string_pretty(&config).unwrap();
    json_file.write_all(json_string.as_bytes()).expect("Failed to write file");
    
    let example_module = swc_test::modulate("example.ts");
    let example_js = swc_test::generate(example_module.0, example_module.1).unwrap();

    let mut example_js_file = File::create("example_edit.ts").expect("Failed to create file");
    example_js_file.write_all(example_js.as_bytes()).expect("Failed to write file");
}


#[derive(Deserialize, Serialize)]
struct Config {
    routes: Vec<Route>,
}

#[derive(Deserialize, Serialize)]
struct Route {
    name: String,
    page: Option<String>,
    controller: Option<String>,
    guard: Option<String>,
    routes: Option<Vec<Route>>,
}