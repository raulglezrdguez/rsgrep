use std::env;

fn main() {
    let args = env::args().collect::<Vec<String>>();

    dbg!(args);
}
