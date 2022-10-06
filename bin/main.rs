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
    check("examples/type_4.rl"); // parse error
    check("examples/type_5.rl"); // duplicate error

    check("examples/skillset_1.rl");

    check("examples/data_1.rl");
    check("examples/data_2.rl");
    check("examples/data_3.rl");
    check("examples/data_4.rl"); // duplicate error
    check("examples/data_5.rl"); // resolve error

    check("examples/resource_1.rl");
    check("examples/resource_2.rl");
    check("examples/resource_3.rl"); // duplicate error (resource)
    check("examples/resource_4.rl"); // duplicate error (state)
}
