use rl::*;
use std::env;

fn check(file: &str) {
    println!(
        "-------------------------[{}]-------------------------",
        file
    );
    let mut model = Model::empty();
    match process_file(&mut model, file) {
        Ok(_) => println!("{}", model),
        _ => {}
    }
}

fn main() {
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "info")
    }
    // pretty_env_logger::init();
    env_logger::init();

    check("examples/type_1.rl");
    check("examples/type_2.rl");
    check("examples/type_3.rl");

    check("examples/skillset_1.rl");

    check("examples/data_1.rl");
    check("examples/data_2.rl");
    check("examples/data_3.rl");

    check("examples/resource_1.rl");
    check("examples/resource_2.rl");

    check("examples/event_1.rl");
}
