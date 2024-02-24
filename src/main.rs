use std::io::Read;

use toml::Value;

fn main() {
    let mut content = String::new();
    std::io::stdin().read_to_string(&mut content).unwrap();
    println!(
        "{}",
        serde_json::to_string(&content.parse::<Value>().unwrap()).unwrap()
    );
}
