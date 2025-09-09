mod config;

use config::load_config;

fn main() {
    let _config = load_config();

    println!("Starting up...");
}
