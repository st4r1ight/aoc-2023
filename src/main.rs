use std::env;
mod one;

fn main() {
    let args = env::args().collect::<Vec<_>>();
    match args[1].as_str() {
        "1" => one::do_tasks(),
        _ => println!("Haven't done that one yet"),
    }
}
