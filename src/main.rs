use std::env;

fn main() {
    match env::var("HOME") {
        Ok(var) => log::info!("$HOME: {}", var),
        Err(e) => log::error!("{}", e),
    }
    println!("Hello, world!");
}
